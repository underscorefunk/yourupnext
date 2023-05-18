/// # Name Component

use crate::prelude::*;

pub type Name = str;

/// ## Name > Command Applicables (Cmd)
/// A simple wrapper for entity commands so that they can be composed together with other pipelines.
/// `Cmd` is a facade for `cmd` functions.
pub enum Cmd {
    Set(PubId, &'static Name),
}

impl Applicable for Cmd {
    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {
            Cmd::Set(pub_id,name) => cmd::set(state, pub_id, name),
        }
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to( State::default() )
    }
}

/// ## Name > Command (cmd)

pub mod cmd {
    use super::*;

    /// COMMAND > Set the `Name` of an entity
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = Entity::Add(100).apply_to_default().unwrap();
    /// let renamed_state = name::Cmd::Set( 100, "AName").apply_to(state).unwrap();
    /// assert_eq!(name::qry::get(&renamed_state,100), "AName")
    /// ```
    pub fn set(mut state: State, entity_pub_id: PubId, new_name: &'static Name) -> CmdResult<State> {
        let id = entity::qry::id( &state, entity_pub_id);
        state.name.update(id, new_name.to_string())?;
        Ok(state)
    }
}

/// ## Name > Query (qry)

pub mod qry {
    use super::*;

    /// QUERY > Get the `Name` of an entity
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = Entity::Add(100).apply_to_default().unwrap();
    /// let state = name::Cmd::Set(100, "AName").apply_to(state).unwrap();
    ///
    /// assert_eq!(name::qry::get(&state,100), "AName".to_string() );
    /// ```
    pub fn get(state: &State, entity_pub_id: PubId) -> String {
        let id = entity::qry::id( state, entity_pub_id);
        state.name.get(id).unwrap_or_default()
    }
}