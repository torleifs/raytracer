use std::f64::consts;

use super::Tuple;
use super::Matrix;

use crate::util;
  #[test]
  fn a_tuple_with_w_1_is_a_point() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert!(a.is_point())
  }

  #[test]
  fn a_tuple_with_w_0_is_a_vector() {
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);
    assert!(a.is_vector())
  }

  #[test]
  fn create_point_w_is_1() {
    let a = Tuple::point(4.0, -4.0, 3.0);
    assert_eq!(a.w, 1.0);
  }
  #[test]
  fn create_vector_w_is_0() {
    let a = Tuple::vector(4.0, -4.0, 3.0);
    assert_eq!(a.w, 0.0);
  }
  #[test]
  fn adding_two_tuples() {
    let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
    let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);
    let answer = a + &b;
    let correct_answer = Tuple {
      x: 1.0,
      y: 1.0,
      z: 6.0,
      w: 1.0,
    };
    assert!(answer.is_equal(&correct_answer));
  }

  #[test]
  #[should_panic]
  fn adding_two_points_causes_panic() {
    let a = Tuple::point(3.0, -2.0, 5.0);
    let b = Tuple::point(-2.0, 3.0, 1.0);
    let _ = a + &b;
  }
  #[test]
  fn subtracting_two_points_results_in_vector() {
    let a = Tuple::point(3.0, 2.0, 1.0);
    let b = Tuple::point(5.0, 6.0, 7.0);
    let answer = a - &b;
    let correct_answer = Tuple::vector(-2.0, -4.0, -6.0);
    assert!(answer.is_equal(&correct_answer));
  }
  #[test]
  fn subtracting_vector_from_point_results_in_point() {
    let a = Tuple::point(3.0, 2.0, 1.0);
    let b = Tuple::vector(5.0, 6.0, 7.0);
    let answer = a - &b;
    let correct_answer = Tuple::point(-2.0, -4.0, -6.0);
    assert!(answer.is_equal(&correct_answer));
  }

  #[test]
  fn subtracting_vector_from_vector_results_in_vector() {
    let a = Tuple::vector(3.0, 2.0, 1.0);
    let b = Tuple::vector(5.0, 6.0, 7.0);
    let answer = a - &b;
    let correct_answer = Tuple::vector(-2.0, -4.0, -6.0);
    assert!(answer.is_equal(&correct_answer));
  }

  #[test]
  fn subtract_vector_from_zero_vector() {
    let zero = Tuple::vector(0.0, 0.0, 0.0);
    let b = Tuple::vector(1.0, -2.0, 3.0);
    let answer = zero - &b;
    let correct_answer = Tuple::vector(-1.0, 2.0, -3.0);
    assert!(answer.is_equal(&correct_answer));
  }
  #[test]
  fn negate_tuple() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let negated_tuple = -a;
    let correct_answer = Tuple::new(-1.0, 2.0, -3.0, 4.0);
    assert!(negated_tuple.is_equal(&correct_answer))
  }
  #[test]
  fn multiply_tuple_by_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let answer = a * 3.5;
    let correct_answer = Tuple::new(3.5, -7.0, 10.5, -14.0);
    assert!(answer.is_equal(&correct_answer));
  }

  #[test]
  fn multiply_tuple_by_fraction() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let answer = a * 0.5;
    let correct_answer = Tuple::new(0.5, -1.0, 1.5, -2.0);
    assert!(answer.is_equal(&correct_answer));
  }
  #[test]
  fn divide_tuple_by_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let answer = a / 2.0;
    let correct_answer = Tuple::new(0.5, -1.0, 1.5, -2.0);
    assert!(answer.is_equal(&correct_answer));
  }
  #[test]
  fn magnitude_of_vectors() {
    let a = Tuple::vector(1.0, 0.0, 0.0);
    assert!(util::equal(a.magnitude(), 1.0));
    let a = Tuple::vector(0.0, 1.0, 0.0);
    assert!(util::equal(a.magnitude(), 1.0));
    let a = Tuple::vector(0.0, 0.0, 1.0);
    assert!(util::equal(a.magnitude(), 1.0));
    let a = Tuple::vector(1.0, 2.0, 3.0);
    assert!(util::equal(a.magnitude(), (14.0 as f64).sqrt()));
    let a = Tuple::vector(-1.0, -2.0, -3.0);
    assert!(util::equal(a.magnitude(), (14 as f64).sqrt()));
  }
  #[test]
  fn normalizing_vectors() {
    let a = Tuple::vector(4.0, 0.0, 0.0);
    let b = a.normalize();
    assert!(b.is_equal(&Tuple::vector(1.0, 0.0, 0.0)));
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = a.normalize();
    assert!(b.is_equal(&Tuple::vector(
      1.0 / (14.0 as f64).sqrt(),
      2.0 / (14.0 as f64).sqrt(),
      3.0 / (14.0 as f64).sqrt()
    )));
  }
  #[test]
  fn magnitude_of_normalized_vector() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = a.normalize();
    assert!(util::equal(b.magnitude(), 1.0));
  }
  #[test]
  fn dot_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert!(util::equal(Tuple::dot(&a, &b), 20.0));
  }

  #[test]
  fn cross_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    let answer = Tuple::cross(&a, &b);
    assert!(answer.is_equal(&Tuple::vector(-1.0, 2.0, -1.0)));
    let answer = Tuple::cross(&b, &a);
    assert!(answer.is_equal(&Tuple::vector(1.0, -2.0, 1.0)));
  }

  #[test]
  fn reflect_45dg_vector() {
    let v = Tuple::vector(1., -1., 0.);
    let n = Tuple::vector(0., 1., 0.);
    let r = Tuple::reflect(&v, &n);

    assert_eq!(r, Tuple::vector(1., 1., 0.))
  }

  #[test]
  fn reflect_slanted_surface() {
    let v = Tuple::vector(0., -1., 0.);
    let n = Tuple::vector((2.0 as f64).sqrt() / 2., (2.0 as f64).sqrt() / 2., 0.);

    let r = Tuple::reflect(&v, &n);

    assert_eq!(r, Tuple::vector(1., 0., 0.))
  }
