/// # Sequenced Play
///
/// IMPORTANT: Consider the graph structure of turn order
/// and how it can be flattened to a single sequence of play
/// by walking the graph and flattening it.
///
/// TODO ... Consider renaming turn play
use crate::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum SeqPlay {
    AddTurn(ScenarioId, EntityId),
    RemoveTurn(EntityId),
    Enable(ScenarioId),
    Disable(ScenarioId),
}

impl Applicable for SeqPlay {
    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {
            SeqPlay::AddTurn(scenario_id, entity_id) => {
                cmd::add_turn(state, scenario_id, entity_id)
            }
            SeqPlay::RemoveTurn(entity_id) => cmd::remove_turn(state, entity_id),
            SeqPlay::Enable(scenario_id) => cmd::enable(state, scenario_id),
            SeqPlay::Disable(scenario_id) => cmd::disable(state, scenario_id),
        }
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to(State::default())
    }
}

/// ## Sequenced Play Commands

pub mod cmd {
    use super::*;

    /// COMMAND > Add a turn to sequenced play
    ///
    /// The new turn will receive a starting TurnState based on the scenario's TurnState.
    ///
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply( Scenario::Add(100) )
    ///     .apply( Character::Add(200, "ACharacter"))
    ///     .apply(|state|seq_play::cmd::add_turn(state, 100, 200));
    ///
    /// assert!(state.is_ok());
    /// assert_eq!(turn_state::qry::get(&state.unwrap(), 200), TurnStatus::Free );
    /// ```
    pub fn add_turn(
        mut state: State,
        scenario_id: ScenarioId,
        entity_id: EntityId,
    ) -> CmdResult<State> {
        if !scenario::qry::exists(&state, scenario_id) {
            return Err("Can not add turn to non existent scenario".into());
        }
        if scenario::qry::exists(&state, entity_id) {
            return Err("Can not add turn for scenario entity.".into());
        }

        let turn_state = qry::new_turn_init_state(&state, scenario_id);

        state
            .apply(|state| scenario::cmd::assign_entity(state, scenario_id, entity_id))
            .apply(|state| turn_order::cmd::add_turn(state, scenario_id, entity_id))
            .apply(|state| turn_state::cmd::set(state, entity_id, turn_state))
    }

    /// COMMAND > Remove a turn from sequenced play
    ///
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply( Scenario::Add(100) )
    ///     .apply( Character::Add(200, "ACharacter"))
    ///     .apply(|state|seq_play::cmd::add_turn(state, 100, 200))
    ///     .apply(|state|seq_play::cmd::remove_turn(state,200));
    ///
    /// assert!(state.is_ok());
    /// let state = state.unwrap();
    /// assert_eq!(turn_state::qry::get(&state, 200), TurnStatus::None );
    /// ```
    pub fn remove_turn(mut state: State, entity_id: EntityId) -> CmdResult<State> {
        let scenario_id = scenario::qry::find_entity(&state, entity_id);
        if scenario_id.is_none() {
            return Err("Unable to remove turn for entity that isn't in a scenario".into());
        }
        let scenario_id = scenario_id.unwrap();
        state
            .apply(|state| turn_order::cmd::remove_turn(state, scenario_id, entity_id))
            .apply(|state| turn_state::cmd::set(state, entity_id, TurnStatus::None))
    }

    /// Command > Trigger sequenced play mode for a scenario and the entities that have turns
    ///
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply(
    ///         Scenario::Add(50)
    ///     ).apply_with(
    ///         vec![(100,"A"),(200,"B"),(300,"C")],
    ///         |(character_id, name)| Character::Add(character_id, name)
    ///     ).apply_with(
    ///         vec![(50,100),(50,200),(50,300)],
    ///         |(scenario_id, character_id)| SeqPlay::AddTurn(scenario_id, character_id)
    ///     ).apply(
    ///         SeqPlay::Enable(50)
    ///     );
    ///
    /// assert!(state.is_ok() );
    /// let state = state.unwrap();
    /// assert_eq!(turn_state::qry::get(&state, 50), TurnStatus::Active);
    /// assert_eq!(turn_state::qry::get(&state, 100), TurnStatus::Available)
    /// ```
    pub fn enable(mut state: State, scenario_id: ScenarioId) -> CmdResult<State> {
        if turn_state::qry::get(&state, scenario_id) != TurnStatus::Free {
            return Err(
                "Unable to enter sequenced play for scenario that isn't in free play".into(),
            );
        }
        let turns = turn_order::qry::sequence(&state, scenario_id);

        state
            .apply(|state| turn_state::cmd::set(state, scenario_id, TurnStatus::Active))
            .apply_with(turns, |turn| {
                move |state| turn_state::cmd::set(state, turn, TurnStatus::Available)
            })
    }

    /// COMMAND > Trigger free play mode for a scenario
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply(
    ///         Scenario::Add(50)
    ///     ).apply_with(
    ///         vec![(100,"A"),(200,"B"),(300,"C")],
    ///         |(character_id, name)| Character::Add(character_id, name)
    ///     ).apply_with(
    ///         vec![(50,100),(50,200),(50,300)],
    ///         |(scenario_id, character_id)| SeqPlay::AddTurn(scenario_id, character_id)
    ///     ).apply(
    ///         SeqPlay::Enable(50)
    ///     ).apply(
    ///         SeqPlay::Disable(50)
    ///     );
    /// assert!(state.is_ok() );
    /// let state = state.unwrap();
    /// assert_eq!(turn_state::qry::get(&state, 50), TurnStatus::Free);
    /// assert_eq!(turn_state::qry::get(&state, 100), TurnStatus::Free)
    /// ```
    pub fn disable(mut state: State, scenario_id: ScenarioId) -> CmdResult<State> {
        if turn_state::qry::get(&state, scenario_id) == TurnStatus::Free {
            return Err(
                "Unable to enter free play mode for scenario that is already in free play".into(),
            );
        }

        let turns = turn_order::qry::sequence(&state, scenario_id);

        state
            .apply(|state| turn_state::cmd::set(state, scenario_id, TurnStatus::Free))
            .apply_with(turns, |turn| {
                move |state| turn_state::cmd::set(state, turn, TurnStatus::Free)
            })
    }
}

pub mod qry {
    use super::*;

    /// QUERY > Get the turn state of new turns added to a scenario
    pub fn new_turn_init_state(state: &State, scenario_id: ScenarioId) -> TurnStatus {
        match turn_state::qry::get(state, scenario_id) {
            TurnStatus::Free => TurnStatus::Free,
            _ => TurnStatus::Available,
        }
    }
}
