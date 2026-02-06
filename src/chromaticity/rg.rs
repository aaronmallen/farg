use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  marker::PhantomData,
};

#[cfg(feature = "chromaticity-upvp")]
use super::Upvp;
#[cfg(feature = "chromaticity-uv")]
use super::Uv;
use super::Xy;
use crate::{
  component::Component,
  space::{RgbSpec, Srgb},
};

/// RGB-relative chromaticity coordinates (r, g) for a given RGB color space.
///
/// Derived by normalizing linear RGB: r = R/(R+G+B), g = G/(R+G+B).
/// The space parameter `S` determines which RGB primaries are used.
#[derive(Clone, Copy, Debug)]
pub struct Rg<S = Srgb>
where
  S: RgbSpec,
{
  g: Component,
  r: Component,
  _spec: PhantomData<S>,
}

impl<S> Rg<S>
where
  S: RgbSpec,
{
  /// Creates new rg coordinates.
  pub fn new(r: impl Into<Component>, g: impl Into<Component>) -> Self {
    Self {
      g: g.into(),
      r: r.into(),
      _spec: PhantomData,
    }
  }

  /// Creates new rg coordinates in a const context.
  pub const fn new_const(r: f64, g: f64) -> Self {
    Self {
      g: Component::new_const(g),
      r: Component::new_const(r),
      _spec: PhantomData,
    }
  }

  /// Returns the [r, g] components as an array.
  pub fn components(&self) -> [f64; 2] {
    [self.r.0, self.g.0]
  }

  /// Returns the g chromaticity coordinate.
  pub fn g(&self) -> f64 {
    self.g.0
  }

  /// Returns the r chromaticity coordinate.
  pub fn r(&self) -> f64 {
    self.r.0
  }

  /// Converts to CIE 1976 u'v' coordinates via xy.
  #[cfg(feature = "chromaticity-upvp")]
  pub fn to_upvp(&self) -> Upvp {
    self.to_xy().to_upvp()
  }

  /// Converts to CIE 1960 uv coordinates via xy.
  #[cfg(feature = "chromaticity-uv")]
  pub fn to_uv(&self) -> Uv {
    self.to_xy().to_uv()
  }

  /// Converts to CIE 1931 xy coordinates using the RGB space's primaries matrix.
  pub fn to_xy(&self) -> Xy {
    let [r, g] = self.components();
    let b = 1.0 - self.r.0 - self.g.0;
    let matrix = S::xyz_matrix();
    let [x, y, z] = *matrix * [r, g, b];
    let sum = x + y + z;

    if sum == 0.0 {
      Xy::new(0.0, 0.0)
    } else {
      Xy::new(x / sum, y / sum)
    }
  }
}

impl Display for Rg {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "rg({:.precision$}, {:.precision$})",
      self.r,
      self.g,
      precision = f.precision().unwrap_or(4)
    )
  }
}

impl<T> From<[T; 2]> for Rg
where
  T: Into<Component>,
{
  fn from([r, g]: [T; 2]) -> Self {
    Self::new(r, g)
  }
}

#[cfg(feature = "chromaticity-upvp")]
impl From<Upvp> for Rg {
  fn from(upvp: Upvp) -> Self {
    upvp.to_rg::<Srgb>()
  }
}

#[cfg(feature = "chromaticity-uv")]
impl From<Uv> for Rg {
  fn from(uv: Uv) -> Self {
    uv.to_rg::<Srgb>()
  }
}

impl From<Xy> for Rg {
  fn from(xy: Xy) -> Self {
    xy.to_rg::<Srgb>()
  }
}

impl<S> PartialEq for Rg<S>
where
  S: RgbSpec,
{
  fn eq(&self, other: &Self) -> bool {
    self.r == other.r && self.g == other.g
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
      let rg: Rg = Rg::new(0.64, 0.33);

      assert_eq!(format!("{}", rg), "rg(0.6400, 0.3300)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let rg: Rg = Rg::new(0.64, 0.33);

      assert_eq!(format!("{:.2}", rg), "rg(0.64, 0.33)");
      assert_eq!(format!("{:.6}", rg), "rg(0.640000, 0.330000)");
    }
  }

  #[cfg(feature = "chromaticity-upvp")]
  mod from_upvp {
    use super::*;

    #[test]
    fn it_converts_from_upvp() {
      let upvp = Upvp::new(0.19784, 0.46869);
      let rg: Rg = upvp.into();
      let expected: Rg = upvp.to_xy().into();

      assert!(rg == expected);
    }
  }

  #[cfg(feature = "chromaticity-uv")]
  mod from_uv {
    use super::*;

    #[test]
    fn it_converts_from_uv() {
      let uv = Uv::new(0.19784, 0.31246);
      let rg: Rg = uv.into();
      let expected: Rg = uv.to_xy().into();

      assert!(rg == expected);
    }
  }

  mod from_xy {
    use super::*;

    #[test]
    fn it_converts_from_xy() {
      let xy = Xy::new(0.31271, 0.32902);
      let rg: Rg = xy.into();
      let expected = xy.to_rg::<Srgb>();

      assert!(rg == expected);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_values() {
      let a: Rg = Rg::new(0.64, 0.33);
      let b: Rg = Rg::new(0.64, 0.33);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_values() {
      let a: Rg = Rg::new(0.64, 0.33);
      let b: Rg = Rg::new(0.64, 0.34);

      assert_ne!(a, b);
    }
  }

  #[cfg(feature = "chromaticity-upvp")]
  mod to_upvp {
    use super::*;

    #[test]
    fn it_converts_to_upvp_via_xy() {
      let rg: Rg = Rg::new(0.64, 0.33);
      let upvp = rg.to_upvp();
      let expected = rg.to_xy().to_upvp();

      assert!(upvp == expected);
    }
  }

  #[cfg(feature = "chromaticity-uv")]
  mod to_uv {
    use super::*;

    #[test]
    fn it_converts_to_uv_via_xy() {
      let rg: Rg = Rg::new(0.64, 0.33);
      let uv = rg.to_uv();
      let expected = rg.to_xy().to_uv();

      assert!(uv == expected);
    }
  }

  mod to_xy {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_xy_via_matrix() {
      let rg: Rg = Rg::new(0.64, 0.33);
      let xy = rg.to_xy();
      let [r, g] = rg.components();
      let b = 1.0 - r - g;
      let matrix = Srgb::xyz_matrix();
      let [x, y, z] = *matrix * [r, g, b];
      let sum = x + y + z;

      assert_eq!(xy.x(), x / sum);
      assert_eq!(xy.y(), y / sum);
    }

    #[test]
    fn it_handles_zero_sum() {
      let rg: Rg = Rg::new(0.0, 0.0);
      let matrix = Srgb::xyz_matrix();
      let [x, y, z] = *matrix * [0.0, 0.0, 1.0];
      let sum = x + y + z;

      if sum != 0.0 {
        assert_ne!(rg.to_xy().x(), 0.0);
      }
    }

    #[test]
    fn it_roundtrips_through_xy() {
      let original: Rg = Rg::new(0.64, 0.33);
      let xy = original.to_xy();
      let roundtrip: Rg = xy.to_rg::<Srgb>();

      let [r1, g1] = original.components();
      let [r2, g2] = roundtrip.components();

      assert!((r1 - r2).abs() < 1e-10);
      assert!((g1 - g2).abs() < 1e-10);
    }
  }
}
