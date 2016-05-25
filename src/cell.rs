trait Cell {
    fn step<T: Cell>(&self, neighbors: &mut [T]);
}