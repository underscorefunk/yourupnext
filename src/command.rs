use crate::prelude::*;

/// The point of the top level Cmd module is so that all commands can be mixed and matched
/// providing a composable API for enacting state changes.

#[derive(Debug,Eq,PartialEq)]
pub enum Cmd {
    Set(Vec<Cmd>),
    Player(Player),
    Character(Character),

    // Player Model
    AddPlayer(PubId, &'static Name),
    RenamePlayer(PubId, &'static Name),
    RemovePlayer(PubId),

    // Character Model
    AddCharacter(PubId, &'static Name),
    AssignCharacterPlayer(PubId,PubId),
    RenameCharacter(PubId, &'static Name),
    RemoveCharacter(PubId),



}

impl Applicable for Cmd {

    fn apply_to(self, state: State) -> CmdResult<State> {
        match self {

            // Command sets
            Cmd::Set( cmd_set ) => cmd_set.apply_to(state),

            // Model commands
            Cmd::Player( cmd ) => cmd.apply_to(state),
            Cmd::Character( cmd ) => cmd.apply_to(state),

            // Player Model
            Cmd::AddPlayer(pub_id, name) => Player::Add( pub_id, name).apply_to(state),
            Cmd::RenamePlayer(pub_id, name) => Player::Rename(pub_id,name).apply_to(state),
            Cmd::RemovePlayer(pub_id) => state
                //RemoveCharacterPlayers
                .apply(Player::Remove(pub_id) ),

            // Character Model
            Cmd::AddCharacter(pub_id, name) => Character::Add( pub_id, name).apply_to(state),
            Cmd::AssignCharacterPlayer(c_pub_id, p_pub_id) => Character::AssignPlayer( c_pub_id, p_pub_id).apply_to(state),
            Cmd::RenameCharacter(pub_id, name) => Character::Rename( pub_id, name).apply_to(state),
            Cmd::RemoveCharacter(pub_id) => Character::Remove(pub_id).apply_to(state),


        }

    }

    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to( State::default() )
    }

}

