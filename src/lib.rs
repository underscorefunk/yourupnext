#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![feature(trait_alias)]

pub mod prelude;

pub mod command;

// Mid level

pub mod state;
/// A set of action helpers and types
pub mod applicable;


// State components

pub mod registry;
pub mod component;
pub mod structure;
pub mod model;
pub mod queryable;
pub mod error;





