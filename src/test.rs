#![cfg(test)]
use std::ops::Add;
use std::collections::HashMap;

use traits::Cell;
use traits::Coord;
use traits::Engine;
use traits::ReprConsumer;
use traits::Grid;
use engine::sequential::Sequential;
use grid::square::SquareGrid;
use grid::nhood::MooreNhood;
use grid::square::GridCoord;
use repr::CellRepr;
use repr::GridRepr;

/// Implementation of Conway's Game of Life.

#[derive(Clone, Debug)]
enum LifeState {
    Dead,
    Alive
}

#[derive(Clone, Debug)]
struct Life {
    state: LifeState
}

impl Life {
    
    fn alive_count<'a, I>(&self, neighbors: I) -> i32 
        where I: Iterator<Item=Option<&'a Self>> {
        neighbors.map(
            |n| {
                match n {
                    Some(ref cell) => match cell.state {
                        LifeState::Alive => 1,
                        LifeState::Dead => 0
                    },
                    None => 0,
                }
            }
        ).fold(0, Add::add)
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

impl Default for Life {

    fn default() -> Self {
        Life { state: LifeState::Dead }
    }
}

const ALIVE: &'static str = "a";
const DEAD: &'static str = "d";
const STATE: &'static str = "s";

impl Cell for Life {

    fn step<'a, I>(&self, neighbors: I) -> Self 
        where I: Iterator<Item=Option<&'a Self>> {

        let alive_count = self.alive_count(neighbors);

        let new_state = match self.state {
            LifeState::Alive => self.alive_state(alive_count),
            LifeState::Dead => self.dead_state(alive_count)
        };

        println!("{:?} ====> {:?}", self.state, new_state);

        let mut new_cell = self.clone();
        new_cell.state = new_state;

        new_cell
    }

    fn repr(&self, meta: &mut HashMap<&str, &str>) {

        let state_str = match self.state {
            LifeState::Alive => ALIVE,
            LifeState::Dead  => DEAD
        };

        meta.insert(STATE, state_str);
    }

    fn from_repr(&mut self, meta: &HashMap<&str, &str>) {

        let new_state = match meta[STATE] {
            ALIVE => LifeState::Alive,
            DEAD => LifeState::Dead,
            _ => panic!("Unknown state: {}.", meta[STATE])
        };

        self.state = new_state;
    }
}


struct SpinnerTestConsumer {
    vertical: bool
}


impl ReprConsumer for SpinnerTestConsumer {

    fn consume<C: Coord>(&mut self, repr: &GridRepr<C>) {
        assert_eq!(repr.cells.len(), 9);

        println!("{:?}", repr);

        let alive_cells: Vec<&CellRepr<C>> =
            repr.cells.iter().filter(|c| c.state[STATE] == ALIVE).collect();
        assert_eq!(alive_cells.len(), 3);

        self.vertical = !self.vertical;

        println!("{:?}", alive_cells);

        // if spinner is in vertical state
        if alive_cells.iter().all(|c| c.coord.x() == 1) {
            assert!(self.vertical);
        }
        // if spinner is in horizontal state
        if alive_cells.iter().all(|c| c.coord.y() == 1) {
            assert!(!self.vertical);
        }
    }
}

#[test]
fn test_game_of_life() {

    let mut alive_cell = HashMap::new();
    alive_cell.insert(STATE, ALIVE);

    // Vertical spinner
    // D | A | D
    // D | A | D
    // D | A | D
    // Cells not specified below should be equal to default cell value
    let cells = vec![
        CellRepr::new(GridCoord::from_2d(1, 0), Some(&alive_cell)),
        CellRepr::new(GridCoord::from_2d(1, 1), Some(&alive_cell)),
        CellRepr::new(GridCoord::from_2d(1, 2), Some(&alive_cell)),
    ];

    {
        let alive_cells: Vec<&CellRepr<GridCoord>> =
            cells.iter().filter(|c| c.state[STATE] == ALIVE).collect();
        assert_eq!(alive_cells.len(), 3);
    }

    let grid_repr = GridRepr::new(3, 3, Some(cells));

    let nhood = MooreNhood::new();
    let mut grid: SquareGrid<Life, _> = SquareGrid::new(3, 3, nhood);
    grid.from_repr(&grid_repr);

    let consumer = SpinnerTestConsumer { vertical: true };
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(2);
}