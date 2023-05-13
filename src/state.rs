use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {

    /// Holds the mappings of public Ids to internal Ids for each entity
    pub registry: Registry,
    pub entity_type: Component<EntityType>,
    pub name: Component<String>,
    pub character_player: Hierarchy,

    /// A subsystem to handle rounds/turns
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