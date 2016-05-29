use std::collections::HashMap;

use traits::ReprConsumer;


pub struct CellRepr<'a> {
    x: i32,
    y: i32,
    pub state: HashMap<&'a str, &'a str>
}


impl<'a> CellRepr<'a> {

    pub fn new(x: i32, y: i32) -> Self {
        CellRepr {
            x: x,
            y: y,
            state: HashMap::new()
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

    fn consume(&self, repr: &Vec<CellRepr>) {

        for cell in repr.iter() {
            println!("{} {}", cell.x, cell.y);
            for (state, value) in &cell.state {
                println!("{}: {}", state, value);
            }    
        }
    }
}