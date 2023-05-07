use crate::prelude::*;
use std::collections::HashMap;

pub trait ComponentValue = Clone + Eq + PartialEq;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Component<CV: ComponentValue> {
    pub values: HashMap<Id, CV>,
}

impl<CV: ComponentValue> Default for Component<CV> {
    fn default() -> Self {
        Self {
            values: HashMap::default()
        }
    }
}

impl<CV: ComponentValue> Component<CV> {

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn is_set(&self, id: Id) -> bool {
        self.values.contains_key(&id)
    }

    pub fn get(&self, id: Id) -> Option<CV> {
        self.values.get(&id).cloned()
    }

    pub fn insert(&mut self, id: Id, value: CV) -> ActionResult<()> {
        if self.is_set(id) {
            return Err("Can not insert component value that already exists. Use update.".to_string());
        }
        self.values.insert(id, value);
        Ok(())
    }

    pub fn update(&mut self, id: Id, value: CV) -> ActionResult<()> {
        self.values.insert(id, value);
        Ok(())
    }

    pub fn delete(&mut self, id: Id) -> ActionResult<()> {
        if !self.is_set(id) {
            return Err("Can not delete component that was never set".to_string());
        }

        self.values.remove(&id);
        Ok(())
    }
}

