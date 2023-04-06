#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    last_action: Action,
    name: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            last_action: Action::None,
            name: "Default Name".to_string(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Action {
    None,
    Init,
    Rename(String),
}

impl Action {
    fn apply(&self, state: State) -> State {
        match self {
            Action::None => state,
            Action::Init => State::default(),
            Action::Rename(name) => rename(state, name)
        }
    }

    fn apply_all(actions: Vec<Action>, state: State) -> State {
        actions
            .iter()
            .fold(
                state,
                |state, action| action.apply(state),
            )
    }
}

fn rename(state: State, name: &str) -> State {
    State {
        name: name.to_string(),
        ..state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_action() {
        assert_eq!(
            Action::Init.apply(State::default()),
            State::default()
        );
        assert_eq!(
            Action::apply_all(vec![Action::Init], State::default()),
            State::default()
        );
    }

    #[test]
    fn rename_action() {

        let actions = vec![
            Action::Init,
            Action::Rename("New name".to_string())
        ];
        assert_eq!(
            Action::apply_all(actions, State::default()),
            State {
                name: "New name".to_string(),
                ..State::default()
            }
        );
    }
}
