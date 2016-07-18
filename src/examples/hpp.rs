//! Implementation of [HPP model](https://en.wikipedia.org/wiki/HPP_model).
//! Assumes Von Nuemann's neighborhood.

#![cfg(test)]

use traits::Cell;
use traits::Coord;
use traits::Grid;
use traits::Engine;
use traits::Consumer;
use engine::Sequential;
use grid::nhood::VonNeumannNhood;
use grid::square::SquareGrid;


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum Stage {
    Collision,
    Transport,
}


impl Default for Stage {
    fn default() -> Self {
        Stage::Collision
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Down,
    Right,
    Left,
    Up,
}


impl Direction {
    fn opposite(&self) -> Self {

        match *self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    fn perpendicular(&self) -> Self {

        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
        }
    }
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct HPP {
    particles: [bool; 4],
    stage: Stage,
    coord: (i32, i32),
}


impl Cell for HPP {
    type Coord = (i32, i32);

    fn step<'a, I>(&'a self, neighbors: I) -> Self
        where I: Iterator<Item = Option<&'a Self>>
    {

        match self.stage {
            Stage::Collision => self.collision(neighbors),
            Stage::Transport => self.transport(neighbors),
        }
    }

    fn with_coord<C: Coord>(coord: C) -> Self {
        HPP {
            stage: Stage::Collision,
            coord: (coord.x(), coord.y()),
            ..Default::default()
        }
    }

    fn coord(&self) -> &Self::Coord {
        &self.coord
    }

    fn set_coord<C: Coord>(&mut self, coord: &C) {
        self.coord = (coord.x(), coord.y());
    }
}


impl HPP {
    fn collision<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item = Option<&'a Self>>
    {

        let mut new = HPP { stage: Stage::Transport, ..Default::default() };

        let has_head_on = |d: &Direction, op_d: &Direction| {
            self.particle(&d) && self.particle(&op_d) && !self.particle(&d.perpendicular()) &&
            !self.particle(&op_d.perpendicular())
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {

                Some(_) => {
                    let opposite = direction.opposite();

                    let head_on = has_head_on(&direction, &opposite);
                    if head_on {
                        new.set_particle(&direction.perpendicular(), true);
                        new.set_particle(&opposite.perpendicular(), true);
                    } else {
                        let exists = new.particle(&direction) || self.particle(&direction);
                        new.set_particle(&direction, exists);
                    }
                }
                // Rebound
                None => {
                    let opposite = direction.opposite();
                    let head_on = has_head_on(&direction, &opposite);
                    if head_on {
                        new.set_particle(&direction.perpendicular(), true);
                        new.set_particle(&opposite.perpendicular(), true);
                    } else {
                        if self.particle(&direction) {
                            new.set_particle(&opposite, true);
                        }
                    }
                }
            }
        }

        new
    }

    fn transport<'a, I>(&self, neighbors: I) -> Self
        where I: Iterator<Item = Option<&'a Self>>
    {

        let mut new = HPP { stage: Stage::Collision, ..Default::default() };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {
            match neighbor {
                Some(neighbor) => {
                    let opposite = direction.opposite();
                    if neighbor.particle(&opposite) {
                        new.set_particle(&opposite, true);
                    }
                }
                None => {
                    if self.particle(&direction) {
                        new.set_particle(&direction, true);
                    }
                }
            }
        }

        new
    }

    pub fn particle(&self, direction: &Direction) -> bool {
        let index = *direction as usize;
        self.particles[index]
    }

    fn set_particle(&mut self, direction: &Direction, exists: bool) {
        let index = *direction as usize;
        self.particles[index] = exists;
    }

