type Bearing = i16;

#[derive(Debug)]
pub struct Compass {
    bearing: Bearing,
}

impl Compass {
    pub fn new() -> Self {
        Compass { bearing: 0 }
    }

    fn rotate(&mut self, degrees: Bearing) -> Bearing {
        self.bearing += degrees;
        self.bearing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut c = Compass::new();
        assert_eq!(0, c.bearing);
        assert_eq!(90, c.rotate(90));
        assert_eq!(0, c.rotate(-90));
        assert_eq!(0, c.rotate(360));
    }
}
