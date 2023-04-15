use std::collections::HashMap;
use crate::event;
use crate::entity;

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

pub type Id = usize;
pub type Name = String;

pub type Effects = HashMap<entity::Id, Effect>;
pub type EntitiesEffects = HashMap<entity::Id, Id>;
pub type Ids = Vec<Id>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {

    /// Incrementing ids for effect identification
    pub next_effect_id: Id,

    /// The master list of effects keyed by Ids
    pub effects: Effects,

    /// Effect sources/authors
    pub sources: EntitiesEffects,

    /// Effect targets
    pub targets: EntitiesEffects,

    /// Effects that will be ended if the source disappears
    pub source_bound_effects: Ids,

    /// Effects that will be ended if the target disappears
    pub target_bound_effects: Ids,
}

impl Default for State {
    fn default() -> Self {
        Self {
            next_effect_id: 0,
            effects: Effects::default(),
            sources: EntitiesEffects::default(),
            targets: EntitiesEffects::default(),
            source_bound_effects: Ids::default(),
            target_bound_effects: Ids::default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Duration {
    UntilNextTurn,
    UntilNTurns(u8),
    Permanent,
    WhileSourceExists(entity::Id),
    WhileTargetExists(entity::Id),
    WhileSourceAndTargetExists(entity::Id, entity::Id),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Effect {
    duration: Duration,
    label: String,
}

// @todo - next steps are to build the "Held action, until next turn" effect