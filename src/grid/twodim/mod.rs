//! 2D grid with neighbors iter and custom internal coordinate.

mod iter;
mod coord;
mod test;

use std::cmp;
use std::mem;
use std::ops::{Deref, DerefMut};

use scoped_threadpool::Pool;

use traits::Grid;
use traits::Cell;
use traits::EvolutionState;
use traits::Nhood;
use traits::Coord;

use self::iter::Iter;
pub use self::coord::GridCoord;

/// 2D grid. Implemented with two buffers.
/// They are swapped on every evolution step.
/// Old buffer is used for read-only neighbors data.
/// New buffer is writable and mutated through update process.
/// Grid uses one-dimensional `Vec` to store cells.
pub struct TwodimGrid<C, N, Es>
    where C: Cell<State = Es> + Clone,
          N: Nhood<Coord = GridCoord>,
          Es: EvolutionState,
{
    cells: Vec<C>,
    old_cells: Vec<C>,
    evolution_state: Es,
    nhood: N,
    neighbors: Vec<Vec<Option<usize>>>,
    dimensions: GridCoord,
    rows: u32,
    cols: u32,
    pool: Pool,
    parts: Vec<(usize, usize)>,
}


impl<C, N, Es> TwodimGrid<C, N, Es>
    where C: Cell<State = Es> + Clone,
          N: Nhood<Coord = GridCoord>,
          Es: EvolutionState,
{
    /// Constructs TwodimGrid with given ROWSxCOLS, neighborhood
    /// strategy, initial evolution state, threads count.
    pub fn new(rows: u32, cols: u32, nhood: N, state: C::State, threads: u32) -> Self {

        let len = (rows * cols) as usize;

        let cells = Vec::with_capacity(len);
        let old_cells = Vec::with_capacity(len);
        let neighbors = Vec::with_capacity(len);

        let threads = cmp::max(threads, 1);
        let pool = Pool::new(threads);

        let mut grid = TwodimGrid {
            cells: cells,
            old_cells: old_cells,
            evolution_state: state,
            nhood: nhood,
            neighbors: neighbors,
            rows: rows,
            cols: cols,
            dimensions: GridCoord::from_2d(cols as i32, rows as i32),
            pool: pool,
            parts: Vec::new(),
        };

        grid.init();

        grid
    }

    fn init(&mut self) {

        let cells_count = self.rows * self.cols;

        for offset in 0..cells_count {

            let coord = GridCoord::from_offset(offset, self.rows, self.cols);

            // init neighbors
            let neighbors = self.get_neighbors(&coord);
            self.neighbors.push(neighbors);

            // init cells
            let cell = C::with_coord(coord);
            self.cells.push(cell.clone());
            self.old_cells.push(cell.clone());
        }

        // Init split border indices.
        // Used later in `update` to split `cells` between threads.
        let cells_on_thread = cells_count / self.pool.thread_count();
        let mut start = 0;
        let mut end = cells_on_thread;
        // Emulating `do-while` loop.
        while {
            self.parts.push((start as usize, end as usize));
            start = end + 1;
            end = cmp::min(start + cells_on_thread, cells_count);
            end < cells_count
        } {}
    }

    fn get_neighbors(&self, coord: &GridCoord) -> Vec<Option<usize>> {

        let neighbors_count = self.nhood.neighbors_count();
        let mut neighbors = Vec::with_capacity(neighbors_count);

        let cols = self.cols as i32;
        let rows = self.rows as i32;

        for coord in &self.nhood.neighbors(coord) {

            if coord.x() >= 0 && coord.x() < cols && coord.y() >= 0 && coord.y() < rows {
                neighbors.push(Some(self.offset(coord)));
            } else {
                neighbors.push(None);
            }
        }

        neighbors
    }

    #[inline]
    fn offset<Crd: Coord>(&self, coord: &Crd) -> usize {
        (coord.y() * self.cols as i32 + coord.x()) as usize
    }
}

/// Helper struct to enable sendable mutable pointers.
struct MutPtr<T: ?Sized>(*mut T);

unsafe impl<T: ?Sized> Send for MutPtr<T> {}
unsafe impl<T: ?Sized> Sync for MutPtr<T> {}

impl<T: ?Sized> Deref for MutPtr<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.0 } }
}

impl<T: ?Sized> DerefMut for MutPtr<T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.0 } }
}

#[allow(expl_impl_clone_on_copy)]
impl<T: ?Sized> Clone for MutPtr<T> {
    fn clone(&self) -> Self { MutPtr(self.0) }
}

impl<T: ?Sized> Copy for MutPtr<T> {}

/// Helper struct to enable sendable const pointers.
struct ConstPtr<T: ?Sized>(*const T);

unsafe impl<T: ?Sized> Send for ConstPtr<T> {}
unsafe impl<T: ?Sized> Sync for ConstPtr<T> {}

impl<T: ?Sized> Deref for ConstPtr<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.0 } }
}

#[allow(expl_impl_clone_on_copy)]
impl<T: ?Sized> Clone for ConstPtr<T> {
    fn clone(&self) -> Self { ConstPtr(self.0) }
}

impl<T: ?Sized> Copy for ConstPtr<T> {}


impl<C, N, Es> Grid for TwodimGrid<C, N, Es>
    where C: Cell<State = Es> + Clone,
          N: Nhood<Coord = GridCoord>,
          Es: EvolutionState + Send,
{
    type Cell = C;
    type Coord = GridCoord;

    fn update(&mut self) {
        mem::swap(&mut self.cells, &mut self.old_cells);

        let mut cells = MutPtr(&mut *self.cells as *mut [C]);
        let old_cells = ConstPtr(&*self.old_cells as *const [C]);
        let neighbors = ConstPtr(&*self.neighbors as *const [Vec<Option<usize>>]);
        let neighbors_count = self.nhood.neighbors_count();
        let evolution_state = ConstPtr(&self.evolution_state as *const Es);

        let parts = &self.parts;

        self.pool.scoped(|scope| {
            for &(start, end) in parts {
                scope.execute(move || {
                    for i in start..end {
                        unsafe {
                            let neighbors = neighbors.get_unchecked(i);
                            let neighbors_iter = Iter::new(&*old_cells, neighbors, neighbors_count);

                            let old = (*old_cells).get_unchecked(i);
                            let cell = (*cells).get_unchecked_mut(i);
                            cell.update(old, neighbors_iter, &*evolution_state);
                        }
                    }
                });
            }
        });

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

    fn size(&self) -> Self::Coord { self.dimensions.clone() }
}
