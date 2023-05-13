use crate::prelude::*;

pub type ActionError = String;
pub type CommandResult<ResultOk> = Result<ResultOk, ActionError>;

pub trait Applicable {
    fn apply(self, state: State) -> CommandResult<State>;
    fn apply_default(self) -> CommandResult<State>;
}

impl <T: Applicable> Applicable for Vec<T> {
    fn apply(self, state: State) -> CommandResult<State> {
        self.into_iter()
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
    fn apply_default(self) -> CommandResult<State> {
        self.apply( State::default() )
    }
}

