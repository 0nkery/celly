use serde::{Deserialize, Serialize};

pub trait EvolutionState {
    fn update(&mut self);
}

pub trait Cell: Clone + Serialize + Deserialize {
    type Coord: Coord;
    type State: EvolutionState;

    fn update<'a, I>(&'a self, neighbors: I, &Option<Self::State>) -> Self
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

    fn cells(&self) -> &Vec<Self::Cell>;
    fn dimensions(&self) -> Self::Coord;

    fn set_cells(&mut self, Vec<Self::Cell>);
}

pub trait Consumer {
    type Cell: Cell;

    fn consume<G: Grid<Cell = Self::Cell>>(&mut self, &mut G);
}

pub trait Engine {
    fn run_times(&mut self, i64);
}
