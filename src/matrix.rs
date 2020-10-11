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
    return math::Tuple::tuple(res[0], res[1], res[2], res[3]);
  }
}
impl Matrix {
  pub fn new(rows: usize, cols: usize, data: &[&[f64]]) -> Matrix {
    assert_eq!(rows, data.len());
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
}

#[test]
fn construct_and_inspect_4x4_matrix() {
  let m: Matrix = Matrix::new(
    4,
    4,
    &[
      &[1.0, 2.0, 3.0, 4.0],
      &[5.5, 6.5, 7.5, 8.5],
      &[9.0, 10.0, 11.0, 12.0],
      &[13.5, 14.5, 15.5, 16.5],
    ],
  );
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
  let m = Matrix::new(2, 2, &[&[-3., 5.], &[1., -2.]]);

  assert!(math::equal(m[0][0], -3.));
  assert!(math::equal(m[0][1], 5.));
  assert!(math::equal(m[1][0], 1.));
  assert!(math::equal(m[1][1], -2.));
}

#[test]
fn construct_and_inspect_3x3_matrix() {
  let m = Matrix::new(3, 3, &[&[-3., 5., 0.], &[1., -2., -7.], &[0., 1., 1.]]);

  assert!(math::equal(m[0][0], -3.));
  assert!(math::equal(m[1][1], -2.));
  assert!(math::equal(m[2][2], 1.));
}

#[test]
fn matrix_equality_identical_matrices() {
  let a = Matrix::new(
    4,
    4,
    &[
      &[1., 2., 3., 4.],
      &[5., 6., 7., 8.],
      &[9., 8., 7., 6.],
      &[5., 4., 3., 2.],
    ],
  );

  let b = Matrix::new(
    4,
    4,
    &[
      &[1., 2., 3., 4.],
      &[5., 6., 7., 8.],
      &[9., 8., 7., 6.],
      &[5., 4., 3., 2.],
    ],
  );
  assert_eq!(a, b);
}

#[test]
fn matrix_equality_different_matrices() {
  let a = Matrix::new(
    4,
    4,
    &[
      &[1., 2., 3., 4.],
      &[5., 6., 7., 8.],
      &[9., 8., 7., 6.],
      &[5., 4., 3., 2.],
    ],
  );

  let b = Matrix::new(
    4,
    4,
    &[
      &[2., 3., 4., 5.],
      &[6., 7., 8., 9.],
      &[8., 7., 6., 5.],
      &[4., 3., 2., 1.],
    ],
  );
  assert_ne!(a, b);
}

#[test]
fn matrix_multiplication() {
  let a = Matrix::new(
    4,
    4,
    &[
      &[1., 2., 3., 4.],
      &[5., 6., 7., 8.],
      &[9., 8., 7., 6.],
      &[5., 4., 3., 2.],
    ],
  );

  let b = Matrix::new(
    4,
    4,
    &[
      &[-2., 1., 2., 3.],
      &[3., 2., 1., -1.],
      &[4., 3., 6., 5.],
      &[1., 2., 7., 8.],
    ],
  );
  let c = &a * &b;
  let answer = Matrix::new(
    4,
    4,
    &[
      &[20., 22., 50., 48.],
      &[44., 54., 114., 108.],
      &[40., 58., 110., 102.],
      &[16., 26., 46., 42.],
    ],
  );
  assert_eq!(c, answer);
}

#[test]
fn matrix_multiplication_by_tuple() {
  let a = Matrix::new(
    4,
    4,
    &[
      &[1., 2., 3., 4.],
      &[2., 4., 4., 2.],
      &[8., 6., 4., 1.],
      &[0., 0., 0., 1.],
    ],
  );

  let b = math::Tuple::tuple(1., 2., 3., 1.);
  let c = a * b;
  let answer = math::Tuple::tuple(18., 24., 33., 1.);
  assert_eq!(c, answer);
}

#[test]
fn multiply_matrix_with_identity_matrix() {
  let a = Matrix::new(
    4,
    4,
    &[
      &[0., 1., 2., 4.],
      &[1., 2., 4., 8.],
      &[2., 4., 8., 16.],
      &[4., 8., 16., 32.],
    ],
  );
  let b = Matrix::new(
    4,
    4,
    &[
      &[1., 0., 0., 0.],
      &[0., 1., 0., 0.],
      &[0., 0., 1., 0.],
      &[0., 0., 0., 1.],
    ],
  );
  let answer = &a * &b;
  assert_eq!(a, answer);
}
