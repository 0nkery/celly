pub mod grid;
pub mod engine;
pub mod life;
pub mod repr;

pub mod traits {

    use std::collections::HashMap;

    use repr::CellRepr;

    pub trait Cell {
        fn step<'a, I>(&'a self, neighbors: I) -> Self
            where I: Iterator<Item=Option<&'a Self>>;
        fn repr(&self, meta: &mut HashMap<&str, &str>);
    }
    
    pub trait Grid {
        fn step(&mut self);
        fn repr(&self) -> &Vec<CellRepr>;
    }

    pub trait ReprConsumer {
        fn consume(&self, repr: &Vec<CellRepr>);
    }

    pub trait Engine {
        fn run_times(&mut self, times: i64);
    }

    pub trait Parallel: Sized {
        fn into_parts(self) -> Vec<Self>;
    }
}

