use std::{
    cell::RefCell,
    ops::{Index, IndexMut, Mul},
    vec,
};

use crate::{tuple::Tuple, util::eq_f64};

#[derive(Debug, Clone)]
pub struct Matrix {
    width: usize,
    value: Vec<f64>,
    det: RefCell<Option<f64>>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            width,
            value: vec![f64::default(); width * height],
            det: RefCell::new(None),
        }
    }

    pub fn identity(dimension: usize) -> Self {
        let mut m = Matrix::new(dimension, dimension);
        for i in 0..dimension {
            m[(i, i)] = 1.0
        }
        m
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.value.len() / self.width
    }

    fn row(&self, row: usize) -> Vec<f64> {
        self.value[row * self.width..row * self.width + self.width]
            .iter()
            .map(|v| *v)
            .collect()
    }

    fn column(&self, column: usize) -> Vec<f64> {
        self.value
            .iter()
            .skip(column)
            .step_by(self.width)
            .map(|v| *v)
            .collect()
    }

    pub fn transpose(&self) -> Self {
        Matrix {
            width: self.height(),
            value: (0..self.width)
                .into_iter()
                .map(|c| self.column(c))
                .flat_map(|c| c.into_iter())
                .collect(),
            det: RefCell::new(None),
        }
    }

    fn determinate(&self) -> f64 {
        if let Some(det) = *self.det.borrow() {
            return det;
        }
        let mut det = 0.0;
        if self.width() == 2 || self.height() == 2 {
            det = self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)];
        } else {
            for col in 0..self.width() {
                det += self[(0, col)] * self.cofactor(0, col);
            }
        }
        self.det.replace(Some(det));
        det
    }

    fn sub_matrix(&self, row: usize, column: usize) -> Matrix {
        let mut matrix = Matrix::new(self.width() - 1, self.height() - 1);
        for x in 0..self.width() {
            if x != column {
                for y in 0..self.height() {
                    if y != row {
                        matrix[(
                            if y > row { y - 1 } else { y },
                            if x > column { x - 1 } else { x },
                        )] = self[(y, x)]
                    }
                }
            }
        }
        matrix
    }

    fn minor(&self, row: usize, column: usize) -> f64 {
        self.sub_matrix(row, column).determinate()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -1.0 * minor
        }
    }

    fn is_invertible(&self) -> bool {
        !eq_f64(0.0, self.determinate())
    }

    pub fn inverse(&self) -> Option<Self> {
        if !self.is_invertible() {
            return None;
        }

        let mut inv = Matrix::new(self.width(), self.height());
        let det = self.determinate();

        for row in 0..self.height() {
            for col in 0..self.width() {
                inv[(col, row)] = self.cofactor(row, col) / det;
            }
        }

        Some(inv)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity(4)
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(value: Vec<Vec<f64>>) -> Self {
        Matrix {
            width: value[0].len(),
            value: value.into_iter().flat_map(|r| r).collect(),
            det: RefCell::new(None),
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.value[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.value[y * self.width + x]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.value.len() == other.value.len()
            && self
                .value
                .iter()
                .zip(other.value.iter())
                .all(|(l, r)| eq_f64(*l, *r))
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Matrix::new(self.width(), self.height());
        for row in 0..self.height() {
            for column in 0..self.width() {
                m[(row, column)] = self
                    .row(row)
                    .into_iter()
                    .zip(rhs.column(column).into_iter())
                    .map(|(l, r)| l * r)
                    .sum()
            }
        }
        m
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        assert!(self.height() == 4 && self.width() == 4);
        let vals = (0..self.height())
            .into_iter()
            .map(|i| self.row(i))
            .map(|r| Tuple::new(r[0], r[1], r[2], r[3]))
            .map(|t| t * rhs)
            .collect::<Vec<_>>();

        Tuple::new(vals[0], vals[1], vals[2], vals[3])
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        assert!(self.height() == 4 && self.width() == 4);
        &self * rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::util::eq_f64;

    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let inner = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ];
        let m = Matrix::from(inner);

        assert!(eq_f64(1.0, m[(0, 0)]));
        assert!(eq_f64(4.0, m[(0, 3)]));
        assert!(eq_f64(5.5, m[(1, 0)]));
        assert!(eq_f64(7.5, m[(1, 2)]));
        assert!(eq_f64(11.0, m[(2, 2)]));
        assert!(eq_f64(13.5, m[(3, 0)]));
        assert!(eq_f64(15.5, m[(3, 2)]));
    }

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let inner = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let m = Matrix::from(inner);

        assert!(eq_f64(-3.0, m[(0, 0)]));
        assert!(eq_f64(5.0, m[(0, 1)]));
        assert!(eq_f64(1.0, m[(1, 0)]));
        assert!(eq_f64(-2.0, m[(1, 1)]));
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let inner = vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 11.0, 1.0],
        ];
        let m = Matrix::from(inner);

        assert!(eq_f64(-3.0, m[(0, 0)]));
        assert!(eq_f64(-2.0, m[(1, 1)]));
        assert!(eq_f64(1.0, m[(2, 2)]));
    }

    #[test]
    fn matrix_equality_with_identical_matricies() {
        let a = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a, b)
    }

    #[test]
    fn matrix_equality_with_different_matricies() {
        let a = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 3.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_ne!(a, b)
    }

    #[test]
    fn multiplying_two_matricies() {
        let a = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::from(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = Matrix::from(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(expected, &a * &b);
    }

    #[test]
    fn row_returns_slice_of_values_of_nth_row() {
        let a = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(vec![1.0, 2.0, 3.0, 4.0], a.row(0));
        assert_eq!(vec![5.0, 6.0, 7.0, 8.0], a.row(1));
        assert_eq!(vec![9.0, 8.0, 7.0, 6.0], a.row(2));
        assert_eq!(vec![5.0, 4.0, 3.0, 2.0], a.row(3));
    }

    #[test]
    fn column_returns_slice_of_values_of_nth_column() {
        let a = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(vec![1.0, 5.0, 9.0, 5.0,], a.column(0));
        assert_eq!(vec![2.0, 6.0, 8.0, 4.0,], a.column(1));
        assert_eq!(vec![3.0, 7.0, 7.0, 3.0,], a.column(2));
        assert_eq!(vec![4.0, 8.0, 6.0, 2.0,], a.column(3));
    }

    #[test]
    fn a_matrix_muliplied_by_a_tuple() {
        let a = Matrix::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(expected, &a * b)
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix::from(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);

        let identity = Matrix::identity(4);

        let b = &a * &identity;

        assert_eq!(a, b);

        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(a, &identity * a)
    }

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix::from(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = Matrix::from(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(expected, a.transpose());
    }

    #[test]
    fn calculating_the_determinate_of_a_2x2_matrix() {
        let a = Matrix::from(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);

        assert_eq!(17.0, a.determinate());
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = Matrix::from(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        let expected = Matrix::from(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);

        assert_eq!(expected, a.sub_matrix(0, 2));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = Matrix::from(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);
        let expected = Matrix::from(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);

        assert_eq!(expected, a.sub_matrix(2, 1));
    }

    #[test]
    fn computing_the_submatrix_of_a_3x3_matrix() {
        let a = Matrix::from(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        let b = a.sub_matrix(1, 0);

        assert_eq!(25.0, b.determinate());
        assert_eq!(25.0, a.minor(1, 0));
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = Matrix::from(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(-12.0, a.minor(0, 0));
        assert_eq!(-12.0, a.cofactor(0, 0));
        assert_eq!(25.0, a.minor(1, 0));
        assert_eq!(-25.0, a.cofactor(1, 0));
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = Matrix::from(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);
        assert_eq!(56.0, a.cofactor(0, 0));
        assert_eq!(12.0, a.cofactor(0, 1));
        assert_eq!(-46.0, a.cofactor(0, 2));
        assert_eq!(-196.0, a.determinate());
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = Matrix::from(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(690.0, a.cofactor(0, 0));
        assert_eq!(447.0, a.cofactor(0, 1));
        assert_eq!(210.0, a.cofactor(0, 2));
        assert_eq!(51.0, a.cofactor(0, 3));
        assert_eq!(-4071.0, a.determinate());
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = Matrix::from(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);

        assert_eq!(-2120.0, a.determinate());
        assert!(a.is_invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix::from(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(0.0, a.determinate());
        assert!(!a.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = Matrix::from(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse();

        assert!(b.is_some());

        let b = b.unwrap();

        assert!(eq_f64(532.0, a.determinate()));
        assert!(eq_f64(-160.0, a.cofactor(2, 3)));
        assert!(eq_f64(-160.0 / 532.0, b[(3, 2)]));
        assert!(eq_f64(105.0, a.cofactor(3, 2)));
        assert!(eq_f64(105.0 / 532.0, b[(2, 3)]));

        let expected = Matrix::from(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_eq!(expected, b);
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a = Matrix::from(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);
        let b = a.inverse();

        assert!(b.is_some());

        let b = b.unwrap();

        let expected = Matrix::from(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(expected, b);
    }

    #[test]
    fn calculating_the_inverse_of_third_matrix() {
        let a = Matrix::from(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);
        let b = a.inverse();

        assert!(b.is_some());

        let b = b.unwrap();

        let expected = Matrix::from(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(expected, b);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix::from(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = Matrix::from(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);

        let c = &a * &b;
        assert_eq!(a, &c * &b.inverse().unwrap());
    }
}
