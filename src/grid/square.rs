use std::rc::Rc;

use grid::Grid;
use cell::Cell;


const NEIGHBORS_COUNT: usize = 8;

type Neighbors<C> = [Option<Rc<C>>; NEIGHBORS_COUNT];

pub struct MooreSquareGrid<C: Cell> {
    cells: Vec<Rc<C>>,
    neighbors: Vec<Neighbors<C>>,
    rows: usize,
    cols: usize
}


impl<C: Cell + Clone> MooreSquareGrid<C> {

    pub fn new(rows: usize, cols: usize, initial: C) -> Self {

        let len = rows * cols;
        let cells = vec![Rc::new(initial); len];
        let neighbors = Vec::with_capacity(len);

        let mut grid = MooreSquareGrid {
            cells: cells,
            neighbors: neighbors,
            rows: rows,
            cols: cols
        };
        grid.init_neighbors();

        grid
    }

    /// 0 | 1 | 2
    /// 3 | x | 4
    /// 5 | 6 | 7
    fn init_neighbors(&mut self) {

        let len = self.cells.len() as i32;
        let cols = self.cols as i32;

        for (offset, cell) in self.cells.iter().enumerate() {


            for serial in 0 .. NEIGHBORS_COUNT {

                let neighbor_index: Option<i32>;

                {
                    let offset = offset as i32;
                    let serial = serial as i32;

                    neighbor_index = match serial {
                        i @ 0 ... 2 => Some(offset - cols - (i - 1)),
                        3 => Some(offset - 1),
                        4 => Some(offset + 1),
                        i @ 5 ... 7 => Some(offset + cols - (i - 1)),
                        _ => None
                    };
                }

                if let Some(index) = neighbor_index {
                    if index >= 0 && index < len {
                        self.neighbors[offset][serial] = Some(cell.clone());
                    }
                }
            }
        }
    }
}


impl<C: Cell> Grid for MooreSquareGrid<C> {
    fn step(&self) {}
}
