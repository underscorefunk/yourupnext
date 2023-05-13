use crate::prelude::*;

use std::collections::HashMap;

pub type Id = usize;

pub type PubId = usize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Registry {
    next_id: Id,
    id_dict: HashMap<Id, PubId>,
    pub_dict: HashMap<PubId, Id>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            next_id: 1,
            id_dict: HashMap::default(),
            pub_dict: HashMap::default(),
        }
    }
}

impl Registry {
    pub fn has_id(&self, id: &Id) -> bool {
        self.id_dict.contains_key(id)
    }

    pub fn has_pub_id(&self, pub_id: &PubId) -> bool {
        self.pub_dict.contains_key(pub_id)
    }

    pub fn id(&self, pub_id: &PubId) -> Id {
        match self.pub_dict.get(pub_id).map(|id| *id) {
            Some(id) => id,
            None => 0
        }
    }

    pub fn pub_id(&self, id: &Id) -> Option<PubId> {
        self.id_dict.get(id).map(|pub_id| *pub_id)
    }
}

pub fn register(mut state: State, pub_id: PubId) -> CommandResult<State> {
    if state.registry.has_pub_id(&pub_id) {
        return Err("Entity with PUBLIC ID already exists.".to_string());
    }

    state.registry.id_dict.insert(state.registry.next_id, pub_id.clone());
    state.registry.pub_dict.insert(pub_id, state.registry.next_id);
    state.registry.next_id += 1;

    Ok(state)
}

pub fn deregister(mut state: State, id: Id) -> CommandResult<State> {
    if !state.registry.has_id(&id) {
        return Err("Unable to remove entitiy, missing ID.".to_string());
    }

    let pub_id = state.registry.pub_id(&id);
    if pub_id.is_none() {
        return Err("Unable to remove entitiy, missing PUBLIC ID.".to_string());
    }
    let pub_id = pub_id.unwrap();


    state.registry.id_dict.remove(&id);
    state.registry.pub_dict.remove(&pub_id);

    Ok(state)
}

// ----------------------------------------------------------------------
// Query
// ----------------------------------------------------------------------

/// QUERY > Get the Id for an entity via Public Id
pub fn id(state: &State, pub_id: PubId) -> Id {
    state.registry.id(&pub_id)
}
