//! Most simple engine that runs evolution
//! sequentially. Useful for test purposes
//! and with grids which implemented interior parallelism.

use traits::Cell;
use traits::Consumer;
use traits::Grid;
use traits::Engine;


/// Engine generic over Cell and Consumer running
/// evolution sequentially.
pub struct Sequential<C: Cell, G: Grid<Cell = C>, Con: Consumer<Cell = C>> {
    grid: G,
    consumer: Con,
}

impl<C: Cell, G: Grid<Cell = C>, Con: Consumer<Cell = C>> Sequential<C, G, Con> {
    /// Constructs engine with given Grid and Consumer.
    pub fn new(grid: G, consumer: Con) -> Self {
        Sequential {
            grid: grid,
            consumer: consumer,
        }
    }
}


impl<C: Cell, G: Grid<Cell = C>, Con: Consumer<Cell = C>> Engine for Sequential<C, G, Con> {
    fn run_times(&mut self, times: u64) {
        for _ in 0..times {
            self.grid.update();
            self.consumer.consume(&mut self.grid);
        }
    }
}
