use traits::Grid;
use traits::Cell;
use repr::CellRepr;


const NEIGHBORS_COUNT: usize = 8;

type Neighbors = [Option<usize>; NEIGHBORS_COUNT];


pub struct SquareGrid<'a, C: Cell + Copy> {
    cells: Vec<C>,
    old_cells: Vec<C>,
    neighbors: Vec<Neighbors>,
    cell_reprs: Vec<CellRepr<'a>>,
    rows: i32,
    cols: i32,
}


impl<'a, C: Cell + Copy> SquareGrid<'a, C> {

    pub fn new(rows: i32, cols: i32, initial: C) -> Self {

        let len = (rows * cols) as usize;
        let cells = vec![initial; len];
        let old_cells = Vec::new();
        let neighbors = Vec::with_capacity(len);
        let cell_reprs = Vec::with_capacity(len);

        let mut grid = SquareGrid {
            cells: cells,
            old_cells: old_cells,
            neighbors: neighbors,
            cell_reprs: cell_reprs,
            rows: rows,
            cols: cols,
        };

        grid.init_neighbors();
        grid.init_cell_reprs();

        grid
    }

    #[inline]
    fn offset(&self, i: i32, j: i32) -> usize {
        (i * self.cols + j) as usize
    }

    #[inline]
    fn from_offset(&self, offset: i32) -> (i32, i32) {
        let col = offset % self.cols;
        let row = (offset - col) / self.rows;
        (row, col)
    }

    fn init_cell_reprs(&mut self) {

        let len = self.cells.len();

        for offset in 0 .. len {
            let (x, y) = self.from_offset(offset as i32);
            let mut repr = CellRepr::new(x, y);
            let cell = self.cells[offset];

            cell.repr(&mut repr.state);

            self.cell_reprs.push(repr);
        }
    }

    /// 0 | 1 | 2
    /// 3 | x | 4
    /// 5 | 6 | 7
    fn init_neighbors(&mut self) {

        let len = self.cells.len() as i32;

        for offset in 0 .. len {
            let (i, j) = self.from_offset(offset);

            let neighbors_coords = [
                (i - 1, j - 1), (i - 1, j), (i - 1, j + 1),
                (i, j - 1),     /* x */     (i, j + 1),
                (i + 1, j - 1), (i + 1, j), (i + 1, j + 1)
            ];

            let mut neighbors = [None; NEIGHBORS_COUNT];

            for (serial, coord) in neighbors_coords.iter().enumerate() {
                let (x, y) = *coord;

                if x >= 0 && x < self.cols &&
                   y >= 0 && y < self.rows {
                    neighbors[serial] = Some(self.offset(x, y));
                }
            }

            self.neighbors.push(neighbors);
        }
    }

    fn neighbors_iter<'b>(&self,
                          cells: &'b Vec<C>,
                          neighbors: Neighbors)
        -> MooreSquareGridIterator<'b, C> {

        MooreSquareGridIterator {
            cells: cells,
            neighbors: neighbors,
            index: 0,
        }
    }
}


impl<'a, C: Cell + Copy> Grid for SquareGrid<'a, C> {

    fn step(&mut self) {
        self.old_cells = self.cells.clone();

        for (cell_no, cell) in self.old_cells.iter().enumerate() {
            let neighbors = self.neighbors[cell_no];
            let neighbors_iter = self.neighbors_iter(&self.old_cells, neighbors);
            self.cells[cell_no] = cell.step(neighbors_iter);
        }
    }

    fn repr(&self) -> &Vec<CellRepr> {
        &self.cell_reprs
    }
}


struct MooreSquareGridIterator<'a, C: Cell + 'a> {
    cells: &'a Vec<C>,
    neighbors: Neighbors,
    index: usize,
}


impl<'a, C: Cell> Iterator for MooreSquareGridIterator<'a, C> {
    type Item = Option<&'a C>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.index {
            i @ _  if i < NEIGHBORS_COUNT => {
                let maybe_index = self.neighbors[i];
                match maybe_index {
                    Some(index) => Some(Some(&self.cells[index])),
                    None => Some(None)
                }
            },
            _ => None
        };
        self.index += 1;

        next
    }
}


#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use traits::Cell;
    use traits::Grid;

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct MooreTestCell;

    impl Cell for MooreTestCell {

        fn step<'a, I>(&self, neighbors: I) -> Self 
            where I: Iterator<Item=Option<&'a Self>> {

            // Initial value should be copied
            // There should be 3 neighbors and 5 None values
            // Overall count of neighbors should be equal to NEIGHBORS_COUNT
            let mut total = 0;
            let mut none_cnt = 0;
            let mut neighbors_cnt = 0;
            for neighbor in neighbors {
                match neighbor {
                    None => {
                        none_cnt += 1;
                    },
                    Some(n) => { 
                        assert_eq!(self, n);
                        neighbors_cnt += 1;
                    }
                };
                total += 1;
            }

            assert_eq!(total, super::NEIGHBORS_COUNT);
            assert_eq!(none_cnt, 5);
            assert_eq!(neighbors_cnt, 3);

            self.clone()
        }

        fn repr(&self, _: &mut HashMap<&str, &str>) {}
    }

    #[test]
    fn test_neighbors() {
        let initial = MooreTestCell;
        let mut grid = super::SquareGrid::new(2, 2, initial);
        grid.step();
    }
}
