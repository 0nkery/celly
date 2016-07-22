//! Implementation of [HPP model](https://en.wikipedia.org/wiki/HPP_model).
//! Assumes Von Nuemann's neighborhood.

#![cfg(test)]

use traits::Cell;
use traits::Coord;
use traits::Grid;
use traits::Engine;
use traits::EvolutionState;
use traits::Consumer;
use engine::Sequential;
use grid::nhood::VonNeumannNhood;
use grid::twodim::TwodimGrid;


enum Stage {
    Collision,
    Transport,
}


impl Default for Stage {
    fn default() -> Self { Stage::Collision }
}


struct HPPState {
    stage: Stage,
}

impl HPPState {
    fn new() -> Self { HPPState { stage: Stage::Collision } }
}

impl EvolutionState for HPPState {
    fn update(&mut self) {
        self.stage = match self.stage {
            Stage::Collision => Stage::Transport,
            Stage::Transport => Stage::Collision,
        };
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

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
struct Particles([bool; 4]);

impl Particles {
    fn set(&mut self, direction: &Direction, exists: bool) {
        let index = *direction as usize;
        self.0[index] = exists;
    }

    pub fn get(&self, direction: &Direction) -> bool {
        let index = *direction as usize;
        self.0[index]
    }
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct HPP {
    particles: Particles,
    coord: (i32, i32),
}


impl Cell for HPP {
    type Coord = (i32, i32);
    type State = HPPState;

    fn update<'a, I>(&'a mut self, old: &'a Self, neighbors: I, state: &Self::State)
        where I: Iterator<Item = Option<&'a Self>>,
    {

        match state.stage {
            Stage::Collision => self.collision(old, neighbors),
            Stage::Transport => self.transport(old, neighbors),
        };
    }

    fn with_coord<C: Coord>(coord: C) -> Self {
        HPP { coord: (coord.x(), coord.y()), ..Default::default() }
    }

    fn coord(&self) -> &Self::Coord { &self.coord }

    fn set_coord<C: Coord>(&mut self, coord: &C) { self.coord = (coord.x(), coord.y()); }
}


impl HPP {
    fn collision<'a, I>(&'a mut self, old: &'a Self, neighbors: I)
        where I: Iterator<Item = Option<&'a Self>>,
    {
        let mut new = Particles::default();

        let has_head_on = |d: &Direction, op_d: &Direction| {
            old.particles.get(&d) && old.particles.get(&op_d) &&
            !old.particles.get(&d.perpendicular()) &&
            !old.particles.get(&op_d.perpendicular())
        };

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {

            match neighbor {

                Some(_) => {
                    let opposite = direction.opposite();

                    let head_on = has_head_on(&direction, &opposite);
                    if head_on {
                        new.set(&direction.perpendicular(), true);
                        new.set(&opposite.perpendicular(), true);
                    } else {
                        let exists = new.get(&direction) || old.particles.get(&direction);
                        new.set(&direction, exists);
                    }
                },
                // Rebound
                None => {
                    let opposite = direction.opposite();
                    let head_on = has_head_on(&direction, &opposite);
                    if head_on {
                        new.set(&direction.perpendicular(), true);
                        new.set(&opposite.perpendicular(), true);
                    } else {
                        if old.particles.get(&direction) {
                            new.set(&opposite, true);
                        }
                    }
                },
            }
        }

        self.particles = new;
    }

    fn transport<'a, I>(&mut self, old: &'a Self, neighbors: I)
        where I: Iterator<Item = Option<&'a Self>>,
    {

        let mut new = Particles::default();

        for (neighbor, direction) in neighbors.zip(self.directions().iter()) {
            match neighbor {
                Some(neighbor) => {
                    let opposite = direction.opposite();
                    if neighbor.particles.get(&opposite) {
                        new.set(&opposite, true);
                    }
                },
                None => {
                    if old.particles.get(&direction) {
                        new.set(&direction, true);
                    }
                },
            }
        }

        self.particles = new;
    }





    #[inline]
    pub fn directions(&self) -> [Direction; 4] {
        [Direction::Up, Direction::Left, Direction::Right, Direction::Down]
    }
}


use test_helpers::to_cell;

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
                            if cell.particles.get(d) {
                                " v |"
                            } else {
                                "   |"
                            }
                        },
                        Direction::Right => {
                            if cell.particles.get(d) {
                                "> "
                            } else {
                                "  "
                            }
                        },
                        Direction::Left => {
                            if cell.particles.get(d) {
                                "<|"
                            } else {
                                " |"
                            }
                        },
                        Direction::Up => {
                            if cell.particles.get(d) {
                                " ^ |"
                            } else {
                                "   |"
                            }
                        },
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

impl HPPRulesTestConsumer {
    pub fn new() -> Self { HPPRulesTestConsumer }