///////////////////////////////////////
  #[test]
  fn construct_and_inspect_4x4_matrix() {
      let m: Matrix = Matrix::new(&[
          &[1.0, 2.0, 3.0, 4.0],
          &[5.5, 6.5, 7.5, 8.5],
          &[9.0, 10.0, 11.0, 12.0],
          &[13.5, 14.5, 15.5, 16.5],
      ]);
      assert!(util::equal(m[0][0], 1.0));
      assert!(util::equal(m[0][3], 4.0));
      assert!(util::equal(m[1][0], 5.5));
      assert!(util::equal(m[1][2], 7.5));
  
      assert!(util::equal(m[2][2], 11.0));
  
      assert!(util::equal(m[3][0], 13.5));
      assert!(util::equal(m[3][2], 15.5));
  }
  
  #[test]
  fn construct_and_inspect_2x2_matrix() {
      let m = Matrix::new(&[&[-3., 5.], &[1., -2.]]);
  
      assert!(util::equal(m[0][0], -3.));
      assert!(util::equal(m[0][1], 5.));
      assert!(util::equal(m[1][0], 1.));
      assert!(util::equal(m[1][1], -2.));
  }
  
  #[test]
  fn construct_and_inspect_3x3_matrix() {
      let m = Matrix::new(&[&[-3., 5., 0.], &[1., -2., -7.], &[0., 1., 1.]]);
  
      assert!(util::equal(m[0][0], -3.));
      assert!(util::equal(m[1][1], -2.));
      assert!(util::equal(m[2][2], 1.));
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
  
      let b = Tuple::new(1., 2., 3., 1.);
      let c = &a * &b;
      let answer = Tuple::new(18., 24., 33., 1.);
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
      assert!(util::equal(determinant, 17.));
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
      assert!(util::equal(determinant, 25.));
  
      let minor_1_0 = a.minor(1, 0);
      assert!(util::equal(minor_1_0, 25.));
  }
  
  #[test]
  fn calculate_cofactor_3x3() {
      let a = Matrix::new(&[&[3., 5., 0.], &[2., -1., -7.], &[6., -1., 5.]]);
  
      let minor = a.minor(0, 0);
      assert!(util::equal(minor, -12.));
  
      let cofactor = a.cofactor(0, 0);
      assert!(util::equal(cofactor, -12.));
  
      let minor_1_0 = a.minor(1, 0);
      assert!(util::equal(minor_1_0, 25.));
  
      let cofactor_1_0 = a.cofactor(1, 0);
      assert!(util::equal(cofactor_1_0, -25.));
  }
  #[test]
  fn calculate_determinant_3x3() {
      let a = Matrix::new(&[&[1., 2., 6.], &[-5., 8., -4.], &[2., 6., 4.]]);
      let cofactor_0_0 = a.cofactor(0, 0);
      assert!(util::equal(cofactor_0_0, 56.));
  
      let cofactor_0_1 = a.cofactor(0, 1);
      assert!(util::equal(cofactor_0_1, 12.));
  
      let cofactor_0_2 = a.cofactor(0, 2);
      assert!(util::equal(cofactor_0_2, -46.));
  
      let determinant = a.determinant();
      assert!(util::equal(determinant, -196.));
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
      assert!(util::equal(cofactor_0_0, 690.));
  
      let cofactor_0_1 = a.cofactor(0, 1);
      assert!(util::equal(cofactor_0_1, 447.));
  
      let cofactor_0_2 = a.cofactor(0, 2);
      assert!(util::equal(cofactor_0_2, 210.));
  
      let cofactor_0_3 = a.cofactor(0, 3);
      assert!(util::equal(cofactor_0_3, 51.));
      let determinant = a.determinant();
      assert!(util::equal(determinant, -4071.));
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
      assert!(util::equal(determinant, -2120.));
  
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
      assert!(util::equal(determinant, 0.));
  
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
      assert!(util::equal(determinant, 532.));
  
      let cofactor_2_3 = a.cofactor(2, 3);
      assert!(util::equal(cofactor_2_3, -160.));
      assert!(util::equal(b[3][2], -160. / 532.));
  
      let cofactor_3_2 = a.cofactor(3, 2);
      assert!(util::equal(cofactor_3_2, 105.));
      assert!(util::equal(b[2][3], 105. / 532.));
  
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
  

#[test]
pub fn multiply_by_translation() {
  let transform = Matrix::translation(5., -3., 2.);
  let p =  Tuple::point(-3., 4., 5.);

  assert_eq!(&transform * &p, Tuple::point(2.,1.,7.));
}

#[test]
pub fn multiply_by_inverse_of_translation() {
  let transform = Matrix::translation(5., -3., 2.);
  let inv = match transform.invert() {
    Some(i) => i,
    None => panic!()
  };

  let p = Tuple::point(-3., 4., 5.);

  assert_eq!(&inv * &p, Tuple::point(-8.,7.,3.));
}

#[test]
pub fn translation_does_not_affect_vectors() {
  let transform = Matrix::translation(5., -3., 2.);
  let v = Tuple::vector(-3., 4., 5.);
  assert_eq!(&transform * &v, v);
}

#[test]
pub fn scaling_matrix_applied_to_point() {
  let transform = Matrix::scale(2., 3., 4.);
  let p=  Tuple::point(-4., 6., 8.);

  assert_eq!(&transform * &p, Tuple::point(-8.,18.,32.));
}

#[test]
pub fn scaling_matrix_applied_to_vector() {
  let transform = Matrix::scale(2., 3., 4.);
  let v=  Tuple::vector(-4., 6., 8.);

  assert_eq!(&transform * &v, Tuple::vector(-8.,18.,32.));
}

#[test]
pub fn multiply_by_inverse_of_scaling_matrix() {
  let transform = Matrix::scale(2., 3., 4.);
  let inv = match transform.invert() {
    Some(i) => i,
    None => panic!()
  };

  let v=  Tuple::vector(-4., 6., 8.);

  assert_eq!(&inv * &v, Tuple::vector(-2.,2.,2.));
}


#[test]
pub fn reflection_is_scaling_by_negative_value() {
  let transform = Matrix::scale(-1., 1., 1.);
  let p=  Tuple::point(2., 3., 4.);

  assert_eq!(&transform * &p, Tuple::point(-2.,3.,4.));
}
#[test]
pub fn rotate_point_around_x_axis() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_x(consts::PI / 4.);
    let full_quarter = Matrix::rotation_x(consts::PI / 2.);

    assert_eq!(&half_quarter * &p, Tuple::point(0., (2.0 as f64).sqrt()/2., (2.0 as f64).sqrt()/2.));
    assert_eq!(&full_quarter * &p, Tuple::point(0., 0., 1.));
}

