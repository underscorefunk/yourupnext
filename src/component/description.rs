/// # Description Component

use crate::prelude::*;

pub type Description = str;

/// ## Description > Command Applicables (Cmd)
/// A simple wrapper for entity commands so that they can be composed together with other pipelines.
/// `Cmd` is a facade for `cmd` functions.
pub enum Cmd {
    Set(PubId, &'static Description),
}

impl Applicable for Cmd {
    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {
            Cmd::Set(pub_id,description) => cmd::set(state, pub_id, description),
        }
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to( State::default() )
    }
}

/// ## Description > Command (cmd)

pub mod cmd {
    use super::*;

    /// COMMAND > Set the `Description` of an entity
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = Entity::Add(100).apply_to_default().unwrap();
    /// let descriptiond_state = description::Cmd::Set( 100, "ADescription").apply_to(state).unwrap();
    /// assert_eq!(description::qry::get(&descriptiond_state,100), "ADescription")
    /// ```
    pub fn set(mut state: State, entity_pub_id: PubId, new_description: &'static Description) -> CmdResult<State> {
        let id = entity::qry::id( &state, entity_pub_id);
        state.description.update(id, new_description.to_string())?;
        Ok(state)
    }
}

/// ## Description > Query (qry)

pub mod qry {
    use super::*;

    /// QUERY > Get the `Description` of an entity
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = Entity::Add(100).apply_to_default().unwrap();
    /// let state = description::Cmd::Set(100, "ADescription").apply_to(state).unwrap();
    ///
    /// assert_eq!(description::qry::get(&state,100), "ADescription".to_string() );
    /// ```
    pub fn get(state: &State, entity_pub_id: PubId) -> String {
        let id = entity::qry::id( state, entity_pub_id);
        state.description.get(id).unwrap_or_default()
    }
}