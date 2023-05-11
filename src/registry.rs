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

pub fn register(mut state: State, pub_id: PubId) -> ActionResult<State> {
    if state.registry.has_pub_id(&pub_id) {
        return Err("Entity with PUBLIC ID already exists.".to_string());
    }

    state.registry.id_dict.insert(state.registry.next_id, pub_id.clone());
    state.registry.pub_dict.insert(pub_id, state.registry.next_id);
    state.registry.next_id += 1;

    Ok(state)
}

pub fn deregister(mut state: State, id: Id) -> ActionResult<State> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn register_ok() {
        let result = register(State::default(), 42);

        if result.is_err() {
            // This should always be ok
            assert!(false);
            return;
        }

        let state = result.unwrap();

        assert_eq!(state.registry.next_id, 2);
        assert!(state.registry.has_pub_id(&42));
        assert_eq!(state.registry.id(&42), 1);
    }

    #[test]
    pub fn remove_ok() {
        let result = register(State::default(), 42);
        if result.is_err() {
            // This should always be ok
            assert!(false);
            return;
        }
        let state = result.unwrap();
        let result = deregister(state, 1);
        if result.is_err() {
            // This should always be ok
            assert!(false);
            return;
        }
        let removed_state = result.unwrap();
        assert_eq!(removed_state.registry.next_id, 2);
        assert_eq!(removed_state.registry.has_pub_id(&42), false);
        assert_eq!(removed_state.registry.id(&42), 0);
    }
}