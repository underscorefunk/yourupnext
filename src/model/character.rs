use crate::prelude::*;

// ----------------------------------------------------------------------
// Command
// ----------------------------------------------------------------------

pub fn add(state: State, character_pub_id: PubId, starting_name: String) -> ActionResult<State> {
    let state = Action::RegisterEntity(character_pub_id).apply(state)?;
    let id = state.registry.id(&character_pub_id);

    vec![
        Action::Classify(id, EntityType::Character),
        Action::Rename(id, starting_name),
    ].apply(state)
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
pub fn assign_player(mut state: State, character_pub_id: PubId, player_pub_id: PubId) -> ActionResult<State> {
    let character_id = state.registry.id(&character_pub_id);
    let player_id = state.registry.id(&player_pub_id);

    if state.entity_type.get(character_id) != Some(EntityType::Character) {
        return Err("Can not assign player to character when the target character isn't a Character entity type.".to_string());
    }

    if state.entity_type.get(player_id) != Some(EntityType::Player) {
        return Err("Can not assign player to character when the target player isn't a Player entity type.".to_string());
    }

    state.character_player.set_parent(character_pub_id, player_pub_id)?;

    Ok(state)
}

/// COMMAND > Rename a character
/// ```
/// use yourupnext::prelude::*;
///
/// let state = vec![
///     Cmd::AddCharacter(2,"OriginalName".to_string()),
///     Cmd::RenameCharacter(2,"RenamedCharacter".to_string()),
/// ].apply_default().unwrap();
///
/// // PubId 1 refers to a player, its "player" id is None
/// assert_eq!(character::player(&state,1), None);
pub fn rename(state: State, character_pub_id: PubId, new_name: String) -> ActionResult<State> {
    let id = state.registry.id(&character_pub_id);
    Action::Rename(id, new_name).apply(state)
}


/// COMMAND > Remove a character
///
pub fn remove(mut state: State, character_pub_id: PubId) -> ActionResult<State> {
    let id = state.registry.id(&character_pub_id);
    let _ = state.character_player.remove_parent(character_pub_id);

    Action::DeleteEntity(id).apply(state)
}

// ----------------------------------------------------------------------
// Query
// ----------------------------------------------------------------------

/// QUERY > Retrieve the Public Id (`pub_id`) of a character's `Player`
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

    if state.entity_type.get( parent_id.unwrap_or(0) ) != Some(EntityType::Player) {
        return None;
    }

    parent_id
}

/// QUERY > Retrieve the `Name` of a character
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

    let id = state.registry.id(&character_pub_id);

    if state.entity_type.get( id ) != Some(EntityType::Character) {
        return String::new();
    }
    state.name.get(id).unwrap_or(String::new() )
}