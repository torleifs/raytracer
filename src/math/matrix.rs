use std::cmp;
use std::ops;

use crate::math;
use crate::util;

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
            if !util::equal(*num, other.data[i]) {
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
impl<'a> ops::Mul<&'a math::Tuple> for &'a Matrix {
    type Output = math::Tuple;
    fn mul(self, tuple: &'a math::Tuple) -> math::Tuple {
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

    pub fn translation(x: f64, y: f64, z: f64) -> math::Matrix {
        math::Matrix::new(&[
          &[1., 0., 0., x],
          &[0., 1., 0., y],
          &[0., 0., 1., z],
          &[0., 0., 0., 1.],
        ])
      }
      
    pub fn scale(x: f64, y: f64, z: f64) -> math::Matrix {
        math::Matrix::new(&[
            &[x, 0., 0., 0.],
            &[0., y, 0., 0.],
            &[0., 0., z, 0.],
            &[0., 0., 0., 1.],
        ])
    }
    pub fn rotation_x(angle: f64) -> Matrix {
        math::Matrix::new(&[
            &[1., 0., 0., 0.],
            &[0., angle.cos(), -angle.sin(), 0.],
            &[0., angle.sin(), angle.cos(), 0.],
            &[0., 0., 0., 1.],
        ])
    }
    pub fn rotation_y(angle: f64) -> Matrix {
        math::Matrix::new(&[
            &[angle.cos(), 0., angle.sin(), 0.],
            &[0., 1., 0., 0.],
            &[-angle.sin(), 0., angle.cos(), 0.],
            &[0., 0., 0., 1.],
        ])
    }
    pub fn rotation_z(angle: f64) -> Matrix {
        math::Matrix::new(&[
            &[angle.cos(), -angle.sin(), 0., 0.],
            &[angle.sin(), angle.cos(), 0., 0.],
            &[0., 0., 1., 0.],
            &[0., 0., 0., 1.],
        ])
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
        !util::equal(0., determinant)
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
