use crate::prelude::*;

/// Player Model
///
/// The player model includes COMMANDS and QUERIES to interact with data
/// that is exclusively for players.

// ----------------------------------------------------------------------
// Command
// ----------------------------------------------------------------------

/// COMMAND > Add a player
pub fn add(state: State, player_pub_id: PubId, starting_name: String) -> CommandResult<State> {
    todo!()
    /*
    let state = Action::RegisterEntity(player_pub_id).apply(state)?;
    let id = state.registry.id(&player_pub_id);

    vec![
        Action::Classify(id, EntityType::Player),
        Action::Rename(id, starting_name),
    ].apply(state)

     */
}

pub fn rename(state: State, pub_id: PubId, name: String) -> CommandResult<State> {
    todo!()
    // let id = state.registry.id(&pub_id);
    // Action::Rename(id, name).apply(state)
}

pub fn remove(state: State, pub_id: PubId) -> CommandResult<State> {
    todo!()
    // let id = state.registry.id(&pub_id);
    // Action::DeleteEntity(id).apply(state)
}

// ----------------------------------------------------------------------
// Query
// ----------------------------------------------------------------------


// ----------------------------------------------------------------------
// Utility
// ----------------------------------------------------------------------

