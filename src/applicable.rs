/// # Applicable
///
/// One of the goals of this crate is to explore techniques that can add
/// flexibility and simplicity to a code base. This is important because
/// role playing game tools require flexibility. It's not uncommon to need
/// to amend or modify an activity retroactively as well.
///
/// At the highest level, the pattern used in this crate involves a few
/// simple steps.
///
/// 1) An action is submitted to the application
/// 2a) The action is applied to the State resulting in a new updated state.
/// 2b) The action has a component that throws an error, and the state
///     is not updated.
///
/// The following conventions are used:
///
/// - A *command* requires mutable access to state and will return a Result
///   with an Ok new state or Err with the error message.
/// - A *query* requires a reference to state, should never mutate state, and
///   returns a value.
/// - Process is converted into data by using enums to store *command args* as
///   enum values.
///
/// This module handles the specification (and some implementations) of how
/// a command is applied to state with the `Applicable` trait.
///
/// The following are examples of Applicable in use.
///
///
/// Style 1: Cmd + State = Result
/// ----------------------------
/// 1) Command (Enum Variant): impl Applicable on an enum of commands.
///    The apply_to() implementation will match on `self` (the enum variant)
///    and will return a matched function call with the variant's
///    destructured values.
///
///    `
///     // impl
///     match self {
///         Cmd::TakeDamage(pub_id, dmg) => Character::Damage(state, pub_id, dmg)
///         ...
///     }
///
///     // in use
///     let new_state: CmdResult<State> = Cmd::TakeDamage(10, 62).apply_to(state);
///     // or
///     let new_state: CmdResult<State> = Cmd::TakeDamage(10, 62).apply_to_default();
///    `
///
///
/// 2) Vectors of Commands: The `Applicable` trait is implemented on a Vec of Applicables.
///
///    `
///    // in use
///    vec![ Cmd::TakeDamage, Cmd::ShoutHealthLevel ].apply_to(state);
///    // or
///     vec![ Cmd::TakeDamage, Cmd::ShoutHealthLevel ].apply_to_default();
///    `
///    Note that a trait is required if we wish to add additional implementations to
///    types defined in other crates, like Vec<T: Applicable>
///
///
/// Style 2: State + Cmd = Result
/// ----------------------------
/// 3) State command chaining: Use the apply() method on a `State`. CmdResult implements
///    a single use trait to implement apply() as well. Allowing command of any type to be
///    chained together.
///
///    `
///      let state = State::default()
///        .apply( Player::Add(1,"APlayer".to_string()))
///         .apply( Character::Add(2,"ACharacter".to_string()));
///     `

use crate::prelude::*;

pub type CmdErr = String;
pub type CmdResult<ResultOk> = Result<ResultOk, CmdErr>;

pub trait Applicable {
    fn apply_to(self, state: State) -> CmdResult<State>;
    fn apply_to_default(self) -> CmdResult<State>;
}

/// Allow a vector of things that can be applied
/// to be applied directly from the vector.
impl<T: Applicable> Applicable for Vec<T> {
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
        self.apply_to(State::default())
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
    pub fn apply<T: Applicable>(self, command: T) -> CmdResult<State> {
        command.apply_to(self)
    }
}

/// Specify a single use trait so that we can add impl blocks to types
/// defined outside of this crate.
pub trait ApplicableChainable {
    fn apply<T: Applicable>(self, command: T) -> CmdResult<State>;
}

/// Implement our single use trait for orphan implementations
impl ApplicableChainable for CmdResult<State> {
    /// Used to chain results from applying commands.
    /// See impl State { apply() } for an example.
    fn apply<T: Applicable>(self, command: T) -> CmdResult<State> {
        match self {
            Ok(state) => state.apply(command),
            Err(_) => self
        }
    }
}


/// Implement closures that match the command format as commands
/// They're not serializable but are useful when creating procesing
/// pipelines.
impl <F: FnOnce(State)->CmdResult<State> > Applicable for F {

    /// ```
    /// use yourupnext::prelude::*;
    /// let state = State::default()
    ///     .apply( |state| entity::cmd::add(state, 100) )
    ///     .unwrap();
    ///
    /// assert!( entity::qry::exists(&state, 100));
    ///
    /// ```
    fn apply_to(self, state: State) -> CmdResult<State> {
        self(state)
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self(State::default() )
    }
}