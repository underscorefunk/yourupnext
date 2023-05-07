use crate::prelude::*;

/*
pub fn assign(mut state: State, character_id: Id, player_id: Id) -> ActionResult<State> {
    if state.entity_type.get(character_id) != Some(EntityType::Character) {
        return Err("Can not assign/associate non character entity to player".to_string());
    }

    if state.entity_type.get(player_id) != Some(EntityType::Player) {
        return Err("Can not assign/associate character entity to non player".to_string());
    }

    state.character_player.assign(character_id, player_id)?;

    Ok(state)
}
*/