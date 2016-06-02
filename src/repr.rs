use std::collections::HashMap;

use traits::Coord;
use traits::ReprConsumer;


#[derive(Debug)]
pub struct CellRepr<'a, C: Coord> {
    pub coord: C,
    pub state: HashMap<&'a str, &'a str>
}


impl<'a, C: Coord> CellRepr<'a, C> {

    pub fn new(coord: C, state: Option<&HashMap<&'a str, &'a str>>) -> Self {

        let state = match state {
            Some(map) => map.clone(),
            None => HashMap::new()
        };

        CellRepr {
            coord: coord,
            state: state
        }
    }
}


#[derive(Debug)]
pub struct GridRepr<'a, C: Coord> {
    pub rows: i32,
    pub cols: i32,
    pub cells: Vec<CellRepr<'a, C>>
}

impl<'a, C: Coord> GridRepr<'a, C> {

    pub fn new<'b>(rows: i32,
                   cols: i32,
                   cells: Option<Vec<CellRepr<'a, C>>>)
        -> Self {

        let cells = match cells {
            Some(cs) => cs,
            None => {
                let len = (rows * cols) as usize;
                Vec::with_capacity(len)
            }
        };

        GridRepr {
            rows: rows,
            cols: cols,
            cells: cells
        }
    }
}


pub struct SimpleConsumer;


impl SimpleConsumer {

    pub fn new() -> Self {
        SimpleConsumer
    }
}

impl ReprConsumer for SimpleConsumer {

    fn consume<C: Coord>(&mut self, repr: &GridRepr<C>) {

        for cell in repr.cells.iter() {
            println!("{} {}", cell.coord.x(), cell.coord.y());
            for (state, value) in &cell.state {
                println!("{}: {}", state, value);
            }    
        }
    }
}