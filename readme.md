# Your Up Next
An RPG application that lets you use as many or few rules as you want.

It is build around the idea of:

- Enities (things that can 'do') "People"
- Locations (things that can be 'in' or 'at') "Places"
- Objects (things that can be 'had') "Things"

Only entities are in the initiative/turn order.

Properties of these three distinct types are set by applying effects.

At some point I'll need to figure out how to allow entities to also have location and object 
qualities.

## Reminders
- [] Should start of round and end of round have timers to things that require resolution?
- [] Implement turn counting. When an entity completes its turn, the count should be incremented. 
Pausing/holding and skipping should not be counted.

## Neat Ideas
- Use combat log and descriptive prompts to produce auto generated artwork for what's happening
- Describe locations in english or rough draw maps with layers and have stable diffusion fill 
  in the tile sets