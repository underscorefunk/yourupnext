use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub registry: Registry,
    pub entity_type: Component<EntityType>,
    pub name: Component<String>,
    pub hierarchy: Hierarchy,
    pub round: Round,
}

impl Default for State {
    fn default() -> Self {
        Self {
            registry: Registry::default(),
            entity_type: Component::default(),
            name: Component::default(),
            hierarchy: Hierarchy::default(),
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

    // Associations
    // Link(Id, Id),
    // Unlink(Id),

    // Component
    Classify(Id, EntityType),
    Rename(Id, Name),

}

impl Applicable<Action> for Action {
    fn apply(self, state: State) -> ActionResult<State> {
        match self {
            Action::None => Ok(state),
            Action::Init => Ok(State::default()),

            // Entity
            Action::RegisterEntity(pub_id) => registry::register(state, pub_id ),
            Action::DeleteEntity(id) => registry::deregister(state, id),

            // Ownership
            // Action::Link(subject_id, target_id ) => link(state,subject_id, target_id),
            // Action::Unlink(subject_id ) => unlink(state,subject_id, target_id),

            // Components
            Action::Classify(id, entity_type) => classify(state,id,entity_type),
            Action::Rename(id, new_name) => rename(state,id,new_name)

        }
    }

    fn apply_all(actions: Vec<Action>, state: State) -> ActionResult<State> {
        apply_actions(actions, state)
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
        assert_eq!(
            Action::apply_all(vec![Action::Init], State::default()),
            Ok(State::default())
        );
    }
}