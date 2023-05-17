/// # Player Model

use crate::prelude::*;

/// ## Player > Command Applicables(Cmd)
/// A simple wrapper for player commands so that they can be composed together with other pipelines.
/// `Cmd` is a facade for `cmd` functions.

#[derive(Debug,Eq,PartialEq)]
pub enum Player {
    Add(PubId, &'static Name),
    Remove(PubId),
    Rename(PubId, &'static Name)
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
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id: PubId = 100;
    /// let state = Player::Add(pub_id,"APlayer")
    ///     .apply_to_default()
    ///     .unwrap();
    ///
    /// assert!(player::qry::exists(&state,pub_id));
    /// assert_eq!(player::qry::name(&state,pub_id), "APlayer");
    ///
    /// ```
    pub fn add(state: State, player_pub_id: PubId, starting_name: &'static Name) -> CmdResult<State> {
        vec![
            Entity::Add(player_pub_id),
            Entity::Classify(player_pub_id, EntityType::Player),
            Entity::Name(player_pub_id, starting_name),
        ].apply_to(state)
    }

    /// COMMAND > Remove a player
    /// See Entity::Remove for tests
    pub fn remove(state: State, player_pub_id: PubId) -> CmdResult<State> {
        Entity::Remove(player_pub_id).apply_to(state)
    }

    /// COMMAND > Rename a player
    /// See Entity::Name for tests
    pub fn rename(state: State, player_pub_id: PubId, new_name: &'static Name) -> CmdResult<State> {
        Entity::Name(player_pub_id, new_name).apply_to(state)
    }

}

/// ## Character > Query (qry)

pub mod qry {
    use super::*;

    /// QUERY > Check if a player exists
    /// See `entity_type` component for tests
    pub fn exists(state: &State, player_pub_id: PubId) -> bool {
        entity_type::qry::is(state, player_pub_id, EntityType::Player)
    }

    /// QUERY > Get a player's `Id`
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id: PubId = 100;
    /// let state = Player::Add(pub_id,"APlayer")
    ///     .apply_to_default()
    ///     .unwrap();
    ///
    /// assert_eq!(player::qry::id(&state, pub_id), 1);
    ///
    /// let nonexistant_pub_id: PubId = 200;
    /// assert_eq!(player::qry::id(&state, nonexistant_pub_id), 0);
    ///
    /// ```
    pub fn id(state: &State, player_pub_id: PubId) -> Id {
        match exists(state, player_pub_id) {
            true => entity::qry::id(state, player_pub_id),
            false => 0
        }
    }

    /// QUERY > Get a player's `name` as String
    /// See `name` component for tests
    pub fn name(state: &State, player_pub_id: PubId) -> String {
        name::qry::get(state, player_pub_id)
    }
}