#[test]
pub fn inverse_rotation_rotates_other_direction() {
    let p = Tuple::point(0., 1., 0.);
    let half_quarter = Matrix::rotation_x(consts::PI / 4.);
    let half_quarter_inv = match half_quarter.invert() {
        Some(i) => i,
        None => panic!()
      };

    assert_eq!(&half_quarter_inv * &p, Tuple::point(0., (2.0 as f64).sqrt()/2., -(2.0 as f64).sqrt()/2.))
}

#[test]
pub fn rotate_point_around_y_axis() {
    let p = Tuple::point(0.0, 0.0, 1.0);
    let half_quarter = Matrix::rotation_y(consts::PI / 4.);
    let full_quarter = Matrix::rotation_y(consts::PI / 2.);

    assert_eq!(&half_quarter * &p, Tuple::point((2.0 as f64).sqrt()/2., 0., (2.0 as f64).sqrt()/2.));
    assert_eq!(&full_quarter * &p, Tuple::point(1., 0., 0.));
}

#[test]
pub fn rotate_point_around_z_axis() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_z(consts::PI / 4.);
    let full_quarter = Matrix::rotation_z(consts::PI / 2.);

    assert_eq!(&half_quarter * &p, Tuple::point(-(2.0 as f64).sqrt()/2., (2.0 as f64).sqrt()/2., 0.));
    assert_eq!(&full_quarter * &p, Tuple::point(-1., 0., 0.));
}

