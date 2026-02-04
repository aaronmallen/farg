use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(feature = "chromaticity-upvp")]
use super::Upvp;
#[cfg(feature = "chromaticity-uv")]
use super::Uv;
use crate::{component::Component, space::Xyz};

#[derive(Clone, Copy, Debug)]
pub struct Xy {
  x: Component,
  y: Component,
}

impl Xy {
  pub fn new(x: impl Into<Component>, y: impl Into<Component>) -> Self {
    Self {
      x: x.into(),
      y: y.into(),
    }
  }

  pub const fn new_const(x: f64, y: f64) -> Self {
    Self {
      x: Component::new_const(x),
      y: Component::new_const(y),
    }
  }

  pub fn components(&self) -> [f64; 2] {
    [self.x.0, self.y.0]
  }

  #[cfg(feature = "chromaticity-upvp")]
  pub fn to_upvp(&self) -> Upvp {
    let [x, y] = self.components();
    let denom = -2.0 * x + 12.0 * y + 3.0;

    if denom == 0.0 {
      Upvp::new(0.0, 0.0)
    } else {
      Upvp::new((4.0 * x) / denom, (9.0 * y) / denom)
    }
  }

  #[cfg(feature = "chromaticity-uv")]
  pub fn to_uv(&self) -> Uv {
    let [x, y] = self.components();
    let denom = -2.0 * x + 12.0 * y + 3.0;

    if denom == 0.0 {
      Uv::new(0.0, 0.0)
    } else {
      Uv::new((4.0 * x) / denom, (6.0 * y) / denom)
    }
  }

  pub fn to_xyz(&self, luminance: impl Into<Component>) -> Xyz {
    let luminance = luminance.into().0;
    let [x, y] = self.components();

    if y == 0.0 {
      Xyz::new(0.0, 0.0, 0.0)
    } else {
      Xyz::new((x / y) * luminance, luminance, ((1.0 - x - y) / y) * luminance)
    }
  }

  pub fn x(&self) -> f64 {
    self.x.0
  }

  pub fn y(&self) -> f64 {
    self.y.0
  }
}

impl Display for Xy {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "({:.precision$}, {:.precision$})",
      self.x,
      self.y,
      precision = f.precision().unwrap_or(4)
    )
  }
}

impl<T> From<[T; 2]> for Xy
where
  T: Into<Component>,
{
  fn from([x, y]: [T; 2]) -> Self {
    Self::new(x, y)
  }
}

#[cfg(feature = "chromaticity-upvp")]
impl From<Upvp> for Xy {
  fn from(upvp: Upvp) -> Self {
    upvp.to_xy()
  }
}

#[cfg(feature = "chromaticity-uv")]
impl From<Uv> for Xy {
  fn from(uv: Uv) -> Self {
    uv.to_xy()
  }
}

impl From<Xyz> for Xy {
  fn from(xyz: Xyz) -> Self {
    xyz.chromaticity()
  }
}

impl<T> PartialEq<T> for Xy
where
  T: Into<Xy> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.x == other.x && self.y == other.y
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
      let xy = Xy::new(0.31271, 0.32902);

      assert_eq!(format!("{}", xy), "(0.3127, 0.3290)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let xy = Xy::new(0.31271, 0.32902);

      assert_eq!(format!("{:.2}", xy), "(0.31, 0.33)");
      assert_eq!(format!("{:.6}", xy), "(0.312710, 0.329020)");
    }
  }

  mod from_xyz {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let xy: Xy = xyz.into();

      assert_eq!(xy.x(), 0.95047 / (0.95047 + 1.0 + 1.08883));
      assert_eq!(xy.y(), 1.0 / (0.95047 + 1.0 + 1.08883));
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_values() {
      let a = Xy::new(0.31271, 0.32902);
      let b = Xy::new(0.31271, 0.32902);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_values() {
      let a = Xy::new(0.31271, 0.32902);
      let b = Xy::new(0.31271, 0.33);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_with_array() {
      let xy = Xy::new(0.31271, 0.32902);

      assert_eq!(xy, [0.31271, 0.32902]);
    }
  }

  #[cfg(feature = "chromaticity-upvp")]
  mod to_upvp {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_upvp() {
      let xy = Xy::new(0.31271, 0.32902);
      let upvp = xy.to_upvp();
      let denom = -2.0 * 0.31271 + 12.0 * 0.32902 + 3.0;

      assert_eq!(upvp.u(), (4.0 * 0.31271) / denom);
      assert_eq!(upvp.v(), (9.0 * 0.32902) / denom);
    }

    #[test]
    fn it_handles_zero_denominator() {
      let xy = Xy::new(1.5, 0.0);
      let upvp = xy.to_upvp();

      assert_eq!(upvp.u(), 0.0);
      assert_eq!(upvp.v(), 0.0);
    }
  }

  #[cfg(feature = "chromaticity-uv")]
  mod to_uv {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_uv() {
      let xy = Xy::new(0.31271, 0.32902);
      let uv = xy.to_uv();
      let denom = -2.0 * 0.31271 + 12.0 * 0.32902 + 3.0;

      assert_eq!(uv.u(), (4.0 * 0.31271) / denom);
      assert_eq!(uv.v(), (6.0 * 0.32902) / denom);
    }

    #[test]
    fn it_handles_zero_denominator() {
      let xy = Xy::new(1.5, 0.0);
      let uv = xy.to_uv();

      assert_eq!(uv.u(), 0.0);
      assert_eq!(uv.v(), 0.0);
    }
  }

  mod to_xyz {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_xyz_with_luminance() {
      let xy = Xy::new(0.31271, 0.32902);
      let xyz = xy.to_xyz(1.0);

      assert_eq!(xyz.y(), 1.0);
      assert_eq!(xyz.x(), 0.31271 / 0.32902);
      assert_eq!(xyz.z(), (1.0 - 0.31271 - 0.32902) / 0.32902);
    }

    #[test]
    fn it_handles_zero_y() {
      let xy = Xy::new(0.5, 0.0);
      let xyz = xy.to_xyz(1.0);

      assert_eq!(xyz.x(), 0.0);
      assert_eq!(xyz.y(), 0.0);
      assert_eq!(xyz.z(), 0.0);
    }

    #[test]
    fn it_scales_by_luminance() {
      let xy = Xy::new(0.31271, 0.32902);
      let xyz = xy.to_xyz(0.5);

      assert_eq!(xyz.y(), 0.5);
    }
  }
}
