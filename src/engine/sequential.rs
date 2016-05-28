use grid::Grid;
use engine::Engine;


pub struct Sequential<G: Grid> {
    grid: G
}


impl<G: Grid> Engine for Sequential<G> {
    type Grid = G;

    fn new(grid: G) -> Self {
        Sequential { grid: grid }
    }

    fn run_times(&mut self, times: i64) {
        for _ in 0..times {
            self.grid.step();
        }
    }
}