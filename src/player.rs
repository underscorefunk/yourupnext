use std::collections::HashMap;
use crate::event;

pub type Id = usize;
pub type Name = String;
pub type Players = HashMap<Id, Name>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub next_player_id: Id,
    pub players: Players,
}

impl Default for State {
    fn default() -> Self {
        Self {
            next_player_id: 0,
            players: HashMap::default(),
        }
    }
}




pub fn add(state: event::State, name: &Name) -> event::ActionResult {
    if name.is_empty() {
        return Err("Player name can not be empty.".to_string());
    }

    // If the player name is identical to another, add a number
    // and increase the number of the other one.
    let mut players = state.player.players;
    players.insert(state.player.next_player_id, name.clone());

    Ok(event::State {
        player: State {
            next_player_id: 1 as Id,
            players,
        },
        ..state
    })
}

pub fn rename(state: event::State, player_id: Id, name: &Name) -> event::ActionResult {

    let current_name = state.player.players.get(&player_id);

    if current_name.is_none() {
        return Err("Unable to rename missing player.".to_string());
    }

    if current_name == Some(name){
        return Err("Unable to rename player with unchanged name.".to_string());
    }

    let mut players = state.player.players;
    players.insert(player_id, name.to_string() );

    Ok(event::State{
        player: State {
            players,
            ..state.player
        },
        ..state
    })

}

pub fn remove(state: event::State, player_id: Id) -> event::ActionResult {

    let mut players = state.player.players;
    match players.remove(&player_id) {
        None => Err("Unable to find player to remove.".to_string()),
        Some(_) => Ok(event::State{
            player: State {
                players,
                ..state.player
            },
            ..state
        })
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_player_action_ok() {
        let base_state = event::State::default();
        let actions = vec![
            event::Action::AddPlayer("Jenna".to_string()),
        ];
        let result = event::Action::apply_all(actions, base_state);

        let mut target: Players = HashMap::new();
        target.insert(0, "Jenna".to_string());

        match result {
            Ok(result) => assert_eq!(
                result.player.players,
                target
            ),
            Err(_) => assert!(false) // This should never be reached
        }
    }

    #[test]
    fn add_player_action_err_no_empty_names() {
        let result = event::Action::AddPlayer("".to_string()).apply(event::State::default());
        assert!(result.is_err());
    }

    #[test]
    fn add_player_action_ok_duplicate() {
        let base_state = event::State::default();
        let actions = vec![
            event::Action::AddPlayer("Jenna".to_string()),
            event::Action::AddPlayer("Jenna".to_string()),
        ];
        let result = event::Action::apply_all(actions, base_state);

        let mut target: Players = HashMap::new();
        target.insert(0, "Jenna".to_string());
        target.insert(1, "Jenna".to_string());

        match result {
            Ok(result) => assert_eq!(
                result.player.players,
                target
            ),
            Err(_) => assert!(false) // This should never be reached
        }
    }

    #[test]
    fn rename_player_action_ok() {

        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddPlayer("Jenna".to_string()),
            event::Action::RenamePlayer(0,"Jade".to_string())
        ];

        let result = event::Action::apply_all(actions, base_state);

        let mut target: Players = HashMap::new();
        target.insert(0, "Jade".to_string());

        match result {
            Ok(result) => assert_eq!(
                result.player.players,
                target
            ),
            Err(_) => assert!(false) // This should never be reached
        }
    }

    #[test]
    fn rename_player_action_err_missing_player() {

        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddPlayer("Jenna".to_string()),
            event::Action::RenamePlayer(1,"Jade".to_string())
        ];

        let result = event::Action::apply_all(actions, base_state);

        assert!(result.is_err() );
    }

    #[test]
    fn rename_player_action_err_same_name() {

        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddPlayer("Jenna".to_string()),
            event::Action::RenamePlayer(0,"Jenna".to_string())
        ];

        let result = event::Action::apply_all(actions, base_state);

        assert!(result.is_err() );
    }

    #[test]
    fn remove_player_action_ok() {

        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddPlayer("Jenna".to_string()),
            event::Action::RemovePlayer(0)
        ];

        let result = event::Action::apply_all(actions, base_state);

        let target: Players = HashMap::new();

        match result {
            Ok(result) => assert_eq!(
                result.player.players,
                target
            ),
            Err(_) => assert!(false) // This should never be reached
        }
    }

    #[test]
    fn remove_player_action_err() {

        let base_state = event::State::default();

        let actions = vec![
            event::Action::AddPlayer("Jenna".to_string()),
            event::Action::RemovePlayer(1)
        ];

        let result = event::Action::apply_all(actions, base_state);

        assert!(result.is_err())
    }
}