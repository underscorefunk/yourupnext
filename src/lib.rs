#![allow(dead_code)]
mod event;
mod scenario;
mod player;
mod entity;
mod round;
mod effect;

// Consider that events have time stamps and game id to create hashes for IDs to make everything a pure function
// but unique and DB storable

// Some actions may require user input
// We'll need to make action to open the request and another action to handle closing it
// Triggering an IO being open should prevent anytihng other than an input cancel or input data
// ----------------------------------------------------------------
// Create events source list with undo and redo
// ----------------------------------------------------------------