    #[inline]
    pub fn directions(&self) -> [Direction; 4] {
        [Direction::Up, Direction::Left, Direction::Right, Direction::Down]
    }
}


fn find_cell<C: Cell>(cells: &Vec<C>, x: i32, y: i32) -> HPP {

    assert!(cells.iter().any(|c| c.coord().x() == x && c.coord().y() == y));

    let found = cells.iter()
        .find(|c| c.coord().x() == x && c.coord().y() == y)
        .unwrap();

    to_cell(found)
}


fn pretty_print<G: Grid>(grid: &G) {
    let dim = grid.dimensions();
    let iter_order =
        [vec![Direction::Down], vec![Direction::Right, Direction::Left], vec![Direction::Up]];

    println!("");

    for y in 0..dim.y() {

        for dirs in iter_order.iter() {
            print!("|");
            for x in 0..dim.x() {
                let cell = find_cell(grid.cells(), x, y);
                for d in dirs.iter() {
                    let to_print = match *d {
                        Direction::Down => {
                            if cell.particle(d) {
                                " v |"
                            } else {
                                "   |"
                            }
                        }
                        Direction::Right => {
                            if cell.particle(d) {
                                "> "
                            } else {
                                "  "
                            }
                        }
                        Direction::Left => {
                            if cell.particle(d) {
                                "<|"
                            } else {
                                " |"
                            }
                        }
                        Direction::Up => {
                            if cell.particle(d) {
                                " ^ |"
                            } else {
                                "   |"
                            }
                        }
                    };
                    print!("{}", to_print);
                }
            }
            println!("");
        }
        for _ in 0..dim.x() {
            print!(" ---");
        }
        println!("");
    }
}


#[derive(Debug)]
struct HPPRulesTestConsumer;

use test_helpers::to_cell;


impl HPPRulesTestConsumer {
    pub fn new() -> Self {
        HPPRulesTestConsumer {}
    }

    fn particles_count<C: Cell>(&self, cells: &Vec<C>, direction: &Direction) -> i32 {

        cells.iter()
            .map(|c| to_cell(c))
            .filter(|c: &HPP| c.particle(direction) == true)
            .count() as i32
    }



    fn test_collision<G: Grid>(&self, grid: &G) {
        println!("Collision");

        let left_particle_count = self.particles_count(&grid.cells(), &Direction::Left);
        assert_eq!(left_particle_count, 0);

        let right_particle_count = self.particles_count(&grid.cells(), &Direction::Right);
        assert_eq!(right_particle_count, 2);

        let up_particles_count = self.particles_count(&grid.cells(), &Direction::Up);
        assert_eq!(up_particles_count, 1);

        let down_particles_count = self.particles_count(&grid.cells(), &Direction::Down);
        assert_eq!(down_particles_count, 1);

        let rebound_to_right = find_cell(&grid.cells(), 0, 1);
        assert_eq!(rebound_to_right.particle(&Direction::Right), true);

        let head_on_up = find_cell(&grid.cells(), 1, 0);
        assert_eq!(head_on_up.particle(&Direction::Up), true);

        let head_on_down = find_cell(&grid.cells(), 1, 0);
        assert_eq!(head_on_down.particle(&Direction::Down), true);
    }

    fn test_transport<G: Grid>(&self, grid: &G) {
        println!("Transport");

        let simple_move_to_right = find_cell(&grid.cells(), 1, 2);
        assert_eq!(simple_move_to_right.particle(&Direction::Right), true);

        let move_to_right_after_rebound = find_cell(&grid.cells(), 1, 2);
        assert_eq!(move_to_right_after_rebound.particle(&Direction::Right),
                   true);

        let move_to_down_after_head_on = find_cell(&grid.cells(), 1, 1);
        assert_eq!(move_to_down_after_head_on.particle(&Direction::Down), true);

        let fixed_to_up_after_head_on = find_cell(&grid.cells(), 1, 0);
        assert_eq!(fixed_to_up_after_head_on.particle(&Direction::Up), true);
    }
}

impl Consumer for HPPRulesTestConsumer {
    type Cell = HPP;

