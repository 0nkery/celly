pub mod square_moore;


pub trait Grid {
    fn step(&mut self);
}