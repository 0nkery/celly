pub trait Cell {
    fn step<C: Cell>(&self, neighbors: &[C]);
}