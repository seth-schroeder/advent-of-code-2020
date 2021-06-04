use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Empty,
    Full,
    Floor,
}

#[derive(Debug, PartialEq)]
pub struct CartesianTuple<T> {
    x: T,
    y: T,
}

type Slope = CartesianTuple<i32>;
type Point = CartesianTuple<i32>;

#[derive(Debug)]
pub enum Compass {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Compass {
    fn slope(orientation: &Compass) -> Slope {
        let (x, y) = match orientation {
            Compass::North => (0, -1),
            Compass::NorthEast => (1, -1),
            Compass::East => (1, 0),
            Compass::SouthEast => (1, 1),
            Compass::South => (0, 1),
            Compass::SouthWest => (-1, 1),
            Compass::West => (0, -1),
            Compass::NorthWest => (-1, -1),
        };

        Slope { x, y }
    }
}

impl Compass {
    fn rose() -> Vec<Compass> {
        vec![
            Compass::North,
            Compass::NorthEast,
            Compass::East,
            Compass::SouthEast,
            Compass::South,
            Compass::SouthWest,
            Compass::West,
            Compass::NorthWest,
        ]
    }
}

struct Area {
    width: usize,
    height: usize,
}

impl Area {
    fn contains(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && usize::try_from(point.x).unwrap() < self.width
            && usize::try_from(point.y).unwrap() < self.height
    }

    // this would be cooler as an iterator
    fn advance(from: &Point, orientation: &Compass) -> Point {
        let slope = Compass::slope(orientation);
        Point {
            x: from.x + slope.x,
            y: from.y + slope.y,
        }
    }

    // this reads well by lines of code, but it feels not very Rusty
    fn path(&self, from: &Point, orientation: &Compass) -> Vec<Point> {
        let mut v = Vec::new();

        loop {
            let last = match v.last() {
                Some(point) => point,
                None => from,
            };

            let point = Area::advance(last, orientation);
            eprintln!("{:?} {:?} {:?} {:?}", from, point, last, orientation);
            if !self.contains(&point) {
                break;
            }

            v.push(point);
        }

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let area = Area {
            height: 3,
            width: 3,
        };
        let point = Point { x: 1, y: 1 };
        let path = area.path(&point, &Compass::North);
        assert_eq!(1, path.len());
        assert_eq!(Point { x: 1, y: 0 }, *path.first().unwrap());
    }

    #[test]
    fn test_rose() {
        for orientation in Compass::rose() {
            let area = Area {
                height: 5,
                width: 5,
            };
            let point = Point { x: 2, y: 2 };
            assert_eq!(2, area.path(&point, &orientation).len());
        }
    }
}
