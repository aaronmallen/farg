use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, Div, Mul, Sub},
};

#[cfg(feature = "space-cmy")]
use crate::space::Cmy;
#[cfg(feature = "space-cmyk")]
use crate::space::Cmyk;
#[cfg(feature = "space-hsi")]
use crate::space::Hsi;
#[cfg(feature = "space-hsl")]
use crate::space::Hsl;
#[cfg(feature = "space-hsluv")]
use crate::space::Hsluv;
#[cfg(feature = "space-hsv")]
use crate::space::Hsv;
#[cfg(feature = "space-hwb")]
use crate::space::Hwb;
#[cfg(feature = "space-lab")]
use crate::space::Lab;
#[cfg(feature = "space-lch")]
use crate::space::Lch;
#[cfg(feature = "space-okhsl")]
use crate::space::Okhsl;
#[cfg(feature = "space-okhsv")]
use crate::space::Okhsv;
#[cfg(feature = "space-okhwb")]
use crate::space::Okhwb;
#[cfg(feature = "space-oklab")]
use crate::space::Oklab;
#[cfg(feature = "space-oklch")]
use crate::space::Oklch;
#[cfg(feature = "space-xyy")]
use crate::space::Xyy;
use crate::{
  ColorimetricContext, Illuminant, Observer,
  component::Component,
  space::{ColorSpace, Lchuv, Lms, Luv, Rgb, RgbSpec, Srgb, Xyz, cie::lchuv::max_safe_chroma_for_l},
};

/// HPLuv color space.
///
/// A hue-preserving variant of HSLuv that guarantees all colors at a given saturation
/// and lightness remain within the sRGB gamut regardless of hue rotation. Saturation
/// represents the percentage of the maximum chroma achievable at a given lightness
/// across **all** hues (the inscribed circle of the sRGB gamut at that lightness),
/// rather than at a specific hue as in HSLuv.
///
/// Components: H is hue (0–360°), S is saturation (0–100%), L is lightness (0–100%)
/// mapped directly to CIE L\*. Internally, hue is stored as a 0.0–1.0 fraction and
/// saturation/lightness as 0.0–1.0 (representing 0–100%).
///
/// HPLuv depends on CIE LCh(uv) and is feature-gated behind `space-hpluv`.
#[derive(Clone, Copy, Debug)]
pub struct Hpluv {
  alpha: Component,
  context: ColorimetricContext,
  h: Component,
  l: Component,
  s: Component,
}

impl Hpluv {
  /// The default viewing context for Hpluv (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new HPLuv color from hue (0-360°), saturation (0-100%), and lightness (0-100%).
  pub fn new(h: impl Into<Component>, s: impl Into<Component>, l: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      l: l.into() / 100.0,
      s: s.into() / 100.0,
    }
  }

  /// Creates a new HPLuv color in a const context from hue (0-360°), saturation (0-100%), and lightness (0-100%).
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

  /// Converts to the CIE LCh(uv) color space.
  pub fn to_lchuv(&self) -> Lchuv {
    let l = self.l.0 * 100.0;
    let h = self.h.0 * 360.0;

    if l > 99.9999999 {
      return Lchuv::new(100.0, 0.0, h).with_alpha(self.alpha);
    }
    if l < 0.00000001 {
      return Lchuv::new(0.0, 0.0, h).with_alpha(self.alpha);
    }

    let max_c = max_safe_chroma_for_l(l);
    let c = self.s.0 * max_c;

    Lchuv::new(l, c, h).with_alpha(self.alpha)
  }

  /// Converts to the CIE L*u*v* color space.
  pub fn to_luv(&self) -> Luv {
    self.to_lchuv().to_luv()
  }

  /// Converts to the specified RGB color space.
  pub fn to_rgb<S>(&self) -> Rgb<S>
  where
    S: RgbSpec,
  {
    self.to_lchuv().to_rgb::<S>()
  }

  /// Converts to the CIE XYZ color space.
  pub fn to_xyz(&self) -> Xyz {
    self.to_lchuv().to_xyz()
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
    let mut hpluv = *self;
    hpluv.decrement_h(amount);
    hpluv
  }

  /// Returns a new color with the normalized hue increased by the given amount.
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.increment_h(amount);
    hpluv
  }

  /// Returns a new color with the normalized hue scaled by the given factor.
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.scale_h(factor);
    hpluv
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
    let mut hpluv = *self;
    hpluv.decrement_hue(amount);
    hpluv
  }

  /// Returns a new color with the hue increased by the given degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.increment_hue(amount);
    hpluv
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
    let mut hpluv = *self;
    hpluv.decrement_l(amount);
    hpluv
  }

  /// Returns a new color with normalized lightness increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.increment_l(amount);
    hpluv
  }

  /// Returns a new color with normalized lightness scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.scale_l(factor);
    hpluv
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
    let mut hpluv = *self;
    hpluv.decrement_lightness(amount);
    hpluv
  }

  /// Returns a new color with lightness increased by the given percentage points.
  pub fn with_lightness_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.increment_lightness(amount);
    hpluv
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
    let mut hpluv = *self;
    hpluv.decrement_s(amount);
    hpluv
  }

  /// Returns a new color with normalized saturation increased by the given amount.
  pub fn with_s_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.increment_s(amount);
    hpluv
  }

  /// Returns a new color with normalized saturation scaled by the given factor.
  pub fn with_s_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.scale_s(factor);
    hpluv
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
    let mut hpluv = *self;
    hpluv.decrement_saturation(amount);
    hpluv
  }

  /// Returns a new color with saturation increased by the given percentage points.
  pub fn with_saturation_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hpluv = *self;
    hpluv.increment_saturation(amount);
    hpluv
  }

  /// Alias for [`Self::with_s_scaled_by`].
  pub fn with_saturation_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_s_scaled_by(factor)
  }
}

