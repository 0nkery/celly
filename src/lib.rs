#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![warn(missing_docs)]

//! Library for building cellular automata.
//!
//! Cellular automaton is made by supplying some cell to
//! grid and "engine". Grid stores all cells, chooses
//! how to iterate them, how to split itself etc.
//! Engine runs grid evolution in some way and passes
//! updates to so-called consumers.
//!
//! Consumers are interface parts to outer world.
//! They can be GUI, Web or simple file writers.

extern crate serde;
#[cfg(test)]
extern crate bincode;

pub mod grid;
pub mod engine;
pub mod traits;
mod utils;

mod examples;
