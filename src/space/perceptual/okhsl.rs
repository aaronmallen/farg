use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, Div, Mul, Sub},
};

use super::oklab::{cusp_for_hue, max_chroma_at_lightness, toe_inv};
#[cfg(feature = "space-cmy")]
use crate::space::Cmy;
#[cfg(feature = "space-cmyk")]
use crate::space::Cmyk;
#[cfg(feature = "space-hsi")]
use crate::space::Hsi;
#[cfg(feature = "space-hsl")]
use crate::space::Hsl;
#[cfg(feature = "space-hsv")]
use crate::space::Hsv;
#[cfg(feature = "space-hwb")]
use crate::space::Hwb;
#[cfg(feature = "space-lab")]
use crate::space::Lab;
#[cfg(feature = "space-lch")]
use crate::space::Lch;
#[cfg(feature = "space-lchuv")]
use crate::space::Lchuv;
#[cfg(feature = "space-luv")]
use crate::space::Luv;
#[cfg(feature = "space-okhsv")]
use crate::space::Okhsv;
#[cfg(feature = "space-okhwb")]
use crate::space::Okhwb;
#[cfg(feature = "space-oklch")]
use crate::space::Oklch;
#[cfg(feature = "space-xyy")]
use crate::space::Xyy;
use crate::{
  ColorimetricContext, Illuminant, Observer,
  component::Component,
  space::{ColorSpace, Lms, Oklab, Rgb, RgbSpec, Srgb, Xyz},
};

/// Okhsl perceptual color space (HSL model in the Oklab perceptual framework).
///
/// A perceptually uniform HSL-like color space derived from Oklab. H represents
/// hue stored internally as a 0.0-1.0 fraction (0-360°), S represents saturation
/// (0.0-1.0), and L represents perceived lightness (0.0-1.0). Designed for
/// intuitive color manipulation with perceptual uniformity, using sRGB gamut
/// boundaries for saturation mapping.
#[derive(Clone, Copy, Debug)]
pub struct Okhsl {
  alpha: Component,
  context: ColorimetricContext,
  h: Component,
  l: Component,
  s: Component,
}