impl<T> Add<T> for Hpluv
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Hpluv {
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

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Hpluv {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    #[derive(serde::Deserialize)]
    struct HpluvData {
      h: Component,
      s: Component,
      l: Component,
      #[serde(default = "crate::component::default_alpha")]
      alpha: Component,
    }

    let data = HpluvData::deserialize(deserializer)?;
    Ok(Self {
      h: data.h,
      s: data.s,
      l: data.l,
      alpha: data.alpha,
      context: Self::DEFAULT_CONTEXT,
    })
  }
}

impl Display for Hpluv {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "HPLuv({:.precision$}°, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.hue(),
        self.saturation(),
        self.lightness(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "HPLuv({:.precision$}°, {:.precision$}%, {:.precision$}%)",
        self.hue(),
        self.saturation(),
        self.lightness()
      )
    }
  }
}

impl<T> Div<T> for Hpluv
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Hpluv
where
  T: Into<Component>,
{
  fn from([h, s, l]: [T; 3]) -> Self {
    Self::new(h, s, l)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Hpluv
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_hpluv()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Hpluv
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_hpluv()
  }
}

#[cfg(feature = "space-hsi")]
impl<S> From<Hsi<S>> for Hpluv
where
  S: RgbSpec,
{
  fn from(hsi: Hsi<S>) -> Self {
    hsi.to_hpluv()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Hpluv
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_hpluv()
  }
}

