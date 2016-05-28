pub trait Cell {
    fn step<'a, I>(&'a self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>>;
}