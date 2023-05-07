pub mod entity;

use crate::event;
use entity::Collection;

/// The effect system is an event driven generic data store
/// that operates as an independent subsystem of the app.
///
/// Entities are registered with the effect system.

pub type ActionError = event::ActionError;
pub type ActionResult = Result<State, ActionError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    entities: Collection
}

impl Default for State {
    fn default() -> Self {
        Self {
            entities: Collection::default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    /// A non action that serves as an identity
    None,

    /// Initialize a state
    Init,

    AddEntity( entity::Ref ),
    RemoveEntity( entity::Id )

}


impl Action {

    pub fn apply(self, state: State) -> ActionResult {
        match self {
            Action::None => Ok(state),
            Action::Init => Ok(State::default()),
            Action::AddEntity( entity_ref ) => entity::add( state, entity_ref ),
            Action::RemoveEntity( id ) => entity::remove( state, id )
        }
    }

    pub fn apply_all(actions: Vec<Action>, state: State) -> ActionResult {
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