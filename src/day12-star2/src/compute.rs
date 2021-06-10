pub type Point = (i32, i32);

#[derive(Debug)]
pub struct UnsupportedRotationError {}

type Rotation = (i32, i32);
// simplified from https://en.wikipedia.org/wiki/Rotation_matrix#Common_rotations
const NINETY: Rotation = (0, 1);
const ONE_EIGHTY: Rotation = (-1, 0);
const TWO_SEVENTY: Rotation = (0, -1);

pub fn rotate_counterclockwise(
    point: Point,
    degrees: i32,
) -> Result<Point, UnsupportedRotationError> {
    let rotation = degrees_to_rotation(degrees)?;
    let (x, y) = point;
    let (cos, sin) = rotation;
    let x_ = x * cos - y * sin;
    let y_ = x * sin + y * cos;

    Ok((x_, y_))
}

pub fn rotate_clockwise(point: Point, degrees: i32) -> Result<Point, UnsupportedRotationError> {
    rotate_counterclockwise(point, clockwise_to_counterclockwise(degrees))
}

fn degrees_to_rotation(degrees: i32) -> Result<Rotation, UnsupportedRotationError> {
    match degrees {
        90 => Ok(NINETY),
        180 => Ok(ONE_EIGHTY),
        270 => Ok(TWO_SEVENTY),
        _ => Err(UnsupportedRotationError {}),
    }
}

fn clockwise_to_counterclockwise(degrees: i32) -> i32 {
    let single_rotation = degrees % 360;
    360 - single_rotation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_counterclockwise() {
        assert_eq!((-3, 5), rotate_counterclockwise((5, 3), 90).unwrap());
        assert_eq!((-5, -3), rotate_counterclockwise((5, 3), 180).unwrap());
        assert_eq!((3, -5), rotate_counterclockwise((5, 3), 270).unwrap());
    }

    #[test]
    fn test_rotate_clockwise() {
        assert_eq!((5, 3), rotate_clockwise((-3, 5), 90).unwrap());
        assert_eq!((5, 3), rotate_clockwise((-5, -3), 180).unwrap());
        assert_eq!((5, 3), rotate_clockwise((3, -5), 270).unwrap());
    }

    #[test]
    fn test_clockwise_to_counterclockwise() {
        assert_eq!(270, clockwise_to_counterclockwise(90));
        assert_eq!(180, clockwise_to_counterclockwise(180));
        assert_eq!(90, clockwise_to_counterclockwise(270));
        assert_eq!(270, clockwise_to_counterclockwise(90 + 360));
    }
}