#[test]
pub fn shearing_transform_moves_x_in_proportion_to_y() {
  let transform = Matrix::shear(1., 0., 0., 0., 0., 0.);
  let p = Tuple::point(2., 3., 4.);
  assert_eq!(&transform * &p, Tuple::point(5., 3., 4.));
}

#[test]
pub fn shearing_transform_moves_x_in_proportion_to_z() {
  let transform = Matrix::shear(0., 1., 0., 0., 0., 0.);
  let p = Tuple::point(2., 3., 4.);
  assert_eq!(&transform * &p, Tuple::point(6., 3., 4.));
}

#[test]
pub fn shearing_transform_moves_y_in_proportion_to_x() {
  let transform = Matrix::shear(0., 0., 1., 0., 0., 0.);
  let p = Tuple::point(2., 3., 4.);
  assert_eq!(&transform * &p, Tuple::point(2., 5., 4.));
}

#[test]
pub fn shearing_transform_moves_y_in_proportion_to_z() {
  let transform = Matrix::shear(0., 0., 0., 1., 0., 0.);
  let p = Tuple::point(2., 3., 4.);
  assert_eq!(&transform * &p, Tuple::point(2., 7., 4.));
}

#[test]
pub fn shearing_transform_moves_z_in_proportion_to_x() {
  let transform = Matrix::shear(0., 0., 0., 0., 1., 0.);
  let p = Tuple::point(2., 3., 4.);
  assert_eq!(&transform * &p, Tuple::point(2., 3., 6.));
}


#[test]
pub fn shearing_transform_moves_z_in_proportion_to_y() {
  let transform = Matrix::shear(0., 0., 0., 0., 0., 1.);
  let p = Tuple::point(2., 3., 4.);
  assert_eq!(&transform * &p, Tuple::point(2., 3., 7.));
}

#[test]
pub fn individual_transforms_are_applied_in_sequence() {
  let p= Tuple::point(1., 0., 1.);
  let a= Matrix::rotation_x(consts::PI/2.);
  let b= Matrix::scale(5., 5., 5.);
  let c= Matrix::translation(10., 5., 7.);

  let p2 = &a * &p;
  assert_eq!(p2, Tuple::point(1., -1., 0.));

  let p3 = &b * &p2;
  assert_eq!(p3, Tuple::point(5., -5., 0.));

  let p4 = &c * &p3;
  assert_eq!(p4, Tuple::point(15., 0., 7.));
}

#[test]
pub fn chained_transforms_must_be_applied_reverse() {
  let p = Tuple::point(1., 0., 1.);
  let a= Matrix::rotation_x(consts::PI/2.);
  let b= Matrix::scale(5., 5., 5.);
  let c= Matrix::translation(10., 5., 7.);
  let t = c * b * a;
  assert_eq!(&t * &p, Tuple::point(15., 0., 7.));
}