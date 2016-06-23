use traits::Grid;
use traits::Engine;
use traits::Consumer;


pub struct Sequential<G: Grid, C: Consumer> {
    grid: G,
    consumer: C
}

impl<G: Grid, C: Consumer> Sequential<G, C> {

    pub fn new(grid: G, consumer: C) -> Self {
        Sequential {
            grid: grid,
            consumer: consumer
        }
    }
}


impl<G: Grid, C: Consumer> Engine for Sequential<G, C> {
    fn run_times(&mut self, times: i64) {
        for _ in 0..times {
            self.grid.step();
            self.consumer.consume(&self.grid);
        }
    }
}