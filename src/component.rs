use std::{
  cmp::Ordering,
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, Debug)]
pub struct Component(pub(crate) f64);

impl Component {
  pub fn new(value: impl Into<f64>) -> Self {
    Self(value.into())
  }

  pub const fn new_const(value: f64) -> Self {
    Self(value)
  }

  pub fn clamp(&self, minimum: impl Into<Self>, maximum: impl Into<Self>) -> Self {
    Self(self.0.clamp(minimum.into().0, maximum.into().0))
  }
}

impl<T> Add<T> for Component
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self {
    Self(self.0 + rhs.into().0)
  }
}

impl<T> AddAssign<T> for Component
where
  T: Into<Component>,
{
  fn add_assign(&mut self, rhs: T) {
    self.0 += rhs.into().0
  }
}

impl<T> Div<T> for Component
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self {
    Self(self.0 / rhs.into().0)
  }
}

impl<T> DivAssign<T> for Component
where
  T: Into<Self>,
{
  fn div_assign(&mut self, rhs: T) {
    self.0 /= rhs.into().0
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{:.precision$}", self.0, precision = f.precision().unwrap_or(4))
  }
}

impl From<f32> for Component {
  fn from(value: f32) -> Self {
    Self(value as f64)
  }
}

impl From<f64> for Component {
  fn from(value: f64) -> Self {
    Self(value)
  }
}

impl From<i8> for Component {
  fn from(value: i8) -> Self {
    Self(value as f64)
  }
}

impl From<i16> for Component {
  fn from(value: i16) -> Self {
    Self(value as f64)
  }
}

impl From<i32> for Component {
  fn from(value: i32) -> Self {
    Self(value as f64)
  }
}

impl From<i64> for Component {
  fn from(value: i64) -> Self {
    Self(value as f64)
  }
}

impl From<i128> for Component {
  fn from(value: i128) -> Self {
    Self(value as f64)
  }
}

impl From<isize> for Component {
  fn from(value: isize) -> Self {
    Self(value as f64)
  }
}

impl From<u8> for Component {
  fn from(value: u8) -> Self {
    Self(value as f64)
  }
}

impl From<u16> for Component {
  fn from(value: u16) -> Self {
    Self(value as f64)
  }
}

impl From<u32> for Component {
  fn from(value: u32) -> Self {
    Self(value as f64)
  }
}

impl From<u64> for Component {
  fn from(value: u64) -> Self {
    Self(value as f64)
  }
}

impl From<u128> for Component {
  fn from(value: u128) -> Self {
    Self(value as f64)
  }
}

impl From<usize> for Component {
  fn from(value: usize) -> Self {
    Self(value as f64)
  }
}

impl<T> Mul<T> for Component
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self {
    Self(self.0 * rhs.into().0)
  }
}

impl<T> MulAssign<T> for Component
where
  T: Into<Self>,
{
  fn mul_assign(&mut self, rhs: T) {
    self.0 *= rhs.into().0;
  }
}

impl Neg for Component {
  type Output = Self;

  fn neg(self) -> Self {
    Self(-self.0)
  }
}

impl<T> PartialEq<T> for Component
where
  T: Into<Self> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    self.0 == (*other).into().0
  }
}

impl<T> PartialOrd<T> for Component
where
  T: Into<Self> + Copy,
{
  fn partial_cmp(&self, other: &T) -> Option<Ordering> {
    self.0.partial_cmp(&(*other).into().0)
  }
}

impl<T> Sub<T> for Component
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self {
    Self(self.0 - rhs.into().0)
  }
}

