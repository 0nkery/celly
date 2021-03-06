//! Interfaces on which this library is built.

use serde::{Deserialize, Serialize};

/// Trait represents global state of the
/// entire simulation which can be updated
/// independently from any particular cell.
/// Consider to store there as much as you can
/// because this data will be created only once
/// and will not be copied.
pub trait EvolutionState {
    /// Method is called once between cells' updates.
    fn update(&mut self);
}

/// Main trait should be implemented in user's code.
/// Such structs contain main logic of cellular
/// automaton. Grids can handle only one cell,
/// so it should be all-in-one.
pub trait Cell: Serialize + Deserialize {
    /// Coords supported by Cell.
    type Coord: Coord;
    /// Global state of evolution.
    type State: EvolutionState;

    /// This method is called for every instance of Cell
    /// in grid. Cell can mutate itself. Grid should pass
    /// neighbors, previous version of this Cell and the
    /// global state.
    fn update<'a, I>(&'a mut self, old: &'a Self, neighbors: I, &Self::State)
        where I: Iterator<Item = Option<&'a Self>>;

    /// Constructs Cell with given coord.
    fn with_coord<C: Coord>(C) -> Self;
    /// Getter for cell's coordinate.
    fn coord(&self) -> &Self::Coord;
    /// Setter for cell's coordinate.
    fn set_coord<C: Coord>(&mut self, &C);
}

/// Represents neighborhood for automata.
pub trait Nhood {
    /// Coords this nhood supports.
    type Coord: Coord;

    /// Method returns for any given coord
    /// coordinates of surrounding neighbors.
    fn neighbors(&self, &Self::Coord) -> Vec<Self::Coord>;
    /// Hint for grid.
    fn neighbors_count(&self) -> usize;
}

/// Basic coordinate with three components.
/// This crate is used for creating automata
/// on 2D grids for now, so `z` has default impl.
pub trait Coord: Clone + Serialize + Deserialize {
    /// Build coord from any other representation of coord.
    fn from_2d(x: i32, y: i32) -> Self;

    /// Returns `x` component.
    fn x(&self) -> i32;
    /// Returns `y` component.
    fn y(&self) -> i32;
    /// Returns `z` component.
    fn z(&self) -> i32 { 0 }
}

/// Grid stores cells and updates them. Also
/// grid contains global evolution state.
pub trait Grid {
    /// Grid wants to work with them.
    type Cell: Cell;
    /// Grid knows how to work with them.
    type Coord: Coord;

    /// One step in evolution.
    fn update(&mut self);

    /// Getter for evolution state.
    fn state(&self) -> &<<Self as Grid>::Cell as Cell>::State;
    /// Getter for all cells. It should some type that could
    /// be represented as slice (i.e. `Vec`).
    fn cells(&self) -> &[Self::Cell];
    /// Returns `Coord` with rows and cols counts of grid (2D).
    /// 3D grids would have more dimensions.
    fn size(&self) -> Self::Coord;

    /// This method gives an ability to change grid externally.
    /// It could be done from consumer, for example
    /// (i.e. an app where you reacting to user input),
    /// or from engine (i.e. distributed engine received
    /// updates from nodes).
    fn set_cells(&mut self, Vec<Self::Cell>);
}

/// Interface to the outer world.
pub trait Consumer {
    /// Cells supported by this consumer. Helps
    /// to use cells from grid directly if grid
    /// has the same cell in it.
    type Cell: Cell;

    /// Called once when all cells has been updated.
    fn consume<G>(&mut self, &mut G)
        where G: Grid<Cell = Self::Cell>;
}

/// Interlayer between grid and consumer(s).
pub trait Engine {
    /// Runs evolution fixed number of times.
    fn run_times(&mut self, u64);
}