    fn particles_count(&self, cells: &Vec<HPP>, direction: &Direction) -> i32 {

        cells.iter()
            .filter(|c| c.particles.get(direction) == true)
            .count() as i32
    }



    fn test_collision<G: Grid<Cell = HPP>>(&self, grid: &G) {
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
        assert_eq!(rebound_to_right.particles.get(&Direction::Right), true);

        let head_on_up = find_cell(&grid.cells(), 1, 0);
        assert_eq!(head_on_up.particles.get(&Direction::Up), true);

        let head_on_down = find_cell(&grid.cells(), 1, 0);
        assert_eq!(head_on_down.particles.get(&Direction::Down), true);
    }

    fn test_transport<G: Grid<Cell = HPP>>(&self, grid: &G) {
        println!("Transport");

        let simple_move_to_right = find_cell(&grid.cells(), 1, 2);
        assert_eq!(simple_move_to_right.particles.get(&Direction::Right), true);

        let move_to_right_after_rebound = find_cell(&grid.cells(), 1, 2);
        assert_eq!(move_to_right_after_rebound.particles.get(&Direction::Right),
                   true);

        let move_to_down_after_head_on = find_cell(&grid.cells(), 1, 1);
        assert_eq!(move_to_down_after_head_on.particles.get(&Direction::Down),
                   true);

        let fixed_to_up_after_head_on = find_cell(&grid.cells(), 1, 0);
        assert_eq!(fixed_to_up_after_head_on.particles.get(&Direction::Up),
                   true);
    }
}

impl Consumer for HPPRulesTestConsumer {
    type Cell = HPP;

    fn consume<G: Grid<Cell = Self::Cell>>(&mut self, grid: &mut G) {
        assert_eq!(grid.cells().len(), 9);

        pretty_print(grid);

        match grid.state().stage {
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

    let left_particle = Particles([false, false, true, false]);
    let right_particle = Particles([false, true, false, false]);
    let about_to_collide = Particles([false, true, true, false]);

    let cells = vec![
        HPP { particles: about_to_collide, coord: (1, 0) },
        HPP { particles: left_particle, coord: (0, 1) },
        HPP { particles: right_particle, coord: (0, 2) },
    ];

    let nhood = VonNeumannNhood::new();
    let evolution_state = HPPState::new();
    let mut grid: TwodimGrid<HPP, _> = TwodimGrid::new(3, 3, nhood, evolution_state);
    grid.set_cells(cells);

    pretty_print(&grid);

    let right_particles_count = grid.cells()
        .iter()
        .filter(|c| c.particles.get(&Direction::Right) == true)
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

    fn particles_count(&self, cells: &Vec<HPP>, direction: &Direction) -> i32 {

        cells.iter()
            .filter(|c| c.particles.get(direction) == true)
            .count() as i32
    }

    fn test_collision<G: Grid<Cell = HPP>>(&mut self, grid: &G) {
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

    fn test_transport<G: Grid<Cell = HPP>>(&mut self, grid: &G) {
        println!("Transport");

        let particles_count = self.particles_count(&grid.cells(), &self.cur_direction);
        assert_eq!(particles_count, 5);


        assert!(grid.cells()
            .iter()
            .filter(|c| c.coord().y() == self.cur_y)
            .all(|c| c.particles.get(&self.cur_direction) == true));

        if self.cur_y == 4 {
            self.cur_direction = Direction::Up;
        }
    }
}

impl Consumer for HPPSpreadTestConsumer {
    type Cell = HPP;

    fn consume<G: Grid<Cell = Self::Cell>>(&mut self, grid: &mut G) {
        assert_eq!(grid.cells().len(), 25);

        pretty_print(grid);

        match grid.state().stage {
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

    let down_particle = Particles([false, false, false, true]);

    let mut cells = Vec::new();
    for x in 0..5 {
        cells.push(HPP {
            particles: down_particle,
            coord: (x, 0),
        });
    }

    let nhood = VonNeumannNhood::new();
    let evolution_state = HPPState::new();
    let mut grid: TwodimGrid<HPP, _> = TwodimGrid::new(5, 5, nhood, evolution_state);
    grid.set_cells(cells);

    let consumer = HPPSpreadTestConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    // 2 phases * 10 full cycles = 20 times.
    engine.run_times(20);
}
