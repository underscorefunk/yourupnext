use crate::prelude::*;

/// Character Model
///
/// The character model includes COMMANDS and QUERIES to interact with data
/// that is exclusively for characters.

// ----------------------------------------------------------------------
// Command
// ----------------------------------------------------------------------

/// COMMAND > Add a character
/// ```
// use yourupnext::prelude::*;
// let state = vec![
//     Cmd::AddCharacter(100, "ACharacter".to_string())
// ].apply_default().unwrap();
//
// // Public ID 100 is used to disambiguate from internal
// // Id of 1.
// assert_eq!(character::id(&state,100), 1)
/// ```
pub fn add(state: State, character_pub_id: PubId, starting_name: String) -> CommandResult<State> {
    Err("todo add".to_string())
    // let state = Action::RegisterEntity(character_pub_id).apply(state)?;
    // let id = registry::id(&state, character_pub_id);
    //
    // vec![
    //     Action::Classify(id, EntityType::Character),
    //     Action::Rename(id, starting_name),
    // ].apply(state)
}

/// COMMAND > Assign a character to a player
/// ```
/// use yourupnext::prelude::*;
/// let state = vec![
///     Cmd::AddPlayer(1,"APlayer".to_string()),
///     Cmd::AddCharacter(2,"ACharacter".to_string()),
///     Cmd::AssignCharacterPlayer(2,1)
/// ].apply_default().unwrap();
///
/// assert_eq!(character::player(&state,2), Some(1));
///
/// ```
pub fn assign_player(mut state: State, character_pub_id: PubId, player_pub_id: PubId) -> CommandResult<State> {
    let character_id = entity::qry::id(&state, character_pub_id);
    let player_id = entity::qry::id(&state, player_pub_id);

    if !entity_type::qry::is(&state, character_id, EntityType::Character) {
        return Err("Can not assign player to character when the target character isn't a Character entity type.".to_string());
    }

    if !entity_type::qry::is(&state, player_id, EntityType::Player) {
        return Err("Can not assign player to character when the target player isn't a Player entity type.".to_string());
    }

    state.character_player.set_parent(character_pub_id, player_pub_id)?;

    Ok(state)
}

/// COMMAND > Rename a character
pub fn rename(state: State, character_pub_id: PubId, new_name: String) -> CommandResult<State> {
    entity::cmd::name(state, character_pub_id, new_name)
}

/// COMMAND > Remove a character
///```
/// use yourupnext::prelude::*;
/// let state = vec![
///    Cmd::AddCharacter(1,"ACharacter".to_string()),
///    Cmd::RemoveCharacter(1)
/// ].apply_default().unwrap();
///
/// assert_eq!(character::id(&state,1), 0);
/// ```
pub fn remove(mut state: State, character_pub_id: PubId) -> CommandResult<State> {
    let id = registry::id(&state, character_pub_id);

    // We do not bubble the error because a parent might not exist and
    // that's ok!
    let _ = state.character_player.remove_parent(id);
    entity::cmd::remove(state, character_pub_id.clone())
}

// ----------------------------------------------------------------------
// Query
// ----------------------------------------------------------------------

/// QUERY > Check if a character exists
/// ```
/// use yourupnext::prelude::*;
/// let state = vec![
///     Cmd::AddCharacter(100, "ACharacter".to_string()),
/// ].apply_default().unwrap();
///
/// assert!(character::exists(&state,100));
/// assert!(! character::exists(&state,1));
/// ```
pub fn exists(state: &State, character_pub_id: PubId) -> bool {
    is_character(state, character_pub_id)
}

/// Query > Check if a private Id is a valid character entity type
/// ```
/// use yourupnext::prelude::*;
/// let state = vec![
///     Cmd::AddCharacter(100, "ACharacter".to_string()),
///     Cmd::AddPlayer(102, "APlayer".to_string())
/// ].apply_default().unwrap();
///
/// assert!(character::is_character(&state,100));
/// assert!(! character::is_character(&state,102));
/// ```
pub fn is_character(state: &State, character_pub_id: Id) -> bool {
    entity_type::qry::is(
        state,
        registry::id(state, character_pub_id),
        EntityType::Character,
    )
}

/// QUERY > Get the Id of a character
/// ```
/// use yourupnext::prelude::*;
/// let state = vec![
///     Cmd::AddCharacter(100, "ACharacter".to_string()),
///     Cmd::AddPlayer(102, "APlayer".to_string())
/// ].apply_default().unwrap();
///
/// assert_eq!(character::id(&state,100), 1);
///
/// // A missing internal id is a 0, which will always
/// // fail to match entities. This is done to avoid
/// // unwrapping every query. It's a common query.
/// assert_eq!(character::id(&state,1), 0);
///
/// // Incorrect entity types will yield a zero id as well.
/// assert_eq!(character::id(&state,102), 0);
/// ```
pub fn id(state: &State, character_pub_id: PubId) -> Id {
    let character_id = entity::qry::id(state, character_pub_id);
    match entity::qry::is(state, character_id, EntityType::Character) {
        true => character_id,
        false => 0
    }
}

/// QUERY > Get the Public Id (`pub_id`) of a character's `Player`
/// ```
/// use yourupnext::prelude::*;
///
/// let state = vec![
///     Cmd::AddPlayer(1,"APlayer".to_string()),
///     Cmd::AddCharacter(2,"ACharacter".to_string()),
///     Cmd::AssignCharacterPlayer(2,1),
/// ].apply_default().unwrap();
///
/// // PubId 1 refers to a player, its "player" id is None
/// assert_eq!(character::player(&state,1), None);
///
/// // PubId 2 refers to a character, its "player" id is 1
/// assert_eq!(character::player(&state,2), Some(1));
/// ```
pub fn player(state: &State, character_pub_id: PubId) -> Option<PubId> {
    let child_id = state.registry.id(&character_pub_id);
    let parent_id = state.character_player.parent(child_id);

    if state.entity_type.get(parent_id.unwrap_or(0)) != Some(EntityType::Player) {
        return None;
    }

    parent_id
}

/// QUERY > Get the `Name` of a character
/// ```
/// use yourupnext::prelude::*;
///
/// let state = Cmd::AddCharacter(1,"ACharacter".to_string())
///     .apply_default()
///     .unwrap();
///
/// // PubId 1 refers to a player, its "player" id is None
/// assert_eq!(character::name(&state,1), "ACharacter".to_string());
///
/// // Unnamed characters will return an empty string
/// assert_eq!(character::name(&state,2), String::new() );
/// ```
pub fn name(state: &State, character_pub_id: PubId) -> String {
    name::qry::get(state, character_pub_id)
}
