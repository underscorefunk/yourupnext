use crate::prelude::*;

#[derive(Debug,Eq,PartialEq)]
pub enum Cmd {

    // Player
    AddPlayer(PubId, String),
    RenamePlayer(PubId, String),
    RemovePlayer(PubId),

    // Character
    AddCharacter(PubId, String),
    AssignCharacterPlayer(PubId, PubId),
    RenameCharacter(PubId, String),
    RemoveCharacter(PubId),

    // Subsys Round
    // AddTurn(PubId, Initiative),
    // RemoveTurn(PubId),
    // OrderTurnsByInitiative,
    // UpdateTurn(PubId, TurnStatus),
    // MoveTurn(PubId, i8),
    // MoveTurnBefore(PubId, PubId),
    //
    // ResetTurn(PubId),
    // ActivateTurn(PubId),
    // InterruptTurn(PubId),
    // ActivateDelayedTurn(PubId, PubId),
    // ResumeTurn(PubId),
    // CompleteTurn(PubId),
    // SkipTurn(PubId),
    // DelayTurn(PubId),
    // TiggerDelayedTurn(PubId, usize),
    // NextRound,

}

impl Applicable<Cmd> for Cmd {

    fn apply(self, state: State) -> ActionResult<State> {
        match self {

            // Player
            Cmd::AddPlayer(pub_id, name) => player::add(state, pub_id, name),
            Cmd::RenamePlayer(pub_id, name) => player::rename(state, pub_id, name),
            Cmd::RemovePlayer(pub_id) => player::remove(state, pub_id),

            // Character
            Cmd::AddCharacter(pub_id, name) => character::add(state, pub_id, name),
            Cmd::AssignCharacterPlayer(c_pub_id, p_pub_id) => character::assign_player(state, c_pub_id, p_pub_id),
            Cmd::RenameCharacter(pub_id, name) => character::rename(state, pub_id, name),
            Cmd::RemoveCharacter(pub_id) => character::remove(state, pub_id),

        }

    }

    fn apply_default(self) -> ActionResult<State> {
        self.apply( State::default() )
    }

    fn apply_all(actions: Vec<Cmd>, state: State) -> ActionResult<State> {
        apply_actions(actions, state)
    }

    fn apply_all_default(actions: Vec<Cmd>) -> ActionResult<State> {
        Self::apply_all(actions, State::default() )
    }
}


impl ApplicableVec<Cmd> for Vec<Cmd> {
    fn apply(self, state: State) -> ActionResult<State> {
        Cmd::apply_all(self, state)
    }

    fn apply_default(self) -> ActionResult<State> {
        Cmd::apply_all_default(self)
    }
}

