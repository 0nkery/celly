#![cfg(test)]
use std::ops::Add;
use std::collections::HashMap;

use traits::Cell;
use traits::Engine;
use traits::ReprConsumer;
use engine::sequential::Sequential;
use grid::square_moore::SquareGrid;
use repr::CellRepr;

/// Implementation of Conway's Game of Life.

#[derive(Copy, Clone)]
enum LifeState {
    Dead,
    Alive
}

#[derive(Copy, Clone)]
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

impl Cell for Life {

    fn step<'a, I>(&self, neighbors: I) -> Self 
        where I: Iterator<Item=Option<&'a Self>> {

        let alive = self.alive_count(neighbors);

        let new_state = match self.state {
            LifeState::Alive => self.alive_state(alive),
            LifeState::Dead => self.dead_state(alive)
        };

        let mut new_cell = self.clone();
        new_cell.state = new_state;

        new_cell
    }

    fn repr(&self, meta: &mut HashMap<&str, &str>) {

        let state_str = match self.state {
            LifeState::Alive => "alive",
            LifeState::Dead  => "dead"
        };

        meta.insert("state", state_str);
    }
}


struct TestConsumer;

impl ReprConsumer for TestConsumer {

    fn consume(&self, repr: &Vec<CellRepr>) {
        
    }
}

#[test]
fn test_game_of_life() {
    let initial = Life { state: LifeState::Dead };
    let grid = SquareGrid::new(3, 3, initial);
    let consumer = TestConsumer;
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(1);
}