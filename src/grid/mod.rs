use cell::Cell;

pub trait Grid : Sized {
    fn step(&self);
    fn into_parts(self) -> Vec<Self>;

    fn neighbors<C: Cell>(&self, cell: C) -> &[C];
}