impl<T> SubAssign<T> for Component
where
  T: Into<Self>,
{
  fn sub_assign(&mut self, rhs: T) {
    self.0 -= rhs.into().0;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod add {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_f64() {
      let c = Component::new(1.0);
      let result = c + 0.5;

      assert_eq!(result, 1.5);
    }

    #[test]
    fn it_adds_integer() {
      let c = Component::new(1.0);
      let result = c + 2;

      assert_eq!(result, 3.0);
    }

    #[test]
    fn it_adds_component() {
      let a = Component::new(1.0);
      let b = Component::new(2.0);

      assert_eq!(a + b, 3.0);
    }
  }

  mod add_assign {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_and_assigns() {
      let mut c = Component::new(1.0);
      c += 0.5;

      assert_eq!(c, 1.5);
    }
  }

  mod clamp {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_clamps_to_minimum() {
      let c = Component::new(-0.5);

      assert_eq!(c.clamp(0.0, 1.0), 0.0);
    }

    #[test]
    fn it_clamps_to_maximum() {
      let c = Component::new(1.5);

      assert_eq!(c.clamp(0.0, 1.0), 1.0);
    }

    #[test]
    fn it_returns_value_when_in_range() {
      let c = Component::new(0.5);

      assert_eq!(c.clamp(0.0, 1.0), 0.5);
    }

    #[test]
    fn it_accepts_integer_bounds() {
      let c = Component::new(150.0);

      assert_eq!(c.clamp(0, 100), 100.0);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_displays_with_default_precision() {
      let c = Component::new(1.23456789);

      assert_eq!(format!("{}", c), "1.2346");
    }

    #[test]
    fn it_displays_with_custom_precision() {
      let c = Component::new(1.23456789);

      assert_eq!(format!("{:.2}", c), "1.23");
      assert_eq!(format!("{:.6}", c), "1.234568");
    }
  }

  mod div {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_divides_by_f64() {
      let c = Component::new(6.0);
      let result = c / 2.0;

      assert_eq!(result, 3.0);
    }

    #[test]
    fn it_divides_by_integer() {
      let c = Component::new(6.0);
      let result = c / 3;

      assert_eq!(result, 2.0);
    }
  }

  mod div_assign {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_divides_and_assigns() {
      let mut c = Component::new(6.0);
      c /= 2.0;

      assert_eq!(c, 3.0);
    }
  }

  mod mul {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_by_f64() {
      let c = Component::new(2.0);
      let result = c * 3.0;

      assert_eq!(result, 6.0);
    }

    #[test]
    fn it_multiplies_by_integer() {
      let c = Component::new(2.0);
      let result = c * 3;

      assert_eq!(result, 6.0);
    }
  }

  mod mul_assign {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_and_assigns() {
      let mut c = Component::new(2.0);
      c *= 3.0;

      assert_eq!(c, 6.0);
    }
  }

  mod neg {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_negates_positive() {
      let c = Component::new(1.0);

      assert_eq!(-c, -1.0);
    }

    #[test]
    fn it_negates_negative() {
      let c = Component::new(-1.0);

      assert_eq!(-c, 1.0);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_with_f64() {
      let c = Component::new(1.0);

      assert_eq!(c, 1.0);
      assert_ne!(c, 2.0);
    }

    #[test]
    fn it_compares_with_integer() {
      let c = Component::new(1.0);

      assert_eq!(c, 1);
      assert_ne!(c, 2);
    }

    #[test]
    fn it_compares_with_component() {
      let a = Component::new(1.0);
      let b = Component::new(1.0);
      let c = Component::new(2.0);

      assert_eq!(a, b);
      assert_ne!(a, c);
    }
  }

  mod partial_ord {
    use super::*;

    #[test]
    fn it_orders_with_f64() {
      let c = Component::new(1.0);

      assert!(c < 2.0);
      assert!(c > 0.0);
      assert!(c <= 1.0);
      assert!(c >= 1.0);
    }

    #[test]
    fn it_orders_with_integer() {
      let c = Component::new(1.0);

      assert!(c < 2);
      assert!(c > 0);
    }

    #[test]
    fn it_orders_with_component() {
      let a = Component::new(1.0);
      let b = Component::new(2.0);

      assert!(a < b);
      assert!(b > a);
    }
  }

  mod sub {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_f64() {
      let c = Component::new(3.0);
      let result = c - 1.0;

      assert_eq!(result, 2.0);
    }

    #[test]
    fn it_subtracts_integer() {
      let c = Component::new(3.0);
      let result = c - 1;

      assert_eq!(result, 2.0);
    }
  }

  mod sub_assign {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_and_assigns() {
      let mut c = Component::new(3.0);
      c -= 1.0;

      assert_eq!(c, 2.0);
    }
  }
}
