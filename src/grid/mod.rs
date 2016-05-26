mod square;
use self::square::SquareGrid;

pub trait Grid {
    fn step(&self);
}