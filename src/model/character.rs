use crate::prelude::*;

pub fn add(state: State, character_pub_id: PubId, starting_name: String) -> ActionResult<State> {
    let state = Action::RegisterEntity(character_pub_id).apply(state)?;
    let id = state.registry.id(&character_pub_id);

    Action::apply_all(vec![
        Action::Classify(id, EntityType::Character),
        Action::Rename(id, starting_name),
    ], state)
}

pub fn rename(state: State, character_pub_id: PubId, new_name: String) -> ActionResult<State> {
    let id = state.registry.id(&character_pub_id);
    Action::Rename(id, new_name).apply(state)
}

pub fn remove(state: State, character_pub_id: PubId) -> ActionResult<State> {
    let id = state.registry.id(&character_pub_id);
    Action::apply_all(vec![
        Action::DeleteEntity(id)
    ], state)
}