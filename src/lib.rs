#![feature(custom_derive, plugin)]
#![plugin(serde_macros, clippy)]
#![feature(test)]
#![warn(missing_docs)]

//! Library for building cellular automata.
//!
//! Cellular automaton is made by supplying some cell to
//! grid and "engine". Grid stores all cells, chooses
//! how to iterate them, how to split itself etc.
//! Engine runs grid evolution in some way and passes
//! updates to so-called consumers.
//!
//! Consumers are interface to outer world.
//! They can be GUI, Web or simple file writers.

extern crate serde;
extern crate scoped_threadpool;

#[cfg(test)]
extern crate bincode;
#[cfg(test)]
extern crate test;
#[cfg(test)]
extern crate num_cpus;

pub mod grid;
pub mod engine;
pub mod traits;
mod utils;

mod examples;
