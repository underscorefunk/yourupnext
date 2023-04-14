use crate::event;

pub type Name = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    name: Name,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: "Unnamed Scenario".to_string()
        }
    }
}

pub fn rename(state: event::State, name: &Name) -> event::ActionResult {
    if name.is_empty() {
        return Err("Can not rename to an empty name.".to_string());
    }

    if state.scenario.name.eq(name) {
        return Err("Can not rename to the same name.".to_string());
    }
    Ok(
        event::State {
            scenario: State {
                name: name.to_string(),
                ..state.scenario
            },
            ..state
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Action;

    #[test]
    fn rename_action_ok() {
        assert_eq!(
            event::Action::RenameScenario("New name".to_string())
                .apply(event::State::default()),
            Ok(
                event::State {
                    scenario: State {
                        name: "New name".to_string(),
                    },
                    ..event::State::default()
                }
            )
        );
    }

    #[test]
    fn rename_action_err() {
        let actions = vec![
            Action::RenameScenario("New name".to_string()),
            Action::RenameScenario("".to_string()),
        ];
        let should_be_err = Action::apply_all(actions, event::State::default());
        assert!(should_be_err.is_err());

        let actions = vec![
            Action::RenameScenario("New name".to_string()),
            Action::RenameScenario("New name".to_string()),
        ];
        let should_be_err = Action::apply_all(actions, event::State::default());
        assert!(should_be_err.is_err());
    }
}
