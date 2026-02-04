use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(feature = "chromaticity-upvp")]
use super::Upvp;
use super::Xy;
use crate::{component::Component, space::Xyz};

/// CIE 1960 UCS chromaticity coordinates (u, v).
#[derive(Clone, Copy, Debug)]
pub struct Uv {
  u: Component,
  v: Component,
}

impl Uv {
  pub fn new(u: impl Into<Component>, v: impl Into<Component>) -> Self {
    Self {
      u: u.into(),
      v: v.into(),
    }
  }

  pub const fn new_const(u: f64, v: f64) -> Self {
    Self {
      u: Component::new_const(u),
      v: Component::new_const(v),
    }
  }

  pub fn components(&self) -> [f64; 2] {
    [self.u.0, self.v.0]
  }

  #[cfg(feature = "chromaticity-upvp")]
  pub fn to_upvp(&self) -> Upvp {
    let [u, v] = self.components();
    Upvp::new(u, v * 1.5)
  }

  pub fn to_xy(&self) -> Xy {
    let [u, v] = self.components();
    let denom = 2.0 * u - 8.0 * v + 4.0;

    if denom == 0.0 {
      Xy::new(0.0, 0.0)
    } else {
      Xy::new((3.0 * u) / denom, (2.0 * v) / denom)
    }
  }

  pub fn to_xyz(&self, luminance: impl Into<Component>) -> Xyz {
    self.to_xy().to_xyz(luminance)
  }

  pub fn u(&self) -> f64 {
    self.u.0
  }

  pub fn v(&self) -> f64 {
    self.v.0
  }
}

impl Display for Uv {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "uv({:.precision$}, {:.precision$})",
      self.u,
      self.v,
      precision = f.precision().unwrap_or(4)
    )
  }
}

impl<T> From<[T; 2]> for Uv
where
  T: Into<Component>,
{
  fn from([u, v]: [T; 2]) -> Self {
    Self::new(u, v)
  }
}

#[cfg(feature = "chromaticity-upvp")]
impl From<Upvp> for Uv {
  fn from(upvp: Upvp) -> Self {
    upvp.to_uv()
  }
}

impl From<Xy> for Uv {
  fn from(xy: Xy) -> Self {
    xy.to_uv()
  }
}

impl From<Xyz> for Uv {
  fn from(xyz: Xyz) -> Self {
    Self::from(xyz.chromaticity())
  }
}

impl<T> PartialEq<T> for Uv
where
  T: Into<Uv> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.u == other.u && self.v == other.v
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
      let uv = Uv::new(0.19784, 0.31246);

      assert_eq!(format!("{}", uv), "uv(0.1978, 0.3125)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let uv = Uv::new(0.19784, 0.31246);

      assert_eq!(format!("{:.2}", uv), "uv(0.20, 0.31)");
      assert_eq!(format!("{:.6}", uv), "uv(0.197840, 0.312460)");
    }
  }

  mod from_upvp {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_from_upvp() {
      let upvp = Upvp::new(0.19784, 0.46869);
      let uv: Uv = upvp.into();

      assert_eq!(uv.u(), 0.19784);
      assert_eq!(uv.v(), 0.46869 * (2.0 / 3.0));
    }
  }

  mod from_xy {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_from_xy() {
      let xy = Xy::new(0.31271, 0.32902);
      let uv: Uv = xy.into();
      let denom = -2.0 * 0.31271 + 12.0 * 0.32902 + 3.0;

      assert_eq!(uv.u(), (4.0 * 0.31271) / denom);
      assert_eq!(uv.v(), (6.0 * 0.32902) / denom);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz_via_chromaticity() {
      let xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let uv: Uv = xyz.into();
      let xy = xyz.chromaticity();
      let expected: Uv = xy.into();

      assert!(uv == expected);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_values() {
      let a = Uv::new(0.19784, 0.31246);
      let b = Uv::new(0.19784, 0.31246);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_values() {
      let a = Uv::new(0.19784, 0.31246);
      let b = Uv::new(0.19784, 0.32);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_with_array() {
      let uv = Uv::new(0.19784, 0.31246);

      assert_eq!(uv, [0.19784, 0.31246]);
    }
  }

  mod to_upvp {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_upvp() {
      let uv = Uv::new(0.19784, 0.31246);
      let upvp = uv.to_upvp();

      assert_eq!(upvp.u(), 0.19784);
      assert_eq!(upvp.v(), 0.31246 * 1.5);
    }
  }

  mod to_xy {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_xy() {
      let uv = Uv::new(0.19784, 0.31246);
      let xy = uv.to_xy();
      let denom = 2.0 * 0.19784 - 8.0 * 0.31246 + 4.0;

      assert_eq!(xy.x(), (3.0 * 0.19784) / denom);
      assert_eq!(xy.y(), (2.0 * 0.31246) / denom);
    }

    #[test]
    fn it_handles_zero_denominator() {
      let uv = Uv::new(1.0, 0.75);
      let xy = uv.to_xy();

      assert_eq!(xy.x(), 0.0);
      assert_eq!(xy.y(), 0.0);
    }
  }

  mod to_xyz {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_xyz_via_xy() {
      let uv = Uv::new(0.19784, 0.31246);
      let xyz = uv.to_xyz(1.0);
      let expected = uv.to_xy().to_xyz(1.0);

      assert_eq!(xyz.x(), expected.x());
      assert_eq!(xyz.y(), expected.y());
      assert_eq!(xyz.z(), expected.z());
    }
  }
}
