use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A 3x3 matrix for linear algebra operations.
///
/// Used internally for RGB-to-XYZ conversions, chromatic adaptation transforms,
/// and other 3-component color space transformations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3 {
  data: [[f64; 3]; 3],
}

impl Matrix3 {
  /// Creates a new matrix from row-major data.
  pub const fn new(data: [[f64; 3]; 3]) -> Self {
    Self {
      data,
    }
  }

  /// Returns the matrix data as a 3x3 array in row-major order.
  pub const fn data(&self) -> [[f64; 3]; 3] {
    self.data
  }

  /// Computes the determinant of the matrix.
  pub const fn determinant(&self) -> f64 {
    let [[a, b, c], [d, e, f], [g, h, i]] = self.data;
    a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
  }

  /// Computes the inverse of the matrix.
  pub const fn inverse(&self) -> Self {
    let [[a, b, c], [d, e, f], [g, h, i]] = self.data;
    let inv_det = 1.0 / self.determinant();

    Self::new([
      [
        (e * i - f * h) * inv_det,
        (c * h - b * i) * inv_det,
        (b * f - c * e) * inv_det,
      ],
      [
        (f * g - d * i) * inv_det,
        (a * i - c * g) * inv_det,
        (c * d - a * f) * inv_det,
      ],
      [
        (d * h - e * g) * inv_det,
        (b * g - a * h) * inv_det,
        (a * e - b * d) * inv_det,
      ],
    ])
  }
}

impl Add for Matrix3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self::new([
      [
        self.data[0][0] + rhs.data[0][0],
        self.data[0][1] + rhs.data[0][1],
        self.data[0][2] + rhs.data[0][2],
      ],
      [
        self.data[1][0] + rhs.data[1][0],
        self.data[1][1] + rhs.data[1][1],
        self.data[1][2] + rhs.data[1][2],
      ],
      [
        self.data[2][0] + rhs.data[2][0],
        self.data[2][1] + rhs.data[2][1],
        self.data[2][2] + rhs.data[2][2],
      ],
    ])
  }
}

impl AddAssign for Matrix3 {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Display for Matrix3 {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let [[a, b, c], [d, e, f_], [g, h, i]] = self.data;
    let p = f.precision().unwrap_or(4);

    write!(
      f,
      "[\n  [{a:.p$}, {b:.p$}, {c:.p$}],\n  [{d:.p$}, {e:.p$}, {f_:.p$}],\n  [{g:.p$}, {h:.p$}, {i:.p$}]\n]"
    )
  }
}

impl Div<f64> for Matrix3 {
  type Output = Self;

  fn div(self, rhs: f64) -> Self {
    self * (1.0 / rhs)
  }
}

impl DivAssign<f64> for Matrix3 {
  fn div_assign(&mut self, rhs: f64) {
    *self = *self / rhs;
  }
}

impl From<[[f64; 3]; 3]> for Matrix3 {
  fn from(data: [[f64; 3]; 3]) -> Self {
    Self::new(data)
  }
}

impl Mul for Matrix3 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    let a = self.data;
    let b = rhs.data;

    Self::new([
      [
        a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0],
        a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1],
        a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2],
      ],
      [
        a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0],
        a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1],
        a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2],
      ],
      [
        a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0],
        a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1],
        a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2],
      ],
    ])
  }
}

impl Mul<f64> for Matrix3 {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self {
    Self::new([
      [self.data[0][0] * rhs, self.data[0][1] * rhs, self.data[0][2] * rhs],
      [self.data[1][0] * rhs, self.data[1][1] * rhs, self.data[1][2] * rhs],
      [self.data[2][0] * rhs, self.data[2][1] * rhs, self.data[2][2] * rhs],
    ])
  }
}

impl Mul<Matrix3> for f64 {
  type Output = Matrix3;

  fn mul(self, rhs: Matrix3) -> Matrix3 {
    rhs * self
  }
}

impl Mul<[f64; 3]> for Matrix3 {
  type Output = [f64; 3];

  fn mul(self, rhs: [f64; 3]) -> [f64; 3] {
    let m = self.data;
    let [x, y, z] = rhs;

    [
      m[0][0] * x + m[0][1] * y + m[0][2] * z,
      m[1][0] * x + m[1][1] * y + m[1][2] * z,
      m[2][0] * x + m[2][1] * y + m[2][2] * z,
    ]
  }
}

impl Mul<Matrix3> for [f64; 3] {
  type Output = [f64; 3];

  fn mul(self, rhs: Matrix3) -> [f64; 3] {
    let m = rhs.data;
    let [x, y, z] = self;

    [
      x * m[0][0] + y * m[1][0] + z * m[2][0],
      x * m[0][1] + y * m[1][1] + z * m[2][1],
      x * m[0][2] + y * m[1][2] + z * m[2][2],
    ]
  }
}

impl MulAssign for Matrix3 {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl MulAssign<f64> for Matrix3 {
  fn mul_assign(&mut self, rhs: f64) {
    *self = *self * rhs;
  }
}

impl Neg for Matrix3 {
  type Output = Self;

  fn neg(self) -> Self {
    Self::new([
      [-self.data[0][0], -self.data[0][1], -self.data[0][2]],
      [-self.data[1][0], -self.data[1][1], -self.data[1][2]],
      [-self.data[2][0], -self.data[2][1], -self.data[2][2]],
    ])
  }
}

impl Sub for Matrix3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self::new([
      [
        self.data[0][0] - rhs.data[0][0],
        self.data[0][1] - rhs.data[0][1],
        self.data[0][2] - rhs.data[0][2],
      ],
      [
        self.data[1][0] - rhs.data[1][0],
        self.data[1][1] - rhs.data[1][1],
        self.data[1][2] - rhs.data[1][2],
      ],
      [
        self.data[2][0] - rhs.data[2][0],
        self.data[2][1] - rhs.data[2][1],
        self.data[2][2] - rhs.data[2][2],
      ],
    ])
  }
}

