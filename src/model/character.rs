/// # Character Model

use crate::prelude::*;

/// ## Character > Command Applicables (Cmd)
/// A simple wrapper for character commands so that they can be composed together with other pipelines.
/// `Cmd` is a facade for `cmd` functions.
#[derive(Debug, Eq, PartialEq)]
pub enum Character {
    Add(PubId, Name),
    Remove(PubId),
    Rename(PubId, Name),
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
    /// let name = "ACharacter".to_string();
    /// let state = vec![
    ///     Character::Add(pub_id, name.clone())
    /// ].apply_to_default().unwrap();
    ///
    /// assert!(character::qry::exists(&state,pub_id));
    /// assert_eq!(character::qry::name(&state,pub_id), name.clone());
    /// ```
    pub fn add(state: State, character_pub_id: PubId, starting_name: String) -> CmdResult<State> {
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
    ///
    /// let state = vec![
    ///     Cmd::Character( Character::Add(character_pub_id,"ACharacter".to_string())),
    ///     Cmd::Player( Player::Add(player_pub_id,"APlayer".to_string()) ),
    ///     Cmd::Character( Character::AssignPlayer(character_pub_id,player_pub_id)),
    /// ].apply_to_default().unwrap();
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
    pub fn rename(state: State, character_pub_id: PubId, new_name: String) -> CmdResult<State> {
        entity::Entity::Name(character_pub_id, new_name).apply_to(state)
    }

    /// COMMAND > Remove a character
    ///```
    /// use yourupnext::prelude::*;
    /// let state = vec![
    ///    Cmd::AddCharacter(1,"ACharacter".to_string()),
    ///    Cmd::RemoveCharacter(1)
    /// ].apply_to_default().unwrap();
    ///
    /// assert_eq!(character::id(&state,1), 0);
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
    /// let state = vec![
    ///     Cmd::AddCharacter(100, "ACharacter".to_string()),
    /// ].apply_to_default().unwrap();
    ///
    /// assert!(character::exists(&state,100));
    /// assert!(! character::exists(&state,1));
    /// ```
    pub fn exists(state: &State, character_pub_id: PubId) -> bool {
        is_character(state, character_pub_id)
    }

    /// Query > Check if a private Id is a valid character entity type
    /// ```
    /// use yourupnext::prelude::*;
    /// let state = vec![
    ///     Cmd::AddCharacter(100, "ACharacter".to_string()),
    ///     Cmd::AddPlayer(102, "APlayer".to_string())
    /// ].apply_to_default().unwrap();
    ///
    /// assert!(character::is_character(&state,100));
    /// assert!(! character::is_character(&state,102));
    /// ```
    pub fn is_character(state: &State, character_pub_id: Id) -> bool {
        entity_type::qry::is(state, character_pub_id, EntityType::Character)
    }

    /// QUERY > Get the Id of a character
    /// ```
    /// use yourupnext::prelude::*;
    /// let state = vec![
    ///     Cmd::AddCharacter(100, "ACharacter".to_string()),
    ///     Cmd::AddPlayer(102, "APlayer".to_string())
    /// ].apply_to_default().unwrap();
    ///
    /// assert_eq!(character::id(&state,100), 1);
    ///
    /// // A missing internal id is a 0, which will always
    /// // fail to match entities. This is done to avoid
    /// // unwrapping every query. It's a common query.
    /// assert_eq!(character::id(&state,1), 0);
    ///
    /// // Incorrect entity types will yield a zero id as well.
    /// assert_eq!(character::id(&state,102), 0);
    /// ```
    pub fn id(state: &State, character_pub_id: PubId) -> Id {
        match entity::qry::is(state, character_pub_id, EntityType::Character) {
            true => entity::qry::id(state, character_pub_id),
            false => 0
        }
    }

    /// QUERY > Get the Public Id (`pub_id`) of a character's `Player`
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = vec![
    ///     Cmd::Player( Player::Add(1,"APlayer".to_string()) ),
    ///     Cmd::Character( Character::Add(2,"ACharacter".to_string()) ),
    ///     Cmd::Character( Character::AssignPlayer(2,1) ),
    /// ].apply_to_default().unwrap();
    ///
    /// // PubId 1 refers to a player, its "player" id is None
    /// assert_eq!(character::qry::player(&state,1), None);
    ///
    /// // PubId 2 refers to a character, its "player" id is 1
    /// assert_eq!(character::qry::player(&state,2), Some(1));
    /// ```
    pub fn player(state: &State, character_pub_id: PubId) -> Option<PubId> {
        if ! is_character(state, character_pub_id) {
            return None;
        }
        let child_id = id(state, character_pub_id);
        let parent_id = state.character_player.parent(child_id).unwrap_or(0);
        entity::qry::pub_id(state, parent_id)
    }

    /// QUERY > Get the `Name` of a character
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = Cmd::AddCharacter(1,"ACharacter".to_string())
    ///     .apply_to_default()
    ///     .unwrap();
    ///
    /// // PubId 1 refers to a player, its "player" id is None
    /// assert_eq!(character::name(&state,1), "ACharacter".to_string());
    ///
    /// // Unnamed characters will return an empty string
    /// assert_eq!(character::name(&state,2), String::new() );
    /// ```
    pub fn name(state: &State, character_pub_id: PubId) -> String {
        name::qry::get(state, character_pub_id)
    }
}