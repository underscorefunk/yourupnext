use crate::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EntityAssoc {
    pub id_dict: HashMap<Id, Id>,
    pub assoc_id_dict: HashMap<Id, Id>,
}

impl Default for EntityAssoc {
    fn default() -> Self {
        Self {
            id_dict: HashMap::default(),
            assoc_id_dict: HashMap::default(),
        }
    }
}

impl EntityAssoc {
    pub fn assign(&mut self, id: Id, assoc_id: Id) -> ActionResult<()> {
        self.id_dict.insert(id, assoc_id);
        self.assoc_id_dict.insert(assoc_id, id);
        Ok(())
    }

    pub fn has_assoc(&self, id: &Id) -> bool {
        self.id_dict.contains_key(id)
    }

    pub fn is_assoc_id(&self, assoc_id: &Id) -> bool {
        self.assoc_id_dict.contains_key(assoc_id)
    }

    pub fn id_lookup(&self, assoc_id: &Id) -> Id {
        match self.assoc_id_dict.get(assoc_id).map(|id| *id) {
            Some(id) => id,
            None => 0
        }
    }

    pub fn assoc_id_lookup(&self, id: &Id) -> Id {
        match self.id_dict.get(id).map(|id| *id) {
            Some(id) => id,
            None => 0
        }
    }
}