impl SubAssign for Matrix3 {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod add {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_matrices() {
      let a = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let b = Matrix3::new([[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]]);
      let expected = Matrix3::new([[10.0, 10.0, 10.0], [10.0, 10.0, 10.0], [10.0, 10.0, 10.0]]);

      assert_eq!(a + b, expected);
    }
  }

  mod add_assign {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_and_assigns() {
      let mut a = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let b = Matrix3::new([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
      a += b;
      let expected = Matrix3::new([[2.0, 3.0, 4.0], [5.0, 6.0, 7.0], [8.0, 9.0, 10.0]]);

      assert_eq!(a, expected);
    }
  }

  mod determinant {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_calculates_identity_determinant() {
      let identity = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

      assert_eq!(identity.determinant(), 1.0);
    }

    #[test]
    fn it_calculates_general_determinant() {
      let m = Matrix3::new([[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]]);

      assert_eq!(m.determinant(), 1.0);
    }

    #[test]
    fn it_returns_zero_for_singular_matrix() {
      let m = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

      assert_eq!(m.determinant(), 0.0);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_displays_with_default_precision() {
      let m = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let expected = "[\n  [1.0000, 2.0000, 3.0000],\n  [4.0000, 5.0000, 6.0000],\n  [7.0000, 8.0000, 9.0000]\n]";

      assert_eq!(format!("{}", m), expected);
    }

    #[test]
    fn it_displays_with_custom_precision() {
      let m = Matrix3::new([[1.23456, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
      let expected = "[\n  [1.23, 0.00, 0.00],\n  [0.00, 1.00, 0.00],\n  [0.00, 0.00, 1.00]\n]";

      assert_eq!(format!("{:.2}", m), expected);
    }
  }

  mod div {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_divides_by_scalar() {
      let m = Matrix3::new([[2.0, 4.0, 6.0], [8.0, 10.0, 12.0], [14.0, 16.0, 18.0]]);
      let expected = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

      assert_eq!(m / 2.0, expected);
    }
  }

  mod div_assign {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_divides_and_assigns() {
      let mut m = Matrix3::new([[2.0, 4.0, 6.0], [8.0, 10.0, 12.0], [14.0, 16.0, 18.0]]);
      m /= 2.0;
      let expected = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

      assert_eq!(m, expected);
    }
  }

  mod inverse {
    use super::*;

    #[test]
    fn it_inverts_identity() {
      let identity = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
      let inv = identity.inverse();

      assert_eq!(inv, identity);
    }

    #[test]
    fn it_satisfies_inverse_property() {
      let m = Matrix3::new([[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]]);
      let inv = m.inverse();
      let result = m * inv;
      let identity = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

      for i in 0..3 {
        for j in 0..3 {
          assert!((result.data()[i][j] - identity.data()[i][j]).abs() < 1e-10);
        }
      }
    }
  }

  mod mul {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_matrices() {
      let a = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let identity = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

      assert_eq!(a * identity, a);
    }

    #[test]
    fn it_multiplies_by_scalar() {
      let m = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let expected = Matrix3::new([[2.0, 4.0, 6.0], [8.0, 10.0, 12.0], [14.0, 16.0, 18.0]]);

      assert_eq!(m * 2.0, expected);
    }

    #[test]
    fn it_multiplies_scalar_by_matrix() {
      let m = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let expected = Matrix3::new([[2.0, 4.0, 6.0], [8.0, 10.0, 12.0], [14.0, 16.0, 18.0]]);

      assert_eq!(2.0 * m, expected);
    }

    #[test]
    fn it_multiplies_matrix_by_vector() {
      let m = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]);
      let v = [1.0, 1.0, 1.0];

      assert_eq!(m * v, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn it_multiplies_vector_by_matrix() {
      let m = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]);
      let v = [1.0, 1.0, 1.0];

      assert_eq!(v * m, [1.0, 2.0, 3.0]);
    }
  }

  mod mul_assign {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_and_assigns_matrix() {
      let mut a = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let identity = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
      let expected = a;
      a *= identity;

      assert_eq!(a, expected);
    }

    #[test]
    fn it_multiplies_and_assigns_scalar() {
      let mut m = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      m *= 2.0;
      let expected = Matrix3::new([[2.0, 4.0, 6.0], [8.0, 10.0, 12.0], [14.0, 16.0, 18.0]]);

      assert_eq!(m, expected);
    }
  }

  mod neg {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_negates_matrix() {
      let m = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let expected = Matrix3::new([[-1.0, -2.0, -3.0], [-4.0, -5.0, -6.0], [-7.0, -8.0, -9.0]]);

      assert_eq!(-m, expected);
    }
  }

  mod sub {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_matrices() {
      let a = Matrix3::new([[10.0, 10.0, 10.0], [10.0, 10.0, 10.0], [10.0, 10.0, 10.0]]);
      let b = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
      let expected = Matrix3::new([[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]]);

      assert_eq!(a - b, expected);
    }
  }

  mod sub_assign {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_and_assigns() {
      let mut a = Matrix3::new([[10.0, 10.0, 10.0], [10.0, 10.0, 10.0], [10.0, 10.0, 10.0]]);
      let b = Matrix3::new([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
      a -= b;
      let expected = Matrix3::new([[9.0, 9.0, 9.0], [9.0, 9.0, 9.0], [9.0, 9.0, 9.0]]);

      assert_eq!(a, expected);
    }
  }
}
