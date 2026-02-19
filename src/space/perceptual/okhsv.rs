use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, Div, Mul, Sub},
};

use super::oklab::{cusp_for_hue, toe_inv};
#[cfg(feature = "space-cmy")]
use crate::space::Cmy;
#[cfg(feature = "space-cmyk")]
use crate::space::Cmyk;
#[cfg(feature = "space-hpluv")]
use crate::space::Hpluv;
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
#[cfg(feature = "space-lchuv")]
use crate::space::Lchuv;
#[cfg(feature = "space-luv")]
use crate::space::Luv;
#[cfg(feature = "space-okhsl")]
use crate::space::Okhsl;
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

/// Okhsv perceptual color space (HSV model in the Oklab perceptual framework).
///
/// A perceptually uniform HSV-like color space derived from Oklab. H represents
/// hue stored internally as a 0.0-1.0 fraction (0-360°), S represents saturation
/// (0.0-1.0), and V represents perceived value/brightness (0.0-1.0). Designed for
/// intuitive color manipulation with perceptual uniformity, using sRGB gamut
/// boundaries for saturation mapping.
#[derive(Clone, Copy, Debug)]
pub struct Okhsv {
  alpha: Component,
  context: ColorimetricContext,
  h: Component,
  s: Component,
  v: Component,
}

