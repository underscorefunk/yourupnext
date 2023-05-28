/// # Character Model

use crate::prelude::*;

/// ## Character > Command Applicables (Cmd)
/// A simple wrapper for character commands so that they can be composed together with other pipelines.
/// `Cmd` is a facade for `cmd` functions.

pub type CharacterId = PubId;

#[derive(Debug, Eq, PartialEq)]
pub enum Character {
    Add(CharacterId, &'static Name),
    Remove(CharacterId),
    Rename(CharacterId, &'static Name),
    AssignPlayer(CharacterId, PlayerId),
    RemovePlayer(CharacterId),
    RemovePlayerFromAll(PlayerId)
}

impl Applicable for Character {
    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {
            Character::Add(pub_id, name) => cmd::add(state, pub_id, name),
            Character::Remove(pub_id) => cmd::remove(state, pub_id),
            Character::Rename(pub_id, name) => cmd::rename(state, pub_id, name),
            Character::AssignPlayer(pub_id, player_pub_id) => cmd::assign_player(state, pub_id, player_pub_id),
            Character::RemovePlayer(pub_id) => cmd::remove_player(state, pub_id),
            Character::RemovePlayerFromAll(pub_id) => cmd::remove_player_form_all(state, pub_id)
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
    /// assert_eq!(character::qry::player(&state,character_pub_id), Some(100));
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

        let character_id = entity::qry::id(&state, character_pub_id);
        let player_id = entity::qry::id(&state, player_pub_id);

        state.character_player.set_parent(character_id, player_id)?;

        Ok(state)
    }

    /// COMMAND > Remove a character's assigned player
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let player_pub_id = 100;
    /// let character_pub_id = 200;
    /// let state = State::default()
    ///     .apply( Character::Add(character_pub_id,"ACharacter"))
    ///     .apply( Player::Add(player_pub_id,"APlayer") )
    ///     .apply( Character::AssignPlayer(character_pub_id,player_pub_id))
    ///     .apply( Character::RemovePlayer(character_pub_id) )
    ///     .unwrap();
    ///
    /// assert_eq!( character::qry::player(&state, character_pub_id), None);
    /// ```
    pub fn remove_player(mut state: State, character_pub_id:PubId) -> CmdResult<State> {
        if !entity_type::qry::is(&state, character_pub_id, EntityType::Character) {
            return Err("Can not remove character player for non character entity".to_string());
        }
        let player_pub_id = qry::player(&state, character_pub_id);

        if player_pub_id.is_none() {
            return Err("Can not remove character player where character didn't have a player assigned".to_string());
        }
        let character_id = entity::qry::id(&state, character_pub_id);

        state.character_player.remove_parent(character_id)?;
        Ok(state)

    }

    /// COMMAND > Remove all players assigned to any character
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply( Player::Add(100,"APlayer") )
    ///     .apply( Character::Add(200,"ACharacter"))
    ///     .apply( Character::AssignPlayer(200,100))
    ///     .apply( Character::Add(300,"BCharacter"))
    ///     .apply( Character::AssignPlayer(300,100))
    ///     .unwrap();
    ///
    /// assert_eq!(character::qry::player(&state, 200), Some(100));
    /// assert_eq!(character::qry::player(&state, 300), Some(100));
    ///
    /// let state = state.apply( Character::RemovePlayerFromAll(100)).unwrap();
    ///
    /// assert_eq!(character::qry::player(&state, 200), None);
    /// assert_eq!(character::qry::player(&state, 300), None);
    ///
    /// ```
    pub fn remove_player_form_all(mut state: State, player_pub_id: PubId) -> CmdResult<State> {

        if !entity_type::qry::is(&state, player_pub_id, EntityType::Player) {
            return Err("Can not remove any instances of a player being assigned to characters for non-player entity".to_string());
        }

        let player_id = entity::qry::id(&state, player_pub_id);

        state.character_player.free_children_from(player_id)?;

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
    ///    .apply( Character::Add(100,"ACharacter") )
    ///    .apply( Character::Remove(100) )
    ///    .unwrap();
    ///
    /// assert_eq!(character::qry::id(&state,100), 0);
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
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id: PubId = 123;
    /// let state = Character::Add(pub_id,"ACharacter")
    ///     .apply_to_default()
    ///     .unwrap();
    ///
    /// assert!(character::qry::exists(&state, pub_id));
    /// ```
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
    pub fn id(state: &State, character_pub_id: PubId) -> Id {
        match exists(state, character_pub_id) {
            true => entity::qry::id(state, character_pub_id),
            false => 0
        }
    }

    /// QUERY > Get the Public Id (`pub_id`) of a character's `Player`
    /// ```
    /// use yourupnext::prelude::*;
    /// let player_public_id = 100;
    /// let character_public_id = 200;
    /// let state = State::default()
    ///     .apply( Player::Add(player_public_id,"APlayer") )
    ///     .apply( Character::Add(character_public_id,"ACharacter") )
    ///     .apply( Character::AssignPlayer(character_public_id,player_public_id) )
    ///     .unwrap();
    ///
    /// // PubId 100 refers to a player, its "player" id is None
    /// assert_eq!(character::qry::player(&state,player_public_id), None);
    ///
    /// // PubId 2 refers to a character, its "player" id is 1
    /// assert_eq!(character::qry::player(&state,character_public_id), Some(player_public_id));
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