use std::collections::HashMap;
use crate::event;

pub type Id = usize;
pub type Name = String;

pub type Entities = HashMap<Id, Entity>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entity {
    name: Name,
}

impl Entity {
    fn new(name: Name) -> Self {
        Self {
            name
        }
    }

    fn set_name(entity:&Entity, name: Name) -> Self {
        Self {
            name,
            ..entity.clone()
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            name: "Unnamed Entity".to_string()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub next_entity_id: Id,
    pub entities: Entities,
}

impl Default for State {
    fn default() -> Self {
        Self {
            next_entity_id: 0,
            entities: HashMap::default(),
        }
    }
}


pub fn add(state: event::State, name: &Name) -> event::ActionResult {
    if name.is_empty() {
        return Err("Entity name can not be empty.".to_string());
    }

    let mut entities = state.entity.entities;
    let entity_id = state.entity.next_entity_id;
    entities.insert(
        entity_id,
        Entity::new(name.to_string() ),
    );

    Ok(event::State {
        entity: State {
            next_entity_id: entity_id + 1,
            entities,
        },
        ..state
    })
}

pub fn rename(state: event::State, entity_id: Id, new_name: &Name) -> event::ActionResult {
    let target_entity = state.entity.entities.get(&entity_id);

    if target_entity.is_none() {
        return Err("Unable to rename missing entity.".to_string());
    }

    let target_entity = target_entity.unwrap();

    if &target_entity.name == new_name {
        return Err("Unable to rename entity with unchanged name.".to_string());
    }

    let updated_entity = Entity::set_name(
        &target_entity,
        new_name.to_string()
    );

    let mut entities = state.entity.entities;

    entities.insert(
        entity_id,
        updated_entity
    );

    Ok(event::State {
        entity: State {
            entities,
            ..state.entity
        },
        ..state
    })
}

pub fn remove(state: event::State, entity_id: Id) -> event::ActionResult {
    let mut entities = state.entity.entities;
    match entities.remove(&entity_id) {
        None => Err("Unable to find entity to remove.".to_string()),
        Some(_) => Ok(event::State {
            entity: State {
                entities,
                ..state.entity
            },
            ..state
        })
    }
}

pub fn by_id(state: &event::State, entity_id: Id) -> Option<Entity> {
    match state.entity.entities.get(&entity_id) {
        Some(entity) => Some( entity.clone() ),
        None => None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_entity_action_ok() {
        let base_state = event::State::default();
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string() ),
        ];
        let result = event::Action::apply_all(actions, base_state);

        let mut target: Entities = HashMap::new();
        target.insert(0, Entity::new("Jenna".to_string()));

        match result {
            Ok(result) => assert_eq!(
                result.entity.entities,
                target
            ),
            Err(_) => assert!(false) // This should never be reached
        }
    }

    #[test]
    fn add_entity_action_err_no_empty_names() {
        let result = event::Action::AddEntity("".to_string()).apply(event::State::default());
        assert!(result.is_err());
    }

    #[test]
    fn add_entity_action_ok_duplicate() {
        let base_state = event::State::default();
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::AddEntity("Jenna".to_string()),
        ];
        let result = event::Action::apply_all(actions, base_state);

        let mut target: Entities = HashMap::new();
        target.insert(0, Entity::new("Jenna".to_string()));
        target.insert(1, Entity::new("Jenna".to_string()));

        match result {
            Ok(result) => assert_eq!(
                result.entity.entities,
                target
            ),
            Err(_) => assert!(false) // This should never be reached
        }
    }

    #[test]
    fn rename_entity_action_ok() {
        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::RenameEntity(0, "Jade".to_string()),
        ];

        let result = event::Action::apply_all(actions, base_state);

        let mut target: Entities = HashMap::new();
        target.insert(0, Entity::new("Jade".to_string()));

        match result {
            Ok(result) => assert_eq!(
                result.entity.entities,
                target
            ),
            Err(_) => assert!(false) // This should never be reached
        }
    }

    #[test]
    fn rename_entity_action_err_missing_entity() {
        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::RenameEntity(1, "Jade".to_string()),
        ];

        let result = event::Action::apply_all(actions, base_state);

        assert!(result.is_err());
    }

    #[test]
    fn rename_entity_action_err_same_name() {
        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::RenameEntity(0, "Jenna".to_string()),
        ];

        let result = event::Action::apply_all(actions, base_state);

        assert!(result.is_err());
    }

    #[test]
    fn remove_entity_action_ok() {
        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::RemoveEntity(0),
        ];

        let result = event::Action::apply_all(actions, base_state);

        let target: Entities = HashMap::new();

        match result {
            Ok(result) => assert_eq!(
                result.entity.entities,
                target
            ),
            Err(_) => assert!(false) // This should never be reached
        }
    }

    #[test]
    fn remove_entity_action_err() {
        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::RemoveEntity(1),
        ];

        let result = event::Action::apply_all(actions, base_state);

        assert!(result.is_err())
    }
}