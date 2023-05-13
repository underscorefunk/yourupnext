use crate::prelude::*;

pub type Name = String;

// ----------------------------------------------------------------------
// Command
// ----------------------------------------------------------------------

/// COMMAND > Set the `Name` of an entity
pub fn set(mut state: State, id: Id, new_name: String) -> CommandResult<State> {
    state.name.update(id, new_name)?;
    Ok(state)
}

// ----------------------------------------------------------------------
// Query
// ----------------------------------------------------------------------

/// QUERY > Get the `Name` of an entity
pub fn get(state: &State, character_pub_id: PubId) -> String {
    let id = state.registry.id(&character_pub_id);
    state.name.get(id).unwrap_or(String::new())
}