impl Okhsv {
  /// The default viewing context for Okhsv (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new Okhsv color from hue (0-360°), saturation (0-100%), and value (0-100%).
  pub fn new(h: impl Into<Component>, s: impl Into<Component>, v: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      s: s.into() / 100.0,
      v: v.into() / 100.0,
    }
  }

  /// Creates a new Okhsv color in a const context from hue (0-360°), saturation (0-100%), and value (0-100%).
  pub const fn new_const(h: f64, s: f64, v: f64) -> Self {
    let r = (h / 360.0) % 1.0;

    Self {
      alpha: Component::new_const(1.0),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new_const(if r < 0.0 { r + 1.0 } else { r }),
      s: Component::new_const(s / 100.0),
      v: Component::new_const(v / 100.0),
    }
  }

  /// Returns the [H, S, V] components as normalized values (all 0.0-1.0).
  pub fn components(&self) -> [f64; 3] {
    [self.h.0, self.s.0, self.v.0]
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

  /// Decreases the normalized saturation by the given amount.
  pub fn decrement_s(&mut self, amount: impl Into<Component>) {
    self.s -= amount.into();
  }

  /// Decreases the saturation by the given amount in percentage points.
  pub fn decrement_saturation(&mut self, amount: impl Into<Component>) {
    self.decrement_s(amount.into() / 100.0)
  }

  /// Decreases the normalized value by the given amount.
  pub fn decrement_v(&mut self, amount: impl Into<Component>) {
    self.v -= amount.into();
  }

  /// Decreases the value by the given amount in percentage points.
  pub fn decrement_value(&mut self, amount: impl Into<Component>) {
    self.decrement_v(amount.into() / 100.0)
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

  /// Increases the normalized saturation by the given amount.
  pub fn increment_s(&mut self, amount: impl Into<Component>) {
    self.s += amount.into();
  }

  /// Increases the saturation by the given amount in percentage points.
  pub fn increment_saturation(&mut self, amount: impl Into<Component>) {
    self.increment_s(amount.into() / 100.0)
  }

  /// Increases the normalized value by the given amount.
  pub fn increment_v(&mut self, amount: impl Into<Component>) {
    self.v += amount.into();
  }

  /// Increases the value by the given amount in percentage points.
  pub fn increment_value(&mut self, amount: impl Into<Component>) {
    self.increment_v(amount.into() / 100.0)
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

  /// Scales the normalized saturation by the given factor.
  pub fn scale_s(&mut self, factor: impl Into<Component>) {
    self.s *= factor.into();
  }

  /// Alias for [`Self::scale_s`].
  pub fn scale_saturation(&mut self, factor: impl Into<Component>) {
    self.scale_s(factor)
  }

  /// Scales the normalized value by the given factor.
  pub fn scale_v(&mut self, factor: impl Into<Component>) {
    self.v *= factor.into();
  }

  /// Alias for [`Self::scale_v`].
  pub fn scale_value(&mut self, factor: impl Into<Component>) {
    self.scale_v(factor)
  }

  /// Sets all three components from normalized values.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_h(components[0].clone());
    self.set_s(components[1].clone());
    self.set_v(components[2].clone());
  }

  /// Sets the normalized hue component (0.0-1.0).
  pub fn set_h(&mut self, h: impl Into<Component>) {
    self.h = h.into();
  }

  /// Sets the hue from a value in degrees (0-360°).
  pub fn set_hue(&mut self, hue: impl Into<Component>) {
    self.h = Component::new((hue.into().0 / 360.0).rem_euclid(1.0));
  }

  /// Sets the normalized saturation component (0.0-1.0).
  pub fn set_s(&mut self, s: impl Into<Component>) {
    self.s = s.into();
  }

  /// Sets the saturation from a percentage value (0-100%).
  pub fn set_saturation(&mut self, saturation: impl Into<Component>) {
    self.s = saturation.into() / 100.0;
  }

  /// Sets the normalized value component (0.0-1.0).
  pub fn set_v(&mut self, v: impl Into<Component>) {
    self.v = v.into();
  }

  /// Sets the value from a percentage (0-100%).
  pub fn set_value(&mut self, value: impl Into<Component>) {
    self.v = value.into() / 100.0;
  }

  /// Converts to the Okhwb perceptual color space.
  ///
  /// Uses the standard HSV-to-HWB reparameterization:
  /// W = (1 - S) * V, B = 1 - V.
  #[cfg(feature = "space-okhwb")]
  pub fn to_okhwb(&self) -> Okhwb {
    let [h, s, v] = self.components();

    let w = (1.0 - s) * v;
    let b = 1.0 - v;

    Okhwb::new(h * 360.0, w * 100.0, b * 100.0).with_alpha(self.alpha)
  }

  /// Converts to the Oklab perceptual color space.
  ///
  /// Uses an HSV cone model where V=1, S=1 maps to the cusp (maximum chroma)
  /// and V=1, S=0 maps to white. Reducing V scales toward black.
  pub fn to_oklab(&self) -> Oklab {
    let [h, s, v] = self.components();
    let cusp = cusp_for_hue(h);
    let (l_cusp, c_cusp) = cusp;

    let tv = toe_inv(v);
    let lab_l = tv * (1.0 - s * (1.0 - l_cusp));
    let c = tv * s * c_cusp;

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

  /// Returns the normalized value component (0.0-1.0).
  pub fn v(&self) -> f64 {
    self.v.0
  }

  /// Returns the value as a percentage (0-100%).
  pub fn value(&self) -> f64 {
    self.v.0 * 100.0
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
    let mut okhsv = *self;
    okhsv.decrement_h(amount);
    okhsv
  }

  /// Returns a new color with the normalized hue increased by the given amount.
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.increment_h(amount);
    okhsv
  }

  /// Returns a new color with the normalized hue scaled by the given factor.
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.scale_h(factor);
    okhsv
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
    let mut okhsv = *self;
    okhsv.decrement_hue(amount);
    okhsv
  }

  /// Returns a new color with the hue increased by the given degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.increment_hue(amount);
    okhsv
  }

  /// Alias for [`Self::with_h_scaled_by`].
  pub fn with_hue_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_h_scaled_by(factor)
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
    let mut okhsv = *self;
    okhsv.decrement_s(amount);
    okhsv
  }

  /// Returns a new color with normalized saturation increased by the given amount.
  pub fn with_s_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.increment_s(amount);
    okhsv
  }

  /// Returns a new color with normalized saturation scaled by the given factor.
  pub fn with_s_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.scale_s(factor);
    okhsv
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
    let mut okhsv = *self;
    okhsv.decrement_saturation(amount);
    okhsv
  }

  /// Returns a new color with saturation increased by the given percentage points.
  pub fn with_saturation_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.increment_saturation(amount);
    okhsv
  }

  /// Alias for [`Self::with_s_scaled_by`].
  pub fn with_saturation_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_s_scaled_by(factor)
  }

  /// Returns a new color with the given normalized value.
  pub fn with_v(&self, v: impl Into<Component>) -> Self {
    Self {
      v: v.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized value decreased by the given amount.
  pub fn with_v_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.decrement_v(amount);
    okhsv
  }

  /// Returns a new color with normalized value increased by the given amount.
  pub fn with_v_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.increment_v(amount);
    okhsv
  }

  /// Returns a new color with normalized value scaled by the given factor.
  pub fn with_v_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.scale_v(factor);
    okhsv
  }

  /// Returns a new color with the given value in percentage (0-100%).
  pub fn with_value(&self, value: impl Into<Component>) -> Self {
    Self {
      v: value.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with value decreased by the given percentage points.
  pub fn with_value_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.decrement_value(amount);
    okhsv
  }

  /// Returns a new color with value increased by the given percentage points.
  pub fn with_value_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut okhsv = *self;
    okhsv.increment_value(amount);
    okhsv
  }

  /// Alias for [`Self::with_v_scaled_by`].
  pub fn with_value_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_v_scaled_by(factor)
  }
}

