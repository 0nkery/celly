mod sequential;

use grid::Grid;

use self::sequential::Sequential;

pub trait Engine {
    fn new<T: Grid + Iterator>(grid: T) -> Self;
    fn run(&self);
}