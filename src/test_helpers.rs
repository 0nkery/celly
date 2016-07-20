#![cfg(test)]
use bincode::SizeLimit;
use bincode::serde::{deserialize, serialize};

use traits::Cell;


pub fn to_cell<Cin: Cell, Cout: Cell>(cell: &Cin) -> Cout {
    let encoded: Vec<u8> = serialize(cell, SizeLimit::Infinite).unwrap();
    let cell: Cout = deserialize(&encoded[..]).unwrap();

    cell
}
