type Degree = i16;

#[derive(Debug)]
pub struct Compass {
    degree: Degree,
}

#[derive(Debug, PartialEq)]
pub enum Bearing {
    North,
    East,
    South,
    West,
}

impl Compass {
    pub fn new() -> Self {
        Compass { degree: 0 }
    }

    pub fn rotate(&mut self, degrees: Degree) -> Degree {
        let input = degrees.rem_euclid(360);
        self.degree = (self.degree + input) % 360;
        self.degree
    }

    pub fn bearing(&self) -> Bearing {
        match self.degree {
            0 => Bearing::North,
            90 => Bearing::East,
            180 => Bearing::South,
            270 => Bearing::West,
            _ => panic!(
                "the number of bearings({}) shall not be 3, unless you proceed directly to 4",
                self.degree
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut c = Compass::new();

        assert_eq!(0, c.degree);
        assert_eq!(90, c.rotate(90));
        assert_eq!(180, c.rotate(90));
        assert_eq!(270, c.rotate(90));
        assert_eq!(0, c.rotate(90));

        assert_eq!(270, c.rotate(-90));
        assert_eq!(180, c.rotate(-90));
        assert_eq!(90, c.rotate(-90));
        assert_eq!(0, c.rotate(-90));

        assert_eq!(0, c.rotate(360));
        assert_eq!(0, c.rotate(-720));
    }

    #[test]
    fn test_bearing() {
        let mut c = Compass::new();
        assert_eq!(c.bearing(), Bearing::North);
        c.rotate(90);
        assert_eq!(c.bearing(), Bearing::East);
        c.rotate(90);
        assert_eq!(c.bearing(), Bearing::South);
        c.rotate(90);
        assert_eq!(c.bearing(), Bearing::West);
        c.rotate(90);
        assert_eq!(c.bearing(), Bearing::North);
        c.rotate(-90);
        assert_eq!(c.bearing(), Bearing::West);
    }
}
