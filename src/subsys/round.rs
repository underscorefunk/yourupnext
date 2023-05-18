/// # Round Subsyste
/// Responsible for the order of play and tracking events over time.

use crate::prelude::*;

pub type RoundCount = usize;
pub type Sequence = Vec<Id>;
pub type Initiative = i8;
pub type TurnStates = Component<TurnStatus>;
pub type Initiatives = Component<Initiative>;
pub type TurnCounts = Component<u8>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TurnStatus {
    Available,
    Active,
    Paused,
    Completed,
    Skipped,
    Held(usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Round {
    pub count: RoundCount,
    pub sequence: Sequence,
    pub turn_states: TurnStates,
    pub initiatives: Initiatives,
}

impl Default for Round {
    fn default() -> Self {
        Self {
            count: 0,
            sequence: vec![],
            turn_states: TurnStates::default(),
            initiatives: Initiatives::default(),
        }
    }
}

impl Round {
    pub fn add_turn(&mut self, entity_id: Id, initiative: Initiative) -> CmdResult<()> {
        if self.sequence.contains(&entity_id) {
            return Err("Unable to add entity turn. It already exists in the sequence of play.".to_string());
        }

        self.initiatives.insert(entity_id, initiative)?;
        self.turn_states.insert(entity_id, TurnStatus::Available)?;
        self.sequence.push(entity_id);

        Ok(())
    }

    pub fn remove_turn(&mut self, entity_id: Id) -> CmdResult<()> {
        if !self.sequence.contains(&entity_id) {
            return Err("Unable to remove turn for entity that isn't in the round's sequence.".to_string());
        }

        self.sequence = self.sequence.clone()
            .into_iter()
            .filter(|e_id| e_id != &entity_id)
            .collect();
        self.turn_states.delete(entity_id)?;
        self.initiatives.delete(entity_id)?;

        Ok(())
    }

    pub fn order_turns_by_initiative(&mut self) -> CmdResult<()> {
        if self.sequence.len() < 2 {
            return Err("Two or more items must be in the round sequence to allow for ordering by initiative.".to_string());
        }

        if self.count != 0 {
            return Err("You can not order turns by initiative if play is underway.".to_string());
        }

        let mut sequence = self.initiatives.values.iter()
            .map(|(entity_id, initiative)| (*entity_id, *initiative))
            .collect::<Vec<(Id, Initiative)>>();

        sequence.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        self.sequence = sequence.iter()
            .map(|(entity_id, _)| *entity_id)
            .collect::<Sequence>();

        Ok(())
    }

    pub fn update_turn_state(&mut self, entity_id: Id, turn_status: TurnStatus) -> CmdResult<()> {
        if !self.sequence.contains(&entity_id) {
            return Err("Can not update status of a turn that doesn't exist in the sequence.".to_string());
        }

        self.turn_states.update(entity_id, turn_status)?;

        Ok(())
    }

    pub fn next_round(&mut self) -> CmdResult<()> {
        if self.turn_states.is_empty() {
            return Err("Can not proceed to next round. There are no turns in the sequence.".to_string());
        }
        for (_, turn_state) in self.turn_states.values.iter() {
            match *turn_state {
                TurnStatus::Completed => (),
                TurnStatus::Held(_) => (),
                TurnStatus::Skipped => (),
                _ => return Err("Can not proceed to next round. Some turns have not been completed, held, or skipped.".to_string())
            }
        }
        self.count += 1;
        self.transition_next_round_turn_states()?;

        Ok(())
    }

    fn transition_next_round_turn_states(&mut self) -> CmdResult<()> {
        self.turn_states.values = self.turn_states.values.clone().into_iter().map(
                |(entity_id, turn_status)| {
                    let new_turn_status = match turn_status {
                        TurnStatus::Held(round_count) => TurnStatus::Held(round_count + 1),
                        _ => TurnStatus::Available
                    };
                    (entity_id, new_turn_status)
                }
            ).collect();

        Ok(())
    }


    fn update_turn_order(
        &mut self,
        entity_id: Id,
        direction: i8,
    ) -> CmdResult<()> {
        if direction == 0 {
            return Err("Can not reorder a turn that should move zero spots in the order.".to_string());
        }

        let index = self.sequence.iter().position(|&seq_e_id| seq_e_id == entity_id);

        if index.is_none() {
            return Err("Can not update the order of an entity's turn that isn't in sequence.".to_string());
        }

        let index: usize = index.unwrap();
        let new_index = direction + index as i8;

        if new_index < 0 {
            return Err("Can't move turn that early in the round.".to_string());
        }

        if new_index >= self.sequence.len() as i8 {
            return Err("Can't move turn that late in the round.".to_string());
        }

        self.sequence.remove(index);
        self.sequence.insert(new_index as usize, entity_id);

        Ok(())
    }

    pub fn move_turn_before(
        &mut self,
        entity_id_to_move: Id,
        before_entity_id: Id,
    ) -> CmdResult<()> {
        if !self.sequence.contains(&entity_id_to_move) {
            return Err("Unable to move a nonexistant turn before another turn.".to_string());
        }
        if !self.sequence.contains(&before_entity_id) {
            return Err("Unable to move a turn to before a nonexistant turn.".to_string());
        }
        let entity_id_to_move_index = self.sequence.iter().position(|&seq_e_id| seq_e_id == entity_id_to_move).unwrap();
        let before_entity_id_index = self.sequence.iter().position(|&seq_e_id| seq_e_id == before_entity_id).unwrap();

        let forward_move_offset = match entity_id_to_move_index < before_entity_id_index {
            true => -1,
            false => 0
        };
        let distance: i8 = before_entity_id_index as i8 - entity_id_to_move_index as i8 + forward_move_offset;


        if distance == 0 {
            return Err("Can not move turn before another turn if it's already there.".to_string());
        }

        self.update_turn_order(entity_id_to_move, distance)?;

        Ok(())
    }

    pub fn activate_delayed_turn(
        &mut self,
        entity_id: Id,
        triggering_entity_id: Id,
    ) -> CmdResult<()> {
        if !self.sequence.contains(&entity_id) {
            return Err("Can not activate nonexistant delayed turn.".to_string());
        }

        if !self.sequence.contains(&triggering_entity_id) {
            return Err("Can not activate delayed turn by nonexistant triggering turn.".to_string());
        }

        let turn_status = self.turn_states.get(entity_id);

        if let Some(TurnStatus::Held(held_round_count)) = turn_status {
            let delayed_turn_position = self.sequence.iter().position(|&seq_e_id| seq_e_id == entity_id).unwrap();
            let triggering_turn_position = self.sequence.iter().position(|&seq_e_id| seq_e_id == triggering_entity_id).unwrap();

            let held_from_current_round = held_round_count == 0;
            let held_from_last_round = held_round_count == 1;

            if held_from_current_round && delayed_turn_position < triggering_turn_position ||
                held_from_last_round && triggering_turn_position < delayed_turn_position {
                self.update_turn_state(entity_id, TurnStatus::Active)?;
                return Ok(());
            }

            return Err("Attempting to activate a held turn, triggered by a turn that should happen before the turn was held.".to_string());
        }

        Err("Can not activate a turn that isn't held or doesn't have a turn status".to_string())
    }
}



/*
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn add_turn_action_ok() {
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::AddTurn(0, 10),
        ];

        let result = event::Action::apply_all(actions, event::State::default());

        let sequence: Sequence = vec![0];

        let mut turn_states: TurnStates = HashMap::new();
        turn_states.insert(0, TurnStatus::Available);

        let mut initiatives: Initiatives = HashMap::new();
        initiatives.insert(0, 10);

        match result {
            Ok(result) => {
                assert_eq!(result.round.sequence, sequence);
                assert_eq!(result.round.turn_states, turn_states);
                assert_eq!(result.round.initiatives, initiatives);
            }
            Err(_) => assert!(false) // should always fail
        }
    }

    #[test]
    fn add_turn_action_err_missing_entity() {
        assert!(
            event::Action::apply_all(
                vec![
                    event::Action::AddTurn(0, 10),
                ],
                event::State::default(),
            ).is_err()
        );
    }

    #[test]
    fn add_turn_action_err_duplicate_turn() {
        assert!(
            event::Action::apply_all(
                vec![
                    event::Action::AddEntity("Jenna".to_string()),
                    event::Action::AddTurn(0, 10),
                    event::Action::AddTurn(0, 10),
                ],
                event::State::default(),
            ).is_err()
        );
    }


    #[test]
    fn order_turns_by_initiative_action_ok() {
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),      // id 0
            event::Action::AddEntity("Jessica".to_string()),    // id 1
            event::Action::AddEntity("Kimberly".to_string()),   // id 2
            event::Action::AddTurn(0, 10),
            event::Action::AddTurn(1, -3),
            event::Action::AddTurn(2, 5),
            event::Action::OrderTurnsByInitiative,
        ];

        let result = event::Action::apply_all(actions, event::State::default());

        let sequence: Sequence = vec![1, 2, 0];

        match result {
            Ok(result) => {
                assert_eq!(result.round.sequence, sequence);
            }
            Err(_) => assert!(false) // should always fail
        }
    }


    #[test]
    fn order_turns_by_initiative_action_err_insufficient_entities() {
        assert!(
            event::Action::apply_all(
                vec![
                    event::Action::OrderTurnsByInitiative
                ],
                event::State::default(),
            ).is_err()
        );

        assert!(
            event::Action::apply_all(
                vec![
                    event::Action::AddEntity("Jenna".to_string()),
                    event::Action::OrderTurnsByInitiative,
                ],
                event::State::default(),
            ).is_err()
        );
    }


    #[test]
    fn update_turn_order_action_ok() {
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),      // id 0
            event::Action::AddEntity("Jessica".to_string()),    // id 1
            event::Action::AddEntity("Kimberly".to_string()),   // id 2
            event::Action::AddTurn(0, 5),
            event::Action::AddTurn(1, 10),
            event::Action::AddTurn(2, 15),
        ];

        let state_with_turns = event::Action::apply_all(actions, event::State::default());

        if state_with_turns.is_err() {
            assert!(false); // should always fail
            return;
        }

        let state_with_turns = state_with_turns.unwrap();

        let test_move = |entity_id: entity::Id,
                         direction: i8,
                         target_sequence: Vec<entity::Id>| {
            match event::Action::MoveTurn(entity_id, direction)
                .apply(state_with_turns.clone()) {
                Ok(result) => assert_eq!(result.round.sequence, target_sequence),
                Err(_) => assert!(false)
            }
        };

        // moving the first item right
        test_move(0, 1, vec![1, 0, 2]);
        test_move(0, 2, vec![1, 2, 0]);

        // moving the middle item left and right
        test_move(1, -1, vec![1, 0, 2]);
        test_move(1, 1, vec![0, 2, 1]);

        // moving the last item left
        test_move(2, -1, vec![0, 2, 1]);
        test_move(2, -2, vec![2, 0, 1]);
    }

    #[test]
    fn update_turn_order_action_err_missing_entity() {
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),      // id 0
            event::Action::AddEntity("Jessica".to_string()),    // id 1
            event::Action::AddEntity("Kimberly".to_string()),   // id 2
            event::Action::AddTurn(0, 5),
            event::Action::AddTurn(1, 10),
            event::Action::AddTurn(2, 15),
            event::Action::MoveTurn(3, 1),
        ];

        assert!(
            event::Action::apply_all(
                actions,
                event::State::default(),
            ).is_err());
    }

    #[test]
    fn update_turn_order_action_err_illegal_moves() {
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),      // id 0
            event::Action::AddEntity("Jessica".to_string()),    // id 1
            event::Action::AddEntity("Kimberly".to_string()),   // id 2
            event::Action::AddTurn(0, 5),
            event::Action::AddTurn(1, 10),
            event::Action::AddTurn(2, 15),
        ];

        let state_with_turns = event::Action::apply_all(actions, event::State::default());

        if state_with_turns.is_err() {
            assert!(false); // should always fail
            return;
        }

        let state_with_turns = state_with_turns.unwrap();

        let test_move_err = |entity_id: entity::Id, direction: i8| {
            assert!(
                event::Action::MoveTurn(entity_id, direction)
                    .apply(state_with_turns.clone())
                    .is_err()
            );
        };

        test_move_err(0, -1); // to negative index
        test_move_err(0, 0);  // no movement
        test_move_err(0, 3);  // to out of bounds past the end

        test_move_err(1, -2); // to negative index
        test_move_err(1, 2);  // to out of bounds past the end

        test_move_err(2, -3); // to negative index
        test_move_err(2, 1);  // to out of bounds past the end
    }

    #[test]
    fn remove_turn_action_ok() {
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::AddTurn(0, 10),
            event::Action::RemoveTurn(0),
        ];

        let result = event::Action::apply_all(actions, event::State::default());

        match result {
            Ok(result) => {
                assert_eq!(
                    result.round,
                    State::default()
                )
            }
            Err(_) => assert!(false) // should always fail
        }
    }

    #[test]
    fn remove_turn_action_err_no_turn_to_remove() {
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),
            event::Action::AddTurn(0, 10),
            event::Action::RemoveTurn(1),
        ];

        assert!(
            event::Action::apply_all(
                actions,
                event::State::default(),
            ).is_err()
        );
    }

    #[test]
    fn update_turn_status_action_ok() {
        let base_state = event::Action::apply_all(
            vec![
                event::Action::AddEntity("Jenna".to_string()),
                event::Action::AddTurn(0, 10),
            ],
            event::State::default(),
        ).unwrap();

        let target = |ts: TurnStatus| {
            let mut t: TurnStates = HashMap::new();
            t.insert(0, ts);
            t
        };

        assert_eq!(base_state.round.turn_states, target(TurnStatus::Available));

        let activated_state = event::Action::ActivateTurn(0).apply(base_state.clone()).unwrap();
        assert_eq!(activated_state.round.turn_states, target(TurnStatus::Active));
    }

    #[test]
    fn update_turn_status_action_err() {
        assert!(
            event::Action::ActivateTurn(0)
                .apply(event::State::default())
                .is_err()
        );
    }


    #[test]
    fn next_round_action_ok() {
        let next_round_state = event::Action::apply_all(
            vec![
                event::Action::AddEntity("Jenna".to_string()),
                event::Action::AddEntity("Jessica".to_string()),
                event::Action::AddEntity("Karla".to_string()),
                event::Action::AddTurn(0, 10),
                event::Action::AddTurn(1, 10),
                event::Action::AddTurn(2, 15),
                event::Action::CompleteTurn(0),
                event::Action::SkipTurn(1),
                event::Action::DelayTurn(2),
                event::Action::NextRound,
            ],
            event::State::default(),
        ).unwrap();
        assert_eq!(next_round_state.round.count, 1);

        let mut target_turn_states: TurnStates = HashMap::new();
        target_turn_states.insert(0,TurnStatus::Available);
        target_turn_states.insert(1,TurnStatus::Available);
        target_turn_states.insert(2,TurnStatus::Held(1));

        assert_eq!(next_round_state.round.turn_states, target_turn_states);
    }

    #[test]
    fn next_round_action_err_no_turns() {
        assert!(
            event::Action::apply_all(
                vec![
                    event::Action::AddEntity("Jenna".to_string()),
                    event::Action::NextRound,
                ],
                event::State::default(),
            ).is_err()
        );
    }

    #[test]
    fn next_round_action_err_no_round_completing_turns() {
        let state_with_turn = event::Action::apply_all(
            vec![
                event::Action::AddEntity("Jenna".to_string()),
                event::Action::AddTurn(0, 10),
                event::Action::NextRound,
            ],
            event::State::default(),
        );
        assert!(
            state_with_turn.is_err()
        );
    }


    #[test]
    fn trigger_held_action_ok() {
        let state = event::Action::apply_all(
            vec![
                event::Action::AddEntity("Jenna".to_string()),
                event::Action::AddEntity("Jessica".to_string()),
                event::Action::AddEntity("Karla".to_string()),
                event::Action::AddTurn(0, 10),
                event::Action::AddTurn(1, 10),
                event::Action::AddTurn(2, 10),
                event::Action::DelayTurn(0),
                event::Action::TiggerDelayedTurn(0, 2),
            ],
            event::State::default(),
        ).unwrap();

        let mut target_turn_states = TurnStates::new();
        target_turn_states.insert(0, TurnStatus::Active);
        target_turn_states.insert(1, TurnStatus::Available);
        target_turn_states.insert(2, TurnStatus::Paused);

        assert_eq!(state.round.turn_states, target_turn_states);

        assert_eq!(state.round.sequence, vec![1, 0, 2]);
    }

    #[test]
    fn reorder_turn_to_before_another_turn() {
        let state_with_three_turns = event::Action::apply_all(
            vec![
                event::Action::AddEntity("Jenna".to_string()),
                event::Action::AddEntity("Jessica".to_string()),
                event::Action::AddEntity("Karla".to_string()),
                event::Action::AddTurn(0, 10),
                event::Action::AddTurn(1, 10),
                event::Action::AddTurn(2, 10),
            ],
            event::State::default(),
        ).unwrap();

        let state_with_relative_move = move_turn_before(
            state_with_three_turns.clone(),
            0,
            2,
        ).unwrap();

        assert_eq!(
            state_with_relative_move.round.sequence,
            vec![1, 0, 2]
        );

        let state_with_relative_move = move_turn_before(
            state_with_three_turns.clone(),
            2,
            0,
        ).unwrap();

        assert_eq!(
            state_with_relative_move.round.sequence,
            vec![2, 0, 1]
        );
    }

}
*/