    fn consume<G: Grid>(&mut self, grid: &mut G) {
        assert_eq!(grid.cells().len(), 9);

        pretty_print(grid);

        // We are testing previous state.
        let first: HPP = to_cell(&grid.cells()[0]);

        match first.stage {
            Stage::Collision => self.test_transport(grid),
            Stage::Transport => self.test_collision(grid),
        }
    }
}


#[test]
fn test_rules() {
    // initial          collision        transport
    // | * |> <| * |    | * |^ v| * |    | * |  v| * |
    // |  <| * | * | => |>  | * | * | => | * |> v| * |
    // |>  | * | * |    |>  | * | * |    | * |>  | * |
    // 3x3 grid with 4 particles. There should be
    // one rebound, head-on collision and simple move.

    let left_particle = [false, false, true, false];
    let right_particle = [false, true, false, false];
    let about_to_collide = [false, true, true, false];

    let cells = vec![
        HPP { stage: Stage::Collision, particles: about_to_collide, coord: (1, 0) },
        HPP { stage: Stage::Collision, particles: left_particle, coord: (0, 1) },
        HPP { stage: Stage::Collision, particles: right_particle, coord: (0, 2) },
    ];

    let nhood = VonNeumannNhood::new();
    let mut grid: SquareGrid<HPP, _> = SquareGrid::new(3, 3, nhood);
    grid.set_cells(cells);

    pretty_print(&grid);

    let right_particles_count = grid.cells()
        .iter()
        .map(|c| to_cell(c))
        .filter(|c: &HPP| c.particle(&Direction::Right) == true)
        .count();
    assert_eq!(right_particles_count, 2);

    let consumer = HPPRulesTestConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(2);
}


struct HPPSpreadTestConsumer {
    cur_y: i32,
    cur_direction: Direction,
}

impl HPPSpreadTestConsumer {
    pub fn new() -> Self {
        HPPSpreadTestConsumer {
            cur_y: 0,
            cur_direction: Direction::Down,
        }
    }

    fn particles_count<C: Cell>(&self, cells: &Vec<C>, direction: &Direction) -> i32 {

        cells.iter()
            .map(|c| to_cell(c))
            .filter(|c: &HPP| c.particle(direction) == true)
            .count() as i32
    }

    fn test_collision<G: Grid>(&mut self, grid: &G) {
        println!("Collision");

        if self.cur_y == 0 {
            self.cur_direction = Direction::Down;
        }

        let particles_count = self.particles_count(&grid.cells(), &self.cur_direction);
        assert_eq!(particles_count, 5);


        if self.cur_direction == Direction::Down {
            self.cur_y += 1;
        } else {
            self.cur_y -= 1;
        }
    }

    fn test_transport<G: Grid>(&mut self, grid: &G) {
        println!("Transport");

        let particles_count = self.particles_count(&grid.cells(), &self.cur_direction);
        assert_eq!(particles_count, 5);


        assert!(grid.cells()
            .iter()
            .map(|c| to_cell(c))
            .filter(|c: &HPP| c.coord().y() == self.cur_y)
            .all(|c| c.particle(&self.cur_direction) == true));

        if self.cur_y == 4 {
            self.cur_direction = Direction::Up;
        }
    }
}

impl Consumer for HPPSpreadTestConsumer {
    type Cell = HPP;

    fn consume<G: Grid>(&mut self, grid: &mut G) {
        assert_eq!(grid.cells().len(), 25);

        pretty_print(grid);

        // We are testing previous state.
        let first: HPP = to_cell(&grid.cells()[0]);

        match first.stage {
            Stage::Collision => self.test_transport(grid),
            Stage::Transport => self.test_collision(grid),
        }
    }
}


#[test]
fn test_spread() {
    // initial          later
    // | v v v v v |    | * * * * * |
    // | * * * * * |    | v v v v v |
    // | * * * * * | => | * * * * * | => (so on)
    // | * * * * * |    | * * * * * |
    // | * * * * * |    | * * * * * |
    // 5x5 grid with 5 particles. Particles should move
    // to lower border then rebound, then move to upper and so on.

    let down_particle = [false, false, false, true];

    let mut cells = Vec::new();
    for x in 0..5 {
        cells.push(HPP {
            stage: Stage::Collision,
            particles: down_particle,
            coord: (x, 0),
        });
    }

    let nhood = VonNeumannNhood::new();
    let mut grid: SquareGrid<HPP, _> = SquareGrid::new(5, 5, nhood);
    grid.set_cells(cells);

    let consumer = HPPSpreadTestConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    // 2 phases * 10 full cycles = 20 times.
    engine.run_times(20);
}