impl<T> Add<T> for Okhsv
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Okhsv {
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
impl<'de> serde::Deserialize<'de> for Okhsv {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    #[derive(serde::Deserialize)]
    struct OkhsvData {
      h: Component,
      s: Component,
      v: Component,
      #[serde(default = "crate::component::default_alpha")]
      alpha: Component,
    }

    let data = OkhsvData::deserialize(deserializer)?;
    Ok(Self {
      h: data.h,
      s: data.s,
      v: data.v,
      alpha: data.alpha,
      context: Self::DEFAULT_CONTEXT,
    })
  }
}

impl Display for Okhsv {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Okhsv({:.precision$}°, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.hue(),
        self.saturation(),
        self.value(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "Okhsv({:.precision$}°, {:.precision$}%, {:.precision$}%)",
        self.hue(),
        self.saturation(),
        self.value()
      )
    }
  }
}

impl<T> Div<T> for Okhsv
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Okhsv
where
  T: Into<Component>,
{
  fn from([h, s, v]: [T; 3]) -> Self {
    Self::new(h, s, v)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Okhsv
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_okhsv()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Okhsv
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_okhsv()
  }
}

#[cfg(feature = "space-hpluv")]
impl From<Hpluv> for Okhsv {
  fn from(hpluv: Hpluv) -> Self {
    hpluv.to_okhsv()
  }
}

#[cfg(feature = "space-hsi")]
impl<S> From<Hsi<S>> for Okhsv
where
  S: RgbSpec,
{
  fn from(hsi: Hsi<S>) -> Self {
    hsi.to_okhsv()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Okhsv
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_okhsv()
  }
}

