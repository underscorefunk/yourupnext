/// # Player Model

use crate::prelude::*;

/// ## Player > Command Applicables(Cmd)
/// A simple wrapper for player commands so that they can be composed together with other pipelines.
/// `Cmd` is a facade for `cmd` functions.

#[derive(Debug,Eq,PartialEq)]
pub enum Player {
    Add(PubId, Name),
    Remove(PubId),
    Rename(PubId, Name)
}

impl Applicable for Player {
    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {
            Player::Add(pub_id, starting_name) => cmd::add(state, pub_id, starting_name),
            Player::Remove(pub_id) => cmd::remove(state, pub_id),
            Player::Rename(pub_id, new_name) => cmd::rename(state, pub_id, new_name)
        }
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to( State::default() )
    }
}

/// ## Player > Command (cmd)

pub mod cmd {
    use super::*;

    /// COMMAND > Add a player
    pub fn add(state: State, player_pub_id: PubId, starting_name: String) -> CmdResult<State> {
        vec![
            entity::Entity::Add(player_pub_id),
            entity::Entity::Classify(player_pub_id, EntityType::Player),
            entity::Entity::Name(player_pub_id, starting_name),
        ].apply_to(state)
    }

    /// COMMAND > Remove a player
    pub fn remove(state: State, player_pub_id: PubId) -> CmdResult<State> {
        vec![
            entity::Entity::Remove(player_pub_id),
        ].apply_to(state)
    }

    /// COMMAND > Rename a player
    pub fn rename(state: State, player_pub_id: PubId, new_name: String) -> CmdResult<State> {
        vec![
            entity::Entity::Name(player_pub_id, new_name),
        ].apply_to(state)
    }

}