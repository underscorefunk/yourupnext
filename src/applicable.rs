use crate::prelude::*;

pub type CmdErr = String;
pub type CmdResult<ResultOk> = Result<ResultOk, CmdErr>;

pub trait Applicable {
    fn apply_to(self, state: State) -> CmdResult<State>;
    fn apply_to_default(self) -> CmdResult<State>;
}

impl <T: Applicable> Applicable for Vec<T> {
    fn apply_to(self, state: State) -> CmdResult<State> {
        self.into_iter()
            .fold(
                Ok(state),
                |state, action| {
                    match state {
                        Ok(state) => action.apply_to(state),
                        Err(action_error) => Err(action_error)
                    }
                },
            )
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to( State::default() )
    }
}
