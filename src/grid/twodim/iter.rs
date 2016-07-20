use traits::Cell;


pub struct Iter<'a, C: 'a> {
    cells: &'a Vec<C>,
    neighbors: &'a Vec<Option<usize>>,
    index: usize,
    count: usize,
}


impl<'a, C> Iter<'a, C> {
    pub fn new(cells: &'a Vec<C>, neighbors: &'a Vec<Option<usize>>, count: usize) -> Self {

        Iter {
            cells: cells,
            neighbors: neighbors,
            count: count,
            index: 0,
        }
    }
}


impl<'a, C: Cell> Iterator for Iter<'a, C> {
    type Item = Option<&'a C>;

    fn next(&mut self) -> Option<Self::Item> {

        let next = match self.index {
            i @ _ if i < self.count => {

                let maybe_index = self.neighbors[i];
                match maybe_index {
                    Some(index) => Some(Some(&self.cells[index])),
                    None => Some(None),
                }
            },
            _ => {
                self.index = 0;
                None
            },
        };
        self.index += 1;

        next
    }
}
