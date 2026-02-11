use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, Div, Mul, Sub},
};

#[cfg(feature = "space-cmy")]
use crate::space::Cmy;
#[cfg(feature = "space-cmyk")]
use crate::space::Cmyk;
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
#[cfg(feature = "space-luv")]
use crate::space::Luv;
#[cfg(feature = "space-okhsl")]
use crate::space::Okhsl;
#[cfg(feature = "space-oklch")]
use crate::space::Oklch;
use crate::{
  ColorimetricContext, Illuminant, Observer,
  component::Component,
  space::{ColorSpace, Lms, Okhsv, Oklab, Rgb, RgbSpec, Srgb, Xyz},
};

/// Okhwb perceptual color space (HWB model in the Oklab perceptual framework).
///
/// A perceptually uniform HWB-like color space derived from Oklab via Okhsv. H represents
/// hue stored internally as a 0.0-1.0 fraction (0-360°), W represents whiteness
/// (0.0-1.0), and B represents blackness (0.0-1.0). Designed for intuitive color
/// manipulation with perceptual uniformity, using sRGB gamut boundaries for mapping.
#[derive(Clone, Copy, Debug)]
pub struct Okhwb {
  alpha: Component,
  b: Component,
  context: ColorimetricContext,
  h: Component,
  w: Component,
}

