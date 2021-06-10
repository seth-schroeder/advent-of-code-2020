use crate::compute;

pub struct Ship {
    pub ship: compute::Point,
    pub waypoint: compute::Point,
}

impl Ship {
    pub fn waypoint_gap(&self) -> compute::Point {
        (self.waypoint.0.abs(), self.waypoint.1.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obligatory() {
        assert_eq!(true, !false);
    }
}
