use std::collections::HashMap;
use crate::event;
use crate::entity;
use crate::round;

/// The effect system allows us to register a component. The component can then have effects applied to it.
///
/// Effects are used to create a dynamic data model for entities.
/// Everything that impacts a used "value" should be an effect.
///
/// Some examples include:
/// - Base player stats or level up stat changes (permanent)
/// - Equipment (permanent)
/// - Positional changes, standing/prone (permanent)
/// - Buffs and spell abilities (temporary)
/// - Single
///
/// Short term:
/// 1 - We name an effect and describe how it couples entities together
/// 2 - We author and/or clean up effects based on game play
///
/// How effects will work:
/// 1 - we describe one or more nouns
/// 2 - we describe a change to the noun in some way
///     (may be calculated or not or based on other nouns)
/// 3 - we apply the effect going forward if the noun is queried
///
/// Future ideas. Load character sheets as presets of effects
///
/// A future refactor will convert "String" into a parsable string
/// that can become a set of generic modifiers, values, etc
///
/// Guards required!!!
/// - What if a player is removed from the initiative order. Their turn based things need to end

pub type Id = usize;
pub type Name = String;

pub type Effects = HashMap<Id, Effect>;
pub type EntityEffects = HashMap<entity::Id, Vec<Id>>;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    /// Incrementing ids for effect identification
    pub next_effect_id: Id,

    /// The master list of effects keyed by Ids
    pub effects: Effects,

    /// Source -> Effect
    pub entity_created_effects: EntityEffects,

    /// Target -> Effect
    pub entity_active_effects: EntityEffects,

}

impl Default for State {
    fn default() -> Self {
        Self {
            next_effect_id: 0,
            effects: Effects::default(),
            entity_created_effects: EntityEffects::default(),
            entity_active_effects: EntityEffects::default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Source {
    None,
    // Entity,
    // Location(String),
    // Object(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Target {
    None,
    // SelfEntity,
    // OtherEntity,
    // OtherEntities,
    // Location(String),
    // Object(String),
}

// Duration may need to change to allow for "start of" and "end of"
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Duration {
    None,
    // UntilNextRound,
    //UpToNthRound(round::RoundCount),
    // ends at start of
    // ThroughNthRound(round::RoundCount),
    // ends at end of
    // Forever,
    // WhileSourceExists(entity::Id),
    // WhileTargetExists(entity::Id),
    // WhileSourceAndTargetExists(entity::Id, entity::Id),
    // WhileHaveObject(String),
    // WhileAtLocation(String),
}

/// The effect system will need to be able to be updated in response to
/// events that occur elsewhere. effect::Event enum allows us to list
/// those
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Event {
    // EntityRemoved
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Effect {
    label: String,
    duration: Duration,
}

impl Default for Effect {
    fn default() -> Self {
        Self {
            duration: Duration::None,
            label: "Unknown effect".to_string(),
        }
    }
}


pub fn add(
    state: event::State,
    source: Source,
    target: Target,
    duration: Duration,
    label: String,
) -> event::ActionResult {
    if verify_source(&state, &source).is_err() {
        return Err("Unable to verify source".to_string());
    }

    if verify_target(&state, &target).is_err() {
        return Err("Unable to verify target".to_string());
    }

    if label.is_empty() {
        return Err("Effect label can not be empty".to_string());
    }

    let effect_id = state.effect.next_effect_id;
    let next_effect_id = effect_id + 1;

    // Insert the effect
    let mut effects = state.effect.effects;
    effects.insert(
        effect_id,
        Effect{
            duration,
            label
        }
    );

    // Create effect associations

    Ok(event::State {
        effect: State {
            next_effect_id,
            effects,
            ..state.effect
        },
        ..state
    })
}

fn verify_source(state: &event::State, source: &Source) -> Result<(), String> {
    // Verify entity source
    todo!()
}

fn verify_target(state: &event::State, source: &Target) -> Result<(), String> {
    // Verify entity target
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_effect_action_ok() {
        let actions = vec![
            event::Action::AddEntity("Jenna".to_string()),      // id 0
            event::Action::AddTurn(0, 10),
            event::Action::AddEffect( source::None, target::None, )
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
}
