type Point = (i32, i32);

type Rotation = (i32, i32);
// simplified from https://en.wikipedia.org/wiki/Rotation_matrix#Common_rotations
const NINETY: Rotation = (0, 1);
const ONE_EIGHTY: Rotation = (-1, 0);
const TWO_SEVENTY: Rotation = (0, -1);

fn clockwise_to_counterclockwise(degrees: i32) -> i32 {
    let single_rotation = degrees % 360;
    360 - single_rotation
}

fn degrees_to_rotation(degrees: i32) -> Option<Rotation> {
    match degrees {
        90 => Some(NINETY),
        180 => Some(ONE_EIGHTY),
        270 => Some(TWO_SEVENTY),
        _ => None
    }
}

fn rotate(point: Point, rotation: Rotation) -> Point {
    let (x, y) = point;
    let (cos, sin) = rotation;
    let x_ = x * cos - y * sin;
    let y_ = x * sin + y * cos;

    (x_, y_)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_rotation() {
        assert_eq!((-3, 5), rotate((5, 3), NINETY));
        assert_eq!((-5, -3), rotate((5, 3), ONE_EIGHTY));
        assert_eq!((3, -5), rotate((5, 3), TWO_SEVENTY));
    }

    #[test]
    fn test_clockwise_to_counterclockwise() {
        assert_eq!(270, clockwise_to_counterclockwise(90));
        assert_eq!(180, clockwise_to_counterclockwise(180));
        assert_eq!(90, clockwise_to_counterclockwise(270));
        assert_eq!(270, clockwise_to_counterclockwise(90 + 360));
    }
}
