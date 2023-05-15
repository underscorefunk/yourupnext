use crate::prelude::*;
use crate::registry::Registry;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {

    pub registry: Registry,

    pub entity_type: Component<EntityType>,
    pub name: Component<String>,
    pub character_player: Hierarchy,

    pub round: Round,
}

impl Default for State {
    fn default() -> Self {
        Self {
            registry: Registry::default(),
            entity_type: Component::default(),
            name: Component::default(),
            character_player: Hierarchy::default(),
            round: Round::default(),
        }
    }
}