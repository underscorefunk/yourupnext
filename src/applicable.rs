use crate::prelude::*;

pub type CmdErr = String;
pub type CmdResult<ResultOk> = Result<ResultOk, CmdErr>;

pub trait Applicable {
    fn apply_to(self, state: State) -> CmdResult<State>;
    fn apply_to_default(self) -> CmdResult<State>;
}

/// Allow a vector of things that can be applied
/// to be applied directly from the vector.
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


impl State {
    /// A `State` has the ability to have `Applicable` commands run on it.
    /// This is achieved with `state.apply( some_command/applicable )`.
    /// We also implement apply() on its return type so that apply() calls
    /// can be chained. Best of all, the `Applicables` don't have to be the
    /// same type like they do with vec![].apply(). Rust is so lovely. ❤️🦀
    ///
    /// The trait `ApplicableChainable` needs to be exported and imported
    /// as part of the prelude for this to work.
    ///
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply( Player::Add(1,"APlayer".to_string()))
    ///     .apply( Character::Add(2,"ACharacter".to_string()));
    ///
    /// assert!( character::qry::exists(&state, 2) );
    ///
    /// ```
    ///
    pub fn apply<T: Applicable> (self, command: T) -> CmdResult<State> {
        command.apply_to(self)
    }
}

pub trait ApplicableChainable {
    fn apply<T: Applicable>(self, command: T) -> CmdResult<State>;
}

impl ApplicableChainable for CmdResult<State> {
    fn apply <T: Applicable>(self, command: T) -> CmdResult<State> {
        match self {
            Ok(state) => state.apply(command),
            Err(_) => self
        }
    }
}