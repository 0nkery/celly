#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
#[cfg(test)]
extern crate bincode;

pub mod grid;
pub mod engine;
pub mod traits;

mod examples;
mod test_helpers;
