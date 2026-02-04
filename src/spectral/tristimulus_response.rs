use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::space::Xyz;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TristimulusResponse([f64; 3]);

impl TristimulusResponse {
  pub const fn new(x: f64, y: f64, z: f64) -> Self {
    Self([x, y, z])
  }

  pub fn components(&self) -> [f64; 3] {
    self.0
  }

  pub fn x(&self) -> f64 {
    self.0[0]
  }

  pub fn y(&self) -> f64 {
    self.0[1]
  }

  pub fn z(&self) -> f64 {
    self.0[2]
  }
}

impl Display for TristimulusResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "XYZ({:.precision$}, {:.precision$}, {:.precision$})",
      self.0[0],
      self.0[1],
      self.0[2],
      precision = f.precision().unwrap_or(4)
    )
  }
}

impl<T> From<T> for TristimulusResponse
where
  T: Into<Xyz>,
{
  fn from(xyz: T) -> Self {
    let xyz = xyz.into();
    Self::new(xyz.x(), xyz.y(), xyz.z())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let response = TristimulusResponse::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{}", response), "XYZ(0.1235, 0.2346, 0.3457)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let response = TristimulusResponse::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{:.2}", response), "XYZ(0.12, 0.23, 0.35)");
      assert_eq!(format!("{:.6}", response), "XYZ(0.123457, 0.234568, 0.345679)");
    }
  }

  mod from_xyz {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.6, 0.7);
      let response = TristimulusResponse::from(xyz);

      assert_eq!(response.x(), 0.5);
      assert_eq!(response.y(), 0.6);
      assert_eq!(response.z(), 0.7);
    }

    #[test]
    fn it_converts_from_array() {
      let response = TristimulusResponse::from([0.1, 0.2, 0.3]);

      assert_eq!(response.x(), 0.1);
      assert_eq!(response.y(), 0.2);
      assert_eq!(response.z(), 0.3);
    }
  }
}
