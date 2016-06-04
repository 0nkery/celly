use std::marker::PhantomData;

use traits::Nhood;
use traits::Coord;


pub struct MooreNhood<C: Coord> {
    phantom: PhantomData<C>,
}


impl<C: Coord> MooreNhood<C> {

    pub fn new() -> Self { MooreNhood { phantom: PhantomData } }
}

impl<C: Coord> Nhood for MooreNhood<C> {
    type Coord = C;

    // 0 | 1 | 2
    // 3 | x | 4
    // 5 | 6 | 7
    fn neighbors(&self, coord: &Self::Coord) -> Vec<Self::Coord> {

        let x = coord.x();
        let y = coord.y();

        let neighbors_coords = vec![
            C::from_2d(x - 1, y - 1), C::from_2d(x - 1, y), C::from_2d(x - 1, y + 1),
            C::from_2d(x, y - 1),     /* x */               C::from_2d(x, y + 1),
            C::from_2d(x + 1, y - 1), C::from_2d(x + 1, y), C::from_2d(x + 1, y + 1)
        ];

        neighbors_coords
    }

    fn neighbors_count(&self) -> usize { 8 }
}