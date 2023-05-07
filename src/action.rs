use crate::prelude::*;

pub type ActionError = String;
pub type ActionResult<ResultOk> = Result<ResultOk, ActionError>;

pub trait Applicable<Action> {
    fn apply(self, state: State) -> ActionResult<State>;
    fn apply_all(actions: Vec<Action>, state: State) -> ActionResult<State>;
}

pub fn apply_actions<Action: Applicable<Action>>
(actions: Vec<Action>, state: State) -> ActionResult<State> {
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