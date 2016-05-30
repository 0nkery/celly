use traits::Grid;
use traits::Cell;
use repr::CellRepr;
use repr::GridRepr;


const NEIGHBORS_COUNT: usize = 8;

type Neighbors = [Option<usize>; NEIGHBORS_COUNT];


pub struct SquareGrid<'a, C: Cell + Clone + Default> {
    cells: Vec<C>,
    old_cells: Vec<C>,
    neighbors: Vec<Neighbors>,
    repr: GridRepr<'a>,
    rows: i32,
    cols: i32,
}


impl<'a, C: Cell + Clone + Default> SquareGrid<'a, C> {

    pub fn new(rows: i32, cols: i32) -> Self {

        let len = (rows * cols) as usize;

        let cells = vec![C::default(); len];
        let old_cells = Vec::with_capacity(len);
        let neighbors = Vec::with_capacity(len);
        let repr = GridRepr::new(rows, cols, None);

        let mut grid = SquareGrid {
            cells: cells,
            old_cells: old_cells,
            neighbors: neighbors,
            repr: repr,
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
            let mut repr = CellRepr::new(x, y, None);
            let ref cell = self.cells[offset];

            cell.repr(&mut repr.state);
            self.repr.cells.push(repr);
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


impl<'a, C: Cell + Clone + Default> Grid for SquareGrid<'a, C> {

    fn step(&mut self) {
        self.old_cells = self.cells.clone();

        for (cell_no, cell) in self.old_cells.iter().enumerate() {
            let neighbors = self.neighbors[cell_no];
            let neighbors_iter = self.neighbors_iter(&self.old_cells, neighbors);

            let new_cell = cell.step(neighbors_iter);

            // update representation
            let ref mut cell_repr = self.repr.cells[cell_no];
            new_cell.repr(&mut cell_repr.state);

            self.cells[cell_no] = new_cell;
        }
    }

    fn repr(&self) -> &GridRepr {
        &self.repr
    }

    fn from_repr<'b: 'c, 'c>(&'b mut self, repr: &GridRepr<'c>) {

        if repr.rows != self.rows || repr.cols != self.cols {
            panic!("Mismatched rows and cols on saved representation.");
        }

        for i in 0 .. self.cells.len() {

            let ref mut cell = self.cells[i];
            let ref outer_repr = repr.cells[i];
            cell.from_repr(&outer_repr.state);

            let ref mut inner_repr = self.repr.cells[i];
            cell.repr(&mut inner_repr.state);
        }
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

    use super::SquareGrid;
    use super::NEIGHBORS_COUNT;

    #[derive(Clone, PartialEq, Debug)]
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

            assert_eq!(total, NEIGHBORS_COUNT);
            assert_eq!(none_cnt, 5);
            assert_eq!(neighbors_cnt, 3);

            self.clone()
        }

        fn repr(&self, _: &mut HashMap<&str, &str>) {}
        fn from_repr(&mut self, _: &HashMap<&str, &str>) {}
    }

    impl Default for MooreTestCell {

        fn default() -> Self { MooreTestCell }
    }

    #[test]
    fn test_neighbors() {
        let mut grid: SquareGrid<MooreTestCell> = SquareGrid::new(2, 2);
        grid.step();
    }
}
