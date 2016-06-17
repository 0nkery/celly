#![cfg(test)]
use bincode::SizeLimit;
use bincode::serde::{ serialize, deserialize };

use traits::Coord;
use traits::Cell;


impl Coord for (i32, i32) {
    fn from_2d(x: i32, y: i32) -> Self { (x, y) }

    fn x(&self) -> i32 { self.0 }
    fn y(&self) -> i32 { self.1 }
}

pub fn to_cell<Cin: Cell, Cout: Cell>(cell: &Cin) -> Cout {
    let encoded: Vec<u8> = serialize(cell, SizeLimit::Infinite).unwrap();
    let cell: Cout = deserialize(&encoded[..]).unwrap();

    cell
}