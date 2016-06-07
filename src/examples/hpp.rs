#![cfg(test)]

use std::collections::HashMap;
use std::str::FromStr;

use traits::Cell;

/// Implementation of [HPP model](https://en.wikipedia.org/wiki/HPP_model).
/// Assumes Von Nuemann's neighborhood.

#[derive(Clone, Copy, Debug)]
enum Stage {
    Collision,
    Transport
}

const COLLISION: &'static str = "c";
const TRANSPORT: &'static str = "t";

impl Stage {

    fn from_str(string: &str) -> Self {

        match string {
            COLLISION => Stage::Collision,
            TRANSPORT => Stage::Collision,
            _ => panic!("Unknown stage.")
        }
    }
}

impl From<Stage> for &'static str {

    fn from(stage: Stage) -> &'static str {

        match stage {
            Stage::Collision => COLLISION,
            Stage::Transport => TRANSPORT
        }
    }
}


#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down
}

const UP: &'static str = "u";
const LEFT: &'static str = "l";
const RIGHT: &'static str = "r";
const DOWN: &'static str = "d";

impl Direction {

    fn from_str(string: &str) -> Self {

        match string {
            UP => Direction::Up,
            LEFT => Direction::Left,
            RIGHT => Direction::Right,
            DOWN => Direction::Down,
            _ => panic!("Unknown direction.")
        }
    }

    fn opposite(&self) -> Self {

        match *self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up
        }
    }

    fn perpendicular(&self) -> Self {

        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right
        }
    }
}

impl<'a> From<&'a Direction> for &'static str {

    fn from(direction: &'a Direction) -> &'static str {

        match *direction {
            Direction::Up => UP,
            Direction::Left => LEFT,
            Direction::Right => RIGHT,
            Direction::Down => DOWN
        }
    }
}


#[derive(Clone, Debug)]
struct HPP {
    particles: [bool; 4],
    stage: Stage
}


impl Default for HPP {

    fn default() -> Self {

        HPP {
            stage: Stage::Collision,
            particles: [false; 4]
        }
    }
}


const EXISTS: &'static str = "true";
const DOESNT_EXIST: &'static str = "false";
const STAGE: &'static str = "stage";


impl Cell for HPP {

    fn step<'a, I>(&'a self, neighbors: I) -> Self
            where I: Iterator<Item=Option<&'a Self>> {

        match self.stage {
            Stage::Collision => self.collision(neighbors),
            Stage::Transport => self.transport(neighbors),
        }
    }

    fn repr(&self, meta: &mut HashMap<&str, &str>) {

        for direction in self.directions().iter() {

            let exists = self.particle(direction);
            
            meta.insert(
                direction.into(),
                if exists { EXISTS } else { DOESNT_EXIST }
            );
        }

        meta.insert(STAGE, self.stage.into());
    }

    fn from_repr(&mut self, meta: &HashMap<&str, &str>) {

        for (state, value) in meta.iter() {

            match *state {

                STAGE => {
                    self.stage = Stage::from_str(value);
                },
                _ => {
                    let direction = Direction::from_str(state);
                    let exists = bool::from_str(value).unwrap();

                    self.set_particle(&direction, exists);
                }
            }
        }
    }
}


impl HPP {

    fn collision<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>> {

        let mut new = HPP {
            stage: Stage::Transport,
            ..Default::default()
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {

                Some(neighbor) => {
                    let opposite = direction.opposite();
                    let head_on = self.particle(&direction) && neighbor.particle(&opposite);

                    if head_on {
                        new.set_particle(&direction.perpendicular(), self.particle(&direction));
                    }
                },
                // Rebound
                None => {
                    let opposite = direction.opposite();
                    new.set_particle(&opposite, self.particle(direction));
                }
            }
        }

        new
    }

    fn transport<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item=Option<&'a Self>> {

        let mut new = HPP {
            stage: Stage::Collision,
            ..Default::default()
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            if let Some(neighbor) = neighbor {
                let opposite = direction.opposite();
                new.set_particle(&opposite, neighbor.particle(&opposite));
            }
        }

        new
    }

    fn particle(&self, direction: &Direction) -> bool {

        match *direction {
            Direction::Up => self.particles[0],
            Direction::Left => self.particles[1],
            Direction::Right => self.particles[2],
            Direction::Down => self.particles[3]
        }
    }

    fn set_particle(&mut self, direction: &Direction, exists: bool) {

        match *direction {
            Direction::Up => { self.particles[0] = exists },
            Direction::Left => { self.particles[1] = exists },
            Direction::Right => { self.particles[2] = exists },
            Direction::Down => { self.particles[3] = exists }
        }
    }

    #[inline]
    fn directions(&self) -> [Direction; 4] {
        [Direction::Up, Direction::Left, Direction::Right, Direction::Down]
    }
}
