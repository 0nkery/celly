mod moore;
mod von_neumann;

pub use self::moore::MooreNhood;
pub use self::von_neumann::VonNeumannNhood;


mod test_helpers {

    use traits::Coord;

    impl Coord for (i32, i32) {
        fn from_2d(x: i32, y: i32) -> Self { (x, y) }

        fn x(&self) -> i32 { self.0 }
        fn y(&self) -> i32 { self.1 }
    }
}
