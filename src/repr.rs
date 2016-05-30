use std::collections::HashMap;

use traits::ReprConsumer;


pub struct CellRepr<'a> {
    pub x: i32,
    pub y: i32,
    pub state: HashMap<&'a str, &'a str>
}


impl<'a> CellRepr<'a> {

    pub fn new(x: i32, y: i32,
               state: Option<&HashMap<&'a str, &'a str>>)
        -> Self {

        let state = match state {
            Some(map) => map.clone(),
            None => HashMap::new()
        };

        CellRepr {
            x: x,
            y: y,
            state: state
        }
    }
}


pub struct GridRepr<'a> {
    pub rows: i32,
    pub cols: i32,
    pub cells: Vec<CellRepr<'a>>
}

impl<'a> GridRepr<'a> {

    pub fn new<'b>(rows: i32,
                   cols: i32,
                   cells: Option<Vec<CellRepr<'a>>>)
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

    fn consume(&mut self, repr: &GridRepr) {

        for cell in repr.cells.iter() {
            println!("{} {}", cell.x, cell.y);
            for (state, value) in &cell.state {
                println!("{}: {}", state, value);
            }    
        }
    }
}