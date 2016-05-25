trait Grid : Sized {
    fn neighbors<T: Cell>(&self, Cell: T) -> &[T];
    fn split(self) -> Vec<Self>;
}