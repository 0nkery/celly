use std::marker::PhantomData;

use traits::Nhood;
use traits::Coord;


pub struct MooreNhood<C: Coord> {
    phantom: PhantomData<C>,
}


impl<C: Coord> MooreNhood<C> {
    pub fn new() -> Self {
        MooreNhood { phantom: PhantomData }
    }
}

impl<C: Coord> Nhood for MooreNhood<C> {
    type Coord = C;

    // 0 | 1 | 2
    // 3 | x | 4
    // 5 | 6 | 7
    fn neighbors(&self, coord: &Self::Coord) -> Vec<Self::Coord> {

        let x = coord.x();
        let y = coord.y();

        let neighbors_coords = vec![C::from_2d(x - 1, y - 1),
                                    C::from_2d(x, y - 1),
                                    C::from_2d(x + 1, y - 1),
                                    C::from_2d(x - 1, y),
                                    // x
                                    C::from_2d(x + 1, y),
                                    C::from_2d(x - 1, y + 1),
                                    C::from_2d(x, y + 1),
                                    C::from_2d(x + 1, y + 1)];

        neighbors_coords
    }

    fn neighbors_count(&self) -> usize {
        8
    }
}

#[cfg(test)]
mod tests {

    use traits::Nhood;
    use super::MooreNhood;

    #[test]
    fn test_moore_nhood() {
        let nhood = MooreNhood::new();

        let center = (1, 1);

        let neighbors = nhood.neighbors(&center);
        assert_eq!(neighbors.len(), nhood.neighbors_count());

        assert_eq!(neighbors[0], (0, 0));
        assert_eq!(neighbors[1], (1, 0));
        assert_eq!(neighbors[2], (2, 0));
        assert_eq!(neighbors[3], (0, 1));
        assert_eq!(neighbors[4], (2, 1));
        assert_eq!(neighbors[5], (0, 2));
        assert_eq!(neighbors[6], (1, 2));
        assert_eq!(neighbors[7], (2, 2));
    }
}
