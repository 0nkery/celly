#![cfg(test)]
use std::ops::Add;
use std::collections::HashMap;

use traits::Cell;
use traits::Engine;
use traits::ReprConsumer;
use traits::Grid;
use engine::sequential::Sequential;
use grid::square_moore::SquareGrid;
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

    fn consume(&mut self, repr: &GridRepr) {
        assert_eq!(repr.cells.len(), 9);

        let alive_cells: Vec<&CellRepr> = 
            repr.cells.iter().filter(|c| c.state[STATE] == ALIVE).collect();
        assert_eq!(alive_cells.len(), 3);

        self.vertical = !self.vertical;

        // if spinner is in vertical state
        if alive_cells[0].y == 1 {
            assert!(alive_cells.iter().all(|c| c.y == 1));
            assert!(self.vertical);
        }
        // if spinner is in horizontal state
        else if alive_cells[0].x == 1 {
            assert!(alive_cells.iter().all(|c| c.x == 1));
            assert!(!self.vertical);
        }

    }
}

#[test]
fn test_game_of_life() {

    let mut dead_cell = HashMap::new();
    dead_cell.insert(STATE, DEAD);

    let mut alive_cell = HashMap::new();
    alive_cell.insert(STATE, ALIVE);

    // Vertical spinner
    // D | A | D
    // D | A | D
    // D | A | D
    let mut cells: Vec<CellRepr> = Vec::new();  
    for x in 0 .. 3 {
        for y in 0 .. 3 {

            let state = match y {
                1 => Some(&alive_cell),
                _ => Some(&dead_cell)
            };

            cells.push(CellRepr::new(x, y, state));
        }
    }

    {
        let alive_cells: Vec<&CellRepr> = 
            cells.iter().filter(|c| c.state[STATE] == ALIVE).collect();
        assert_eq!(alive_cells.len(), 3);
    }

    let grid_repr = GridRepr::new(3, 3, Some(cells));

    let mut grid: SquareGrid<Life> = SquareGrid::new(3, 3);
    grid.from_repr(&grid_repr);

    let consumer = SpinnerTestConsumer { vertical: true };
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(2);
}