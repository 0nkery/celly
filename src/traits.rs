use serde::{Deserialize, Serialize};

/// Trait represents global state of the
/// entire simulation which can updated
/// independently from any particular cell.
/// Consider to store there as much as you can
/// because this data will be created only once
/// and will not be copied.
pub trait EvolutionState {
    /// Method is called once between cells' updates.
    fn update(&mut self);
}

pub trait Cell: Serialize + Deserialize {
    type Coord: Coord;
    type State: EvolutionState;

    fn update<'a, I>(&'a mut self, old: &'a Self, neighbors: I, &Self::State)
        where I: Iterator<Item = Option<&'a Self>>;

    fn with_coord<C: Coord>(C) -> Self;
    fn coord(&self) -> &Self::Coord;
    fn set_coord<C: Coord>(&mut self, &C);
}

pub trait Nhood {
    type Coord: Coord;

    fn neighbors(&self, &Self::Coord) -> Vec<Self::Coord>;
    fn neighbors_count(&self) -> usize;
}

pub trait Coord: Clone + Serialize + Deserialize {
    fn from_2d(x: i32, y: i32) -> Self;

    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn z(&self) -> i32 { 0 }
}

pub trait Grid {
    type Cell: Cell;
    type Coord: Coord;

    fn update(&mut self);

    fn state(&self) -> &<<Self as Grid>::Cell as Cell>::State;
    fn cells(&self) -> &Vec<Self::Cell>;
    fn dimensions(&self) -> Self::Coord;

    fn set_cells(&mut self, Vec<Self::Cell>);
}

pub trait Consumer {
    type Cell: Cell;

    fn consume<G>(&mut self, &mut G)
        where G: Grid<Cell = Self::Cell>;
}

/// Interlayer between grid and consumer(s).
pub trait Engine {
    fn run_times(&mut self, i64);
}
