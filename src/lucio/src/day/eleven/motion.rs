// TODO is pub needed?

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct Slope {
    pub x: i8,
    pub y: i8,
}

pub const SLOPES: [Slope; 8] = [
    // North
    Slope { x: 0,  y: -1 },

    // Northeast
    Slope { x: 1,  y: -1 },

    // East
    Slope { x: 1, y: 0 },

    // Southeast
    Slope { x: 1, y: 1 },

    // South
    Slope { x: 0, y: 1 },

    // Southwest
    Slope { x: -1, y: 1 },

    // West
    Slope { x: -1, y: 0 },

    // Northwest
    Slope { x: -1, y: -1 },
];

pub fn move_in_direction(index: usize, slope: i8) -> Option<usize> {
    match slope {
        0 => Some(index),
        1 => index.checked_add(1),
        -1 => index.checked_sub(1),
        _ => None
    }
}