#[cfg(feature = "space-hsluv")]
impl From<Hsluv> for Okhsv {
  fn from(hsluv: Hsluv) -> Self {
    hsluv.to_okhsv()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Okhsv
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_okhsv()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Okhsv
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_okhsv()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Okhsv {
  fn from(lab: Lab) -> Self {
    lab.to_okhsv()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Okhsv {
  fn from(lch: Lch) -> Self {
    lch.to_okhsv()
  }
}

#[cfg(feature = "space-lchuv")]
impl From<Lchuv> for Okhsv {
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_okhsv()
  }
}

impl From<Lms> for Okhsv {
  fn from(lms: Lms) -> Self {
    lms.to_okhsv()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Okhsv {
  fn from(luv: Luv) -> Self {
    luv.to_okhsv()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Okhsv {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_okhsv()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Okhsv {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_okhsv()
  }
}

impl From<Oklab> for Okhsv {
  fn from(oklab: Oklab) -> Self {
    oklab.to_okhsv()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Okhsv {
  fn from(oklch: Oklch) -> Self {
    oklch.to_okhsv()
  }
}

impl<S> From<Rgb<S>> for Okhsv
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_okhsv()
  }
}

#[cfg(feature = "space-xyy")]
impl From<Xyy> for Okhsv {
  fn from(xyy: Xyy) -> Self {
    xyy.to_okhsv()
  }
}

impl From<Xyz> for Okhsv {
  fn from(xyz: Xyz) -> Self {
    xyz.to_okhsv()
  }
}

impl<T> Mul<T> for Okhsv
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Okhsv
where
  T: Into<Okhsv> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.h == other.h && self.s == other.s && self.v == other.v
  }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Okhsv {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeStruct;

    let field_count = if self.alpha.0 < 1.0 { 4 } else { 3 };
    let mut state = serializer.serialize_struct("Okhsv", field_count)?;
    state.serialize_field("h", &self.h)?;
    state.serialize_field("s", &self.s)?;
    state.serialize_field("v", &self.v)?;
    if self.alpha.0 < 1.0 {
      state.serialize_field("alpha", &self.alpha)?;
    }
    state.end()
  }
}

impl<T> Sub<T> for Okhsv
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Okhsv {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Okhsv {
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
    fn it_adds_two_okhsv_values_via_rgb() {
      let a = Okhsv::new(0.0, 50.0, 25.0);
      let b = Okhsv::new(0.0, 50.0, 25.0);
      let result = a + b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.v().is_finite());
    }
  }

  mod components {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_components_as_array() {
      let okhsv = Okhsv::new(180.0, 50.0, 75.0);
      let [h, s, v] = okhsv.components();

      assert_eq!(h, 0.5);
      assert_eq!(s, 0.5);
      assert_eq!(v, 0.75);
    }
  }

  mod decrement_h {
    use super::*;

    #[test]
    fn it_subtracts_from_h() {
      let mut okhsv = Okhsv::new(180.0, 50.0, 50.0);
      okhsv.decrement_h(0.25);

      assert!((okhsv.h() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut okhsv = Okhsv::new(36.0, 50.0, 50.0);
      okhsv.decrement_h(0.2);

      assert!((okhsv.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_subtracts_degrees_from_hue() {
      let mut okhsv = Okhsv::new(180.0, 50.0, 50.0);
      okhsv.decrement_hue(90.0);

      assert!((okhsv.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod decrement_s {
    use super::*;

    #[test]
    fn it_subtracts_from_s() {
      let mut okhsv = Okhsv::new(0.0, 50.0, 50.0);
      okhsv.decrement_s(0.2);

      assert!((okhsv.s() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_saturation {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_saturation() {
      let mut okhsv = Okhsv::new(0.0, 50.0, 50.0);
      okhsv.decrement_saturation(20.0);

      assert!((okhsv.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_v {
    use super::*;

    #[test]
    fn it_subtracts_from_v() {
      let mut okhsv = Okhsv::new(0.0, 50.0, 50.0);
      okhsv.decrement_v(0.2);

      assert!((okhsv.v() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_value {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_value() {
      let mut okhsv = Okhsv::new(0.0, 50.0, 50.0);
      okhsv.decrement_value(20.0);

      assert!((okhsv.value() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let okhsv = Okhsv::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", okhsv), "Okhsv(120.00°, 50.00%, 75.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let okhsv = Okhsv::new(120.12345, 50.6789, 75.4321);

      assert_eq!(format!("{:.4}", okhsv), "Okhsv(120.1235°, 50.6789%, 75.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let okhsv = Okhsv::new(120.0, 50.0, 75.0).with_alpha(0.5);

      assert_eq!(format!("{}", okhsv), "Okhsv(120.00°, 50.00%, 75.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let okhsv = Okhsv::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", okhsv), "Okhsv(120.00°, 50.00%, 75.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_okhsv_values_via_rgb() {
      let a = Okhsv::new(0.0, 50.0, 50.0);
      let b = Okhsv::new(0.0, 50.0, 50.0);
      let result = a / b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.v().is_finite());
    }
  }

  mod from_array {
    use super::*;

    #[test]
    fn it_creates_from_f64_array() {
      let okhsv = Okhsv::from([180.0, 50.0, 75.0]);

      assert!((okhsv.hue() - 180.0).abs() < 1e-10);
      assert!((okhsv.saturation() - 50.0).abs() < 1e-10);
      assert!((okhsv.value() - 75.0).abs() < 1e-10);
    }
  }

  mod from_oklab {
    use super::*;

    #[test]
    fn it_converts_from_oklab() {
      let oklab = Oklab::new(0.5, 0.0, 0.0);
      let okhsv = Okhsv::from(oklab);

      assert!((okhsv.v() - toe(0.5)).abs() < 1e-10);
      assert!(okhsv.s() < 1e-3);
    }

    #[test]
    fn it_converts_black() {
      let oklab = Oklab::new(0.0, 0.0, 0.0);
      let okhsv = Okhsv::from(oklab);

      assert!(okhsv.v().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let oklab = Oklab::new(1.0, 0.0, 0.0);
      let okhsv = Okhsv::from(oklab);

      assert!((okhsv.v() - 1.0).abs() < 1e-10);
      assert!(okhsv.s() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.5);
      let okhsv = Okhsv::from(oklab);

      assert!((okhsv.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let okhsv = Okhsv::from(rgb);

      assert!((okhsv.v() - 1.0).abs() < 1e-3);
      assert!(okhsv.s() < 1e-3);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let okhsv = Okhsv::from(rgb);

      assert!(okhsv.v().abs() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let okhsv = Okhsv::from(rgb);

      assert!((okhsv.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let okhsv = Okhsv::from(xyz);

      assert!(okhsv.h().is_finite());
      assert!(okhsv.s().is_finite());
      assert!(okhsv.v().is_finite());
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let okhsv = Okhsv::from(xyz);

      assert!((okhsv.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod h {
    use super::*;

    #[test]
    fn it_returns_normalized_hue() {
      let okhsv = Okhsv::new(180.0, 50.0, 50.0);

      assert!((okhsv.h() - 0.5).abs() < 1e-10);
    }
  }

  mod hue {
    use super::*;

    #[test]
    fn it_returns_hue_in_degrees() {
      let okhsv = Okhsv::new(180.0, 50.0, 50.0);

      assert!((okhsv.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod increment_h {
    use super::*;

    #[test]
    fn it_adds_to_h() {
      let mut okhsv = Okhsv::new(90.0, 50.0, 50.0);
      okhsv.increment_h(0.25);

      assert!((okhsv.h() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_above_one() {
      let mut okhsv = Okhsv::new(324.0, 50.0, 50.0);
      okhsv.increment_h(0.2);

      assert!((okhsv.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_adds_degrees_to_hue() {
      let mut okhsv = Okhsv::new(90.0, 50.0, 50.0);
      okhsv.increment_hue(90.0);

      assert!((okhsv.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod increment_s {
    use super::*;

    #[test]
    fn it_adds_to_s() {
      let mut okhsv = Okhsv::new(0.0, 25.0, 50.0);
      okhsv.increment_s(0.25);

      assert!((okhsv.s() - 0.5).abs() < 1e-10);
    }
  }

  mod increment_saturation {
    use super::*;

    #[test]
    fn it_adds_percentage_to_saturation() {
      let mut okhsv = Okhsv::new(0.0, 25.0, 50.0);
      okhsv.increment_saturation(25.0);

      assert!((okhsv.saturation() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_v {
    use super::*;

    #[test]
    fn it_adds_to_v() {
      let mut okhsv = Okhsv::new(0.0, 50.0, 25.0);
      okhsv.increment_v(0.25);

      assert!((okhsv.v() - 0.5).abs() < 1e-10);
    }
  }

  mod increment_value {
    use super::*;

    #[test]
    fn it_adds_percentage_to_value() {
      let mut okhsv = Okhsv::new(0.0, 50.0, 25.0);
      okhsv.increment_value(25.0);

      assert!((okhsv.value() - 50.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_okhsv_values_via_rgb() {
      let a = Okhsv::new(0.0, 50.0, 50.0);
      let b = Okhsv::new(0.0, 50.0, 50.0);
      let result = a * b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.v().is_finite());
    }
  }

  mod new {
    use super::*;

    #[test]
    fn it_creates_with_default_alpha() {
      let okhsv = Okhsv::new(180.0, 50.0, 75.0);

      assert!((okhsv.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let okhsv = Okhsv::new(180.0, 50.0, 75.0);

      assert_eq!(okhsv.context().illuminant().name(), "D65");
    }

    #[test]
    fn it_normalizes_hue_to_zero_one() {
      let okhsv = Okhsv::new(450.0, 50.0, 50.0);

      assert!((okhsv.h() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      let okhsv = Okhsv::new(-90.0, 50.0, 50.0);

      assert!((okhsv.h() - 0.75).abs() < 1e-10);
    }
  }

  mod new_const {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_hue_to_0_1() {
      const OKHSV: Okhsv = Okhsv::new_const(270.0, 50.0, 50.0);

      assert_eq!(OKHSV.h(), 0.75);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      const OKHSV: Okhsv = Okhsv::new_const(-90.0, 50.0, 50.0);

      assert_eq!(OKHSV.h(), 0.75);
    }

    #[test]
    fn it_normalizes_saturation_to_0_1() {
      const OKHSV: Okhsv = Okhsv::new_const(0.0, 75.0, 50.0);

      assert_eq!(OKHSV.s(), 0.75);
    }

    #[test]
    fn it_normalizes_value_to_0_1() {
      const OKHSV: Okhsv = Okhsv::new_const(0.0, 50.0, 75.0);

      assert_eq!(OKHSV.v(), 0.75);
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Okhsv::new(180.0, 50.0, 50.0);
      let b = Okhsv::new(180.0, 50.0, 50.0);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Okhsv::new(180.0, 50.0, 50.0);
      let b = Okhsv::new(180.0, 50.0, 60.0);

      assert!(a != b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Okhsv::new(180.0, 50.0, 50.0).with_alpha(0.5);
      let b = Okhsv::new(180.0, 50.0, 50.0);

      assert!(a != b);
    }
  }

  mod s {
    use super::*;

    #[test]
    fn it_returns_normalized_saturation() {
      let okhsv = Okhsv::new(0.0, 75.0, 50.0);

      assert!((okhsv.s() - 0.75).abs() < 1e-10);
    }
  }

  mod saturation {
    use super::*;

    #[test]
    fn it_returns_saturation_as_percentage() {
      let okhsv = Okhsv::new(0.0, 75.0, 50.0);

      assert!((okhsv.saturation() - 75.0).abs() < 1e-10);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_multiplies_h_by_factor() {
      let mut okhsv = Okhsv::new(90.0, 50.0, 50.0);
      okhsv.scale_h(2.0);

      assert!((okhsv.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_s {
    use super::*;

    #[test]
    fn it_multiplies_s_by_factor() {
      let mut okhsv = Okhsv::new(0.0, 25.0, 50.0);
      okhsv.scale_s(2.0);

      assert!((okhsv.s() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_v {
    use super::*;

    #[test]
    fn it_multiplies_v_by_factor() {
      let mut okhsv = Okhsv::new(0.0, 50.0, 25.0);
      okhsv.scale_v(2.0);

      assert!((okhsv.v() - 0.5).abs() < 1e-10);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_okhsv_values_via_rgb() {
      let a = Okhsv::new(0.0, 50.0, 50.0);
      let b = Okhsv::new(0.0, 25.0, 25.0);
      let result = a - b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.v().is_finite());
    }
  }

  #[cfg(feature = "space-okhwb")]
  mod to_okhwb {
    use super::*;

    #[test]
    fn it_converts_to_okhwb() {
      let okhsv = Okhsv::new(210.0, 80.0, 60.0);
      let okhwb = okhsv.to_okhwb();

      assert!((okhwb.hue() - 210.0).abs() < 1e-10);
      assert!((okhwb.w() - (1.0 - 0.8) * 0.6).abs() < 1e-10);
      assert!((okhwb.b() - (1.0 - 0.6)).abs() < 1e-10);
    }

    #[test]
    fn it_converts_black() {
      let okhsv = Okhsv::new(0.0, 0.0, 0.0);
      let okhwb = okhsv.to_okhwb();

      assert!((okhwb.blackness() - 100.0).abs() < 1e-10);
      assert!(okhwb.whiteness().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let okhsv = Okhsv::new(0.0, 0.0, 100.0);
      let okhwb = okhsv.to_okhwb();

      assert!((okhwb.whiteness() - 100.0).abs() < 1e-10);
      assert!(okhwb.blackness().abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_through_okhwb() {
      let original = Okhsv::new(210.0, 80.0, 60.0);
      let roundtrip = Okhsv::from(original.to_okhwb());

      assert!((original.h() - roundtrip.h()).abs() < 1e-10);
      assert!((original.s() - roundtrip.s()).abs() < 1e-10);
      assert!((original.v() - roundtrip.v()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let okhsv = Okhsv::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let okhwb = okhsv.to_okhwb();

      assert!((okhwb.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_oklab {
    use super::*;

    #[test]
    fn it_converts_achromatic_gray() {
      let okhsv = Okhsv::new(0.0, 0.0, 50.0);
      let oklab = okhsv.to_oklab();

      assert!((oklab.l() - toe_inv(0.5)).abs() < 1e-10);
      assert!(oklab.a().abs() < 1e-10);
      assert!(oklab.b().abs() < 1e-10);
    }

    #[test]
    fn it_converts_black() {
      let okhsv = Okhsv::new(0.0, 100.0, 0.0);
      let oklab = okhsv.to_oklab();

      assert!(oklab.l().abs() < 1e-10);
      assert!(oklab.a().abs() < 1e-10);
      assert!(oklab.b().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let okhsv = Okhsv::new(0.0, 0.0, 100.0);
      let oklab = okhsv.to_oklab();

      assert!((oklab.l() - 1.0).abs() < 1e-10);
      assert!(oklab.a().abs() < 1e-10);
      assert!(oklab.b().abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_through_oklab() {
      let original = Okhsv::new(210.0, 80.0, 50.0);
      let roundtrip = Okhsv::from(original.to_oklab());

      assert!((original.h() - roundtrip.h()).abs() < 1e-10);
      assert!((original.s() - roundtrip.s()).abs() < 1e-10);
      assert!((original.v() - roundtrip.v()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let okhsv = Okhsv::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let oklab = okhsv.to_oklab();

      assert!((oklab.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let okhsv = Okhsv::new(0.0, 50.0, 50.0);
      let rgb = okhsv.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_converts_black() {
      let okhsv = Okhsv::new(0.0, 0.0, 0.0);
      let rgb = okhsv.to_rgb::<Srgb>();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_white() {
      let okhsv = Okhsv::new(0.0, 0.0, 100.0);
      let rgb = okhsv.to_rgb::<Srgb>();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_preserves_alpha() {
      let okhsv = Okhsv::new(120.0, 50.0, 50.0).with_alpha(0.7);
      let rgb = okhsv.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let okhsv = Okhsv::new(120.0, 50.0, 50.0);
      let xyz = okhsv.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_preserves_alpha() {
      let okhsv = Okhsv::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let xyz = okhsv.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let okhsv = Okhsv::try_from("#FF5733").unwrap();

      assert!(okhsv.v() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Okhsv::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod v {
    use super::*;

    #[test]
    fn it_returns_normalized_value() {
      let okhsv = Okhsv::new(0.0, 50.0, 75.0);

      assert!((okhsv.v() - 0.75).abs() < 1e-10);
    }
  }

  mod value {
    use super::*;

    #[test]
    fn it_returns_value_as_percentage() {
      let okhsv = Okhsv::new(0.0, 50.0, 75.0);

      assert!((okhsv.value() - 75.0).abs() < 1e-10);
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let okhsv = Okhsv::new(180.0, 50.0, 50.0);
      let result = okhsv.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let okhsv = Okhsv::new(180.0, 50.0, 50.0);
      let context = ColorimetricContext::default();
      let result = okhsv.with_context(context);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhsv_with_new_h() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_h(0.75);

      assert_eq!(result.h(), 0.75);
      assert_eq!(result.s(), okhsv.s());
      assert_eq!(result.v(), okhsv.v());
    }
  }

  mod with_h_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_decremented_h() {
      let okhsv = Okhsv::new(180.0, 50.0, 50.0);
      let result = okhsv.with_h_decremented_by(0.25);

      assert!((result.h() - 0.25).abs() < 1e-10);
      assert!((okhsv.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_incremented_h() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_h_incremented_by(0.25);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_scaled_h() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_new_hue_in_degrees() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
      assert_eq!(result.s(), okhsv.s());
      assert_eq!(result.v(), okhsv.v());
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_decremented_hue() {
      let okhsv = Okhsv::new(180.0, 50.0, 50.0);
      let result = okhsv.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_incremented_hue() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_hue_incremented_by(90.0);

      assert!((result.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhsv_with_new_s() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_s(0.75);

      assert_eq!(result.s(), 0.75);
      assert_eq!(result.h(), okhsv.h());
      assert_eq!(result.v(), okhsv.v());
    }
  }

  mod with_s_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_decremented_s() {
      let okhsv = Okhsv::new(0.0, 50.0, 50.0);
      let result = okhsv.with_s_decremented_by(0.2);

      assert!((result.s() - 0.3).abs() < 1e-10);
    }
  }

  mod with_s_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_incremented_s() {
      let okhsv = Okhsv::new(0.0, 25.0, 50.0);
      let result = okhsv.with_s_incremented_by(0.25);

      assert!((result.s() - 0.5).abs() < 1e-10);
    }
  }

  mod with_s_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_scaled_s() {
      let okhsv = Okhsv::new(0.0, 25.0, 50.0);
      let result = okhsv.with_s_scaled_by(2.0);

      assert!((result.s() - 0.5).abs() < 1e-10);
    }
  }

  mod with_saturation {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_new_saturation_in_percent() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_saturation(75.0);

      assert!((result.saturation() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), okhsv.h());
      assert_eq!(result.v(), okhsv.v());
    }
  }

  mod with_saturation_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_decremented_saturation() {
      let okhsv = Okhsv::new(0.0, 50.0, 50.0);
      let result = okhsv.with_saturation_decremented_by(20.0);

      assert!((result.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod with_saturation_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_incremented_saturation() {
      let okhsv = Okhsv::new(0.0, 25.0, 50.0);
      let result = okhsv.with_saturation_incremented_by(25.0);

      assert!((result.saturation() - 50.0).abs() < 1e-10);
    }
  }

  mod with_v {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_okhsv_with_new_v() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_v(0.75);

      assert_eq!(result.v(), 0.75);
      assert_eq!(result.h(), okhsv.h());
      assert_eq!(result.s(), okhsv.s());
    }
  }

  mod with_v_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_decremented_v() {
      let okhsv = Okhsv::new(0.0, 50.0, 50.0);
      let result = okhsv.with_v_decremented_by(0.2);

      assert!((result.v() - 0.3).abs() < 1e-10);
    }
  }

  mod with_v_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_incremented_v() {
      let okhsv = Okhsv::new(0.0, 50.0, 25.0);
      let result = okhsv.with_v_incremented_by(0.25);

      assert!((result.v() - 0.5).abs() < 1e-10);
    }
  }

  mod with_v_scaled_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_scaled_v() {
      let okhsv = Okhsv::new(0.0, 50.0, 25.0);
      let result = okhsv.with_v_scaled_by(2.0);

      assert!((result.v() - 0.5).abs() < 1e-10);
    }
  }

  mod with_value {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_new_value_in_percent() {
      let okhsv = Okhsv::new(90.0, 50.0, 50.0);
      let result = okhsv.with_value(75.0);

      assert!((result.value() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), okhsv.h());
      assert_eq!(result.s(), okhsv.s());
    }
  }

  mod with_value_decremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_decremented_value() {
      let okhsv = Okhsv::new(0.0, 50.0, 50.0);
      let result = okhsv.with_value_decremented_by(20.0);

      assert!((result.value() - 30.0).abs() < 1e-10);
    }
  }

  mod with_value_incremented_by {
    use super::*;

    #[test]
    fn it_returns_okhsv_with_incremented_value() {
      let okhsv = Okhsv::new(0.0, 50.0, 25.0);
      let result = okhsv.with_value_incremented_by(25.0);

      assert!((result.value() - 50.0).abs() < 1e-10);
    }
  }
}
