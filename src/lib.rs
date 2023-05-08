#![allow(dead_code)]
#![feature(trait_alias)]

pub mod prelude;


// High level

/// Commands are the highest level actions and interact with public Ids
pub mod command;
pub mod command_test;


// Mid level

/// Core library state and mid level actions used by Commands
pub mod state;
/// A set of action helpers and types
pub mod action;


// State components

pub mod registry;
pub mod component;
pub mod structure;
pub mod model;
pub mod subsys;




