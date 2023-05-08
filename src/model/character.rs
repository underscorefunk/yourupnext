use crate::prelude::*;

// ----------------------------------------------------------------------
// Command
// ----------------------------------------------------------------------

pub fn add(state: State, character_pub_id: PubId, starting_name: String) -> ActionResult<State> {
    let state = Action::RegisterEntity(character_pub_id).apply(state)?;
    let id = state.registry.id(&character_pub_id);

    Action::apply_all(vec![
        Action::Classify(id, EntityType::Character),
        Action::Rename(id, starting_name),
    ], state)
}

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


pub fn rename(state: State, character_pub_id: PubId, new_name: String) -> ActionResult<State> {
    let id = state.registry.id(&character_pub_id);
    Action::Rename(id, new_name).apply(state)
}

pub fn remove(mut state: State, character_pub_id: PubId) -> ActionResult<State> {
    let id = state.registry.id(&character_pub_id);
    let _ = state.character_player.remove_parent(character_pub_id);

    Action::DeleteEntity(id).apply(state)
}

// ----------------------------------------------------------------------
// Query
// ----------------------------------------------------------------------

