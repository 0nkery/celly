#![cfg(test)]
use traits::Cell;
use traits::Coord;
use traits::Engine;
use traits::Consumer;
use traits::Grid;
use engine::Sequential;
use grid::twodim::TwodimGrid;
use grid::nhood::MooreNhood;
use grid::EmptyState;
use utils::find_cell;

/// Implementation of Conway's Game of Life.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum LifeState {
    Dead,
    Alive,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Life {
    state: LifeState,
    coord: (i32, i32),
}

impl Life {
    fn alive_count<'a, I>(&self, neighbors: I) -> u32
        where I: Iterator<Item = Option<&'a Self>>,
    {

        neighbors.filter(|n| match *n {
                Some(n) => n.state == LifeState::Alive,
                None => false,
            })
            .count() as u32
    }

    #[inline]
    fn dead_state(&self, alive: u32) -> LifeState {
        match alive {
            3 => LifeState::Alive,
            _ => LifeState::Dead,
        }
    }

    #[inline]
    fn alive_state(&self, alive: u32) -> LifeState {
        match alive {
            2 | 3 => LifeState::Alive,
            _ => LifeState::Dead,
        }
    }
}


impl Cell for Life {
    type Coord = (i32, i32);
    type State = EmptyState;

    fn update<'a, I>(&'a mut self, old: &'a Self, neighbors: I, _: &Self::State)
        where I: Iterator<Item = Option<&'a Self>>,
    {
        let alive_count = self.alive_count(neighbors);

        let new_state = match old.state {
            LifeState::Alive => self.alive_state(alive_count),
            LifeState::Dead => self.dead_state(alive_count),
        };

        self.state = new_state;
    }

    fn with_coord<C: Coord>(coord: C) -> Self {
        Life {
            state: LifeState::Dead,
            coord: (coord.x(), coord.y()),
        }
    }

    fn coord(&self) -> &Self::Coord { &self.coord }

    fn set_coord<C: Coord>(&mut self, coord: &C) { self.coord = (coord.x(), coord.y()); }
}


fn pretty_print<G: Grid<Cell = Life>>(grid: &G) {
    let dims = grid.size();

    println!("");

    for y in 0..dims.y() {
        for x in 0..dims.x() {
            let cell = find_cell(grid.cells(), x, y);
            match cell.state {
                LifeState::Dead => print!("D |"),
                LifeState::Alive => print!("A |"),
            };
        }
        println!("");
    }

    println!("");
}

struct SpinnerTestConsumer {
    vertical: bool,
}


impl SpinnerTestConsumer {
    pub fn new() -> Self { SpinnerTestConsumer { vertical: true } }
}


impl Consumer for SpinnerTestConsumer {
    type Cell = Life;

    fn consume<G: Grid<Cell = Self::Cell>>(&mut self, grid: &mut G) {
        assert_eq!(grid.cells().len(), 9);

        pretty_print(grid);

        let dead_cells_count = grid.cells()
            .iter()
            .filter(|c| c.state == LifeState::Dead)
            .count();
        assert_eq!(dead_cells_count, 6);

        let alive_cells = || {
            grid.cells()
                .iter()
                .filter(|c| c.state == LifeState::Alive)
        };
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
    let mut grid: TwodimGrid<Life, _, _> = TwodimGrid::new(3, 3, nhood, EmptyState, 1);

    // Should be in default state
    let default_state = LifeState::Dead;

    assert!(grid.cells()
        .iter()
        .all(|c| c.state == default_state));

    // Vertical spinner
    // D | A | D
    // D | A | D
    // D | A | D
    let cells = vec![Life {
                         state: LifeState::Alive,
                         coord: (1, 0),
                     },
                     Life {
                         state: LifeState::Alive,
                         coord: (1, 1),
                     },
                     Life {
                         state: LifeState::Alive,
                         coord: (1, 2),
                     }];

    grid.set_cells(cells);

    pretty_print(&grid);

    let consumer = SpinnerTestConsumer::new();
    let mut engine = Sequential::new(grid, consumer);
    engine.run_times(2);
}
