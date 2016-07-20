use std::marker::PhantomData;

use traits::Nhood;
use traits::Coord;


pub struct VonNeumannNhood<C: Coord> {
    phantom: PhantomData<C>,
}


impl<C: Coord> VonNeumannNhood<C> {
    pub fn new() -> Self {
        VonNeumannNhood { phantom: PhantomData }
    }
}

impl<C: Coord> Nhood for VonNeumannNhood<C> {
    type Coord = C;

    // - | 0 | -
    // 1 | x | 2
    // - | 3 | -
    fn neighbors(&self, coord: &Self::Coord) -> Vec<Self::Coord> {

        let x = coord.x();
        let y = coord.y();

        let neighbors_coords = vec![
                                      C::from_2d(x, y - 1),
            C::from_2d(x - 1, y),     /* x */               C::from_2d(x + 1, y),
                                      C::from_2d(x, y + 1),
        ];

        neighbors_coords
    }

    fn neighbors_count(&self) -> usize {
        4
    }
}


#[cfg(test)]
mod tests {

    use traits::Nhood;
    use super::VonNeumannNhood;

    #[test]
    fn test_von_neumann_nhood() {
        let nhood = VonNeumannNhood::new();

        let center = (1, 1);

        let neighbors = nhood.neighbors(&center);
        assert_eq!(neighbors.len(), nhood.neighbors_count());

        assert_eq!(neighbors[0], (1, 0));
        assert_eq!(neighbors[1], (0, 1));
        assert_eq!(neighbors[2], (2, 1));
        assert_eq!(neighbors[3], (1, 2));
    }
}
