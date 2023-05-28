# Your Up Next
An RPG application that lets you use as many or few rules or points of data as you want.



## Concepts
- **Scenario:** This is your play session. It encompasses everything that happens through the 
  course of play.
- **Player:** A player is a participant in the scenario and has the ability to control characters, 
  items they control, etc
- **Entities:**
  - Character: A player
  - Item: TBD
  - Location: TBD
- **Turn:** Handles the sequence of play, ordering *CharacterTurns* and *RoundEvents*. 
- **Effect:** Applies modifiers to entities, similar to a 'component' in an ECS.

## Architecture
The library uses an event sourcing approach.


An entity is just an ID

Player (Entity)
Character (Entity)
Item (Entity)
Location (Entity)
Effect (Entity)

Systems need to be names


Code Style
- Code documentation
  - Bold Ideas **Command**, **Query**, etc
  - Back tick structs, enums, nouns, and code references
- Do not test functions that are just facades. Only test the lowest version on the strata
- The prelude should expose common Traits, Structs, Enums. Functions should be namespaced as 
modules to avoid variable name collisions.
- Commands should be processed as pipelines to have errors bubble up
- Queries are done as functions


- A parent can `capture` a child
- A child is `assigned` to a parent


## Reminders
- Implement "PubId.into(&State) for Id" and Id.into(&State) for PubId
- [] Should start of round and end of round have timers to things that require resolution?
- [] Implement turn counting. When an entity completes its turn, the count should be incremented. 
Pausing/holding and skipping should not be counted.

## Neat Ideas
- Use combat log and descriptive prompts to produce auto generated artwork for what's happening
- Describe locations in english or rough draw maps with layers and have stable diffusion fill 
  in the tile sets

- Create initiative groups for when the part is split

// ---------
// Consider that events have time stamps and game id to create hashes for IDs to make everything a pure function
// but unique and DB storable

// Some actions may require user input
// We'll need to make action to open the request and another action to handle closing it
// Triggering an IO being open should prevent anytihng other than an input cancel or input data
// ----------------------------------------------------------------
// Create events source list with undo and redo
// ----------------------------------------------------------------
//



- Public Identifiers (PubId) have aliases for context so that enum variant data that only displays the type can hint at the usage.
-