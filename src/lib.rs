pub mod grid;
pub mod engine;
pub mod repr;

mod examples;


pub mod traits {

    use std::collections::HashMap;

    use repr::GridRepr;

    pub trait Cell: Default + Clone {
        fn step<'a, I>(&'a self, neighbors: I) -> Self
            where I: Iterator<Item=Option<&'a Self>>;
        fn repr(&self, meta: &mut HashMap<&str, &str>);
        fn from_repr(&mut self, meta: &HashMap<&str, &str>);
    }

    pub trait Nhood {
        type Coord: Coord;

        fn neighbors(&self, coord: &Self::Coord) -> Vec<Self::Coord>;
        fn neighbors_count(&self) -> usize;
    }

    pub trait Coord {
        fn from_2d(x: i32, y: i32) -> Self;

        fn x(&self) -> i32;
        fn y(&self) -> i32;
        fn z(&self) -> i32 { 0 }
    }
    
    pub trait Grid {
        type Coord: Coord;

        fn step(&mut self);
        fn repr(&self) -> &GridRepr<Self::Coord>;
        fn from_repr<'a: 'b, 'b, C: Coord>(&'a mut self, &GridRepr<'b, C>);
    }

    pub trait ReprConsumer {
        fn consume<C: Coord>(&mut self, repr: &GridRepr<C>);
    }

    pub trait Engine {
        fn run_times(&mut self, times: i64);
    }
}

