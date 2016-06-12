#![cfg(test)]

use std::collections::HashMap;
use std::str::FromStr;

use traits::Cell;
use traits::Coord;
use traits::Grid;
use traits::Engine;
use traits::ReprConsumer;
use repr::CellRepr;
use repr::GridRepr;
use engine::sequential::Sequential;
use grid::nhood::VonNeumannNhood;
use grid::square::GridCoord;
use grid::square::SquareGrid;

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


#[derive(Clone, Copy, Debug, PartialEq)]
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
                    else {
                        let particle = new.particle(&direction) || self.particle(&direction);
                        new.set_particle(&direction, particle);
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

            match neighbor {
                Some(neighbor) => {
                    let opposite = direction.opposite();
                    new.set_particle(
                        &opposite,
                        neighbor.particle(&opposite) || self.particle(&opposite)
                    );
                },
                None => {
                    new.set_particle(&direction, self.particle(&direction))
                }
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


#[derive(Debug)]
struct HPPTestConsumer;


impl HPPTestConsumer {
    
    pub fn new() -> Self {
        HPPTestConsumer { }
    }

    fn particles_count<C: Coord>(&self,
                                 cells: &Vec<CellRepr<C>>,
                                 direction: &'static str) -> i32 {

        cells.iter()
             .filter(|c| c.state.get(direction).unwrap_or(&DOESNT_EXIST) == &EXISTS)
             .count() as i32
    }

    fn find_cell<'a: 'b, 'b, C: Coord>(&'a self,
                                       cells: &'a Vec<CellRepr<'a, C>>,
                                       x: i32, y: i32) -> &'b CellRepr<C> {

        cells.iter().find(|c| c.coord.x() == x &&
                              c.coord.y() == y).unwrap()
    }

    fn test_collision<C: Coord>(&self, repr: &GridRepr<C>) {

        let left_particle_count = self.particles_count(&repr.cells, LEFT);
        assert_eq!(left_particle_count, 0);

        let right_particle_count = self.particles_count(&repr.cells, RIGHT);
        assert_eq!(right_particle_count, 2);

        let up_particles_count = self.particles_count(&repr.cells, UP);
        assert_eq!(up_particles_count, 1);

        let down_particles_count = self.particles_count(&repr.cells, DOWN);
        assert_eq!(down_particles_count, 1);

        let rebound_to_right = self.find_cell(&repr.cells, 0, 1);
        assert_eq!(rebound_to_right.state.get(RIGHT).unwrap(), &EXISTS);

        let head_on_up = self.find_cell(&repr.cells, 1, 0);
        assert_eq!(head_on_up.state.get(UP).unwrap(), &EXISTS);

        let head_on_down = self.find_cell(&repr.cells, 2, 0);
        assert_eq!(head_on_down.state.get(DOWN).unwrap(), &EXISTS);
    }

    fn test_transport<C: Coord>(&self, repr: &GridRepr<C>) {
        let simple_move_to_right = self.find_cell(&repr.cells, 1, 2);
        assert_eq!(simple_move_to_right.state.get(RIGHT).unwrap(), &EXISTS);

        let move_to_right_after_rebound = self.find_cell(&repr.cells, 1, 2);
        assert_eq!(move_to_right_after_rebound.state.get(RIGHT).unwrap(), &EXISTS);

        let move_to_down_after_head_on = self.find_cell(&repr.cells, 2, 1);
        assert_eq!(move_to_down_after_head_on.state.get(DOWN).unwrap(), &EXISTS);

        let fixed_to_up_after_head_on = self.find_cell(&repr.cells, 1, 0);
        assert_eq!(fixed_to_up_after_head_on.state.get(UP).unwrap(), &EXISTS);
    }

    fn pretty_print<C: Coord>(&self, repr: &GridRepr<C>) {
        println!("");

        for y in 0 .. 3 {
            print!("|");

            for x in 0 .. 3 {
                let cell = self.find_cell(&repr.cells, x, y);
                let maybe_particle = 
                    cell.state.iter()
                              .find(|&(k, v)| k != &STAGE && v == &EXISTS);

                let to_print = match maybe_particle {
                    Some((&UP, _)) => " ^ |",
                    Some((&LEFT, _)) => " < |",
                    Some((&RIGHT, _)) => " > |",
                    Some((&DOWN, _)) => " v |",
                    None => " * |",

                    Some((_, _)) => panic!("Unknown particle direction.")
                };
                print!("{}", to_print);
            }

            println!("");
        }
    }
}

impl ReprConsumer for HPPTestConsumer {

    fn consume<C: Coord>(&mut self, repr: &GridRepr<C>) {
        assert_eq!(repr.cells.len(), 9);

        self.pretty_print(repr);

        // We are testing previous state.
        match *repr.cells[0].state.get(STAGE).unwrap() {
            COLLISION => self.test_transport(repr),
            TRANSPORT => self.test_collision(repr),
            _ => panic!("Unknown cell stage.")
        }
    }
}


#[test]
fn test_particles() {
    // initial      collision    transport
    // | * > < |    | * ^ v |    | * ^ * |
    // | < * * | => | > * * | => | * > v |
    // | > * * |    | > * * |    | * > * |
    // 3x3 grid with 4 particles. There should be 
    // one rebound, head-on collision and simple move.

    let mut left_particle = HashMap::new();
    left_particle.insert(LEFT, EXISTS);

    let mut right_particle = HashMap::new();
    right_particle.insert(RIGHT, EXISTS);

    let cells = vec![
        CellRepr::new(GridCoord::from_2d(1, 0), Some(&right_particle)),
        CellRepr::new(GridCoord::from_2d(2, 0), Some(&left_particle)),
        CellRepr::new(GridCoord::from_2d(0, 1), Some(&left_particle)),
        CellRepr::new(GridCoord::from_2d(0, 2), Some(&right_particle)),
    ];

    let grid_repr = GridRepr::new(3, 3, Some(cells));

    let nhood = VonNeumannNhood::new();
    let mut grid: SquareGrid<HPP, _> = SquareGrid::new(3, 3, nhood);
    grid.from_repr(&grid_repr);

    let right_particles_count = 
        grid.repr().cells.iter()
            .filter(|c| c.state.get(RIGHT).unwrap_or(&DOESNT_EXIST) == &EXISTS)
            .count();
    assert_eq!(right_particles_count, 2);

    let consumer = HPPTestConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(2);
}
