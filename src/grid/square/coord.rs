use traits::Coord;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GridCoord {
    x: i32,
    y: i32,
}


impl GridCoord {

    #[inline]
    pub fn from_offset(offset: i32, rows: i32, cols: i32) -> GridCoord {
        let col = offset % cols;
        let row = (offset - col) / rows;
        
        GridCoord {
            x: col,
            y: row,
        }
    }
}


impl Coord for GridCoord {

    fn from_2d(x: i32, y: i32) -> Self {
        GridCoord {
            x: x,
            y: y,
        }
    }

    fn x(&self) -> i32 { self.x }

    fn y(&self) -> i32 { self.y }
}


#[cfg(test)]
mod tests {

    use traits::Coord;
    use traits::Nhood;
    use grid::nhood::MooreNhood;
    use super::GridCoord;

    #[test]
    fn test_from_offset() {
        let c = GridCoord::from_offset(0, 10, 10);
        assert_eq!(c.x(), 0);
        assert_eq!(c.y(), 0);

        let c = GridCoord::from_offset(10, 10, 10);
        assert_eq!(c.x(), 0);
        assert_eq!(c.y(), 1);

        let c = GridCoord::from_offset(100, 10, 10);
        assert_eq!(c.x(), 0);
        assert_eq!(c.y(), 10);
    }

    #[test]
    fn test_with_moore_nhood() {
        // 3 x 3 square grid with Moore's neighborhood

        let nhood = MooreNhood::new();

        let center = GridCoord::from_2d(1, 1);

        let neighbors = nhood.neighbors(&center);
        assert_eq!(neighbors.len(), nhood.neighbors_count());

        assert_eq!(neighbors[0], GridCoord::from_2d(0, 0));
        assert_eq!(neighbors[1], GridCoord::from_2d(1, 0));
        assert_eq!(neighbors[2], GridCoord::from_2d(2, 0));
        assert_eq!(neighbors[3], GridCoord::from_2d(0, 1));
        assert_eq!(neighbors[4], GridCoord::from_2d(2, 1));
        assert_eq!(neighbors[5], GridCoord::from_2d(0, 2));
        assert_eq!(neighbors[6], GridCoord::from_2d(1, 2));
        assert_eq!(neighbors[7], GridCoord::from_2d(2, 2));
    }
}