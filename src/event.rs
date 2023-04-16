use crate::scenario;
use crate::player;
use crate::entity;
use crate::round;
use crate::effect;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub scenario: scenario::State,
    pub player: player::State,
    pub entity: entity::State,
    pub round: round::State,
    pub effect: effect::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            scenario: scenario::State::default(),
            player: player::State::default(),
            entity: entity::State::default(),
            round: round::State::default(),
            effect: effect::State::default()
        }
    }
}


pub type ActionError = String;
pub type ActionResult = Result<State, ActionError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    /// A non action that serves as an identity
    None,

    /// Initialize a state
    Init,

    /// A grouping of actions that can be run as a set
    Proc(Vec<Action>),

    // Scenario
    RenameScenario(scenario::Name),

    // Player
    AddPlayer(player::Name),
    RenamePlayer(player::Id, player::Name),
    RemovePlayer(player::Id),

    // Entity
    AddEntity(entity::Name),
    RenameEntity(entity::Id, entity::Name),
    RemoveEntity(entity::Id),

    // Round
    AddTurn(entity::Id, round::Initiative),
    RemoveTurn(entity::Id),
    OrderTurnsByInitiative,
    UpdateTurn(entity::Id, round::TurnStatus),
    MoveTurn(entity::Id, i8),
    MoveTurnBefore(entity::Id, entity::Id),

    ResetTurn(entity::Id),
    ActivateTurn(entity::Id),
    InterruptTurn(entity::Id),
    ActivateDelayedTurn(entity::Id, entity::Id),
    ResumeTurn(entity::Id),
    CompleteTurn(entity::Id),
    SkipTurn(entity::Id),
    DelayTurn(entity::Id),
    TiggerDelayedTurn(entity::Id, usize),
    NextRound,
}

impl Action {
    pub fn apply(self, state: State) -> Result<State, ActionError> {
        match self {

            // State and Actions
            Action::None => Ok(state),
            Action::Init => Ok(State::default()),
            Action::Proc(actions) => Self::apply_all(actions.clone(), state),

            // Scenario
            Action::RenameScenario(name) => scenario::rename(state, name),

            // Player
            Action::AddPlayer(name) => player::add(state, name),
            Action::RenamePlayer(player_id, player_name) => player::rename(state, player_id, player_name),
            // @todo — Remove player needs to remove entities associated with it
            Action::RemovePlayer(player_id) => player::remove(state, player_id),

            // Entity
            Action::AddEntity(entity_name) => entity::add(state, entity_name),
            Action::RenameEntity(entity_id, entity_name) => entity::rename(state, entity_id, entity_name),
            // @todo — Remove entity needs to remove turns associated with it
            Action::RemoveEntity(entity_id) => entity::remove(state, entity_id),

            // Round
            Action::AddTurn(entity_id, initiative) => round::add_turn(state, entity_id, initiative),
            // @todo — Remove turn needs to remove effects associated with it that are
            //          flagged as bound to the entity life cycle (short lived)
            Action::RemoveTurn(entity_id) => round::remove_turn(state, entity_id),
            Action::OrderTurnsByInitiative => round::order_turns_by_initiative(state),
            Action::MoveTurn(entity_id, offset) => round::update_turn_order(state, entity_id, offset),
            Action::MoveTurnBefore(entity_id, before_entity_id) => round::move_turn_before(state, entity_id, before_entity_id),

            // Turn States/Status
            Action::UpdateTurn(entity_id, turn_status) => round::update_turn_state(state, entity_id, turn_status),
            Action::ResetTurn(entity_id) => round::update_turn_state(state, entity_id, round::TurnStatus::Available),
            Action::InterruptTurn(entity_id) => round::update_turn_state(state, entity_id, round::TurnStatus::Paused),
            Action::ResumeTurn(entity_id) => round::update_turn_state(state, entity_id, round::TurnStatus::Active),
            Action::ActivateTurn(entity_id) => round::update_turn_state(state, entity_id, round::TurnStatus::Active),
            Action::ActivateDelayedTurn(entity_id, triggering_entity_id) => round::activate_delayed_turn(state, entity_id, triggering_entity_id),
            Action::CompleteTurn(entity_id) => round::update_turn_state(state, entity_id, round::TurnStatus::Completed),
            Action::SkipTurn(entity_id) => round::update_turn_state(state, entity_id, round::TurnStatus::Skipped),
            Action::DelayTurn(entity_id) => {
                Action::apply_all(vec![
                    // Add held action marker
                    Action::UpdateTurn(entity_id, round::TurnStatus::Held(0))
                ], state)
            }
            Action::TiggerDelayedTurn(entity_id, triggering_entity_id) => {
                Action::apply_all(vec![
                    // Todo — Clear held action marker
                    Action::InterruptTurn(triggering_entity_id),
                    Action::MoveTurnBefore(entity_id, triggering_entity_id),
                    Action::ActivateDelayedTurn(entity_id, triggering_entity_id),
                ], state)
            }


            // Action::NextTurn => round::activate_next_turn(state),
            Action::NextRound => round::next_round(state),
            // Action::ActivateDelayedTurn(entity_id) => Ok(state)
        }
    }

    pub fn apply_all(actions: Vec<Action>, state: State) -> Result<State, ActionError> {
        actions
            .into_iter()
            .fold(
                Ok(state),
                |state, action| {
                    match state {
                        Ok(state) => action.apply(state),
                        Err(action_error) => Err(action_error)
                    }
                },
            )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_action() {
        assert_eq!(
            Action::Init.apply(State::default()),
            Ok(State::default())
        );
        assert_eq!(
            Action::apply_all(vec![Action::Init], State::default()),
            Ok(State::default())
        );
    }
}