use std::rc::Rc;

use grid::Grid;
use cell::Cell;


const NEIGHBORS_COUNT: usize = 8;

type Neighbors<C> = [Option<Rc<C>>; NEIGHBORS_COUNT];

pub struct SquareGrid<C: Cell> {
    cells: Vec<Rc<C>>,
    neighbors: Vec<Neighbors<C>>,
    rows: usize,
    cols: usize
}


impl<C: Cell + Clone> SquareGrid<C> {

    pub fn new(rows: usize, cols: usize, initial: C) -> Self {

        let len = rows * cols;
        let cells = vec![Rc::new(initial); len];
        let neighbors = Vec::with_capacity(len);

        let mut grid = SquareGrid {
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

                let neighbor_index: i32;

                {
                    let offset = offset as i32;
                    let serial = serial as i32;

                    neighbor_index = match serial {
                        i @ 0 ... 2 => offset - cols - (i - 1),
                        3 => offset - 1,
                        4 => offset + 1,
                        i @ 5 ... 7 => offset + cols - (i - 1),
                        // such values will be ignored later
                        _ => len
                    };
                }

                if neighbor_index >= 0 && neighbor_index < len {
                    self.neighbors[offset][serial] = Some(cell.clone());
                }
                else {
                    self.neighbors[offset][serial] = None;
                }
            }
        }
    }
}


impl<C: Cell> Grid for SquareGrid<C> {
    fn step(&self) {}
}
