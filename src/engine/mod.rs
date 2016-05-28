pub mod sequential;

use grid::Grid;


pub trait Engine {
    type Grid: Grid;

    fn new(grid: Self::Grid) -> Self;
    fn run_times(&mut self, times: i64);
}


pub trait Parallel: Sized {
    fn into_parts(self) -> Vec<Self>;
}