#[cfg(feature = "space-hsluv")]
impl From<Hsluv> for Hpluv {
  fn from(hsluv: Hsluv) -> Self {
    hsluv.to_hpluv()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Hpluv
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_hpluv()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Hpluv
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_hpluv()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Hpluv {
  fn from(lab: Lab) -> Self {
    lab.to_hpluv()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Hpluv {
  fn from(lch: Lch) -> Self {
    lch.to_hpluv()
  }
}

impl From<Lchuv> for Hpluv {
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_hpluv()
  }
}

impl From<Lms> for Hpluv {
  fn from(lms: Lms) -> Self {
    lms.to_hpluv()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Hpluv {
  fn from(luv: Luv) -> Self {
    luv.to_hpluv()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Hpluv {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_hpluv()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Hpluv {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_hpluv()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Hpluv {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_hpluv()
  }
}

#[cfg(feature = "space-oklab")]
impl From<Oklab> for Hpluv {
  fn from(oklab: Oklab) -> Self {
    oklab.to_hpluv()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Hpluv {
  fn from(oklch: Oklch) -> Self {
    oklch.to_hpluv()
  }
}

impl<S> From<Rgb<S>> for Hpluv
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_hpluv()
  }
}

#[cfg(feature = "space-xyy")]
impl From<Xyy> for Hpluv {
  fn from(xyy: Xyy) -> Self {
    xyy.to_hpluv()
  }
}

impl From<Xyz> for Hpluv {
  fn from(xyz: Xyz) -> Self {
    Self::from(Lchuv::from(xyz)).with_alpha(xyz.alpha())
  }
}

impl<T> Mul<T> for Hpluv
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Hpluv
where
  T: Into<Hpluv> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.h == other.h && self.s == other.s && self.l == other.l
  }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Hpluv {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeStruct;

    let field_count = if self.alpha.0 < 1.0 { 4 } else { 3 };
    let mut state = serializer.serialize_struct("Hpluv", field_count)?;
    state.serialize_field("h", &self.h)?;
    state.serialize_field("s", &self.s)?;
    state.serialize_field("l", &self.l)?;
    if self.alpha.0 < 1.0 {
      state.serialize_field("alpha", &self.alpha)?;
    }
    state.end()
  }
}

impl<T> Sub<T> for Hpluv
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_hpluv_values_via_rgb() {
      let a = Hpluv::new(0.0, 50.0, 25.0);
      let b = Hpluv::new(0.0, 50.0, 25.0);
      let result = a + b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod decrement_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_h() {
      let mut hpluv = Hpluv::new(180.0, 50.0, 50.0);
      hpluv.decrement_h(0.25);

      assert_eq!(hpluv.h(), 0.25);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hpluv = Hpluv::new(36.0, 50.0, 50.0);
      hpluv.decrement_h(0.2);

      assert!((hpluv.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_subtracts_degrees_from_hue() {
      let mut hpluv = Hpluv::new(180.0, 50.0, 50.0);
      hpluv.decrement_hue(90.0);

      assert!((hpluv.hue() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hpluv = Hpluv::new(30.0, 50.0, 50.0);
      hpluv.decrement_hue(60.0);

      assert!((hpluv.hue() - 330.0).abs() < 1e-10);
    }
  }

  mod decrement_l {
    use super::*;

    #[test]
    fn it_subtracts_from_l() {
      let mut hpluv = Hpluv::new(0.0, 50.0, 50.0);
      hpluv.decrement_l(0.2);

      assert!((hpluv.l() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_lightness {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_lightness() {
      let mut hpluv = Hpluv::new(0.0, 50.0, 50.0);
      hpluv.decrement_lightness(20.0);

      assert!((hpluv.lightness() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_s {
    use super::*;

    #[test]
    fn it_subtracts_from_s() {
      let mut hpluv = Hpluv::new(0.0, 50.0, 50.0);
      hpluv.decrement_s(0.2);

      assert!((hpluv.s() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_saturation {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_saturation() {
      let mut hpluv = Hpluv::new(0.0, 50.0, 50.0);
      hpluv.decrement_saturation(20.0);

      assert!((hpluv.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let hpluv = Hpluv::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", hpluv), "HPLuv(120.00°, 50.00%, 75.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let hpluv = Hpluv::new(120.12345, 50.6789, 75.4321);

      assert_eq!(format!("{:.4}", hpluv), "HPLuv(120.1235°, 50.6789%, 75.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let hpluv = Hpluv::new(120.0, 50.0, 75.0).with_alpha(0.5);

      assert_eq!(format!("{}", hpluv), "HPLuv(120.00°, 50.00%, 75.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let hpluv = Hpluv::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", hpluv), "HPLuv(120.00°, 50.00%, 75.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_hpluv_values_via_rgb() {
      let a = Hpluv::new(0.0, 50.0, 50.0);
      let b = Hpluv::new(0.0, 50.0, 50.0);
      let result = a / b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod from_lchuv {
    use super::*;

    #[test]
    fn it_converts_from_lchuv() {
      let lchuv = Lchuv::new(50.0, 10.0, 120.0);
      let hpluv: Hpluv = lchuv.into();

      assert!((hpluv.hue() - 120.0).abs() < 1e-6);
      assert!((hpluv.lightness() - 50.0).abs() < 1e-6);
      assert!(hpluv.saturation() > 0.0);
      assert!(hpluv.saturation() <= 100.0);
    }

    #[test]
    fn it_handles_white() {
      let lchuv = Lchuv::new(100.0, 0.0, 0.0);
      let hpluv: Hpluv = lchuv.into();

      assert!((hpluv.lightness() - 100.0).abs() < 1e-6);
      assert!((hpluv.saturation()).abs() < 1e-6);
    }

    #[test]
    fn it_handles_black() {
      let lchuv = Lchuv::new(0.0, 0.0, 0.0);
      let hpluv: Hpluv = lchuv.into();

      assert!((hpluv.lightness()).abs() < 1e-6);
      assert!((hpluv.saturation()).abs() < 1e-6);
    }
  }

  mod from_lms {
    use super::*;

    #[test]
    fn it_converts_from_lms_via_xyz() {
      let lms = Lms::new(0.5, 0.5, 0.5);
      let hpluv: Hpluv = lms.into();

      assert!(hpluv.h().is_finite());
      assert!(hpluv.s().is_finite());
      assert!(hpluv.l().is_finite());
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 0.0, 0.0);
      let hpluv: Hpluv = rgb.into();

      assert!((hpluv.hue() - 12.17).abs() < 0.1);
      assert!(hpluv.saturation() > 100.0); // Out-of-gamut inscribed circle: S > 100%
      assert!((hpluv.lightness() - 53.23).abs() < 0.1);
    }

    #[test]
    fn it_converts_white_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 1.0, 1.0);
      let hpluv: Hpluv = rgb.into();

      assert!((hpluv.saturation()).abs() < 1e-6);
      assert!((hpluv.lightness() - 100.0).abs() < 0.01);
    }

    #[test]
    fn it_converts_black_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 0.0);
      let hpluv: Hpluv = rgb.into();

      assert!((hpluv.saturation()).abs() < 1e-6);
      assert!((hpluv.lightness()).abs() < 1e-6);
    }

    #[test]
    fn it_converts_gray_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let hpluv: Hpluv = rgb.into();

      assert!(hpluv.saturation() < 1.0);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz_via_lchuv() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let hpluv: Hpluv = xyz.into();

      assert!(hpluv.h().is_finite());
      assert!(hpluv.s().is_finite());
      assert!(hpluv.l().is_finite());
    }
  }

  mod increment_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_h() {
      let mut hpluv = Hpluv::new(90.0, 50.0, 50.0);
      hpluv.increment_h(0.25);

      assert_eq!(hpluv.h(), 0.5);
    }

    #[test]
    fn it_wraps_above_one() {
      let mut hpluv = Hpluv::new(324.0, 50.0, 50.0);
      hpluv.increment_h(0.2);

      assert!((hpluv.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_adds_degrees_to_hue() {
      let mut hpluv = Hpluv::new(90.0, 50.0, 50.0);
      hpluv.increment_hue(90.0);

      assert!((hpluv.hue() - 180.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_above_360() {
      let mut hpluv = Hpluv::new(300.0, 50.0, 50.0);
      hpluv.increment_hue(90.0);

      assert!((hpluv.hue() - 30.0).abs() < 1e-10);
    }
  }

  mod increment_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_l() {
      let mut hpluv = Hpluv::new(0.0, 50.0, 25.0);
      hpluv.increment_l(0.25);

      assert_eq!(hpluv.l(), 0.5);
    }
  }

  mod increment_lightness {
    use super::*;

    #[test]
    fn it_adds_percentage_to_lightness() {
      let mut hpluv = Hpluv::new(0.0, 50.0, 25.0);
      hpluv.increment_lightness(25.0);

      assert!((hpluv.lightness() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_s() {
      let mut hpluv = Hpluv::new(0.0, 25.0, 50.0);
      hpluv.increment_s(0.25);

      assert_eq!(hpluv.s(), 0.5);
    }
  }

  mod increment_saturation {
    use super::*;

    #[test]
    fn it_adds_percentage_to_saturation() {
      let mut hpluv = Hpluv::new(0.0, 25.0, 50.0);
      hpluv.increment_saturation(25.0);

      assert!((hpluv.saturation() - 50.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_hpluv_values_via_rgb() {
      let a = Hpluv::new(0.0, 50.0, 50.0);
      let b = Hpluv::new(0.0, 50.0, 50.0);
      let result = a * b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod new_const {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_hue_to_0_1() {
      const HPLUV: Hpluv = Hpluv::new_const(270.0, 50.0, 50.0);

      assert_eq!(HPLUV.h(), 0.75);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      const HPLUV: Hpluv = Hpluv::new_const(-90.0, 50.0, 50.0);

      assert_eq!(HPLUV.h(), 0.75);
    }

    #[test]
    fn it_normalizes_saturation_to_0_1() {
      const HPLUV: Hpluv = Hpluv::new_const(0.0, 75.0, 50.0);

      assert_eq!(HPLUV.s(), 0.75);
    }

    #[test]
    fn it_normalizes_lightness_to_0_1() {
      const HPLUV: Hpluv = Hpluv::new_const(0.0, 50.0, 75.0);

      assert_eq!(HPLUV.l(), 0.75);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_hpluv_values() {
      let a = Hpluv::new(180.0, 50.0, 50.0);
      let b = Hpluv::new(180.0, 50.0, 50.0);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_hpluv_values() {
      let a = Hpluv::new(180.0, 50.0, 50.0);
      let b = Hpluv::new(180.0, 50.0, 60.0);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Hpluv::new(180.0, 50.0, 50.0).with_alpha(0.5);
      let b = Hpluv::new(180.0, 50.0, 50.0);

      assert_ne!(a, b);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_multiplies_h_by_factor() {
      let mut hpluv = Hpluv::new(90.0, 50.0, 50.0);
      hpluv.scale_h(2.0);

      assert!((hpluv.h() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_when_exceeding_one() {
      let mut hpluv = Hpluv::new(270.0, 50.0, 50.0);
      hpluv.scale_h(2.0);

      assert!((hpluv.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_l_by_factor() {
      let mut hpluv = Hpluv::new(0.0, 50.0, 25.0);
      hpluv.scale_l(2.0);

      assert_eq!(hpluv.l(), 0.5);
    }
  }

  mod scale_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_s_by_factor() {
      let mut hpluv = Hpluv::new(0.0, 25.0, 50.0);
      hpluv.scale_s(2.0);

      assert_eq!(hpluv.s(), 0.5);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_hpluv_values_via_rgb() {
      let a = Hpluv::new(0.0, 50.0, 50.0);
      let b = Hpluv::new(0.0, 25.0, 25.0);
      let result = a - b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod to_lchuv {
    use super::*;

    #[test]
    fn it_converts_to_lchuv() {
      let hpluv = Hpluv::new(120.0, 50.0, 60.0);
      let lchuv = hpluv.to_lchuv();

      assert!((lchuv.l() - 60.0).abs() < 1e-6);
      assert!((lchuv.hue() - 120.0).abs() < 1e-6);
      assert!(lchuv.chroma() > 0.0);
    }

    #[test]
    fn it_handles_white() {
      let hpluv = Hpluv::new(0.0, 100.0, 100.0);
      let lchuv = hpluv.to_lchuv();

      assert!((lchuv.l() - 100.0).abs() < 1e-6);
      assert!((lchuv.chroma()).abs() < 1e-6);
    }

    #[test]
    fn it_handles_black() {
      let hpluv = Hpluv::new(0.0, 100.0, 0.0);
      let lchuv = hpluv.to_lchuv();

      assert!((lchuv.l()).abs() < 1e-6);
      assert!((lchuv.chroma()).abs() < 1e-6);
    }

    #[test]
    fn it_preserves_alpha() {
      let hpluv = Hpluv::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let lchuv = hpluv.to_lchuv();

      assert!((lchuv.alpha() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_with_from_lchuv() {
      let original = Hpluv::new(210.0, 60.0, 40.0);
      let lchuv = original.to_lchuv();
      let back: Hpluv = lchuv.into();

      assert!((back.hue() - original.hue()).abs() < 0.01);
      assert!((back.saturation() - original.saturation()).abs() < 0.01);
      assert!((back.lightness() - original.lightness()).abs() < 0.01);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_rgb() {
      let hpluv = Hpluv::new(120.0, 100.0, 50.0);
      let _rgb: Rgb<Srgb> = hpluv.to_rgb();
    }

    #[test]
    fn it_converts_white() {
      let hpluv = Hpluv::new(0.0, 0.0, 100.0);
      let rgb: Rgb<Srgb> = hpluv.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_black() {
      let hpluv = Hpluv::new(0.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = hpluv.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_roundtrips_with_from_rgb() {
      let original = Hpluv::new(210.0, 60.0, 40.0);
      let rgb: Rgb<Srgb> = original.to_rgb();
      let back: Hpluv = rgb.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.lightness() - original.lightness()).abs() < 1.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let hpluv = Hpluv::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let rgb: Rgb<Srgb> = hpluv.to_rgb();

      assert!((rgb.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz_via_lchuv() {
      let hpluv = Hpluv::new(120.0, 100.0, 50.0);
      let xyz = hpluv.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_roundtrips_with_from_xyz() {
      let original = Hpluv::new(210.0, 60.0, 40.0);
      let xyz = original.to_xyz();
      let back: Hpluv = xyz.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.lightness() - original.lightness()).abs() < 1.0);
    }
  }

  mod with_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_new_h() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_h(0.75);

      assert_eq!(result.h(), 0.75);
      assert_eq!(result.s(), hpluv.s());
      assert_eq!(result.l(), hpluv.l());
    }
  }

  mod with_h_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_decremented_h() {
      let hpluv = Hpluv::new(180.0, 50.0, 50.0);
      let result = hpluv.with_h_decremented_by(0.25);

      assert_eq!(result.h(), 0.25);
      assert_eq!(hpluv.h(), 0.5);
    }
  }

  mod with_h_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_incremented_h() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_h_incremented_by(0.25);

      assert_eq!(result.h(), 0.5);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_scaled_h() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_new_hue_in_degrees() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
      assert_eq!(result.s(), hpluv.s());
      assert_eq!(result.l(), hpluv.l());
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_decremented_hue() {
      let hpluv = Hpluv::new(180.0, 50.0, 50.0);
      let result = hpluv.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
      assert!((hpluv.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_incremented_hue() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_hue_incremented_by(90.0);

      assert!((result.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_new_l() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_l(0.75);

      assert_eq!(result.l(), 0.75);
      assert_eq!(result.h(), hpluv.h());
      assert_eq!(result.s(), hpluv.s());
    }
  }

  mod with_l_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_decremented_l() {
      let hpluv = Hpluv::new(0.0, 50.0, 50.0);
      let result = hpluv.with_l_decremented_by(0.2);

      assert!((result.l() - 0.3).abs() < 1e-10);
      assert_eq!(hpluv.l(), 0.5);
    }
  }

  mod with_l_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_incremented_l() {
      let hpluv = Hpluv::new(0.0, 50.0, 25.0);
      let result = hpluv.with_l_incremented_by(0.25);

      assert_eq!(result.l(), 0.5);
    }
  }

  mod with_l_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_scaled_l() {
      let hpluv = Hpluv::new(0.0, 50.0, 25.0);
      let result = hpluv.with_l_scaled_by(2.0);

      assert_eq!(result.l(), 0.5);
    }
  }

  mod with_lightness {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_new_lightness_in_percent() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_lightness(75.0);

      assert!((result.lightness() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hpluv.h());
      assert_eq!(result.s(), hpluv.s());
    }
  }

  mod with_lightness_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_decremented_lightness() {
      let hpluv = Hpluv::new(0.0, 50.0, 50.0);
      let result = hpluv.with_lightness_decremented_by(20.0);

      assert!((result.lightness() - 30.0).abs() < 1e-10);
    }
  }

  mod with_lightness_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_incremented_lightness() {
      let hpluv = Hpluv::new(0.0, 50.0, 25.0);
      let result = hpluv.with_lightness_incremented_by(25.0);

      assert!((result.lightness() - 50.0).abs() < 1e-10);
    }
  }

  mod with_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_new_s() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_s(0.75);

      assert_eq!(result.s(), 0.75);
      assert_eq!(result.h(), hpluv.h());
      assert_eq!(result.l(), hpluv.l());
    }
  }

  mod with_s_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_decremented_s() {
      let hpluv = Hpluv::new(0.0, 50.0, 50.0);
      let result = hpluv.with_s_decremented_by(0.2);

      assert!((result.s() - 0.3).abs() < 1e-10);
      assert_eq!(hpluv.s(), 0.5);
    }
  }

  mod with_s_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_incremented_s() {
      let hpluv = Hpluv::new(0.0, 25.0, 50.0);
      let result = hpluv.with_s_incremented_by(0.25);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_s_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hpluv_with_scaled_s() {
      let hpluv = Hpluv::new(0.0, 25.0, 50.0);
      let result = hpluv.with_s_scaled_by(2.0);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_saturation {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_new_saturation_in_percent() {
      let hpluv = Hpluv::new(90.0, 50.0, 50.0);
      let result = hpluv.with_saturation(75.0);

      assert!((result.saturation() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hpluv.h());
      assert_eq!(result.l(), hpluv.l());
    }
  }

  mod with_saturation_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_decremented_saturation() {
      let hpluv = Hpluv::new(0.0, 50.0, 50.0);
      let result = hpluv.with_saturation_decremented_by(20.0);

      assert!((result.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod with_saturation_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hpluv_with_incremented_saturation() {
      let hpluv = Hpluv::new(0.0, 25.0, 50.0);
      let result = hpluv.with_saturation_incremented_by(25.0);

      assert!((result.saturation() - 50.0).abs() < 1e-10);
    }
  }
}