impl Okhsl {
  /// The default viewing context for Okhsl (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new Okhsl color from hue (0-360°), saturation (0-100%), and lightness (0-100%).
  pub fn new(h: impl Into<Component>, s: impl Into<Component>, l: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      l: l.into() / 100.0,
      s: s.into() / 100.0,
    }
  }

  /// Creates a new Okhsl color in a const context from hue (0-360°), saturation (0-100%), and lightness (0-100%).
  pub const fn new_const(h: f64, s: f64, l: f64) -> Self {
    let r = (h / 360.0) % 1.0;

    Self {
      alpha: Component::new_const(1.0),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new_const(if r < 0.0 { r + 1.0 } else { r }),
      l: Component::new_const(l / 100.0),
      s: Component::new_const(s / 100.0),
    }
  }

  /// Returns the [H, S, L] components as normalized values (all 0.0-1.0).
  pub fn components(&self) -> [f64; 3] {
    [self.h.0, self.s.0, self.l.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases the normalized hue by the given amount (wraps around 0.0-1.0).
  pub fn decrement_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 - amount.into().0).rem_euclid(1.0));
  }

  /// Decreases the hue by the given amount in degrees (wraps around 0-360°).
  pub fn decrement_hue(&mut self, amount: impl Into<Component>) {
    self.decrement_h(amount.into() / 360.0)
  }

  /// Decreases the normalized lightness by the given amount.
  pub fn decrement_l(&mut self, amount: impl Into<Component>) {
    self.l -= amount.into();
  }

  /// Decreases the lightness by the given amount in percentage points.
  pub fn decrement_lightness(&mut self, amount: impl Into<Component>) {
    self.decrement_l(amount.into() / 100.0)
  }

  /// Decreases the normalized saturation by the given amount.
  pub fn decrement_s(&mut self, amount: impl Into<Component>) {
    self.s -= amount.into();
  }

  /// Decreases the saturation by the given amount in percentage points.
  pub fn decrement_saturation(&mut self, amount: impl Into<Component>) {
    self.decrement_s(amount.into() / 100.0)
  }

  /// Returns the normalized hue component (0.0-1.0).
  pub fn h(&self) -> f64 {
    self.h.0
  }

  /// Returns the hue in degrees (0-360°).
  pub fn hue(&self) -> f64 {
    self.h.0 * 360.0
  }

  /// Increases the normalized hue by the given amount (wraps around 0.0-1.0).
  pub fn increment_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 + amount.into().0).rem_euclid(1.0));
  }

  /// Increases the hue by the given amount in degrees (wraps around 0-360°).
  pub fn increment_hue(&mut self, amount: impl Into<Component>) {
    self.increment_h(amount.into() / 360.0)
  }

  /// Increases the normalized lightness by the given amount.
  pub fn increment_l(&mut self, amount: impl Into<Component>) {
    self.l += amount.into();
  }

  /// Increases the lightness by the given amount in percentage points.
  pub fn increment_lightness(&mut self, amount: impl Into<Component>) {
    self.increment_l(amount.into() / 100.0)
  }

  /// Increases the normalized saturation by the given amount.
  pub fn increment_s(&mut self, amount: impl Into<Component>) {
    self.s += amount.into();
  }

  /// Increases the saturation by the given amount in percentage points.
  pub fn increment_saturation(&mut self, amount: impl Into<Component>) {
    self.increment_s(amount.into() / 100.0)
  }

  /// Returns the normalized lightness component (0.0-1.0).
  pub fn l(&self) -> f64 {
    self.l.0
  }

  /// Returns the lightness as a percentage (0-100%).
  pub fn lightness(&self) -> f64 {
    self.l.0 * 100.0
  }

  /// Returns the normalized saturation component (0.0-1.0).
  pub fn s(&self) -> f64 {
    self.s.0
  }

  /// Returns the saturation as a percentage (0-100%).
  pub fn saturation(&self) -> f64 {
    self.s.0 * 100.0
  }

  /// Scales the normalized hue by the given factor (wraps around 0.0-1.0).
  pub fn scale_h(&mut self, factor: impl Into<Component>) {
    self.h = Component::new((self.h.0 * factor.into().0).rem_euclid(1.0));
  }

  /// Alias for [`Self::scale_h`].
  pub fn scale_hue(&mut self, factor: impl Into<Component>) {
    self.scale_h(factor)
  }

  /// Scales the normalized lightness by the given factor.
  pub fn scale_l(&mut self, factor: impl Into<Component>) {
    self.l *= factor.into();
  }

  /// Alias for [`Self::scale_l`].
  pub fn scale_lightness(&mut self, factor: impl Into<Component>) {
    self.scale_l(factor)
  }

  /// Scales the normalized saturation by the given factor.
  pub fn scale_s(&mut self, factor: impl Into<Component>) {
    self.s *= factor.into();
  }

  /// Alias for [`Self::scale_s`].
  pub fn scale_saturation(&mut self, factor: impl Into<Component>) {
    self.scale_s(factor)
  }

  /// Sets all three components from normalized values.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_h(components[0].clone());
    self.set_s(components[1].clone());
    self.set_l(components[2].clone());
  }

  /// Sets the normalized hue component (0.0-1.0).
  pub fn set_h(&mut self, h: impl Into<Component>) {
    self.h = h.into();
  }

  /// Sets the hue from a value in degrees (0-360°).
  pub fn set_hue(&mut self, hue: impl Into<Component>) {
    self.h = Component::new((hue.into().0 / 360.0).rem_euclid(1.0));
  }

  /// Sets the normalized lightness component (0.0-1.0).
  pub fn set_l(&mut self, l: impl Into<Component>) {
    self.l = l.into();
  }

  /// Sets the lightness from a percentage value (0-100%).
  pub fn set_lightness(&mut self, lightness: impl Into<Component>) {
    self.l = lightness.into() / 100.0;
  }

  /// Sets the normalized saturation component (0.0-1.0).
  pub fn set_s(&mut self, s: impl Into<Component>) {
    self.s = s.into();
  }

  /// Sets the saturation from a percentage value (0-100%).
  pub fn set_saturation(&mut self, saturation: impl Into<Component>) {
    self.s = saturation.into() / 100.0;
  }

  /// Converts to the Oklab perceptual color space.
  pub fn to_oklab(&self) -> Oklab {
    let [h, s, l] = self.components();
    let lab_l = toe_inv(l);
    let cusp = cusp_for_hue(h);
    let max_c = max_chroma_at_lightness(cusp, lab_l);
    let c = s * max_c;

    let h_rad = h * 2.0 * std::f64::consts::PI;
    let a = c * h_rad.cos();
    let b = c * h_rad.sin();

    Oklab::new(lab_l, a, b).with_alpha(self.alpha)
  }

  /// Converts to the specified RGB color space.
  pub fn to_rgb<S>(&self) -> Rgb<S>
  where
    S: RgbSpec,
  {
    self.to_oklab().to_rgb::<S>()
  }

  /// Converts to the CIE XYZ color space.
  pub fn to_xyz(&self) -> Xyz {
    self.to_oklab().to_xyz()
  }

  /// Returns this color with a different viewing context (without adaptation).
  pub fn with_context(&self, context: ColorimetricContext) -> Self {
    Self {
      context,
      ..*self
    }
  }

  /// Returns a new color with the given normalized hue value.
  pub fn with_h(&self, h: impl Into<Component>) -> Self {
    Self {
      h: h.into(),
      ..*self
    }
  }

  /// Returns a new color with the normalized hue decreased by the given amount.
  pub fn with_h_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.decrement_h(amount);
    okhsl
  }

  /// Returns a new color with the normalized hue increased by the given amount.
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.increment_h(amount);
    okhsl
  }

  /// Returns a new color with the normalized hue scaled by the given factor.
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.scale_h(factor);
    okhsl
  }

  /// Returns a new color with the given hue in degrees (0-360°).
  pub fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self {
      h: Component::new((hue.into().0 / 360.0).rem_euclid(1.0)),
      ..*self
    }
  }

  /// Returns a new color with the hue decreased by the given degrees.
  pub fn with_hue_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.decrement_hue(amount);
    okhsl
  }

  /// Returns a new color with the hue increased by the given degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.increment_hue(amount);
    okhsl
  }

  /// Alias for [`Self::with_h_scaled_by`].
  pub fn with_hue_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_h_scaled_by(factor)
  }

  /// Returns a new color with the given normalized lightness value.
  pub fn with_l(&self, l: impl Into<Component>) -> Self {
    Self {
      l: l.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized lightness decreased by the given amount.
  pub fn with_l_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.decrement_l(amount);
    okhsl
  }

  /// Returns a new color with normalized lightness increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.increment_l(amount);
    okhsl
  }

  /// Returns a new color with normalized lightness scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.scale_l(factor);
    okhsl
  }

  /// Returns a new color with the given lightness in percentage (0-100%).
  pub fn with_lightness(&self, lightness: impl Into<Component>) -> Self {
    Self {
      l: lightness.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with lightness decreased by the given percentage points.
  pub fn with_lightness_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.decrement_lightness(amount);
    okhsl
  }

  /// Returns a new color with lightness increased by the given percentage points.
  pub fn with_lightness_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.increment_lightness(amount);
    okhsl
  }

  /// Alias for [`Self::with_l_scaled_by`].
  pub fn with_lightness_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_l_scaled_by(factor)
  }

  /// Returns a new color with the given normalized saturation value.
  pub fn with_s(&self, s: impl Into<Component>) -> Self {
    Self {
      s: s.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized saturation decreased by the given amount.
  pub fn with_s_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.decrement_s(amount);
    okhsl
  }

  /// Returns a new color with normalized saturation increased by the given amount.
  pub fn with_s_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.increment_s(amount);
    okhsl
  }

  /// Returns a new color with normalized saturation scaled by the given factor.
  pub fn with_s_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.scale_s(factor);
    okhsl
  }

  /// Returns a new color with the given saturation in percentage (0-100%).
  pub fn with_saturation(&self, saturation: impl Into<Component>) -> Self {
    Self {
      s: saturation.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with saturation decreased by the given percentage points.
  pub fn with_saturation_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.decrement_saturation(amount);
    okhsl
  }

  /// Returns a new color with saturation increased by the given percentage points.
  pub fn with_saturation_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsl = *self;
    okhsl.increment_saturation(amount);
    okhsl
  }

  /// Alias for [`Self::with_s_scaled_by`].
  pub fn with_saturation_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_s_scaled_by(factor)
  }
}

