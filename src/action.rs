use crate::prelude::*;

pub type ActionError = String;
pub type ActionResult<ResultOk> = Result<ResultOk, ActionError>;

pub trait Applicable<Action> {
    fn apply(self, state: State) -> ActionResult<State>;
    fn apply_default(self) -> ActionResult<State>;
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

impl <Action: Applicable<Action>> Applicable<Action> for Vec<Action> {
    fn apply(self, state: State) -> ActionResult<State> {
        apply_actions(self, state)
    }
    fn apply_default(self) -> ActionResult<State> {
        self.apply( State::default() )
    }
}

