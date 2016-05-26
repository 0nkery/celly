use grid::Grid;

mod sequential;
use self::sequential::Sequential;

pub trait Engine {
    type Grid: Grid;

    fn new(grid: Self::Grid) -> Self;
    fn run_times(&self, times: i64);
}


pub trait Parallel: Sized {
    fn into_parts(self) -> Vec<Self>;
}