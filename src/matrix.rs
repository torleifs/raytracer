use crate::math;
use std::cmp;
use std::ops;

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    data: Vec<f64>,
}

impl ops::Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols..(index + 1) * self.cols]
    }
}
impl ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.cols..(index + 1) * self.cols]
    }
}
impl cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }
        if self.data.len() != other.data.len() {
            return false;
        }
        for (i, num) in self.data.iter().enumerate() {
            if !math::equal(*num, other.data[i]) {
                return false;
            }
        }
        return true;
    }
}

impl<'a> ops::Mul for &'a Matrix {
    type Output = Matrix;
    fn mul(self, other: &'a Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut res = Matrix {
            rows: self.rows,
            cols: other.cols,
            data: vec![0.; self.rows * other.cols],
        };
        for row in 0..self.rows {
            for column in 0..self.cols {
                let mut dot = 0.0;
                for i in 0..self.rows {
                    dot += self[row][i] * other[i][column];
                }
                res[row][column] = dot;
            }
        }
        res
    }
}
impl ops::Mul<math::Tuple> for Matrix {
    type Output = math::Tuple;
    fn mul(self, tuple: math::Tuple) -> math::Tuple {
        assert_eq!(self.cols, 4);
        let mut res = [0.; 4];
        for row in 0..self.rows {
            res[row] = self[row][0] * tuple.x
                + self[row][1] * tuple.y
                + self[row][2] * tuple.z
                + self[row][3] * tuple.w
        }
        return math::Tuple::new(res[0], res[1], res[2], res[3]);
    }
}
impl Matrix {
    pub fn new(data: &[&[f64]]) -> Matrix {
        let rows = data.len();
        let cols = data[0].len();
        for r in data {
            assert_eq!(cols, r.len());
        }
        let mut matrix_data: Vec<f64> = Vec::new();
        for row in data {
            matrix_data.extend(*row);
        }

        Matrix {
            rows,
            cols,
            data: matrix_data,
        }
    }
    pub fn new_identity_matrix(dim: usize) -> Matrix {
        let mut data: Vec<f64> = vec![0.; dim * dim];

        for col in 0..dim {
            for row in 0..dim {
                if col == row {
                    data[dim * col + col] = 1.;
                }
            }
        }
        Matrix {
            rows: dim,
            cols: dim,
            data,
        }
    }
    pub fn new_empty_matrix(dim: usize) -> Matrix {
        let data: Vec<f64> = vec![0.; dim * dim];
        Matrix {
            rows: dim,
            cols: dim,
            data,
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut data: Vec<f64> = Vec::new();
        for col in 0..self.cols {
            let mut transposed_row: Vec<f64> = Vec::new();
            for row in 0..self.rows {
                transposed_row.push(self.data[row * (self.cols) + col])
            }
            data.extend(transposed_row);
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }
    pub fn determinant(&self) -> f64 {
        assert_eq!(self.cols, self.rows);
        if self.cols == 2 {
            return self[0][0] * self[1][1] - self[0][1] * self[1][0];
        } else {
            let mut determinant = 0.;
            for col in 0..self.cols {
                determinant += self[0][col] * self.cofactor(0, col);
            }
            return determinant;
        }
    }
    pub fn sub_matrix(&self, row: usize, column: usize) -> Matrix {
        assert_eq!(self.rows, self.cols);

        let mut new_data = self.data.clone();
        //remove row:
        let start_index = row * self.cols;
        new_data.drain(start_index..start_index + self.cols);
        let rows = self.rows - 1;
        //remove col:
        for row in 0..rows {
            new_data.remove(row * self.cols + column - row);
        }
        Matrix {
            rows: self.rows - 1,
            cols: self.cols - 1,
            data: new_data,
        }
    }
    pub fn minor(&self, row: usize, column: usize) -> f64 {
        let sub_matrix = self.sub_matrix(row, column);
        return sub_matrix.determinant();
    }
    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn is_invertible(&self) -> bool {
        let determinant = self.determinant();
        !math::equal(0., determinant)
    }
    pub fn invert(&self) -> Option<Matrix> {
        assert_eq!(self.rows, self.cols);
        if !self.is_invertible() {
            return None;
        }

        let determinant = self.determinant();
        let mut inverse = Matrix::new_empty_matrix(self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = self.cofactor(row, col);
                inverse[col][row] = c / determinant;
            }
        }
        Some(inverse)
    }
}

#[test]
fn construct_and_inspect_4x4_matrix() {
    let m: Matrix = Matrix::new(&[
        &[1.0, 2.0, 3.0, 4.0],
        &[5.5, 6.5, 7.5, 8.5],
        &[9.0, 10.0, 11.0, 12.0],
        &[13.5, 14.5, 15.5, 16.5],
    ]);
    assert!(math::equal(m[0][0], 1.0));
    assert!(math::equal(m[0][3], 4.0));
    assert!(math::equal(m[1][0], 5.5));
    assert!(math::equal(m[1][2], 7.5));

    assert!(math::equal(m[2][2], 11.0));

    assert!(math::equal(m[3][0], 13.5));
    assert!(math::equal(m[3][2], 15.5));
}

#[test]
fn construct_and_inspect_2x2_matrix() {
    let m = Matrix::new(&[&[-3., 5.], &[1., -2.]]);

    assert!(math::equal(m[0][0], -3.));
    assert!(math::equal(m[0][1], 5.));
    assert!(math::equal(m[1][0], 1.));
    assert!(math::equal(m[1][1], -2.));
}

#[test]
fn construct_and_inspect_3x3_matrix() {
    let m = Matrix::new(&[&[-3., 5., 0.], &[1., -2., -7.], &[0., 1., 1.]]);

    assert!(math::equal(m[0][0], -3.));
    assert!(math::equal(m[1][1], -2.));
    assert!(math::equal(m[2][2], 1.));
}

#[test]
fn matrix_equality_identical_matrices() {
    let a = Matrix::new(&[
        &[1., 2., 3., 4.],
        &[5., 6., 7., 8.],
        &[9., 8., 7., 6.],
        &[5., 4., 3., 2.],
    ]);

    let b = Matrix::new(&[
        &[1., 2., 3., 4.],
        &[5., 6., 7., 8.],
        &[9., 8., 7., 6.],
        &[5., 4., 3., 2.],
    ]);
    assert_eq!(a, b);
}

#[test]
fn matrix_equality_different_matrices() {
    let a = Matrix::new(&[
        &[1., 2., 3., 4.],
        &[5., 6., 7., 8.],
        &[9., 8., 7., 6.],
        &[5., 4., 3., 2.],
    ]);

    let b = Matrix::new(&[
        &[2., 3., 4., 5.],
        &[6., 7., 8., 9.],
        &[8., 7., 6., 5.],
        &[4., 3., 2., 1.],
    ]);
    assert_ne!(a, b);
}

#[test]
fn matrix_multiplication() {
    let a = Matrix::new(&[
        &[1., 2., 3., 4.],
        &[5., 6., 7., 8.],
        &[9., 8., 7., 6.],
        &[5., 4., 3., 2.],
    ]);

    let b = Matrix::new(&[
        &[-2., 1., 2., 3.],
        &[3., 2., 1., -1.],
        &[4., 3., 6., 5.],
        &[1., 2., 7., 8.],
    ]);
    let c = &a * &b;
    let answer = Matrix::new(&[
        &[20., 22., 50., 48.],
        &[44., 54., 114., 108.],
        &[40., 58., 110., 102.],
        &[16., 26., 46., 42.],
    ]);
    assert_eq!(c, answer);
}

#[test]
fn matrix_multiplication_by_tuple() {
    let a = Matrix::new(&[
        &[1., 2., 3., 4.],
        &[2., 4., 4., 2.],
        &[8., 6., 4., 1.],
        &[0., 0., 0., 1.],
    ]);

    let b = math::Tuple::new(1., 2., 3., 1.);
    let c = a * b;
    let answer = math::Tuple::new(18., 24., 33., 1.);
    assert_eq!(c, answer);
}

#[test]
fn multiply_matrix_with_identity_matrix() {
    let a = Matrix::new(&[
        &[0., 1., 2., 4.],
        &[1., 2., 4., 8.],
        &[2., 4., 8., 16.],
        &[4., 8., 16., 32.],
    ]);
    let b = Matrix::new(&[
        &[1., 0., 0., 0.],
        &[0., 1., 0., 0.],
        &[0., 0., 1., 0.],
        &[0., 0., 0., 1.],
    ]);
    let answer = &a * &b;
    assert_eq!(a, answer);
}
#[test]
fn transpose_matrix() {
    let a = Matrix::new(&[
        &[0., 9., 3., 0.],
        &[9., 8., 0., 8.],
        &[1., 8., 5., 3.],
        &[0., 0., 5., 8.],
    ]);

    let a_transposed = a.transpose();

    let transposed_correctly = Matrix::new(&[
        &[0., 9., 1., 0.],
        &[9., 8., 8., 0.],
        &[3., 0., 5., 5.],
        &[0., 8., 3., 8.],
    ]);
    assert_eq!(a_transposed, transposed_correctly)
}

#[test]
fn transpose_identity_matrix() {
    let identity = Matrix::new_identity_matrix(4);
    let transposed_identity = identity.transpose();

    assert_eq!(identity, transposed_identity);
}

#[test]
fn calculate_determinant_2x2() {
    let m = Matrix::new(&[&[1., 5.], &[-3., 2.]]);
    let determinant = m.determinant();
    assert!(math::equal(determinant, 17.));
}

#[test]
fn submatrix_of_3x3() {
    let a = Matrix::new(&[&[1., 5., 0.], &[-3., 2., 7.], &[0., 6., -3.]]);
    let submatrix = a.sub_matrix(0, 2);
    let correct_submatrix = Matrix::new(&[&[-3., 2.], &[0., 6.]]);
    assert_eq!(submatrix, correct_submatrix);
}

#[test]
fn submatrix_of_4x4() {
    let a = Matrix::new(&[
        &[-6., 1., 1., 6.],
        &[-8., 5., 8., 6.],
        &[-1., 0., 8., 2.],
        &[-7., 1., -1., 1.],
    ]);
    let submatrix = a.sub_matrix(2, 1);
    let correct_submatrix = Matrix::new(&[&[-6., 1., 6.], &[-8., 8., 6.], &[-7., -1., 1.]]);

    assert_eq!(submatrix, correct_submatrix);
}

#[test]
fn calculate_minor_3x3() {
    let a = Matrix::new(&[&[3., 5., 0.], &[2., -1., -7.], &[6., -1., 5.]]);
    let sub_matrix = a.sub_matrix(1, 0);

    let determinant = sub_matrix.determinant();
    assert!(math::equal(determinant, 25.));

    let minor_1_0 = a.minor(1, 0);
    assert!(math::equal(minor_1_0, 25.));
}

#[test]
fn calculate_cofactor_3x3() {
    let a = Matrix::new(&[&[3., 5., 0.], &[2., -1., -7.], &[6., -1., 5.]]);

    let minor = a.minor(0, 0);
    assert!(math::equal(minor, -12.));

    let cofactor = a.cofactor(0, 0);
    assert!(math::equal(cofactor, -12.));

    let minor_1_0 = a.minor(1, 0);
    assert!(math::equal(minor_1_0, 25.));

    let cofactor_1_0 = a.cofactor(1, 0);
    assert!(math::equal(cofactor_1_0, -25.));
}
#[test]
fn calculate_determinant_3x3() {
    let a = Matrix::new(&[&[1., 2., 6.], &[-5., 8., -4.], &[2., 6., 4.]]);
    let cofactor_0_0 = a.cofactor(0, 0);
    assert!(math::equal(cofactor_0_0, 56.));

    let cofactor_0_1 = a.cofactor(0, 1);
    assert!(math::equal(cofactor_0_1, 12.));

    let cofactor_0_2 = a.cofactor(0, 2);
    assert!(math::equal(cofactor_0_2, -46.));

    let determinant = a.determinant();
    assert!(math::equal(determinant, -196.));
}

#[test]
fn calculate_determinant_4x4() {
    let a = Matrix::new(&[
        &[-2., -8., 3., 5.],
        &[-3., 1., 7., 3.],
        &[1., 2., -9., 6.],
        &[-6., 7., 7., -9.],
    ]);
    let cofactor_0_0 = a.cofactor(0, 0);
    assert!(math::equal(cofactor_0_0, 690.));

    let cofactor_0_1 = a.cofactor(0, 1);
    assert!(math::equal(cofactor_0_1, 447.));

    let cofactor_0_2 = a.cofactor(0, 2);
    assert!(math::equal(cofactor_0_2, 210.));

    let cofactor_0_3 = a.cofactor(0, 3);
    assert!(math::equal(cofactor_0_3, 51.));
    let determinant = a.determinant();
    assert!(math::equal(determinant, -4071.));
}

#[test]
fn invertible_matrix_is_invertible() {
    let a = Matrix::new(&[
        &[6., 4., 4., 4.],
        &[5., 5., 7., 6.],
        &[4., -9., 3., -7.],
        &[9., 1., 7., -6.],
    ]);

    let determinant = a.determinant();
    assert!(math::equal(determinant, -2120.));

    assert!(a.is_invertible());
}

#[test]
fn non_invertible_matrix_is_not_invertible() {
    let a = Matrix::new(&[
        &[-4., 2., -2., -3.],
        &[9., 6., 2., 6.],
        &[0., -5., 1., -5.],
        &[0., 0., 0., 0.],
    ]);

    let determinant = a.determinant();
    assert!(math::equal(determinant, 0.));

    assert!(!a.is_invertible());
}

#[test]
fn calculate_inverse_of_matrix() {
    let a = Matrix::new(&[
        &[-5., 2., 6., -8.],
        &[1., -5., 1., 8.],
        &[7., 7., -6., -7.],
        &[1., -3., 7., 4.],
    ]);
    let b = match a.invert() {
        Some(i) => i,
        None => panic!(),
    };

    let determinant = a.determinant();
    assert!(math::equal(determinant, 532.));

    let cofactor_2_3 = a.cofactor(2, 3);
    assert!(math::equal(cofactor_2_3, -160.));
    assert!(math::equal(b[3][2], -160. / 532.));

    let cofactor_3_2 = a.cofactor(3, 2);
    assert!(math::equal(cofactor_3_2, 105.));
    assert!(math::equal(b[2][3], 105. / 532.));

    let correctly_inverted_a = Matrix::new(&[
        &[0.21805, 0.45113, 0.24060, -0.04511],
        &[-0.80827, -1.45677, -0.44361, 0.52068],
        &[-0.07895, -0.22368, -0.05263, 0.19737],
        &[-0.52256, -0.81391, -0.30075, 0.30639],
    ]);
    assert_eq!(b, correctly_inverted_a);
}

#[test]
fn calculate_inverse_of_another_matrix() {
    let a = Matrix::new(&[
        &[8., -5., 9., 2.],
        &[7., 5., 6., 1.],
        &[-6., 0., 9., 6.],
        &[-3., 0., -9., -4.],
    ]);
    let b = match a.invert() {
        Some(i) => i,
        None => panic!(),
    };

    let correctly_inverted_a = Matrix::new(&[
        &[-0.15385, -0.15385, -0.28205, -0.53846],
        &[-0.07692, 0.12308, 0.02564, 0.03077],
        &[0.35897, 0.35897, 0.43590, 0.92308],
        &[-0.69231, -0.69231, -0.76923, -1.92308],
    ]);
    assert_eq!(b, correctly_inverted_a);
}

#[test]
fn calculate_inverse_of_a_third_matrix() {
    let a = Matrix::new(&[
        &[9., 3., 0., 9.],
        &[-5., -2., -6., -3.],
        &[-4., 9., 6., 4.],
        &[-7., 6., 6., 2.],
    ]);
    let b = match a.invert() {
        Some(i) => i,
        None => panic!(),
    };

    let correctly_inverted_a = Matrix::new(&[
        &[-0.04074, -0.07778, 0.14444, -0.22222],
        &[-0.07778, 0.03333, 0.36667, -0.33333],
        &[-0.02901, -0.14630, -0.10926, 0.12963],
        &[0.17778, 0.06667, -0.26667, 0.33333],
    ]);
    assert_eq!(b, correctly_inverted_a);
}

#[test]
fn multiply_by_inverse() {
    let a = Matrix::new(&[
        &[3., -9., 7., 3.],
        &[3., -8., 2., -9.],
        &[-4., 4., 4., 1.],
        &[-6., 5., -1., 1.],
    ]);

    let b = Matrix::new(&[
        &[8., 2., 2., 2.],
        &[3., -1., 7., 0.],
        &[7., 0., 5., 4.],
        &[6., -2., 0., 5.],
    ]);
    let c = &a * &b;
    let inverse_b = match b.invert() {
        Some(i) => i,
        None => panic!(),
    };
    let a_2 = &c * &inverse_b;
    assert_eq!(a_2, a);
}