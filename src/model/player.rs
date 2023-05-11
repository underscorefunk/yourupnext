use crate::prelude::*;

pub fn add(state: State, pub_id: PubId, name: String) -> ActionResult<State> {
    let state = Action::RegisterEntity(pub_id).apply(state)?;
    let id = state.registry.id(&pub_id);

    vec![
        Action::Classify(id, EntityType::Player),
        Action::Rename(id, name),
    ].apply(state)
}

pub fn rename(state: State, pub_id: PubId, name: String) -> ActionResult<State> {
    let id = state.registry.id(&pub_id);
    Action::Rename(id, name).apply(state)
}

pub fn remove(state: State, pub_id: PubId) -> ActionResult<State> {
    let id = state.registry.id(&pub_id);
    Action::DeleteEntity(id).apply(state)
}