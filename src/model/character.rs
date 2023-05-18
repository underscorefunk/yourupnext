/// # Character Model

use crate::prelude::*;

/// ## Character > Command Applicables (Cmd)
/// A simple wrapper for character commands so that they can be composed together with other pipelines.
/// `Cmd` is a facade for `cmd` functions.
#[derive(Debug, Eq, PartialEq)]
pub enum Character {
    Add(PubId, &'static Name),
    Remove(PubId),
    Rename(PubId, &'static Name),
    AssignPlayer(PubId, PubId),
}

impl Applicable for Character {
    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {
            Character::Add(pub_id, name) => cmd::add(state, pub_id, name),
            Character::Remove(pub_id) => cmd::remove(state, pub_id),
            Character::Rename(pub_id, name) => cmd::rename(state, pub_id, name),
            Character::AssignPlayer(pub_id, player_pub_id) => cmd::assign_player(state, pub_id, player_pub_id),
        }
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to(State::default())
    }
}

/// ## Character > Command (cmd)

pub mod cmd {
    use super::*;

    /// COMMAND > Add a character
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id = 100;
    /// let state = State::default()
    ///     .apply(Character::Add(pub_id, "ACharacter"))
    ///     .unwrap();
    ///
    /// assert!(character::qry::exists(&state,pub_id));
    /// assert_eq!(character::qry::name(&state,pub_id), "ACharacter".to_string());
    /// ```
    pub fn add(state: State, character_pub_id: PubId, starting_name: &'static Name) -> CmdResult<State> {
        vec![
            Entity::Add(character_pub_id),
            Entity::Classify(character_pub_id, EntityType::Character),
            Entity::Name(character_pub_id, starting_name),
        ].apply_to(state)
    }

    /// COMMAND > Assign a character to a player
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let player_pub_id = 100;
    /// let character_pub_id = 200;
    /// let state = State::default()
    ///     .apply( Character::Add(character_pub_id,"ACharacter"))
    ///     .apply( Player::Add(player_pub_id,"APlayer") )
    ///     .apply( Character::AssignPlayer(character_pub_id,player_pub_id))
    ///     .unwrap();
    ///
    /// println!("{:#?}", state);
    /// println!("{:?}", character_pub_id);
    /// println!("{:?}", character::qry::player(&state,character_pub_id) );
    /// assert_eq!(character::qry::player(&state,character_pub_id), Some(player_pub_id));
    ///
    /// ```
    pub fn assign_player(mut state: State, character_pub_id: PubId, player_pub_id: PubId) -> CmdResult<State> {

        if !entity_type::qry::is(&state, character_pub_id, EntityType::Character) {
            return Err("Can not assign player to character when the target character isn't a Character entity type.".to_string());
        }

        if !entity_type::qry::is(&state, player_pub_id, EntityType::Player) {
            return Err("Can not assign player to character when the target player isn't a Player entity type.".to_string());
        }

        state.character_player.set_parent(character_pub_id, player_pub_id)?;

        Ok(state)
    }

    /// COMMAND > Rename a character
    pub fn rename(state: State, character_pub_id: PubId, new_name: &'static Name) -> CmdResult<State> {
        Entity::Name(character_pub_id, new_name).apply_to(state)
    }

    /// COMMAND > Remove a character
    ///```
    /// use yourupnext::prelude::*;
    /// let state = State::default()
    ///    .apply( Character::Add(1,"ACharacter") )
    ///    .apply( Character::Remove(1) )
    ///    .unwrap();
    ///
    /// assert_eq!(character::qry::id(&state,1), 0);
    /// ```
    pub fn remove(mut state: State, character_pub_id: PubId) -> CmdResult<State> {
        let id = character::qry::id(&state, character_pub_id);

        // We do not bubble the error because a parent might not exist and
        // that's ok!
        let _ = state.character_player.remove_parent(id);
        entity::cmd::remove(state, character_pub_id.clone())
    }
}


/// ## Character > Query (qry)

pub mod qry {
    use super::*;

    /// QUERY > Check if a character exists
    /// See `entity_type` component for tests
    pub fn exists(state: &State, character_pub_id: PubId) -> bool {
        entity_type::qry::is(state, character_pub_id, EntityType::Character)
    }

    /// QUERY > Get a character's `Id`
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id: PubId = 100;
    /// let state = Character::Add(pub_id,"ACharacter")
    ///     .apply_to_default()
    ///     .unwrap();
    ///
    /// assert_eq!(character::qry::id(&state, pub_id), 1);
    ///
    /// let nonexistant_pub_id: PubId = 200;
    /// assert_eq!(character::qry::id(&state, nonexistant_pub_id), 0);
    ///
    /// ```
    pub fn id(state: &State, player_pub_id: PubId) -> Id {
        match exists(state, player_pub_id) {
            true => entity::qry::id(state, player_pub_id),
            false => 0
        }
    }

    /// QUERY > Get the Public Id (`pub_id`) of a character's `Player`
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply( Player::Add(1,"APlayer") )
    ///     .apply( Character::Add(2,"ACharacter") )
    ///     .apply( Character::AssignPlayer(2,1) )
    ///     .unwrap();
    ///
    /// // PubId 1 refers to a player, its "player" id is None
    /// assert_eq!(character::qry::player(&state,1), None);
    ///
    /// // PubId 2 refers to a character, its "player" id is 1
    /// assert_eq!(character::qry::player(&state,2), Some(1));
    /// ```
    pub fn player(state: &State, character_pub_id: PubId) -> Option<PubId> {
        if ! exists(state, character_pub_id) {
            return None;
        }
        let child_id = id(state, character_pub_id);
        let parent_id = state.character_player.parent(child_id).unwrap_or(0);
        entity::qry::pub_id(state, parent_id)
    }

    /// QUERY > Get a characters's `name` as String
    /// See `name` component for tests
    pub fn name(state: &State, player_pub_id: PubId) -> String {
        name::qry::get(state, player_pub_id)
    }
}