impl<T> Add<T> for Okhsl
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Okhsl {
  fn alpha(&self) -> f64 {
    self.alpha.0
  }

  fn components(&self) -> [f64; 3] {
    self.components()
  }

  fn set_alpha(&mut self, alpha: impl Into<Component>) {
    self.alpha = alpha.into().clamp(0.0, 1.0);
  }

  fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_components(components)
  }

  fn to_xyz(&self) -> Xyz {
    self.to_xyz()
  }
}

impl Display for Okhsl {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Okhsl({:.precision$}°, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.hue(),
        self.saturation(),
        self.lightness(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "Okhsl({:.precision$}°, {:.precision$}%, {:.precision$}%)",
        self.hue(),
        self.saturation(),
        self.lightness()
      )
    }
  }
}

impl<T> Div<T> for Okhsl
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Okhsl
where
  T: Into<Component>,
{
  fn from([h, s, l]: [T; 3]) -> Self {
    Self::new(h, s, l)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Okhsl
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_okhsl()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Okhsl
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_okhsl()
  }
}

#[cfg(feature = "space-hsi")]
impl<S> From<Hsi<S>> for Okhsl
where
  S: RgbSpec,
{
  fn from(hsi: Hsi<S>) -> Self {
    hsi.to_okhsl()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Okhsl
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_okhsl()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Okhsl
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_okhsl()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Okhsl
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_okhsl()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Okhsl {
  fn from(lab: Lab) -> Self {
    lab.to_okhsl()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Okhsl {
  fn from(lch: Lch) -> Self {
    lch.to_okhsl()
  }
}

#[cfg(feature = "space-lchuv")]
impl From<Lchuv> for Okhsl {
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_okhsl()
  }
}

impl From<Lms> for Okhsl {
  fn from(lms: Lms) -> Self {
    lms.to_okhsl()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Okhsl {
  fn from(luv: Luv) -> Self {
    luv.to_okhsl()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Okhsl {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_okhsl()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Okhsl {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_okhsl()
  }
}

impl From<Oklab> for Okhsl {
  fn from(oklab: Oklab) -> Self {
    oklab.to_okhsl()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Okhsl {
  fn from(oklch: Oklch) -> Self {
    oklch.to_okhsl()
  }
}

impl<S> From<Rgb<S>> for Okhsl
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_okhsl()
  }
}

#[cfg(feature = "space-xyy")]
impl From<Xyy> for Okhsl {
  fn from(xyy: Xyy) -> Self {
    xyy.to_okhsl()
  }
}

impl From<Xyz> for Okhsl {
  fn from(xyz: Xyz) -> Self {
    xyz.to_okhsl()
  }
}

impl<T> Mul<T> for Okhsl
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Okhsl
where
  T: Into<Okhsl> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.h == other.h && self.s == other.s && self.l == other.l
  }
}

impl<T> Sub<T> for Okhsl
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Okhsl {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Okhsl {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::space::perceptual::oklab::toe;

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_okhsl_values_via_rgb() {
      let a = Okhsl::new(0.0, 50.0, 25.0);
      let b = Okhsl::new(0.0, 50.0, 25.0);
      let result = a + b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod components {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_components_as_array() {
      let okhsl = Okhsl::new(180.0, 50.0, 75.0);
      let [h, s, l] = okhsl.components();

      assert_eq!(h, 0.5);
      assert_eq!(s, 0.5);
      assert_eq!(l, 0.75);
    }
  }

  mod decrement_h {
    use super::*;

    #[test]
    fn it_subtracts_from_h() {
      let mut okhsl = Okhsl::new(180.0, 50.0, 50.0);
      okhsl.decrement_h(0.25);

      assert!((okhsl.h() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut okhsl = Okhsl::new(36.0, 50.0, 50.0);
      okhsl.decrement_h(0.2);

      assert!((okhsl.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_subtracts_degrees_from_hue() {
      let mut okhsl = Okhsl::new(180.0, 50.0, 50.0);
      okhsl.decrement_hue(90.0);

      assert!((okhsl.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod decrement_l {
    use super::*;

    #[test]
    fn it_subtracts_from_l() {
      let mut okhsl = Okhsl::new(0.0, 50.0, 50.0);
      okhsl.decrement_l(0.2);

      assert!((okhsl.l() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_lightness {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_lightness() {
      let mut okhsl = Okhsl::new(0.0, 50.0, 50.0);
      okhsl.decrement_lightness(20.0);

      assert!((okhsl.lightness() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_s {
    use super::*;

    #[test]
    fn it_subtracts_from_s() {
      let mut okhsl = Okhsl::new(0.0, 50.0, 50.0);
      okhsl.decrement_s(0.2);

      assert!((okhsl.s() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_saturation {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_saturation() {
      let mut okhsl = Okhsl::new(0.0, 50.0, 50.0);
      okhsl.decrement_saturation(20.0);

      assert!((okhsl.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let okhsl = Okhsl::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", okhsl), "Okhsl(120.00°, 50.00%, 75.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let okhsl = Okhsl::new(120.12345, 50.6789, 75.4321);

      assert_eq!(format!("{:.4}", okhsl), "Okhsl(120.1235°, 50.6789%, 75.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let okhsl = Okhsl::new(120.0, 50.0, 75.0).with_alpha(0.5);

      assert_eq!(format!("{}", okhsl), "Okhsl(120.00°, 50.00%, 75.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let okhsl = Okhsl::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", okhsl), "Okhsl(120.00°, 50.00%, 75.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_okhsl_values_via_rgb() {
      let a = Okhsl::new(0.0, 50.0, 50.0);
      let b = Okhsl::new(0.0, 50.0, 50.0);
      let result = a / b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod from_array {
    use super::*;

    #[test]
    fn it_creates_from_f64_array() {
      let okhsl = Okhsl::from([180.0, 50.0, 75.0]);

      assert!((okhsl.hue() - 180.0).abs() < 1e-10);
      assert!((okhsl.saturation() - 50.0).abs() < 1e-10);
      assert!((okhsl.lightness() - 75.0).abs() < 1e-10);
    }
  }

  mod from_oklab {
    use super::*;

    #[test]
    fn it_converts_from_oklab() {
      let oklab = Oklab::new(0.5, 0.0, 0.0);
      let okhsl = Okhsl::from(oklab);

      assert!((okhsl.l() - toe(0.5)).abs() < 1e-10);
      assert!(okhsl.s() < 1e-3);
    }

    #[test]
    fn it_converts_black() {
      let oklab = Oklab::new(0.0, 0.0, 0.0);
      let okhsl = Okhsl::from(oklab);

      assert!(okhsl.l().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let oklab = Oklab::new(1.0, 0.0, 0.0);
      let okhsl = Okhsl::from(oklab);

      assert!((okhsl.l() - 1.0).abs() < 1e-10);
      assert!(okhsl.s() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.5);
      let okhsl = Okhsl::from(oklab);

      assert!((okhsl.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let okhsl = Okhsl::from(rgb);

      assert!((okhsl.l() - 1.0).abs() < 1e-3);
      assert!(okhsl.s() < 1e-3);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let okhsl = Okhsl::from(rgb);

      assert!(okhsl.l().abs() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let okhsl = Okhsl::from(rgb);

      assert!((okhsl.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let okhsl = Okhsl::from(xyz);

      assert!(okhsl.h().is_finite());
      assert!(okhsl.s().is_finite());
      assert!(okhsl.l().is_finite());
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let okhsl = Okhsl::from(xyz);

      assert!((okhsl.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod h {
    use super::*;

    #[test]
    fn it_returns_normalized_hue() {
      let okhsl = Okhsl::new(180.0, 50.0, 50.0);

      assert!((okhsl.h() - 0.5).abs() < 1e-10);
    }
  }

  mod hue {
    use super::*;

    #[test]
    fn it_returns_hue_in_degrees() {
      let okhsl = Okhsl::new(180.0, 50.0, 50.0);

      assert!((okhsl.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod increment_h {
    use super::*;

    #[test]
    fn it_adds_to_h() {
      let mut okhsl = Okhsl::new(90.0, 50.0, 50.0);
      okhsl.increment_h(0.25);

      assert!((okhsl.h() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_above_one() {
      let mut okhsl = Okhsl::new(324.0, 50.0, 50.0);
      okhsl.increment_h(0.2);

      assert!((okhsl.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_adds_degrees_to_hue() {
      let mut okhsl = Okhsl::new(90.0, 50.0, 50.0);
      okhsl.increment_hue(90.0);

      assert!((okhsl.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod increment_l {
    use super::*;

    #[test]
    fn it_adds_to_l() {
      let mut okhsl = Okhsl::new(0.0, 50.0, 25.0);
      okhsl.increment_l(0.25);

      assert!((okhsl.l() - 0.5).abs() < 1e-10);
    }
  }

  mod increment_lightness {
    use super::*;

    #[test]
    fn it_adds_percentage_to_lightness() {
      let mut okhsl = Okhsl::new(0.0, 50.0, 25.0);
      okhsl.increment_lightness(25.0);

      assert!((okhsl.lightness() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_s {
    use super::*;

    #[test]
    fn it_adds_to_s() {
      let mut okhsl = Okhsl::new(0.0, 25.0, 50.0);
      okhsl.increment_s(0.25);

      assert!((okhsl.s() - 0.5).abs() < 1e-10);
    }
  }

  mod increment_saturation {
    use super::*;

    #[test]
    fn it_adds_percentage_to_saturation() {
      let mut okhsl = Okhsl::new(0.0, 25.0, 50.0);
      okhsl.increment_saturation(25.0);

      assert!((okhsl.saturation() - 50.0).abs() < 1e-10);
    }
  }

  mod l {
    use super::*;

    #[test]
    fn it_returns_normalized_lightness() {
      let okhsl = Okhsl::new(0.0, 50.0, 75.0);

      assert!((okhsl.l() - 0.75).abs() < 1e-10);
    }
  }

  mod lightness {
    use super::*;

    #[test]
    fn it_returns_lightness_as_percentage() {
      let okhsl = Okhsl::new(0.0, 50.0, 75.0);

      assert!((okhsl.lightness() - 75.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_okhsl_values_via_rgb() {
      let a = Okhsl::new(0.0, 50.0, 50.0);
      let b = Okhsl::new(0.0, 50.0, 50.0);
      let result = a * b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod new {
    use super::*;

    #[test]
    fn it_creates_with_default_alpha() {
      let okhsl = Okhsl::new(180.0, 50.0, 75.0);

      assert!((okhsl.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let okhsl = Okhsl::new(180.0, 50.0, 75.0);

      assert_eq!(okhsl.context().illuminant().name(), "D65");
    }

    #[test]
    fn it_normalizes_hue_to_zero_one() {
      let okhsl = Okhsl::new(450.0, 50.0, 50.0);

      assert!((okhsl.h() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      let okhsl = Okhsl::new(-90.0, 50.0, 50.0);

      assert!((okhsl.h() - 0.75).abs() < 1e-10);
    }
  }

  mod new_const {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_hue_to_0_1() {
      const OKHSL: Okhsl = Okhsl::new_const(270.0, 50.0, 50.0);

      assert_eq!(OKHSL.h(), 0.75);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      const OKHSL: Okhsl = Okhsl::new_const(-90.0, 50.0, 50.0);

      assert_eq!(OKHSL.h(), 0.75);
    }

    #[test]
    fn it_normalizes_saturation_to_0_1() {
      const OKHSL: Okhsl = Okhsl::new_const(0.0, 75.0, 50.0);

      assert_eq!(OKHSL.s(), 0.75);
    }

    #[test]
    fn it_normalizes_lightness_to_0_1() {
      const OKHSL: Okhsl = Okhsl::new_const(0.0, 50.0, 75.0);

      assert_eq!(OKHSL.l(), 0.75);
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Okhsl::new(180.0, 50.0, 50.0);
      let b = Okhsl::new(180.0, 50.0, 50.0);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Okhsl::new(180.0, 50.0, 50.0);
      let b = Okhsl::new(180.0, 50.0, 60.0);

      assert!(a != b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Okhsl::new(180.0, 50.0, 50.0).with_alpha(0.5);
      let b = Okhsl::new(180.0, 50.0, 50.0);

      assert!(a != b);
    }
  }

  mod s {
    use super::*;

    #[test]
    fn it_returns_normalized_saturation() {
      let okhsl = Okhsl::new(0.0, 75.0, 50.0);

      assert!((okhsl.s() - 0.75).abs() < 1e-10);
    }
  }

  mod saturation {
    use super::*;

    #[test]
    fn it_returns_saturation_as_percentage() {
      let okhsl = Okhsl::new(0.0, 75.0, 50.0);

      assert!((okhsl.saturation() - 75.0).abs() < 1e-10);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_multiplies_h_by_factor() {
      let mut okhsl = Okhsl::new(90.0, 50.0, 50.0);
      okhsl.scale_h(2.0);

      assert!((okhsl.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_l {
    use super::*;

    #[test]
    fn it_multiplies_l_by_factor() {
      let mut okhsl = Okhsl::new(0.0, 50.0, 25.0);
      okhsl.scale_l(2.0);

      assert!((okhsl.l() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_s {
    use super::*;

    #[test]
    fn it_multiplies_s_by_factor() {
      let mut okhsl = Okhsl::new(0.0, 25.0, 50.0);
      okhsl.scale_s(2.0);

      assert!((okhsl.s() - 0.5).abs() < 1e-10);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_okhsl_values_via_rgb() {
      let a = Okhsl::new(0.0, 50.0, 50.0);
      let b = Okhsl::new(0.0, 25.0, 25.0);
      let result = a - b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod to_oklab {
    use super::*;

    #[test]
    fn it_converts_achromatic_gray() {
      let okhsl = Okhsl::new(0.0, 0.0, 50.0);
      let oklab = okhsl.to_oklab();

      assert!((oklab.l() - toe_inv(0.5)).abs() < 1e-10);
      assert!(oklab.a().abs() < 1e-10);
      assert!(oklab.b().abs() < 1e-10);
    }

    #[test]
    fn it_converts_black() {
      let okhsl = Okhsl::new(0.0, 100.0, 0.0);
      let oklab = okhsl.to_oklab();

      assert!(oklab.l().abs() < 1e-10);
      assert!(oklab.a().abs() < 1e-10);
      assert!(oklab.b().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let okhsl = Okhsl::new(0.0, 0.0, 100.0);
      let oklab = okhsl.to_oklab();

      assert!((oklab.l() - 1.0).abs() < 1e-10);
      assert!(oklab.a().abs() < 1e-10);
      assert!(oklab.b().abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_through_oklab() {
      let original = Okhsl::new(210.0, 80.0, 50.0);
      let roundtrip = Okhsl::from(original.to_oklab());

      assert!((original.h() - roundtrip.h()).abs() < 1e-10);
      assert!((original.s() - roundtrip.s()).abs() < 1e-10);
      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let okhsl = Okhsl::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let oklab = okhsl.to_oklab();

      assert!((oklab.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let okhsl = Okhsl::new(0.0, 50.0, 50.0);
      let rgb = okhsl.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_converts_black() {
      let okhsl = Okhsl::new(0.0, 0.0, 0.0);
      let rgb = okhsl.to_rgb::<Srgb>();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_white() {
      let okhsl = Okhsl::new(0.0, 0.0, 100.0);
      let rgb = okhsl.to_rgb::<Srgb>();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_preserves_alpha() {
      let okhsl = Okhsl::new(120.0, 50.0, 50.0).with_alpha(0.7);
      let rgb = okhsl.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let okhsl = Okhsl::new(120.0, 50.0, 50.0);
      let xyz = okhsl.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_preserves_alpha() {
      let okhsl = Okhsl::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let xyz = okhsl.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let okhsl = Okhsl::try_from("#FF5733").unwrap();

      assert!(okhsl.l() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Okhsl::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let okhsl = Okhsl::new(180.0, 50.0, 50.0);
      let result = okhsl.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let okhsl = Okhsl::new(180.0, 50.0, 50.0);
      let context = ColorimetricContext::default();
      let result = okhsl.with_context(context);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhsl_with_new_h() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_h(0.75);

      assert_eq!(result.h(), 0.75);
      assert_eq!(result.s(), okhsl.s());
      assert_eq!(result.l(), okhsl.l());
    }
  }

  mod with_h_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_decremented_h() {
      let okhsl = Okhsl::new(180.0, 50.0, 50.0);
      let result = okhsl.with_h_decremented_by(0.25);

      assert!((result.h() - 0.25).abs() < 1e-10);
      assert!((okhsl.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_incremented_h() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_h_incremented_by(0.25);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_scaled_h() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_new_hue_in_degrees() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
      assert_eq!(result.s(), okhsl.s());
      assert_eq!(result.l(), okhsl.l());
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_decremented_hue() {
      let okhsl = Okhsl::new(180.0, 50.0, 50.0);
      let result = okhsl.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_incremented_hue() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_hue_incremented_by(90.0);

      assert!((result.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhsl_with_new_l() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_l(0.75);

      assert_eq!(result.l(), 0.75);
      assert_eq!(result.h(), okhsl.h());
      assert_eq!(result.s(), okhsl.s());
    }
  }

  mod with_l_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_decremented_l() {
      let okhsl = Okhsl::new(0.0, 50.0, 50.0);
      let result = okhsl.with_l_decremented_by(0.2);

      assert!((result.l() - 0.3).abs() < 1e-10);
    }
  }

  mod with_l_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_incremented_l() {
      let okhsl = Okhsl::new(0.0, 50.0, 25.0);
      let result = okhsl.with_l_incremented_by(0.25);

      assert!((result.l() - 0.5).abs() < 1e-10);
    }
  }

  mod with_l_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_scaled_l() {
      let okhsl = Okhsl::new(0.0, 50.0, 25.0);
      let result = okhsl.with_l_scaled_by(2.0);

      assert!((result.l() - 0.5).abs() < 1e-10);
    }
  }

  mod with_lightness {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_new_lightness_in_percent() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_lightness(75.0);

      assert!((result.lightness() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), okhsl.h());
      assert_eq!(result.s(), okhsl.s());
    }
  }

  mod with_lightness_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_decremented_lightness() {
      let okhsl = Okhsl::new(0.0, 50.0, 50.0);
      let result = okhsl.with_lightness_decremented_by(20.0);

      assert!((result.lightness() - 30.0).abs() < 1e-10);
    }
  }

  mod with_lightness_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_incremented_lightness() {
      let okhsl = Okhsl::new(0.0, 50.0, 25.0);
      let result = okhsl.with_lightness_incremented_by(25.0);

      assert!((result.lightness() - 50.0).abs() < 1e-10);
    }
  }

  mod with_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhsl_with_new_s() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_s(0.75);

      assert_eq!(result.s(), 0.75);
      assert_eq!(result.h(), okhsl.h());
      assert_eq!(result.l(), okhsl.l());
    }
  }

  mod with_s_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_decremented_s() {
      let okhsl = Okhsl::new(0.0, 50.0, 50.0);
      let result = okhsl.with_s_decremented_by(0.2);

      assert!((result.s() - 0.3).abs() < 1e-10);
    }
  }

  mod with_s_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_incremented_s() {
      let okhsl = Okhsl::new(0.0, 25.0, 50.0);
      let result = okhsl.with_s_incremented_by(0.25);

      assert!((result.s() - 0.5).abs() < 1e-10);
    }
  }

  mod with_s_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_scaled_s() {
      let okhsl = Okhsl::new(0.0, 25.0, 50.0);
      let result = okhsl.with_s_scaled_by(2.0);

      assert!((result.s() - 0.5).abs() < 1e-10);
    }
  }

  mod with_saturation {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_new_saturation_in_percent() {
      let okhsl = Okhsl::new(90.0, 50.0, 50.0);
      let result = okhsl.with_saturation(75.0);

      assert!((result.saturation() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), okhsl.h());
      assert_eq!(result.l(), okhsl.l());
    }
  }

  mod with_saturation_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_decremented_saturation() {
      let okhsl = Okhsl::new(0.0, 50.0, 50.0);
      let result = okhsl.with_saturation_decremented_by(20.0);

      assert!((result.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod with_saturation_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsl_with_incremented_saturation() {
      let okhsl = Okhsl::new(0.0, 25.0, 50.0);
      let result = okhsl.with_saturation_incremented_by(25.0);

      assert!((result.saturation() - 50.0).abs() < 1e-10);
    }
  }
}
