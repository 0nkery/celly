use grid::Grid;
use cell::Cell;


const NEIGHBORS_COUNT: usize = 8;

type Neighbors = [Option<usize>; NEIGHBORS_COUNT];


pub struct SquareGrid<C: Cell + Copy> {
    cells: Vec<C>,
    old_cells: Vec<C>,
    neighbors: Vec<Neighbors>,
    rows: i32,
    cols: i32,
}


impl<C: Cell + Copy> SquareGrid<C> {

    pub fn new(rows: i32, cols: i32, initial: C) -> Self {

        let len = (rows * cols) as usize;
        let cells = vec![initial; len];
        let old_cells = Vec::new();
        let neighbors = Vec::with_capacity(len);

        let mut grid = SquareGrid {
            cells: cells,
            old_cells: old_cells,
            neighbors: neighbors,
            rows: rows,
            cols: cols,
        };
        grid.init_neighbors();

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

    fn neighbors_iter<'a>(&self,
                          cells: &'a Vec<C>,
                          neighbors: Neighbors)
        -> MooreSquareGridIterator<'a, C> {

        MooreSquareGridIterator {
            cells: cells,
            neighbors: neighbors,
            index: 0,
        }
    }
}


impl<C: Cell + Copy> Grid for SquareGrid<C> {

    fn step(&mut self) {
        self.old_cells = self.cells.clone();

        for (cell_no, cell) in self.old_cells.iter().enumerate() {
            let neighbors = self.neighbors[cell_no];
            let neighbors_iter = self.neighbors_iter(&self.old_cells, neighbors);
            self.cells[cell_no] = cell.step(neighbors_iter);
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


mod test {

    use cell::Cell;
    use grid::Grid;

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
    }

    #[test]
    fn test_neighbors() {
        let initial = MooreTestCell;
        let mut grid = super::SquareGrid::new(2, 2, initial);
        grid.step();
    }
}
