/// # Scenario Model
/// A scenario is a structural idea that holds a topic or focus of play.
/// You can think of a campaign as a series of scenarios. Scenarios are
/// entities and as such can be nested or assigned to another scenario.
///
/// There must always be one root scenario.
///
/// ## Structure
/// Scenarios are a bucket that contain associated `Entities`, which
/// populate the scenario. Entities may or may not have turns in the
/// scenario, e.g. they may be associated but purely as context.
/// A scenario may be nested by being assigned to another scenario.
///
/// ## (Free/Active/Completed) Scenarios
/// A scenario has a TurnState to indicate if it is active in
/// *sequenced play (seq_play)*
///
/// The `SeqPlay` is responsible for handling actual turn/round progression.
/// This module is intended to handle the association aspect.
///
/// ## Todo
/// - Assignment of a character to one scenario should remove them from a
///   previous scenario.
/// - Review hierarchical "assignment" of nested scenarios
/// - Confirm that another scenario capturing an entity will safely move it
///   if required.
///
use crate::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Scenario {
    // Base scenario
    Add(PubId),
    Remmove(PubId),
    Rename(PubId, &'static Name),
    Describe(PubId, &'static Name),

    CaptureEntity(PubId, PubId),
    ReleaseEntity(PubId),
    ReleaseAllEntities(PubId),
}

impl Applicable for Scenario {
    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {
            Scenario::Add(pub_id) => cmd::add(state, pub_id),
            Scenario::Remmove(pub_id) => cmd::remove(state, pub_id),
            Scenario::Rename(pub_id, name) => cmd::rename(state, pub_id, name),
            Scenario::Describe(pub_id, description) => cmd::describe(state, pub_id, description),

            Scenario::CaptureEntity(pub_id, entity_pub_id) => {
                cmd::assign_entity(state, pub_id, entity_pub_id)
            }
            Scenario::ReleaseEntity(entity_pub_id) => {
                cmd::release_entity(state, entity_pub_id)
            }
            Scenario::ReleaseAllEntities(pub_id) => cmd::release_all_entities(state, pub_id),
        }
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to(State::default())
    }
}

pub mod cmd {
    use super::*;

    /// COMMAND > Start a scenario
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id: PubId = 100;
    ///
    /// let state = State::default()
    ///     .apply( Scenario::Add( pub_id ) )
    ///     .unwrap();
    ///
    /// assert!(scenario::qry::exists(&state,pub_id));
    ///
    /// ```
    pub fn add(state: State, scenario_pub_id: PubId) -> CmdResult<State> {
        state
            .apply(Entity::Add(scenario_pub_id))
            .apply(Entity::Classify(scenario_pub_id, EntityType::Scenario))
    }

    /// COMMAND > End a scenario (remove)
    /// See Entity::Remove for tests
    pub fn remove(state: State, scenario_pub_id: PubId) -> CmdResult<State> {
        Entity::Remove(scenario_pub_id).apply_to(state)
    }

    /// COMMAND > Rename a scenario
    /// See Entity::Name for tests
    pub fn rename(
        state: State,
        scenario_pub_id: PubId,
        new_name: &'static Name,
    ) -> CmdResult<State> {
        Entity::Name(scenario_pub_id, new_name).apply_to(state)
    }

    /// COMMAND > Describe the scenario
    /// See Entity::Description for tests
    pub fn describe(
        state: State,
        scenario_pub_id: PubId,
        desc: &'static Description,
    ) -> CmdResult<State> {
        Entity::Describe(scenario_pub_id, desc).apply_to(state)
    }

    /// COMMAND > Assign a character to a scenario
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let scenario_pub_id = 100;
    /// let character_pub_id = 200;
    ///
    /// let state = State::default()
    ///     .apply( Scenario::Add(scenario_pub_id) )
    ///     .apply( Character::Add(character_pub_id,"ACharacter"))
    ///     .apply( Scenario::CaptureEntity(scenario_pub_id,character_pub_id))
    ///     .unwrap();
    ///
    /// assert_eq!(scenario::qry::find_character(&state,character_pub_id), Some(100));
    ///
    /// ```
    pub fn assign_entity(
        mut state: State,
        scenario_pub_id: PubId,
        character_pub_id: PubId,
    ) -> CmdResult<State> {
        if !entity_type::qry::is(&state, scenario_pub_id, EntityType::Scenario) {
            return Err(
                "Can not assign character to scenario when the target scenario isn't a scenario."
                    .to_string(),
            );
        }

        if !entity_type::qry::is(&state, character_pub_id, EntityType::Character) {
            return Err("Can not assign character to scenario when the subject character isn't a character.".to_string());
        }

        let scenario_id = entity::qry::id(&state, scenario_pub_id);
        let character_id = entity::qry::id(&state, character_pub_id);

        state
            .character_scenario
            .set_parent(character_id, scenario_id)?;

        Ok(state)
    }

    /// COMMAND > Remove a character from any scenario
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let scenario_pub_id = 100;
    /// let character_pub_id = 200;
    ///
    /// let state = State::default()
    ///     .apply( Scenario::Add(scenario_pub_id) )
    ///     .apply( Character::Add(character_pub_id, "ACharacter") )
    ///     .apply( Scenario::CaptureEntity(scenario_pub_id, character_pub_id) )
    ///     .unwrap();
    /// assert_eq!(scenario::qry::find_character(&state,character_pub_id), Some(100));
    ///
    /// let state = state
    ///     .apply( Scenario::ReleaseEntity(character_pub_id))
    ///     .unwrap();
    /// assert_eq!( character::qry::player(&state, character_pub_id), None);
    /// ```
    pub fn release_entity(mut state: State, character_pub_id: PubId) -> CmdResult<State> {
        if !entity_type::qry::is(&state, character_pub_id, EntityType::Character) {
            return Err(
                "Can not remove character from scenario with non character entity".to_string(),
            );
        }

        let character_id = entity::qry::id(&state, character_pub_id);

        if !state.character_scenario.is_child(character_id) {
            return Err("Can not release a character from a scenario when the character is not assigned to a scenario.".to_string());
        }

        state.character_scenario.remove_parent(character_id)?;

        Ok(state)
    }

    /// COMMAND > Remove/drain all characters from a scenario
    pub fn release_all_entities(mut state: State, scenario_pub_id: PubId) -> CmdResult<State> {
        Ok(state)
    }
}

pub mod qry {
    use super::*;

    /// QUERY > Check if a scenario exists
    /// See `entity_type` component for tests
    pub fn exists(state: &State, scenario_pub_id: PubId) -> bool {
        entity_type::qry::is(state, scenario_pub_id, EntityType::Scenario)
    }

    /// QUERY > Get a scenario's `Id`
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id = 100;
    /// let state = State::default()
    ///         .apply( Scenario::Add(pub_id))
    ///         .unwrap();
    ///
    /// assert_eq!(scenario::qry::id(&state, pub_id), 1);
    ///
    /// let nonexistant_pub_id: PubId = 200;
    /// assert_eq!(scenario::qry::id(&state, nonexistant_pub_id), 0);
    ///
    /// ```
    pub fn id(state: &State, scenario_pub_id: PubId) -> Id {
        match exists(state, scenario_pub_id) {
            true => entity::qry::id(state, scenario_pub_id),
            false => 0,
        }
    }

    /// QUERY > Get a scenario's `name` as String
    /// See `name` component for tests
    pub fn name(state: &State, scenario_pub_id: PubId) -> String {
        name::qry::get(state, scenario_pub_id)
    }

    /// QUERY > Get a scenario's `description` as String
    /// See `description` component for tests
    pub fn description(state: &State, scenario_pub_id: PubId) -> String {
        description::qry::get(state, scenario_pub_id)
    }

    /// QUERY > Get a character's `scenario` assignment if any
    pub fn find_character(state: &State, character_pub_id: PubId) -> Option<PubId> {
        let character_id = entity::qry::id(state, character_pub_id);
        let scenario_id = state.character_scenario.parent(character_id).unwrap_or(0);
        entity::qry::pub_id(state, scenario_id)
    }
}
