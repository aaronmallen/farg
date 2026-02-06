use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::space::Lms;

/// An LMS cone response at a single wavelength.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConeResponse([f64; 3]);

impl ConeResponse {
  /// Creates a new cone response from L, M, S values.
  pub const fn new(l: f64, m: f64, s: f64) -> Self {
    Self([l, m, s])
  }

  /// Returns the [L, M, S] components as an array.
  pub fn components(&self) -> [f64; 3] {
    self.0
  }

  /// Returns the L (long) cone response.
  pub fn l(&self) -> f64 {
    self.0[0]
  }

  /// Alias for [`Self::l`].
  pub fn long(&self) -> f64 {
    self.l()
  }

  /// Returns the M (medium) cone response.
  pub fn m(&self) -> f64 {
    self.0[1]
  }

  /// Alias for [`Self::m`].
  pub fn medium(&self) -> f64 {
    self.m()
  }

  /// Returns the S (short) cone response.
  pub fn s(&self) -> f64 {
    self.0[2]
  }

  /// Alias for [`Self::s`].
  pub fn short(&self) -> f64 {
    self.s()
  }
}

impl Display for ConeResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "LMS({:.precision$}, {:.precision$}, {:.precision$})",
      self.0[0],
      self.0[1],
      self.0[2],
      precision = f.precision().unwrap_or(4)
    )
  }
}

impl<T> From<T> for ConeResponse
where
  T: Into<Lms>,
{
  fn from(lms: T) -> Self {
    let lms = lms.into();
    Self::new(lms.l(), lms.m(), lms.s())
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
      let response = ConeResponse::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{}", response), "LMS(0.1235, 0.2346, 0.3457)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let response = ConeResponse::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{:.2}", response), "LMS(0.12, 0.23, 0.35)");
      assert_eq!(format!("{:.6}", response), "LMS(0.123457, 0.234568, 0.345679)");
    }
  }

  mod from_lms {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_from_lms() {
      let lms = Lms::new(0.5, 0.6, 0.7);
      let response = ConeResponse::from(lms);

      assert_eq!(response.l(), 0.5);
      assert_eq!(response.m(), 0.6);
      assert_eq!(response.s(), 0.7);
    }

    #[test]
    fn it_converts_from_array() {
      let response = ConeResponse::from([0.1, 0.2, 0.3]);

      assert_eq!(response.l(), 0.1);
      assert_eq!(response.m(), 0.2);
      assert_eq!(response.s(), 0.3);
    }
  }

  mod long {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_aliases_l() {
      let response = ConeResponse::new(0.5, 0.6, 0.7);

      assert_eq!(response.long(), response.l());
    }
  }

  mod medium {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_aliases_m() {
      let response = ConeResponse::new(0.5, 0.6, 0.7);

      assert_eq!(response.medium(), response.m());
    }
  }

  mod short {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_aliases_s() {
      let response = ConeResponse::new(0.5, 0.6, 0.7);

      assert_eq!(response.short(), response.s());
    }
  }
}
