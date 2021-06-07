use array2d::Array2D;
use core::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct CartesianArray2D<T: Clone> {
    pub backing_store: Array2D<T>,
}

impl<T: Clone> CartesianArray2D<T> {
    fn transform_coords(&self, x_y: &(usize, usize)) -> (usize, usize) {
        let (row, col) = x_y;
        let row_ = self.backing_store.num_rows() - 1 - col;
        let col_ = *row;

        (row_, col_)
    }

    pub fn num_columns(&self) -> usize {
        self.backing_store.num_columns()
    }

    pub fn num_rows(&self) -> usize {
        self.backing_store.num_rows()
    }
}

impl<T: Clone> Index<(usize, usize)> for CartesianArray2D<T> {
    type Output = T;
    fn index(&self, x_y: (usize, usize)) -> &Self::Output {
        &self.backing_store[self.transform_coords(&x_y)]
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for CartesianArray2D<T> {
    fn index_mut(&mut self, x_y: (usize, usize)) -> &mut Self::Output {
        let (x, y) = self.transform_coords(&x_y);
        self.backing_store.get_mut(x, y).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Vec<Vec<char>> {
        vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]
    }

    #[test]
    fn test_write() {
        let arr2d = Array2D::from_rows(&data());

        assert_eq!('a', arr2d[(0, 0)]);

        let mut carr2d = CartesianArray2D {
            backing_store: arr2d,
        };

        assert_eq!('a', carr2d[(0, 2)]);
        carr2d[(0, 2)] = 'z';
        assert_eq!('z', carr2d[(0, 2)]);

        assert_eq!('z', carr2d.backing_store[(0, 0)]);
    }

    #[test]
    fn test_read() {
        let arr2d = Array2D::from_rows(&data());

        assert_eq!('a', arr2d[(0, 0)]);
        assert_eq!('b', arr2d[(0, 1)]);
        assert_eq!('g', arr2d[(2, 0)]);
        assert_eq!('h', arr2d[(2, 1)]);

        let carr2d = CartesianArray2D {
            backing_store: arr2d,
        };
        assert_eq!('a', carr2d[(0, 2)]);
        assert_eq!('g', carr2d[(0, 0)]);
        assert_eq!('i', carr2d[(2, 0)]);
        assert_eq!('f', carr2d[(2, 1)]);
    }
}
