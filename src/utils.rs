use traits::Coord;

#[cfg(test)]
pub use self::test_utils::find_cell;

impl Coord for (i32, i32) {
    fn from_2d(x: i32, y: i32) -> Self { (x, y) }

    fn x(&self) -> i32 { self.0 }
    fn y(&self) -> i32 { self.1 }
}


#[cfg(test)]
mod test_utils {
    use traits::Cell;
    use traits::Coord;

    pub fn find_cell<C: Cell + Clone>(cells: &[C], x: i32, y: i32) -> C {

        assert!(cells.iter().any(|c| c.coord().x() == x && c.coord().y() == y));

        let found = cells.iter()
            .find(|c| c.coord().x() == x && c.coord().y() == y)
            .unwrap();

        found.clone()
    }
}
