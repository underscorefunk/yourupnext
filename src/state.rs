use crate::prelude::*;
use crate::registry::Registry;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub registry: Registry,
    pub entity_type: Component<EntityType>,

    pub name: Component<String>,
    pub description: Component<String>,
    pub turn_state: Component<TurnState>,
    pub turn_count: Component<TurnCount>,
    pub turn_order: Component<TurnOrder>,

    pub character_player: Hierarchy,

    pub character_scenario: Hierarchy, //make this an association (Scenario members)

}

impl Default for State {
    fn default() -> Self {
        Self {
            registry: Registry::default(),
            entity_type: Component::default(),
            name: Component::default(),
            description: Component::default(),
            turn_state: Component::default(),
            turn_count: Component::default(),
            turn_order: Component::default(),

            character_player: Hierarchy::default(),
            character_scenario: Hierarchy::default(),
        }
    }
}

pub mod qry {
    use super::*;

    // id
    // ids
    // pub_id
    // pub_ids
}