/// # Player Model

use crate::prelude::*;

/// ## Player > Command Applicables (Cmd)
/// A simple wrapper for player commands so that they can be composed together with other pipelines.
/// `Cmd` is a facade for `cmd` functions.

pub type PlayerId = PubId;

#[derive(Debug,Eq,PartialEq)]
pub enum Player {
    Add(PlayerId, &'static Name),
    Remove(PlayerId),
    Rename(PlayerId, &'static Name)
}

impl Applicable for Player {
    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {
            Player::Add(player_id, starting_name) => cmd::add(state, player_id, starting_name),
            Player::Remove(player_id) => cmd::remove(state, player_id),
            Player::Rename(player_id, new_name) => cmd::rename(state, player_id, new_name)
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
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let player_id: PlayerId = 100;
    /// let state = Player::Add(player_id,"APlayer")
    ///     .apply_to_default()
    ///     .unwrap();
    ///
    /// assert!(player::qry::exists(&state,player_id));
    /// assert_eq!(player::qry::name(&state,player_id), "APlayer".to_string());
    ///
    /// ```
    pub fn add(state: State, player_id: PlayerId, starting_name: &'static Name) -> CmdResult<State> {
        vec![
            Entity::Add(player_id),
            Entity::Classify(player_id, EntityType::Player),
            Entity::Name(player_id, starting_name),
        ].apply_to(state)
    }

    /// COMMAND > Remove a player
    /// See Entity::Remove for tests
    pub fn remove(state: State, player_id: PlayerId) -> CmdResult<State> {
        Entity::Remove(player_id).apply_to(state)
    }

    /// COMMAND > Rename a player
    /// See Entity::Name for tests
    pub fn rename(state: State, player_id: PlayerId, new_name: &'static Name) -> CmdResult<State> {
        Entity::Name(player_id, new_name).apply_to(state)
    }

}

/// ## Character > Query (qry)

pub mod qry {
    use super::*;

    /// QUERY > Check if a player exists
    /// See `entity_type` component for tests
    pub fn exists(state: &State, player_id: PlayerId) -> bool {
        entity_type::qry::is(state, player_id, EntityType::Player)
    }

    /// QUERY > Get a player's `Id`
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let player_id: PlayerId = 100;
    /// let state = Player::Add(player_id,"APlayer")
    ///     .apply_to_default()
    ///     .unwrap();
    ///
    /// assert_eq!(player::qry::id(&state, player_id), 1);
    ///
    /// let nonexistant_player_id: PlayerId = 200;
    /// assert_eq!(player::qry::id(&state, nonexistant_player_id), 0);
    ///
    /// ```
    pub fn id(state: &State, player_id: PlayerId) -> Id {
        match exists(state, player_id) {
            true => entity::qry::id(state, player_id),
            false => 0
        }
    }

    /// QUERY > Get a player's `name` as String
    /// See `name` component for tests
    pub fn name(state: &State, player_id: PlayerId) -> String {
        name::qry::get(state, player_id)
    }
}