use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  marker::PhantomData,
};

use super::{RgbSpec, space::Rgb};
use crate::component::Component;

/// Linear (scene-referred) RGB values before transfer function encoding.
///
/// Components are stored as normalized values in the 0.0-1.0 range.
#[derive(Clone, Copy, Debug)]
pub struct LinearRgb<S>
where
  S: RgbSpec,
{
  alpha: Component,
  b: Component,
  g: Component,
  r: Component,
  _spec: PhantomData<S>,
}

impl<S> LinearRgb<S>
where
  S: RgbSpec,
{
  /// Creates linear RGB from normalized component values.
  ///
  /// Values outside 0.0-1.0 are preserved to retain out-of-gamut information.
  pub fn from_normalized(r: impl Into<Component>, g: impl Into<Component>, b: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      b: b.into(),
      g: g.into(),
      r: r.into(),
      _spec: PhantomData,
    }
  }

  /// Creates linear RGB from 8-bit (0-255) component values.
  pub fn from_u8(r: impl Into<Component>, g: impl Into<Component>, b: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      b: b.into() / 255.0,
      g: g.into() / 255.0,
      r: r.into() / 255.0,
      _spec: PhantomData,
    }
  }

  /// Creates linear RGB from 8-bit (0-255) component values.
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self::from_u8(r, g, b)
  }

  /// Creates linear RGB from 8-bit values in a const context.
  pub const fn new_const(r: u8, g: u8, b: u8) -> Self {
    Self {
      alpha: Component::new_const(1.0),
      b: Component::new_const(b as f64 / 255.0),
      g: Component::new_const(g as f64 / 255.0),
      r: Component::new_const(r as f64 / 255.0),
      _spec: PhantomData,
    }
  }

  /// Returns the alpha (transparency) value on a 0.0 to 1.0 scale.
  pub fn alpha(&self) -> f64 {
    self.alpha.0
  }

  /// Returns the normalized blue component (0.0-1.0).
  pub fn b(&self) -> f64 {
    self.b.0
  }

  /// Returns the blue component as a u8 (0-255).
  pub fn blue(&self) -> u8 {
    (self.b.0 * 255.0).round() as u8
  }

  /// Returns the [R, G, B] components as normalized values.
  pub fn components(&self) -> [f64; 3] {
    [self.r.0, self.g.0, self.b.0]
  }

  /// Returns the normalized green component (0.0-1.0).
  pub fn g(&self) -> f64 {
    self.g.0
  }

  /// Returns the green component as a u8 (0-255).
  pub fn green(&self) -> u8 {
    (self.g.0 * 255.0).round() as u8
  }

  /// Returns the normalized red component (0.0-1.0).
  pub fn r(&self) -> f64 {
    self.r.0
  }

  /// Returns the red component as a u8 (0-255).
  pub fn red(&self) -> u8 {
    (self.r.0 * 255.0).round() as u8
  }

  /// Applies the transfer function to produce encoded (gamma-corrected) RGB values.
  pub fn to_encoded(&self) -> Rgb<S> {
    let r = S::TRANSFER_FUNCTION.encode(self.r);
    let g = S::TRANSFER_FUNCTION.encode(self.g);
    let b = S::TRANSFER_FUNCTION.encode(self.b);
    Rgb::from_normalized(r, g, b).with_alpha(self.alpha)
  }

  /// Returns a new color with the given alpha value on a 0.0 to 1.0 scale.
  pub fn with_alpha(&self, alpha: impl Into<Component>) -> Self {
    Self {
      alpha: alpha.into().clamp(0.0, 1.0),
      ..*self
    }
  }
}

impl<S> Display for LinearRgb<S>
where
  S: RgbSpec,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Linear {}({}, {}, {}, {:.0}%)",
        S::NAME,
        self.red(),
        self.green(),
        self.blue(),
        self.alpha.0 * 100.0
      )
    } else {
      write!(
        f,
        "Linear {}({}, {}, {})",
        S::NAME,
        self.red(),
        self.green(),
        self.blue()
      )
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::space::{ColorSpace, Srgb};

  mod from_normalized {
    use super::*;

    #[test]
    fn it_preserves_out_of_range_values() {
      let linear = LinearRgb::<Srgb>::from_normalized(1.5, -0.5, 0.5);

      assert!((linear.r() - 1.5).abs() < 1e-10);
      assert!((linear.g() - -0.5).abs() < 1e-10);
      assert!((linear.b() - 0.5).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_space_name_and_8bit_values() {
      let linear = LinearRgb::<Srgb>::new(128, 64, 32);

      assert_eq!(format!("{}", linear), "Linear sRGB(128, 64, 32)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let linear = LinearRgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);

      assert_eq!(format!("{}", linear), "Linear sRGB(128, 64, 32, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let linear = LinearRgb::<Srgb>::new(128, 64, 32);

      assert_eq!(format!("{}", linear), "Linear sRGB(128, 64, 32)");
    }
  }

  mod to_encoded {
    use super::*;

    #[test]
    fn it_applies_transfer_function_encoding() {
      let linear = LinearRgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let encoded = linear.to_encoded();

      assert!(encoded.r() > linear.r());
      assert!(encoded.g() > linear.g());
      assert!(encoded.b() > linear.b());
    }

    #[test]
    fn it_roundtrips_with_rgb_to_linear() {
      let original = LinearRgb::<Srgb>::from_normalized(0.25, 0.5, 0.75);
      let encoded = original.to_encoded();
      let back = encoded.to_linear();

      assert!((back.r() - original.r()).abs() < 1e-10);
      assert!((back.g() - original.g()).abs() < 1e-10);
      assert!((back.b() - original.b()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let linear = LinearRgb::<Srgb>::from_normalized(0.5, 0.5, 0.5).with_alpha(0.3);
      let encoded = linear.to_encoded();

      assert!((encoded.alpha() - 0.3).abs() < 1e-10);
    }
  }
}
