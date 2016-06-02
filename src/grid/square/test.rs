#![cfg(test)]
use std::collections::HashMap;

use traits::Cell;
use traits::Grid;
use grid::nhood::MooreNhood;
use grid::square::SquareGrid;

#[derive(Clone, Debug)]
struct MooreTestCell;

impl Cell for MooreTestCell {

    fn step<'a, I>(&self, neighbors: I) -> Self 
        where I: Iterator<Item=Option<&'a Self>> {

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
                }
            };
            total += 1;
        }

        assert_eq!(total, 8);
        assert_eq!(none_cnt, 5);
        assert_eq!(neighbors_cnt, 3);

        self.clone()
    }

    fn repr(&self, _: &mut HashMap<&str, &str>) {}
    fn from_repr(&mut self, _: &HashMap<&str, &str>) {}
}

impl Default for MooreTestCell {

    fn default() -> Self { MooreTestCell }
}

#[test]
fn test_neighbors() {
    let nhood = MooreNhood::new();
    let mut grid: SquareGrid<MooreTestCell, _> = SquareGrid::new(2, 2, nhood);
    grid.step();
}