impl Okhwb {
  /// The default viewing context for Okhwb (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new Okhwb color from hue (0-360°), whiteness (0-100%), and blackness (0-100%).
  pub fn new(h: impl Into<Component>, w: impl Into<Component>, b: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      b: b.into() / 100.0,
      context: Self::DEFAULT_CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      w: w.into() / 100.0,
    }
  }

  /// Creates a new Okhwb color in a const context from hue (0-360°), whiteness (0-100%), and blackness (0-100%).
  pub const fn new_const(h: f64, w: f64, b: f64) -> Self {
    let r = (h / 360.0) % 1.0;

    Self {
      alpha: Component::new_const(1.0),
      b: Component::new_const(b / 100.0),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new_const(if r < 0.0 { r + 1.0 } else { r }),
      w: Component::new_const(w / 100.0),
    }
  }

  /// Returns the normalized blackness component (0.0-1.0).
  pub fn b(&self) -> f64 {
    self.b.0
  }

  /// Returns the blackness as a percentage (0-100%).
  pub fn blackness(&self) -> f64 {
    self.b.0 * 100.0
  }

  /// Returns the [H, W, B] components as normalized values (all 0.0-1.0).
  pub fn components(&self) -> [f64; 3] {
    [self.h.0, self.w.0, self.b.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases the normalized blackness by the given amount.
  pub fn decrement_b(&mut self, amount: impl Into<Component>) {
    self.b -= amount.into();
  }

  /// Decreases the blackness by the given amount in percentage points.
  pub fn decrement_blackness(&mut self, amount: impl Into<Component>) {
    self.decrement_b(amount.into() / 100.0)
  }

  /// Decreases the normalized hue by the given amount (wraps around 0.0-1.0).
  pub fn decrement_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 - amount.into().0).rem_euclid(1.0));
  }

  /// Decreases the hue by the given amount in degrees (wraps around 0-360°).
  pub fn decrement_hue(&mut self, amount: impl Into<Component>) {
    self.decrement_h(amount.into() / 360.0)
  }

  /// Decreases the normalized whiteness by the given amount.
  pub fn decrement_w(&mut self, amount: impl Into<Component>) {
    self.w -= amount.into();
  }

  /// Decreases the whiteness by the given amount in percentage points.
  pub fn decrement_whiteness(&mut self, amount: impl Into<Component>) {
    self.decrement_w(amount.into() / 100.0)
  }

  /// Returns the normalized hue component (0.0-1.0).
  pub fn h(&self) -> f64 {
    self.h.0
  }

  /// Returns the hue in degrees (0-360°).
  pub fn hue(&self) -> f64 {
    self.h.0 * 360.0
  }

  /// Increases the normalized blackness by the given amount.
  pub fn increment_b(&mut self, amount: impl Into<Component>) {
    self.b += amount.into();
  }

  /// Increases the blackness by the given amount in percentage points.
  pub fn increment_blackness(&mut self, amount: impl Into<Component>) {
    self.increment_b(amount.into() / 100.0)
  }

  /// Increases the normalized hue by the given amount (wraps around 0.0-1.0).
  pub fn increment_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 + amount.into().0).rem_euclid(1.0));
  }

  /// Increases the hue by the given amount in degrees (wraps around 0-360°).
  pub fn increment_hue(&mut self, amount: impl Into<Component>) {
    self.increment_h(amount.into() / 360.0)
  }

  /// Increases the normalized whiteness by the given amount.
  pub fn increment_w(&mut self, amount: impl Into<Component>) {
    self.w += amount.into();
  }

  /// Increases the whiteness by the given amount in percentage points.
  pub fn increment_whiteness(&mut self, amount: impl Into<Component>) {
    self.increment_w(amount.into() / 100.0)
  }

  /// Scales the normalized blackness by the given factor.
  pub fn scale_b(&mut self, factor: impl Into<Component>) {
    self.b *= factor.into();
  }

  /// Alias for [`Self::scale_b`].
  pub fn scale_blackness(&mut self, factor: impl Into<Component>) {
    self.scale_b(factor)
  }

  /// Scales the normalized hue by the given factor (wraps around 0.0-1.0).
  pub fn scale_h(&mut self, factor: impl Into<Component>) {
    self.h = Component::new((self.h.0 * factor.into().0).rem_euclid(1.0));
  }

  /// Alias for [`Self::scale_h`].
  pub fn scale_hue(&mut self, factor: impl Into<Component>) {
    self.scale_h(factor)
  }

  /// Scales the normalized whiteness by the given factor.
  pub fn scale_w(&mut self, factor: impl Into<Component>) {
    self.w *= factor.into();
  }

  /// Alias for [`Self::scale_w`].
  pub fn scale_whiteness(&mut self, factor: impl Into<Component>) {
    self.scale_w(factor)
  }

  /// Sets all three components from normalized values.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_h(components[0].clone());
    self.set_w(components[1].clone());
    self.set_b(components[2].clone());
  }

  /// Sets the normalized blackness component (0.0-1.0).
  pub fn set_b(&mut self, b: impl Into<Component>) {
    self.b = b.into();
  }

  /// Sets the blackness from a percentage value (0-100%).
  pub fn set_blackness(&mut self, blackness: impl Into<Component>) {
    self.b = blackness.into() / 100.0;
  }

  /// Sets the normalized hue component (0.0-1.0).
  pub fn set_h(&mut self, h: impl Into<Component>) {
    self.h = h.into();
  }

  /// Sets the hue from a value in degrees (0-360°).
  pub fn set_hue(&mut self, hue: impl Into<Component>) {
    self.h = Component::new((hue.into().0 / 360.0).rem_euclid(1.0));
  }

  /// Sets the normalized whiteness component (0.0-1.0).
  pub fn set_w(&mut self, w: impl Into<Component>) {
    self.w = w.into();
  }

  /// Sets the whiteness from a percentage value (0-100%).
  pub fn set_whiteness(&mut self, whiteness: impl Into<Component>) {
    self.w = whiteness.into() / 100.0;
  }

  /// Converts to the Okhsv perceptual color space.
  ///
  /// Uses the standard HWB-to-HSV reparameterization:
  /// V = 1 - B, S = 1 - W/V (or 0 when V is zero).
  pub fn to_okhsv(&self) -> Okhsv {
    let [h, w, b] = self.components();

    let v = 1.0 - b;
    let s = if v == 0.0 { 0.0 } else { 1.0 - (w / v) };

    Okhsv::new(h * 360.0, s * 100.0, v * 100.0).with_alpha(self.alpha)
  }

  /// Converts to the Oklab perceptual color space.
  pub fn to_oklab(&self) -> Oklab {
    self.to_okhsv().to_oklab()
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

  /// Returns the normalized whiteness component (0.0-1.0).
  pub fn w(&self) -> f64 {
    self.w.0
  }

  /// Returns the whiteness as a percentage (0-100%).
  pub fn whiteness(&self) -> f64 {
    self.w.0 * 100.0
  }

  /// Returns this color with a different viewing context (without adaptation).
  pub fn with_context(&self, context: ColorimetricContext) -> Self {
    Self {
      context,
      ..*self
    }
  }

  /// Returns a new color with the given normalized blackness value.
  pub fn with_b(&self, b: impl Into<Component>) -> Self {
    Self {
      b: b.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized blackness decreased by the given amount.
  pub fn with_b_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.decrement_b(amount);
    okhwb
  }

  /// Returns a new color with normalized blackness increased by the given amount.
  pub fn with_b_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.increment_b(amount);
    okhwb
  }

  /// Returns a new color with normalized blackness scaled by the given factor.
  pub fn with_b_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.scale_b(factor);
    okhwb
  }

  /// Returns a new color with the given blackness in percentage (0-100%).
  pub fn with_blackness(&self, blackness: impl Into<Component>) -> Self {
    Self {
      b: blackness.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with blackness decreased by the given percentage points.
  pub fn with_blackness_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.decrement_blackness(amount);
    okhwb
  }

  /// Returns a new color with blackness increased by the given percentage points.
  pub fn with_blackness_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.increment_blackness(amount);
    okhwb
  }

  /// Alias for [`Self::with_b_scaled_by`].
  pub fn with_blackness_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_b_scaled_by(factor)
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
    let mut okhwb = *self;
    okhwb.decrement_h(amount);
    okhwb
  }

  /// Returns a new color with the normalized hue increased by the given amount.
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.increment_h(amount);
    okhwb
  }

  /// Returns a new color with the normalized hue scaled by the given factor.
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.scale_h(factor);
    okhwb
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
    let mut okhwb = *self;
    okhwb.decrement_hue(amount);
    okhwb
  }

  /// Returns a new color with the hue increased by the given degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.increment_hue(amount);
    okhwb
  }

  /// Alias for [`Self::with_h_scaled_by`].
  pub fn with_hue_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_h_scaled_by(factor)
  }

  /// Returns a new color with the given normalized whiteness value.
  pub fn with_w(&self, w: impl Into<Component>) -> Self {
    Self {
      w: w.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized whiteness decreased by the given amount.
  pub fn with_w_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.decrement_w(amount);
    okhwb
  }

  /// Returns a new color with normalized whiteness increased by the given amount.
  pub fn with_w_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.increment_w(amount);
    okhwb
  }

  /// Returns a new color with normalized whiteness scaled by the given factor.
  pub fn with_w_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.scale_w(factor);
    okhwb
  }

  /// Returns a new color with the given whiteness in percentage (0-100%).
  pub fn with_whiteness(&self, whiteness: impl Into<Component>) -> Self {
    Self {
      w: whiteness.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with whiteness decreased by the given percentage points.
  pub fn with_whiteness_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.decrement_whiteness(amount);
    okhwb
  }

  /// Returns a new color with whiteness increased by the given percentage points.
  pub fn with_whiteness_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhwb = *self;
    okhwb.increment_whiteness(amount);
    okhwb
  }

  /// Alias for [`Self::with_w_scaled_by`].
  pub fn with_whiteness_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_w_scaled_by(factor)
  }
}

