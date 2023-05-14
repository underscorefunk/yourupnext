use crate::prelude::*;

pub type Name = String;

/// ## Name > Command (cmd)

pub mod cmd {
    use super::*;

    /// COMMAND > Set the `Name` of an entity
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default();
    /// let state = entity::cmd::add( state, 100).unwrap();
    /// let renamed_state = name::cmd::set( state, 100, "AName".to_string() ).unwrap();
    /// assert_eq!(name::qry::get(&renamed_state,100), "AName".to_string() )
    /// ```
    pub fn set(mut state: State, entity_pub_id: PubId, new_name: String) -> CommandResult<State> {
        let id = entity::qry::id( &state, entity_pub_id);
        state.name.update(id, new_name)?;
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
    /// let state = State::default();
    /// let state = entity::cmd::add( state, 100).unwrap();
    /// let renamed_state = name::cmd::set( state, 100, "AName".to_string() ).unwrap();
    /// assert_eq!(name::qry::get(&renamed_state,100), "AName".to_string() )
    /// ```
    pub fn get(state: &State, entity_pub_id: PubId) -> String {
        let id = entity::qry::id( state, entity_pub_id);
        state.name.get(id).unwrap_or(String::new())
    }
}