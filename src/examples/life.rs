#![cfg(test)]
use traits::Cell;
use traits::Coord;
use traits::Engine;
use traits::Consumer;
use traits::Grid;
use engine::Sequential;
use grid::square::SquareGrid;
use grid::nhood::MooreNhood;

/// Implementation of Conway's Game of Life.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum LifeState {
    Dead,
    Alive
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Life {
    state: LifeState,
    coord: (i32, i32)
}

impl Life {
    
    fn alive_count<'a, I>(&self, neighbors: I) -> i32 
        where I: Iterator<Item=Option<&'a Self>> {

        neighbors.filter(|n| match *n {
            Some(n) => n.state == LifeState::Alive,
            None => false
        }).count() as i32
    }

    #[inline]
    fn dead_state(&self, alive: i32) -> LifeState {
        match alive {
            3 => LifeState::Alive,
            _ => LifeState::Dead
        }
    }

    #[inline]
    fn alive_state(&self, alive: i32) -> LifeState {
        match alive {
            2 | 3 => LifeState::Alive,
            _ => LifeState::Dead
        }
    }
}


impl Cell for Life {
    type Coord = (i32, i32);

    fn step<'a, I>(&self, neighbors: I) -> Self 
        where I: Iterator<Item=Option<&'a Self>> {

        let alive_count = self.alive_count(neighbors);

        let new_state = match self.state {
            LifeState::Alive => self.alive_state(alive_count),
            LifeState::Dead => self.dead_state(alive_count)
        };

        let mut new_cell = self.clone();
        new_cell.state = new_state;

        new_cell
    }

    fn with_coord<C: Coord>(coord: C) -> Self {
        Life { state: LifeState::Dead, coord: (coord.x(), coord.y()) }
    }

    fn coord(&self) -> &Self::Coord {
        &self.coord
    }

    fn set_coord<C: Coord>(&mut self, coord: &C) {
        self.coord = (coord.x(), coord.y());
    }
}


use test_helpers::to_cell;

struct SpinnerTestConsumer {
    vertical: bool
}


impl SpinnerTestConsumer {

    pub fn new() -> Self {
        SpinnerTestConsumer { vertical: true }
    }
}


impl Consumer for SpinnerTestConsumer {

    type Cell = Life;

    fn consume<G: Grid>(&mut self, grid: &G) {
        assert_eq!(grid.cells().len(), 9);

        let dead_cells_count =
            grid.cells()
                .iter()
                .map(|c| to_cell(c))
                .filter(|c: &Life| c.state == LifeState::Dead).count();
        assert_eq!(dead_cells_count, 6);

        let alive_cells = ||
            grid.cells()
                .iter()
                .map(|c| to_cell(c))
                .filter(|c: &Life| c.state == LifeState::Alive);
        assert_eq!(alive_cells().count(), 3);

        self.vertical = !self.vertical;

        // if spinner is in vertical state
        if alive_cells().all(|c| c.coord.x() == 1) {
            assert!(self.vertical);
        }
        // if spinner is in horizontal state
        if alive_cells().all(|c| c.coord.y() == 1) {
            assert!(!self.vertical);
        }
    }
}

#[test]
fn test_game_of_life() {

    let nhood = MooreNhood::new();
    let mut grid: SquareGrid<Life, _> = SquareGrid::new(3, 3, nhood);

    // Should be in default state
    let default_state = LifeState::Dead;

    assert!(grid.cells()
                .iter()
                .map(|c| to_cell(c))
                .all(|c: Life| c.state == default_state));

    // Vertical spinner
    // D | A | D
    // D | A | D
    // D | A | D
    let cells = vec![
        Life { state: LifeState::Alive, coord: (1, 0) },
        Life { state: LifeState::Alive, coord: (1, 1) },
        Life { state: LifeState::Alive, coord: (1, 2) }
    ];

    grid.set_cells(cells);

    let consumer = SpinnerTestConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(2);
}
