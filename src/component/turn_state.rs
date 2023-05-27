/// # Turn State Component
/// Scenarios can have a turn state that dictates which mode they're in.
/// Free, Active, or Completed. This allows us to cycle
/// through all of the sequences and then all of the turns.
///

use crate::prelude::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TurnStatus {
    Free,
    Available,
    Active,
    Paused,
    Completed,
    Skipped,
    Held(usize),
    None
}

pub mod cmd {
    use super::*;

    /// COMMAND > Set the state of an entity's turn
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply( Entity::Add(100) )
    ///     .apply(|state|turn_state::cmd::set(state,100,TurnStatus::Available))
    ///     .unwrap();
    ///
    /// assert_eq!(turn_state::qry::get(&state, 100), TurnStatus::Available);
    ///
    /// let state = state
    ///     .apply(|state|turn_state::cmd::set(state,100, TurnStatus::None))
    ///     .unwrap();
    ///
    /// assert_eq!( turn_state::qry::get(&state, 100), TurnStatus::None );
    /// assert!( state.turn_state.is_empty() );
    /// ```
    pub fn set(mut state: State, pub_id: PubId, turn_state: TurnStatus) -> CmdResult<State> {

        let id = entity::qry::id(&state, pub_id);

        // None types shouldn't be stored
        if turn_state == TurnStatus::None {
            state.turn_state.delete(id);
            return Ok(state);
        }

        state.turn_state.update(id, turn_state)?;
        Ok(state)
    }
}

pub mod qry {
    use super::*;

    /// QUERY > Get the state of an entity's turn
    pub fn get(state: &State, pub_id: PubId) -> TurnStatus {
        let id = entity::qry::id(state, pub_id);
        if !state.registry.has_id(&id) || id == 0 {
            return TurnStatus::None;
        }
        match state.turn_state.get(id) {
            Some(turn_state) => turn_state,
            _ => TurnStatus::None
        }
    }
}