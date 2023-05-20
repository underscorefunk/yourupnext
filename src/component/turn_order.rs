/// # Turn Order Component

use crate::prelude::*;

pub type TurnOrder = Vec<Id>;

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
            return Err("Can not add turn to nonexistant or nonscenario entity".to_string());
        }

        if !entity::qry::exists(&state, turn_entity_pub_id) {
            return Err("Can not add nonexitant entity to a scenario's turn order".to_string());
        }

        let scenario_id = scenario::qry::id(&state, scenario_pub_id);
        let mut sequence = state.turn_order.get(scenario_id).unwrap_or_default();
        let entity_id = entity::qry::id(&state, turn_entity_pub_id);

        sequence.push(entity_id);

        state.turn_order.update( scenario_id, sequence );

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
    pub fn remove_turn(mut state: State, scenario_pub_id: PubId, turn_entity_pub_id: PubId ) -> CmdResult<State>{

        if !scenario::qry::exists(&state, scenario_pub_id) {
            return Err("Can not remove turn for nonexistant or nonscenario entity".to_string());
        }

        if !entity::qry::exists(&state, turn_entity_pub_id) {
            return Err("Can not remove nonexitant entity to a scenario's turn order".to_string());
        }

        let scenario_id = scenario::qry::id(&state, scenario_pub_id);
        let mut sequence = state.turn_order.get(scenario_id).unwrap_or_default();
        let entity_id = entity::qry::id(&state, turn_entity_pub_id);

        if ! sequence.contains(&entity_id) {
            return Err("Can not remove turn that isn't in the turn order".to_string() );
        }

        sequence.retain(|sequenced_id| sequenced_id != &entity_id );
        state.turn_order.update( scenario_id, sequence );

        Ok(state)

    }


    // COMMAND > Reorder turn

    // COMMAND > Set (add multiple and replace)
}

pub mod qry {
    use super::*;

    /// QUERY > Get the sequence of Ids that expresses the order of turns
    pub fn sequence(state: &State, scenario_pub_id: PubId) -> TurnOrder {
        let id = entity::qry::id(state, scenario_pub_id);
        let ids = state.turn_order.get(id).unwrap_or_default();
        entity::qry::pub_ids(state, ids)
    }
}