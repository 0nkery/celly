use traits::Cell;
use traits::Consumer;
use traits::Grid;
use traits::Engine;


pub struct Sequential<C: Cell, G: Grid<Cell=C>, Con: Consumer<Cell=C>> {
    grid: G,
    consumer: Con
}

impl<C: Cell, G: Grid<Cell=C>, Con: Consumer<Cell=C>> Sequential<C, G, Con> {

    pub fn new(grid: G, consumer: Con) -> Self {
        Sequential {
            grid: grid,
            consumer: consumer
        }
    }
}


impl<C: Cell, G: Grid<Cell=C>, Con: Consumer<Cell=C>> Engine for Sequential<C, G, Con> {
    fn run_times(&mut self, times: i64) {
        for _ in 0..times {
            self.grid.step();
            self.consumer.consume(&mut self.grid);
        }
    }
}