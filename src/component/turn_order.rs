/// # Turn Order Component
/// Todo - Add "Supported EntityTypes"

use crate::prelude::*;

pub type TurnOrder = Vec<Id>;

#[derive(Debug, Eq, PartialEq)]
pub enum TurnPosition {
    Start,
    Before(PubId),
    After(PubId),
    End,
}


pub mod cmd {
    use super::*;

    /// COMMAND > Add a turn to the order of turns for a scenario
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let scenario_pub_id = 100;
    /// let state = State::default()
    ///         .apply( Scenario::Add(scenario_pub_id) )
    ///         .apply( Character::Add(200, "ACharacter") )
    ///         .apply( Character::Add(300, "BCharacter") )
    ///         .apply( |state| turn_order::cmd::add_turn(state, scenario_pub_id, 200))
    ///         .apply( |state| turn_order::cmd::add_turn(state, scenario_pub_id, 300))
    ///         .unwrap();
    ///
    /// assert_eq!(turn_order::qry::sequence(&state, scenario_pub_id), vec![200, 300] );
    /// ```
    pub fn add_turn(mut state: State, scenario_pub_id: PubId, turn_entity_pub_id: PubId) -> CmdResult<State> {
        if !scenario::qry::exists(&state, scenario_pub_id) {
            return Err("Can not add turn to nonexistant or nonscenario entity".into());
        }

        if !qry::is_supported_turn_order_type(&state, turn_entity_pub_id) {
            return Err("Can not add a turn for an unsupported entity type".into());
        }

        let scenario_id = scenario::qry::id(&state, scenario_pub_id);
        let mut sequence = state.turn_order.get(scenario_id).unwrap_or_default();
        let entity_id = entity::qry::id(&state, turn_entity_pub_id);

        sequence.push(entity_id);

        state.turn_order.update(scenario_id, sequence);

        Ok(state)
    }

    /// COMMAND > Remove turn
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let scenario_pub_id = 100;
    /// let state = State::default()
    ///         .apply( Scenario::Add(scenario_pub_id) )
    ///         .apply( Character::Add(200, "ACharacter") )
    ///         .apply( Character::Add(300, "BCharacter") )
    ///         .apply( |state| turn_order::cmd::add_turn(state, scenario_pub_id, 200))
    ///         .apply( |state| turn_order::cmd::add_turn(state, scenario_pub_id, 300))
    ///         .apply( |state| turn_order::cmd::remove_turn(state, scenario_pub_id, 200))
    ///         .unwrap();
    ///
    /// assert_eq!(turn_order::qry::sequence(&state, scenario_pub_id), vec![300] );
    /// ```
    pub fn remove_turn(mut state: State, scenario_pub_id: PubId, turn_entity_pub_id: PubId) -> CmdResult<State> {
        if !scenario::qry::exists(&state, scenario_pub_id) {
            return Err("Can not remove turn for nonexistant or nonscenario entity".into());
        }

        if !qry::is_supported_turn_order_type(&state, turn_entity_pub_id) {
            return Err("Can not remove a turn for an unsupported entity type".into());
        }

        let scenario_id = scenario::qry::id(&state, scenario_pub_id);
        let mut sequence = state.turn_order.get(scenario_id).unwrap_or_default();
        let entity_id = entity::qry::id(&state, turn_entity_pub_id);

        if !sequence.contains(&entity_id) {
            return Err("Can not remove turn that isn't in the turn order".into());
        }

        sequence.retain(|sequenced_id| sequenced_id != &entity_id);
        state.turn_order.update(scenario_id, sequence);

        Ok(state)
    }

