pub mod grid;
pub mod engine;

mod examples;


pub mod traits {

    pub trait Cell: Default + Clone + Binary {

        fn step<'a, I>(&'a self, neighbors: I) -> Self
            where I: Iterator<Item=Option<&'a Self>>;
    }

    pub trait Nhood {
        type Coord: Coord;

        fn neighbors(&self, &Self::Coord) -> Vec<Self::Coord>;

        fn neighbors_count(&self) -> usize;
    }

    pub trait Coord: PartialEq + Clone {
        fn from_2d(x: i32, y: i32) -> Self;

        fn x(&self) -> i32;
        fn y(&self) -> i32;
        fn z(&self) -> i32 { 0 }
    }
    
    pub trait Grid {
        type Cell: Cell;
        type Coord: Coord;

        fn step(&mut self);

        fn cells(&self) -> &Vec<Self::Cell>;
        fn dimensions(&self) -> Self::Coord;

        fn restore<G: Grid>(&mut self, &G);
    }

    pub trait ReprConsumer {
        fn consume<G: Grid>(&mut self, repr: &G);
    }

    pub trait Engine {
        fn run_times(&mut self, times: i64);
    }

    pub trait Binary: From<Vec<u8>> {
        fn bytes(&self) -> &[u8];
    }
}