impl<T> Add<T> for Okhwb
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Okhwb {
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

impl Display for Okhwb {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Okhwb({:.precision$}°, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.hue(),
        self.whiteness(),
        self.blackness(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "Okhwb({:.precision$}°, {:.precision$}%, {:.precision$}%)",
        self.hue(),
        self.whiteness(),
        self.blackness()
      )
    }
  }
}

impl<T> Div<T> for Okhwb
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Okhwb
where
  T: Into<Component>,
{
  fn from([h, w, b]: [T; 3]) -> Self {
    Self::new(h, w, b)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Okhwb
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_okhwb()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Okhwb
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_okhwb()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Okhwb
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_okhwb()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Okhwb
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_okhwb()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Okhwb
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_okhwb()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Okhwb {
  fn from(lab: Lab) -> Self {
    lab.to_okhwb()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Okhwb {
  fn from(lch: Lch) -> Self {
    lch.to_okhwb()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Okhwb {
  fn from(luv: Luv) -> Self {
    luv.to_okhwb()
  }
}

impl From<Lms> for Okhwb {
  fn from(lms: Lms) -> Self {
    lms.to_okhwb()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Okhwb {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_okhwb()
  }
}

impl From<Okhsv> for Okhwb {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_okhwb()
  }
}

impl From<Oklab> for Okhwb {
  fn from(oklab: Oklab) -> Self {
    oklab.to_okhwb()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Okhwb {
  fn from(oklch: Oklch) -> Self {
    oklch.to_okhwb()
  }
}

impl<S> From<Rgb<S>> for Okhwb
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_okhwb()
  }
}

impl From<Xyz> for Okhwb {
  fn from(xyz: Xyz) -> Self {
    xyz.to_okhwb()
  }
}

impl<T> Mul<T> for Okhwb
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Okhwb
where
  T: Into<Okhwb> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.h == other.h && self.w == other.w && self.b == other.b
  }
}

impl<T> Sub<T> for Okhwb
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Okhwb {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Okhwb {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_okhwb_values_via_rgb() {
      let a = Okhwb::new(0.0, 25.0, 25.0);
      let b = Okhwb::new(0.0, 25.0, 25.0);
      let result = a + b;

      assert!(result.h().is_finite());
      assert!(result.w().is_finite());
      assert!(result.b().is_finite());
    }
  }

  mod decrement_b {
    use super::*;

    #[test]
    fn it_subtracts_from_b() {
      let mut okhwb = Okhwb::new(0.0, 50.0, 50.0);
      okhwb.decrement_b(0.2);

      assert!((okhwb.b() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_blackness {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_blackness() {
      let mut okhwb = Okhwb::new(0.0, 50.0, 50.0);
      okhwb.decrement_blackness(20.0);

      assert!((okhwb.blackness() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_h {
    use super::*;

    #[test]
    fn it_subtracts_from_h() {
      let mut okhwb = Okhwb::new(180.0, 50.0, 50.0);
      okhwb.decrement_h(0.25);

      assert!((okhwb.h() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut okhwb = Okhwb::new(36.0, 50.0, 50.0);
      okhwb.decrement_h(0.2);

      assert!((okhwb.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_subtracts_degrees_from_hue() {
      let mut okhwb = Okhwb::new(180.0, 50.0, 50.0);
      okhwb.decrement_hue(90.0);

      assert!((okhwb.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod decrement_w {
    use super::*;

    #[test]
    fn it_subtracts_from_w() {
      let mut okhwb = Okhwb::new(0.0, 50.0, 50.0);
      okhwb.decrement_w(0.2);

      assert!((okhwb.w() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_whiteness {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_whiteness() {
      let mut okhwb = Okhwb::new(0.0, 50.0, 50.0);
      okhwb.decrement_whiteness(20.0);

      assert!((okhwb.whiteness() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let okhwb = Okhwb::new(120.0, 25.0, 50.0);

      assert_eq!(format!("{}", okhwb), "Okhwb(120.00°, 25.00%, 50.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let okhwb = Okhwb::new(120.12345, 25.6789, 50.4321);

      assert_eq!(format!("{:.4}", okhwb), "Okhwb(120.1235°, 25.6789%, 50.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let okhwb = Okhwb::new(120.0, 25.0, 50.0).with_alpha(0.5);

      assert_eq!(format!("{}", okhwb), "Okhwb(120.00°, 25.00%, 50.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let okhwb = Okhwb::new(120.0, 25.0, 50.0);

      assert_eq!(format!("{}", okhwb), "Okhwb(120.00°, 25.00%, 50.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_okhwb_values_via_rgb() {
      let a = Okhwb::new(0.0, 25.0, 25.0);
      let b = Okhwb::new(0.0, 25.0, 25.0);
      let result = a / b;

      assert!(result.h().is_finite());
      assert!(result.w().is_finite());
      assert!(result.b().is_finite());
    }
  }

  mod from_okhsv {
    use super::*;

    #[test]
    fn it_converts_from_okhsv() {
      let okhsv = Okhsv::new(210.0, 80.0, 60.0);
      let okhwb = Okhwb::from(okhsv);

      assert!((okhwb.hue() - 210.0).abs() < 1e-10);
      assert!((okhwb.w() - (1.0 - 0.8) * 0.6).abs() < 1e-10);
      assert!((okhwb.b() - (1.0 - 0.6)).abs() < 1e-10);
    }

    #[test]
    fn it_converts_black() {
      let okhsv = Okhsv::new(0.0, 0.0, 0.0);
      let okhwb = Okhwb::from(okhsv);

      assert!((okhwb.blackness() - 100.0).abs() < 1e-10);
      assert!(okhwb.whiteness().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let okhsv = Okhsv::new(0.0, 0.0, 100.0);
      let okhwb = Okhwb::from(okhsv);

      assert!((okhwb.whiteness() - 100.0).abs() < 1e-10);
      assert!(okhwb.blackness().abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let okhsv = Okhsv::new(120.0, 50.0, 50.0).with_alpha(0.5);
      let okhwb = Okhwb::from(okhsv);

      assert!((okhwb.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_oklab {
    use super::*;

    #[test]
    fn it_converts_from_oklab() {
      let oklab = Oklab::new(0.5, 0.0, 0.0);
      let okhwb = Okhwb::from(oklab);

      assert!(okhwb.h().is_finite());
      assert!(okhwb.w().is_finite());
      assert!(okhwb.b().is_finite());
    }

    #[test]
    fn it_converts_black() {
      let oklab = Oklab::new(0.0, 0.0, 0.0);
      let okhwb = Okhwb::from(oklab);

      assert!((okhwb.blackness() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let oklab = Oklab::new(1.0, 0.0, 0.0);
      let okhwb = Okhwb::from(oklab);

      assert!((okhwb.whiteness() - 100.0).abs() < 1e-3);
      assert!(okhwb.blackness().abs() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.5);
      let okhwb = Okhwb::from(oklab);

      assert!((okhwb.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let okhwb = Okhwb::from(rgb);

      assert!((okhwb.whiteness() - 100.0).abs() < 1.0);
      assert!(okhwb.blackness().abs() < 1.0);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let okhwb = Okhwb::from(rgb);

      assert!((okhwb.blackness() - 100.0).abs() < 1.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let okhwb = Okhwb::from(rgb);

      assert!((okhwb.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let okhwb = Okhwb::from(xyz);

      assert!(okhwb.h().is_finite());
      assert!(okhwb.w().is_finite());
      assert!(okhwb.b().is_finite());
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let okhwb = Okhwb::from(xyz);

      assert!((okhwb.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod increment_b {
    use super::*;

    #[test]
    fn it_adds_to_b() {
      let mut okhwb = Okhwb::new(0.0, 50.0, 25.0);
      okhwb.increment_b(0.25);

      assert!((okhwb.b() - 0.5).abs() < 1e-10);
    }
  }

  mod increment_blackness {
    use super::*;

    #[test]
    fn it_adds_percentage_to_blackness() {
      let mut okhwb = Okhwb::new(0.0, 50.0, 25.0);
      okhwb.increment_blackness(25.0);

      assert!((okhwb.blackness() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_h {
    use super::*;

    #[test]
    fn it_adds_to_h() {
      let mut okhwb = Okhwb::new(90.0, 50.0, 50.0);
      okhwb.increment_h(0.25);

      assert!((okhwb.h() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_above_one() {
      let mut okhwb = Okhwb::new(324.0, 50.0, 50.0);
      okhwb.increment_h(0.2);

      assert!((okhwb.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_adds_degrees_to_hue() {
      let mut okhwb = Okhwb::new(90.0, 50.0, 50.0);
      okhwb.increment_hue(90.0);

      assert!((okhwb.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod increment_w {
    use super::*;

    #[test]
    fn it_adds_to_w() {
      let mut okhwb = Okhwb::new(0.0, 25.0, 50.0);
      okhwb.increment_w(0.25);

      assert!((okhwb.w() - 0.5).abs() < 1e-10);
    }
  }

  mod increment_whiteness {
    use super::*;

    #[test]
    fn it_adds_percentage_to_whiteness() {
      let mut okhwb = Okhwb::new(0.0, 25.0, 50.0);
      okhwb.increment_whiteness(25.0);

      assert!((okhwb.whiteness() - 50.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_okhwb_values_via_rgb() {
      let a = Okhwb::new(0.0, 25.0, 25.0);
      let b = Okhwb::new(0.0, 25.0, 25.0);
      let result = a * b;

      assert!(result.h().is_finite());
      assert!(result.w().is_finite());
      assert!(result.b().is_finite());
    }
  }

  mod new {
    use super::*;

    #[test]
    fn it_creates_with_default_alpha() {
      let okhwb = Okhwb::new(180.0, 25.0, 50.0);

      assert!((okhwb.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let okhwb = Okhwb::new(180.0, 25.0, 50.0);

      assert_eq!(okhwb.context().illuminant().name(), "D65");
    }

    #[test]
    fn it_normalizes_hue_to_zero_one() {
      let okhwb = Okhwb::new(450.0, 50.0, 50.0);

      assert!((okhwb.h() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      let okhwb = Okhwb::new(-90.0, 50.0, 50.0);

      assert!((okhwb.h() - 0.75).abs() < 1e-10);
    }
  }

  mod new_const {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_hue_to_0_1() {
      const OKHWB: Okhwb = Okhwb::new_const(270.0, 25.0, 50.0);

      assert_eq!(OKHWB.h(), 0.75);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      const OKHWB: Okhwb = Okhwb::new_const(-90.0, 25.0, 50.0);

      assert_eq!(OKHWB.h(), 0.75);
    }

    #[test]
    fn it_normalizes_whiteness_to_0_1() {
      const OKHWB: Okhwb = Okhwb::new_const(0.0, 75.0, 50.0);

      assert_eq!(OKHWB.w(), 0.75);
    }

    #[test]
    fn it_normalizes_blackness_to_0_1() {
      const OKHWB: Okhwb = Okhwb::new_const(0.0, 50.0, 75.0);

      assert_eq!(OKHWB.b(), 0.75);
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Okhwb::new(180.0, 25.0, 50.0);
      let b = Okhwb::new(180.0, 25.0, 50.0);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Okhwb::new(180.0, 25.0, 50.0);
      let b = Okhwb::new(180.0, 25.0, 60.0);

      assert!(a != b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Okhwb::new(180.0, 25.0, 50.0).with_alpha(0.5);
      let b = Okhwb::new(180.0, 25.0, 50.0);

      assert!(a != b);
    }
  }

  mod scale_b {
    use super::*;

    #[test]
    fn it_multiplies_b_by_factor() {
      let mut okhwb = Okhwb::new(0.0, 50.0, 25.0);
      okhwb.scale_b(2.0);

      assert!((okhwb.b() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_multiplies_h_by_factor() {
      let mut okhwb = Okhwb::new(90.0, 50.0, 50.0);
      okhwb.scale_h(2.0);

      assert!((okhwb.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_w {
    use super::*;

    #[test]
    fn it_multiplies_w_by_factor() {
      let mut okhwb = Okhwb::new(0.0, 25.0, 50.0);
      okhwb.scale_w(2.0);

      assert!((okhwb.w() - 0.5).abs() < 1e-10);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_okhwb_values_via_rgb() {
      let a = Okhwb::new(0.0, 50.0, 25.0);
      let b = Okhwb::new(0.0, 25.0, 12.5);
      let result = a - b;

      assert!(result.h().is_finite());
      assert!(result.w().is_finite());
      assert!(result.b().is_finite());
    }
  }

  mod to_okhsv {
    use super::*;

    #[test]
    fn it_converts_to_okhsv() {
      let okhwb = Okhwb::new(210.0, 20.0, 40.0);
      let okhsv = okhwb.to_okhsv();

      assert!((okhsv.hue() - 210.0).abs() < 1e-10);
      assert!((okhsv.v() - 0.6).abs() < 1e-10);
      assert!((okhsv.s() - (1.0 - 0.2 / 0.6)).abs() < 1e-10);
    }

    #[test]
    fn it_converts_black() {
      let okhwb = Okhwb::new(0.0, 0.0, 100.0);
      let okhsv = okhwb.to_okhsv();

      assert!(okhsv.v().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let okhwb = Okhwb::new(0.0, 100.0, 0.0);
      let okhsv = okhwb.to_okhsv();

      assert!((okhsv.v() - 1.0).abs() < 1e-10);
      assert!(okhsv.s().abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_through_okhsv() {
      let original = Okhwb::new(210.0, 20.0, 40.0);
      let roundtrip = Okhwb::from(original.to_okhsv());

      assert!((original.h() - roundtrip.h()).abs() < 1e-10);
      assert!((original.w() - roundtrip.w()).abs() < 1e-10);
      assert!((original.b() - roundtrip.b()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let okhwb = Okhwb::new(120.0, 25.0, 25.0).with_alpha(0.3);
      let okhsv = okhwb.to_okhsv();

      assert!((okhsv.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_oklab {
    use super::*;

    #[test]
    fn it_converts_to_oklab() {
      let okhwb = Okhwb::new(120.0, 25.0, 25.0);
      let oklab = okhwb.to_oklab();

      assert!(oklab.l().is_finite());
      assert!(oklab.a().is_finite());
      assert!(oklab.b().is_finite());
    }

    #[test]
    fn it_preserves_alpha() {
      let okhwb = Okhwb::new(120.0, 25.0, 25.0).with_alpha(0.3);
      let oklab = okhwb.to_oklab();

      assert!((oklab.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let okhwb = Okhwb::new(0.0, 25.0, 25.0);
      let rgb = okhwb.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_converts_black() {
      let okhwb = Okhwb::new(0.0, 0.0, 100.0);
      let rgb = okhwb.to_rgb::<Srgb>();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_white() {
      let okhwb = Okhwb::new(0.0, 100.0, 0.0);
      let rgb = okhwb.to_rgb::<Srgb>();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_preserves_alpha() {
      let okhwb = Okhwb::new(120.0, 25.0, 25.0).with_alpha(0.7);
      let rgb = okhwb.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let okhwb = Okhwb::new(120.0, 25.0, 25.0);
      let xyz = okhwb.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_preserves_alpha() {
      let okhwb = Okhwb::new(120.0, 25.0, 25.0).with_alpha(0.3);
      let xyz = okhwb.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let okhwb = Okhwb::try_from("#FF5733").unwrap();

      assert!(okhwb.w() > 0.0 || okhwb.b() > 0.0 || okhwb.h() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Okhwb::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let okhwb = Okhwb::new(180.0, 25.0, 50.0);
      let result = okhwb.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_b {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhwb_with_new_b() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_b(0.75);

      assert_eq!(result.b(), 0.75);
      assert_eq!(result.h(), okhwb.h());
      assert_eq!(result.w(), okhwb.w());
    }
  }

  mod with_b_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_decremented_b() {
      let okhwb = Okhwb::new(0.0, 50.0, 50.0);
      let result = okhwb.with_b_decremented_by(0.2);

      assert!((result.b() - 0.3).abs() < 1e-10);
      assert!((okhwb.b() - 0.5).abs() < 1e-10);
    }
  }

  mod with_b_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_incremented_b() {
      let okhwb = Okhwb::new(0.0, 50.0, 25.0);
      let result = okhwb.with_b_incremented_by(0.25);

      assert!((result.b() - 0.5).abs() < 1e-10);
    }
  }

  mod with_b_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_scaled_b() {
      let okhwb = Okhwb::new(0.0, 50.0, 25.0);
      let result = okhwb.with_b_scaled_by(2.0);

      assert!((result.b() - 0.5).abs() < 1e-10);
    }
  }

  mod with_blackness {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_new_blackness_in_percent() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_blackness(75.0);

      assert!((result.blackness() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), okhwb.h());
      assert_eq!(result.w(), okhwb.w());
    }
  }

  mod with_blackness_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_decremented_blackness() {
      let okhwb = Okhwb::new(0.0, 50.0, 50.0);
      let result = okhwb.with_blackness_decremented_by(20.0);

      assert!((result.blackness() - 30.0).abs() < 1e-10);
    }
  }

  mod with_blackness_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_incremented_blackness() {
      let okhwb = Okhwb::new(0.0, 50.0, 25.0);
      let result = okhwb.with_blackness_incremented_by(25.0);

      assert!((result.blackness() - 50.0).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let okhwb = Okhwb::new(180.0, 25.0, 50.0);
      let context = ColorimetricContext::default();
      let result = okhwb.with_context(context);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhwb_with_new_h() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_h(0.75);

      assert_eq!(result.h(), 0.75);
      assert_eq!(result.w(), okhwb.w());
      assert_eq!(result.b(), okhwb.b());
    }
  }

  mod with_h_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_decremented_h() {
      let okhwb = Okhwb::new(180.0, 50.0, 50.0);
      let result = okhwb.with_h_decremented_by(0.25);

      assert!((result.h() - 0.25).abs() < 1e-10);
      assert!((okhwb.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_incremented_h() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_h_incremented_by(0.25);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_scaled_h() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_new_hue_in_degrees() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
      assert_eq!(result.w(), okhwb.w());
      assert_eq!(result.b(), okhwb.b());
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_decremented_hue() {
      let okhwb = Okhwb::new(180.0, 50.0, 50.0);
      let result = okhwb.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_incremented_hue() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_hue_incremented_by(90.0);

      assert!((result.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_w {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhwb_with_new_w() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_w(0.75);

      assert_eq!(result.w(), 0.75);
      assert_eq!(result.h(), okhwb.h());
      assert_eq!(result.b(), okhwb.b());
    }
  }

  mod with_w_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_decremented_w() {
      let okhwb = Okhwb::new(0.0, 50.0, 50.0);
      let result = okhwb.with_w_decremented_by(0.2);

      assert!((result.w() - 0.3).abs() < 1e-10);
      assert!((okhwb.w() - 0.5).abs() < 1e-10);
    }
  }

  mod with_w_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_incremented_w() {
      let okhwb = Okhwb::new(0.0, 25.0, 50.0);
      let result = okhwb.with_w_incremented_by(0.25);

      assert!((result.w() - 0.5).abs() < 1e-10);
    }
  }

  mod with_w_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_scaled_w() {
      let okhwb = Okhwb::new(0.0, 25.0, 50.0);
      let result = okhwb.with_w_scaled_by(2.0);

      assert!((result.w() - 0.5).abs() < 1e-10);
    }
  }

  mod with_whiteness {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_new_whiteness_in_percent() {
      let okhwb = Okhwb::new(90.0, 50.0, 50.0);
      let result = okhwb.with_whiteness(75.0);

      assert!((result.whiteness() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), okhwb.h());
      assert_eq!(result.b(), okhwb.b());
    }
  }

  mod with_whiteness_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_decremented_whiteness() {
      let okhwb = Okhwb::new(0.0, 50.0, 50.0);
      let result = okhwb.with_whiteness_decremented_by(20.0);

      assert!((result.whiteness() - 30.0).abs() < 1e-10);
    }
  }

  mod with_whiteness_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhwb_with_incremented_whiteness() {
      let okhwb = Okhwb::new(0.0, 25.0, 50.0);
      let result = okhwb.with_whiteness_incremented_by(25.0);

      assert!((result.whiteness() - 50.0).abs() < 1e-10);
    }
  }
}
