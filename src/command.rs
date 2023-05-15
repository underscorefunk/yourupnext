use crate::prelude::*;

/// The point of the top level Cmd module is so that all commands can be mixed and matched
/// providing a composable API for enacting state changes.

#[derive(Debug,Eq,PartialEq)]
pub enum Cmd {
    Set(Vec<Cmd>),
    Player(Player),
    Entity(Entity),
    Character(Character),
}

impl Applicable for Cmd {

    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {

            Cmd::Set( cmd_set ) => cmd_set.apply_to(state),

            Cmd::Entity( cmd ) => cmd.apply_to(state),
            Cmd::Player( cmd ) => cmd.apply_to(state),
            Cmd::Character( cmd ) => cmd.apply_to(state)

            // Player
            // Cmd::AddPlayer(pub_id, name) => player::Cmd::Add( pub_id, name).apply_to(state),
            // Cmd::RenamePlayer(pub_id, name) => player::rename(state, pub_id, name),
            // Cmd::RemovePlayer(pub_id) => player::remove(state, pub_id),
            //
            // // Character
            // Cmd::AddCharacter(pub_id, name) => character::add(state, pub_id, name),
            // Cmd::AssignCharacterPlayer(c_pub_id, p_pub_id) => character::assign_player(state, c_pub_id, p_pub_id),
            // Cmd::RenameCharacter(pub_id, name) => character::rename(state, pub_id, name),
            // Cmd::RemoveCharacter(pub_id) => character::remove(state, pub_id),

        }

    }

    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to( State::default() )
    }

}

