use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EntityType {
    Player,
    Character,
    Item,
    Location,
    Effect,
}

impl EntityType {
    fn label(self) -> String {
        match self {
            EntityType::Player => "Player".to_string(),
            EntityType::Character => "Character".to_string(),
            EntityType::Item => "Item".to_string(),
            EntityType::Location => "Location".to_string(),
            EntityType::Effect => "Effect".to_string(),
        }
    }
}

pub fn classify(
    mut state: State,
    id: Id,
    assigned_entity_type: EntityType,
) -> ActionResult<State> {
    state.entity_type.insert(id, assigned_entity_type)?;
    Ok(state)
}