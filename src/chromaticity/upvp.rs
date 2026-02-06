use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(feature = "chromaticity-rg")]
use super::Rg;
#[cfg(feature = "chromaticity-uv")]
use super::Uv;
use super::Xy;
#[cfg(feature = "chromaticity-rg")]
use crate::space::RgbSpec;
use crate::{component::Component, space::Xyz};

/// CIE 1976 UCS chromaticity coordinates (u', v').
#[derive(Clone, Copy, Debug)]
pub struct Upvp {
  u: Component,
  v: Component,
}

impl Upvp {
  /// Creates new u'v' coordinates.
  pub fn new(u: impl Into<Component>, v: impl Into<Component>) -> Self {
    Self {
      u: u.into(),
      v: v.into(),
    }
  }

  /// Creates new u'v' coordinates in a const context.
  pub const fn new_const(u: f64, v: f64) -> Self {
    Self {
      u: Component::new_const(u),
      v: Component::new_const(v),
    }
  }

  /// Returns the [u', v'] components as an array.
  pub fn components(&self) -> [f64; 2] {
    [self.u.0, self.v.0]
  }

  /// Converts to rg chromaticity coordinates in the given RGB space.
  #[cfg(feature = "chromaticity-rg")]
  pub fn to_rg<S>(&self) -> Rg<S>
  where
    S: RgbSpec,
  {
    self.to_xy().to_rg::<S>()
  }

  /// Converts to CIE 1960 uv coordinates.
  #[cfg(feature = "chromaticity-uv")]
  pub fn to_uv(&self) -> Uv {
    let [u, v] = self.components();
    Uv::new(u, v * (2.0 / 3.0))
  }

  /// Converts to CIE 1931 xy coordinates.
  pub fn to_xy(&self) -> Xy {
    let [u, v] = self.components();
    let denom = 6.0 * u - 16.0 * v + 12.0;

    if denom == 0.0 {
      Xy::new(0.0, 0.0)
    } else {
      Xy::new((9.0 * u) / denom, (4.0 * v) / denom)
    }
  }

  /// Reconstructs XYZ tristimulus values via xy with the given luminance.
  pub fn to_xyz(&self, luminance: impl Into<Component>) -> Xyz {
    self.to_xy().to_xyz(luminance)
  }

  /// Returns the u' coordinate.
  pub fn u(&self) -> f64 {
    self.u.0
  }

  /// Returns the v' coordinate.
  pub fn v(&self) -> f64 {
    self.v.0
  }
}

impl Display for Upvp {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "u'v'({:.precision$}, {:.precision$})",
      self.u,
      self.v,
      precision = f.precision().unwrap_or(4)
    )
  }
}

impl<T> From<[T; 2]> for Upvp
where
  T: Into<Component>,
{
  fn from([u, v]: [T; 2]) -> Self {
    Self::new(u, v)
  }
}

#[cfg(feature = "chromaticity-rg")]
impl<S> From<Rg<S>> for Upvp
where
  S: RgbSpec,
{
  fn from(rg: Rg<S>) -> Self {
    rg.to_upvp()
  }
}

#[cfg(feature = "chromaticity-uv")]
impl From<Uv> for Upvp {
  fn from(uv: Uv) -> Self {
    uv.to_upvp()
  }
}

impl From<Xy> for Upvp {
  fn from(xy: Xy) -> Self {
    xy.to_upvp()
  }
}

impl From<Xyz> for Upvp {
  fn from(xyz: Xyz) -> Self {
    Self::from(xyz.chromaticity())
  }
}

impl<T> PartialEq<T> for Upvp
where
  T: Into<Upvp> + Copy,
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
      let upvp = Upvp::new(0.19784, 0.46869);

      assert_eq!(format!("{}", upvp), "u'v'(0.1978, 0.4687)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let upvp = Upvp::new(0.19784, 0.46869);

      assert_eq!(format!("{:.2}", upvp), "u'v'(0.20, 0.47)");
      assert_eq!(format!("{:.6}", upvp), "u'v'(0.197840, 0.468690)");
    }
  }

  mod from_uv {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_from_uv() {
      let uv = Uv::new(0.19784, 0.31246);
      let upvp: Upvp = uv.into();

      assert_eq!(upvp.u(), 0.19784);
      assert_eq!(upvp.v(), 0.31246 * 1.5);
    }
  }

  mod from_xy {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_from_xy() {
      let xy = Xy::new(0.31271, 0.32902);
      let upvp: Upvp = xy.into();
      let denom = -2.0 * 0.31271 + 12.0 * 0.32902 + 3.0;

      assert_eq!(upvp.u(), (4.0 * 0.31271) / denom);
      assert_eq!(upvp.v(), (9.0 * 0.32902) / denom);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz_via_chromaticity() {
      let xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let upvp: Upvp = xyz.into();
      let xy = xyz.chromaticity();
      let expected: Upvp = xy.into();

      assert!(upvp == expected);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_values() {
      let a = Upvp::new(0.19784, 0.46869);
      let b = Upvp::new(0.19784, 0.46869);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_values() {
      let a = Upvp::new(0.19784, 0.46869);
      let b = Upvp::new(0.19784, 0.47);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_with_array() {
      let upvp = Upvp::new(0.19784, 0.46869);

      assert_eq!(upvp, [0.19784, 0.46869]);
    }
  }

  #[cfg(feature = "chromaticity-rg")]
  mod to_rg {
    use super::*;
    use crate::space::Srgb;

    #[test]
    fn it_converts_to_rg_via_xy() {
      let upvp = Upvp::new(0.19784, 0.46869);
      let rg: Rg<Srgb> = upvp.to_rg();
      let expected: Rg<Srgb> = upvp.to_xy().to_rg();

      assert!(rg == expected);
    }
  }

  mod to_uv {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_uv() {
      let upvp = Upvp::new(0.19784, 0.46869);
      let uv = upvp.to_uv();

      assert_eq!(uv.u(), 0.19784);
      assert_eq!(uv.v(), 0.46869 * (2.0 / 3.0));
    }
  }

  mod to_xy {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_xy() {
      let upvp = Upvp::new(0.19784, 0.46869);
      let xy = upvp.to_xy();
      let denom = 6.0 * 0.19784 - 16.0 * 0.46869 + 12.0;

      assert_eq!(xy.x(), (9.0 * 0.19784) / denom);
      assert_eq!(xy.y(), (4.0 * 0.46869) / denom);
    }

    #[test]
    fn it_handles_zero_denominator() {
      let upvp = Upvp::new(2.0, 1.5);
      let xy = upvp.to_xy();

      assert_eq!(xy.x(), 0.0);
      assert_eq!(xy.y(), 0.0);
    }
  }

  mod to_xyz {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_xyz_via_xy() {
      let upvp = Upvp::new(0.19784, 0.46869);
      let xyz = upvp.to_xyz(1.0);
      let expected = upvp.to_xy().to_xyz(1.0);

      assert_eq!(xyz.x(), expected.x());
      assert_eq!(xyz.y(), expected.y());
      assert_eq!(xyz.z(), expected.z());
    }
  }
}
