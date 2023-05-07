use std::collections::HashMap;
use crate::effect::{State, Action, ActionResult};

pub type Id = usize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ref {
    None
}

pub type Items = HashMap<Id, Ref>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Collection {
    pub next_id: Id,
    pub items: Items,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            next_id: 0,
            items: Items::default(),
        }
    }
}


pub fn locate(entities: &Items, needle: &Ref) -> Option<Id> {
    for (hs_id, hs_ref) in entities.iter() {
        if needle == hs_ref {
            return Some(*hs_id);
        }
    }
    None
}

pub fn add(state: State, entity_ref: Ref) -> ActionResult {
    let entities = state.entities;

    if entity_ref != Ref::None && locate(&entities.items, &entity_ref).is_some() {
        return Err("Can not add an effect reference that already exists".to_string());
    }

    let mut items = entities.items;
    items.insert(entities.next_id, entity_ref);
    let next_id = entities.next_id + 1;

    Ok(State {
        entities: Collection {
            next_id,
            items,
        },
        ..state
    })
}

pub fn remove(state: State, entity_id: Id) -> ActionResult {
    let mut items = state.entities.items;
    match items.remove(&entity_id) {
        None => Err("Unable to remove nonexistant entity".to_string() ),
        Some(_) => Ok(State {
            entities: Collection {
                items,
                ..state.entities
            },
            ..state
        })
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn add_item() {
        let actions = vec![
            Action::Init,
            Action::AddEntity(Ref::None),
            Action::AddEntity(Ref::None),
        ];

        let state = Action::apply_all(
            actions,
            State::default(),
        );


        let mut entities = Items::default();
        entities.insert(0, Ref::None);
        entities.insert(1, Ref::None);

        let target = Collection {
            next_id: 2,
            items: entities,
        };

        assert_eq!(
            state.unwrap().entities,
            target
        );
    }
}