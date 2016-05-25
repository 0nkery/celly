use grid::Grid;
use cell::Cell;


pub struct SquareGrid<C: Cell> {
	cells: Vec<C>
}


impl<C: Cell> Grid for SquareGrid<C> {
	
}