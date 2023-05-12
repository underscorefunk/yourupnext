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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    None,
    Init,

    // Entity
    RegisterEntity(PubId),
    DeleteEntity(Id),

    // Component
    Classify(Id, EntityType),
    Rename(Id, Name),

}

impl Applicable for Action {
    fn apply(self, state: State) -> ActionResult<State> {
        match self {
            Action::None => Ok(state),
            Action::Init => Ok(State::default()),

            // Entity
            Action::RegisterEntity(pub_id) => register(state, pub_id),
            Action::DeleteEntity(id) => deregister(state, id),

            // Components
            Action::Classify(id, entity_type) => classify(state, id, entity_type),
            Action::Rename(id, new_name) => rename(state, id, new_name)
        }
    }

    fn apply_default(self) -> ActionResult<State> {
        self.apply( State::default() )
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_action() {
        assert_eq!(
            Action::Init.apply(State::default()),
            Ok(State::default())
        );
    }
}