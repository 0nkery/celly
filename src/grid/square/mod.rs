mod iter;
mod coord;
mod test;

use traits::Grid;
use traits::Cell;
use traits::Nhood;
use traits::Coord;

use self::iter::Iter;
pub use self::coord::GridCoord;


pub struct SquareGrid<C, N>
    where C: Cell,
          N: Nhood<Coord=GridCoord> {

    cells: Vec<C>,
    old_cells: Vec<C>,
    nhood: N,
    neighbors: Vec<Vec<Option<usize>>>,
    dimensions: GridCoord,
    rows: i32,
    cols: i32
}


impl<C, N> SquareGrid<C, N>
    where C: Cell,
          N: Nhood<Coord=GridCoord> {

    pub fn new(rows: i32, cols: i32, nhood: N) -> Self {

        let len = (rows * cols) as usize;

        let cells = vec![C::default(); len];
        let old_cells = Vec::with_capacity(len);
        let neighbors = Vec::with_capacity(len);

        let mut grid = SquareGrid {
            cells: cells,
            old_cells: old_cells,
            nhood: nhood,
            neighbors: neighbors,
            rows: rows,
            cols: cols,
            dimensions: GridCoord::from_2d(cols, rows)
        };

        grid.init();

        grid
    }

    fn init(&mut self) {

        for offset in 0 .. self.cells.len() {

            let coord = GridCoord::from_offset(offset as i32, self.rows, self.cols);

            // init neighbors
            let neighbors = self.get_neighbors(&coord);
            self.neighbors.push(neighbors);
        }
    }

    fn get_neighbors(&self, coord: &GridCoord) -> Vec<Option<usize>> {

        let neighbors_count = self.nhood.neighbors_count();
        let mut neighbors = Vec::with_capacity(neighbors_count);

        for coord in self.nhood.neighbors(coord).iter() {

            if coord.x() >= 0 && coord.x() < self.cols &&
               coord.y() >= 0 && coord.y() < self.rows {

                neighbors.push(Some(self.offset(coord)));
            }
            else {
                neighbors.push(None);
            }
        }

        neighbors
    }

    fn neighbors_iter<'b>(&self,
                          cells: &'b Vec<C>,
                          neighbors: &'b Vec<Option<usize>>) -> Iter<'b, C> {

        Iter::new(cells, neighbors, self.nhood.neighbors_count())
    }

    #[inline]
    pub fn offset(&self, coord: &GridCoord) -> usize {
        (coord.y() * self.cols + coord.x()) as usize
    }
}


impl<C, N> Grid for SquareGrid<C, N>
    where C: Cell,
          N: Nhood<Coord=GridCoord> {

    type Cell = C;
    type Coord = GridCoord;

    fn step(&mut self) {
        self.old_cells = self.cells.clone();

        for (cell_no, cell) in self.old_cells.iter().enumerate() {

            let ref neighbors = self.neighbors[cell_no];
            let neighbors_iter = self.neighbors_iter(&self.old_cells, &neighbors);

            let new_cell = cell.step(neighbors_iter);

            self.cells[cell_no] = new_cell;
        }
    }

    fn restore<G: Grid>(&mut self, other: &G) {

        debug_assert_eq!(self.dimensions().x(), other.dimensions().x());
        debug_assert_eq!(self.dimensions().y(), other.dimensions().y());

    }

    fn cells(&self) -> &Vec<C> {
        &self.cells
    }

    fn dimensions(&self) -> Self::Coord {
        self.dimensions.clone()
    }
}
