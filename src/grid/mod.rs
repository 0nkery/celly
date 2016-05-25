use cell::Cell;

mod square;
use self::square::SquareGrid;

pub trait Grid : Sized {
    fn step(&self);
    fn into_parts(self) -> Vec<Self>;

    fn neighbors<C: Cell>(&self, cell: C) -> &[C];
}