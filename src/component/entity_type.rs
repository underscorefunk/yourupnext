use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EntityType {
    Player,
    Character,
    Item,
    Location,
    Effect,
    Generic,
    Missing
}

impl EntityType {
    fn label(self) -> String {
        match self {
            EntityType::Player => "Player".to_string(),
            EntityType::Character => "Character".to_string(),
            EntityType::Item => "Item".to_string(),
            EntityType::Location => "Location".to_string(),
            EntityType::Effect => "Effect".to_string(),
            EntityType::Generic => "Generic".to_string(),
            EntityType::Missing => "Missing".to_string(),
        }
    }
}

// ----------------------------------------------------------------------
// Command
// ----------------------------------------------------------------------

// Prevent classification as "Missing"
pub fn classify(
    mut state: State,
    id: Id,
    assigned_entity_type: EntityType,
) -> CommandResult<State> {
    state.entity_type.insert(id, assigned_entity_type)?;
    Ok(state)
}

// ----------------------------------------------------------------------
// Query
// ----------------------------------------------------------------------

///Query > Check if the type clasification of an entity is a specific type
pub fn is(state: &State, id: Id, entity_type: EntityType) -> bool {
    get(state,id) == entity_type
}

///Query > Check if the type classification of an entity is a specific type
pub fn get(state: &State, id: Id) -> EntityType {
    if ! state.registry.has_id(&id) || id == 0 {
        return EntityType::Missing;
    }
    match state.entity_type.get(id) {
        Some(entity_type) => entity_type,
        None => EntityType::Generic
    }
}