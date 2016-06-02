pub mod grid;
pub mod engine;
pub mod life;
pub mod repr;

pub mod traits {

    use std::collections::HashMap;

    use repr::GridRepr;

    pub trait Cell {
        fn step<'a, I>(&'a self, neighbors: I) -> Self
            where I: Iterator<Item=Option<&'a Self>>;
        fn repr(&self, meta: &mut HashMap<&str, &str>);
        fn from_repr(&mut self, meta: &HashMap<&str, &str>);
    }
    
    pub trait Grid {
        fn step(&mut self);
        fn repr(&self) -> &GridRepr;
        fn from_repr<'a: 'b, 'b>(&'a mut self, &GridRepr<'b>);
    }

    pub trait ReprConsumer {
        fn consume(&mut self, repr: &GridRepr);
    }

    pub trait Engine {
        fn run_times(&mut self, times: i64);
    }
}

