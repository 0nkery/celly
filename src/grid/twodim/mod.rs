mod iter;
mod coord;
mod test;

use std::mem;

use traits::Grid;
use traits::Cell;
use traits::EvolutionState;
use traits::Nhood;
use traits::Coord;

use self::iter::Iter;
pub use self::coord::GridCoord;


pub struct TwodimGrid<C, N>
    where C: Cell,
          N: Nhood<Coord = GridCoord>,
{
    cells: Vec<C>,
    old_cells: Vec<C>,
    evolution_state: C::State,
    nhood: N,
    neighbors: Vec<Vec<Option<usize>>>,
    dimensions: GridCoord,
    rows: i32,
    cols: i32,
}


impl<C, N> TwodimGrid<C, N>
    where C: Cell,
          N: Nhood<Coord = GridCoord>,
{
    pub fn new(rows: i32, cols: i32, nhood: N, state: C::State) -> Self {

        let len = (rows * cols) as usize;

        let cells = Vec::with_capacity(len);
        let old_cells = Vec::with_capacity(len);
        let neighbors = Vec::with_capacity(len);

        let mut grid = TwodimGrid {
            cells: cells,
            old_cells: old_cells,
            evolution_state: state,
            nhood: nhood,
            neighbors: neighbors,
            rows: rows,
            cols: cols,
            dimensions: GridCoord::from_2d(cols, rows),
        };

        grid.init();

        grid
    }

    fn init(&mut self) {

        for offset in 0..self.rows * self.cols {

            let coord = GridCoord::from_offset(offset as i32, self.rows, self.cols);

            // init neighbors
            let neighbors = self.get_neighbors(&coord);
            self.neighbors.push(neighbors);

            // init cells
            let cell = C::with_coord(coord);
            self.cells.push(cell.clone());
            self.old_cells.push(cell.clone());
        }
    }

    fn get_neighbors(&self, coord: &GridCoord) -> Vec<Option<usize>> {

        let neighbors_count = self.nhood.neighbors_count();
        let mut neighbors = Vec::with_capacity(neighbors_count);

        for coord in self.nhood.neighbors(coord).iter() {

            if coord.x() >= 0 && coord.x() < self.cols && coord.y() >= 0 && coord.y() < self.rows {
                neighbors.push(Some(self.offset(coord)));
            } else {
                neighbors.push(None);
            }
        }

        neighbors
    }

    fn neighbors_iter<'b>(&self,
                          cells: &'b Vec<C>,
                          neighbors: &'b Vec<Option<usize>>)
                          -> Iter<'b, C> {

        Iter::new(cells, neighbors, self.nhood.neighbors_count())
    }

    #[inline]
    pub fn offset<Crd: Coord>(&self, coord: &Crd) -> usize {
        (coord.y() * self.cols + coord.x()) as usize
    }
}


impl<C, N> Grid for TwodimGrid<C, N>
    where C: Cell,
          N: Nhood<Coord = GridCoord>,
{
    type Cell = C;
    type Coord = GridCoord;

    fn update(&mut self) {
        mem::swap(&mut self.cells, &mut self.old_cells);

        for (cell_no, cell) in self.old_cells.iter().enumerate() {

            let ref neighbors = self.neighbors[cell_no];
            let neighbors_iter = self.neighbors_iter(&self.old_cells, &neighbors);

            let mut new_cell = cell.update(neighbors_iter, &self.evolution_state);
            new_cell.set_coord(cell.coord());

            self.cells[cell_no] = new_cell;
        }

        self.evolution_state.update();
    }

    fn set_cells(&mut self, new_cells: Vec<Self::Cell>) {

        for cell in new_cells.into_iter() {

            let index;

            {
                index = self.offset(cell.coord());
            }

            self.cells[index] = cell;
        }
    }

    fn cells(&self) -> &Vec<Self::Cell> { &self.cells }

    fn state(&self) -> &<<Self as Grid>::Cell as Cell>::State { &self.evolution_state }

    fn dimensions(&self) -> Self::Coord { self.dimensions.clone() }
}