    /// COMMAND > Move a turn in a turn order
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let scenario_pub_id = 100;
    ///
    /// let state = State::default()
    ///         .apply( Scenario::Add(scenario_pub_id) )
    ///         .apply( Character::Add(200, "ACharacter") )
    ///         .apply( |state| turn_order::cmd::add_turn(state, scenario_pub_id, 200))
    ///         .apply( Character::Add(300, "BCharacter") )
    ///         .apply( |state| turn_order::cmd::add_turn(state, scenario_pub_id, 300))
    ///         .apply( Character::Add(400, "CCharacter") )
    ///         .apply( |state| turn_order::cmd::add_turn(state, scenario_pub_id, 400))
    ///         .apply( Character::Add(500, "DCharacter") )
    ///         .apply( |state| turn_order::cmd::add_turn(state, scenario_pub_id, 500))
    ///         .unwrap();
    ///
    /// assert_eq!(
    ///     turn_order::qry::sequence(&state, scenario_pub_id),
    ///     vec![200, 300, 400, 500]
    /// );
    ///
    /// let state_c_first = state.clone()
    ///     .apply(|state|turn_order::cmd::move_turn(state, scenario_pub_id, 400, TurnPosition::Start) )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     turn_order::qry::sequence(&state_c_first, scenario_pub_id),
    ///     vec![400, 200, 300, 500]
    /// );
    ///
    /// let state_c_last = state.clone()
    ///     .apply(|state|turn_order::cmd::move_turn(state, scenario_pub_id, 400, TurnPosition::End) )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     turn_order::qry::sequence(&state_c_last, scenario_pub_id),
    ///     vec![200, 300, 500, 400]
    /// );
    ///
    /// let state_c_before_b = state.clone()
    ///     .apply(|state|turn_order::cmd::move_turn(state, scenario_pub_id, 400, TurnPosition::Before(300)) )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     turn_order::qry::sequence(&state_c_before_b, scenario_pub_id),
    ///     vec![200, 400, 300, 500 ]
    /// );
    ///
    /// let state_c_after_a = state.clone()
    ///     .apply(|state|turn_order::cmd::move_turn(state, scenario_pub_id, 400, TurnPosition::After(200)) )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     turn_order::qry::sequence(&state_c_after_a, scenario_pub_id),
    ///     vec![200, 400, 300, 500 ]
    /// );
    ///
    /// let state_c_before_a = state.clone()
    ///     .apply(|state|turn_order::cmd::move_turn(state, scenario_pub_id, 400, TurnPosition::Before(200)) )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     turn_order::qry::sequence(&state_c_before_a, scenario_pub_id),
    ///     vec![400, 200, 300, 500 ]
    /// );
    ///
    /// let state_c_after_d = state.clone()
    ///     .apply(|state|turn_order::cmd::move_turn(state, scenario_pub_id, 400, TurnPosition::After(500)) )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     turn_order::qry::sequence(&state_c_after_d, scenario_pub_id),
    ///     vec![200, 300, 500, 400 ]
    /// );
    /// ```
    pub fn move_turn(
        mut state: State,
        scenario_pub_id: PubId,
        turn_entity_pub_id: PubId,
        position: TurnPosition,
    ) -> CmdResult<State> {
        if !scenario::qry::exists(&state, scenario_pub_id) {
            return Err("Can not move a turn in a nonexistant or nonscenario entity".into());
        }

        if !qry::is_supported_turn_order_type(&state, turn_entity_pub_id) {
            return Err("Can not move a turn for an unsupported entity type".into());
        }

        if !qry::contains(&state, scenario_pub_id, turn_entity_pub_id) {
            return Err("You can not move an entity's turn that doesn't exist in a sequence of turns".into());
        }

        let mut sequence = qry::sequence(&state, scenario_pub_id);

        if sequence.len() == 1 {
            return Ok(state);
        }

        let turn_entity_index = match sequence.iter().position(|&x| x == turn_entity_pub_id) {
            Some(index) => index,
            None => return Err("Unable to find the index of the entity you were trying to move".into())
        };

        sequence.remove(turn_entity_index);

        let sequence = match position {
            TurnPosition::Start => {
                sequence.insert(0, turn_entity_pub_id);
                sequence
            }

            TurnPosition::End => {
                sequence.push(turn_entity_pub_id);
                sequence
            }

            TurnPosition::Before(anchor_entity_pub_id) => {
                if anchor_entity_pub_id == turn_entity_pub_id {
                    return Err("Can not move entity in turn order relative to itself".into());
                }
                let anchor_entity_id = match sequence.iter().position(|&x| x == anchor_entity_pub_id) {
                    Some(index) => index,
                    None => return Err("Unable to find anchor entity to place a turn before or after".into())
                };
                sequence.insert(anchor_entity_id, turn_entity_pub_id);
                sequence
            }

            TurnPosition::After(anchor_entity_pub_id) => {
                if anchor_entity_pub_id == turn_entity_pub_id {
                    return Err("Can not move entity in turn order relative to itself".into());
                }
                let anchor_entity_id = match sequence.iter().position(|&x| x == anchor_entity_pub_id) {
                    Some(index) => index + 1,
                    None => return Err("Unable to find anchor entity to place a turn before or after".into())
                };
                sequence.insert(anchor_entity_id, turn_entity_pub_id);
                sequence
            }
        };

        set(state, scenario_pub_id, sequence)
    }

    /// COMMAND > Set (add multiple and replace)
    ///
    pub fn set(mut state: State, scenario_pub_id: PubId, turn_order: TurnOrder) -> CmdResult<State> {
        let scenario_id = entity::qry::id(&state, scenario_pub_id);
        let turn_order_ids = entity::qry::ids(&state, turn_order);
        state.turn_order.update(scenario_id, turn_order_ids);
        Ok(state)
    }


}

pub mod qry {
    use super::*;

    /// QUERY > Check if a sequence of turns contains a specific turn
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let a_scenario_pub_id = 100;
    /// let b_scenario_pub_id = 200;
    /// let state = State::default()
    ///         .apply( Scenario::Add(a_scenario_pub_id) )
    ///         .apply( Character::Add(300, "ACharacter") )
    ///         .apply( |state| turn_order::cmd::add_turn(state, a_scenario_pub_id, 300))
    ///         .apply( Scenario::Add(b_scenario_pub_id) )
    ///         .apply( Character::Add(400, "BCharacter") )
    ///         .apply( |state| turn_order::cmd::add_turn(state, b_scenario_pub_id, 400))
    ///         .unwrap();
    ///
    /// assert_eq!(turn_order::qry::contains(&state, a_scenario_pub_id, 300), true );
    /// assert_eq!(turn_order::qry::contains(&state, b_scenario_pub_id, 300), false );
    /// ```
    pub fn contains(state: &State, scenario_pub_id: PubId, entity_pub_id: PubId) -> bool {
        sequence(state, scenario_pub_id).contains(&entity_pub_id)
    }

    /// QUERY > Get the sequence of Public Ids that expresses the order of turns
    pub fn sequence(state: &State, scenario_pub_id: PubId) -> Vec<PubId> {
        let id = entity::qry::id(state, scenario_pub_id);
        let ids = state.turn_order.get(id).unwrap_or_default();
        entity::qry::pub_ids(state, ids)
    }


    /// QUERY > Get valid support types that can have a turn order
    pub fn is_supported_turn_order_type(state: &State, entity_pub_id: PubId) -> bool {
        match entity_type::qry::get(state, entity_pub_id) {
            EntityType::Player => false,
            EntityType::Scenario => false,
            EntityType::Missing => false,
            EntityType::Character => true,
            EntityType::Item => true,
            EntityType::Location => true,
            EntityType::Effect => true,
            EntityType::Generic => true,
        }
    }
}

