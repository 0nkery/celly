#![cfg(test)]
use traits::Cell;
use traits::Coord;
use traits::Grid;
use grid::nhood::MooreNhood;
use grid::twodim::TwodimGrid;
use grid::EmptyState;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MooreTestCell {
    coord: (i32, i32),
}

impl Cell for MooreTestCell {
    type Coord = (i32, i32);
    type State = EmptyState;

    fn update<'a, I>(&'a mut self, _: &'a Self, neighbors: I, _: &Self::State)
        where I: Iterator<Item = Option<&'a Self>>,
    {

        // Initial value should be copied
        // There should be 3 neighbors and 5 None values
        // Overall count of neighbors should be equal to 8
        let mut total = 0;
        let mut none_cnt = 0;
        let mut neighbors_cnt = 0;
        for neighbor in neighbors {
            match neighbor {
                None => {
                    none_cnt += 1;
                },
                Some(_) => {
                    neighbors_cnt += 1;
                },
            };
            total += 1;
        }

        assert_eq!(total, 8);
        assert_eq!(none_cnt, 5);
        assert_eq!(neighbors_cnt, 3);
    }

    fn with_coord<C: Coord>(coord: C) -> Self { MooreTestCell { coord: (coord.x(), coord.y()) } }

    fn coord(&self) -> &Self::Coord { &self.coord }

    fn set_coord<C: Coord>(&mut self, coord: &C) { self.coord = (coord.x(), coord.y()); }
}


#[test]
fn test_neighbors() {
    let nhood = MooreNhood::new();
    let mut grid: TwodimGrid<MooreTestCell, _, _> = TwodimGrid::new(2, 2, nhood, EmptyState, 1);
    grid.update();
}
