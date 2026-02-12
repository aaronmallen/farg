use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  marker::PhantomData,
  ops::{Add, Div, Mul, Sub},
};

#[cfg(feature = "space-cmy")]
use crate::space::Cmy;
#[cfg(feature = "space-cmyk")]
use crate::space::Cmyk;
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
#[cfg(feature = "space-okhsv")]
use crate::space::Okhsv;
#[cfg(feature = "space-okhwb")]
use crate::space::Okhwb;
#[cfg(feature = "space-oklab")]
use crate::space::Oklab;
#[cfg(feature = "space-oklch")]
use crate::space::Oklch;
#[cfg(feature = "space-hsv")]
use crate::space::{Hsb, Hsv};
use crate::{
  ColorimetricContext,
  component::Component,
  space::{ColorSpace, Lms, Rgb, RgbSpec, Srgb, Xyz},
};

/// HSL (Hue, Saturation, Lightness) color space.
///
/// A cylindrical representation of RGB colors, parameterized by an [`RgbSpec`] that
/// determines the underlying RGB space. Defaults to [`Srgb`] when not specified.
/// Components are stored normalized: hue in 0.0-1.0 (representing 0-360°),
/// saturation and lightness in 0.0-1.0 (representing 0-100%).
#[derive(Clone, Copy, Debug)]
pub struct Hsl<S = Srgb>
where
  S: RgbSpec,
{
  alpha: Component,
  context: ColorimetricContext,
  h: Component,
  l: Component,
  s: Component,
  _spec: PhantomData<S>,
}

impl<S> Hsl<S>
where
  S: RgbSpec,
{
  /// Creates a new HSL color from hue (0-360°), saturation (0-100%), and lightness (0-100%).
  pub fn new(h: impl Into<Component>, s: impl Into<Component>, l: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: S::CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      l: l.into() / 100.0,
      s: s.into() / 100.0,
      _spec: PhantomData,
    }
  }

  /// Creates a new HSL color in a const context from hue (0-360°), saturation (0-100%), and lightness (0-100%).
  pub const fn new_const(h: f64, s: f64, l: f64) -> Self {
    let r = (h / 360.0) % 1.0;

    Self {
      alpha: Component::new_const(1.0),
      context: S::CONTEXT,
      h: Component::new_const(if r < 0.0 { r + 1.0 } else { r }),
      l: Component::new_const(l / 100.0),
      s: Component::new_const(s / 100.0),
      _spec: PhantomData,
    }
  }

  /// Returns the [H, S, L] components as normalized values.
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

  /// Converts this HSL color in the [`Hsb`] color space.
  #[cfg(feature = "space-hsv")]
  pub fn to_hsb(&self) -> Hsb<S> {
    self.to_hsv()
  }

  /// Converts this HSL color in the [`Hsv`] color space.
  #[cfg(feature = "space-hsv")]
  pub fn to_hsv(&self) -> Hsv<S> {
    let [h, s, l] = self.components();

    let v = l + (s * l.min(1.0 - l));
    let ns = if v == 0.0 { 0.0 } else { 2.0 * (1.0 - (l / v)) };

    Hsv::<S>::new(h, ns, v).with_alpha(self.alpha)
  }

  /// Converts this HSL color to an [`Hwb`] color in the specified RGB color space.
  #[cfg(feature = "space-hwb")]
  pub fn to_hwb(&self) -> Hwb<S> {
    let [h, s, l] = self.components();

    let v = if l <= 0.5 { l * (1.0 + s) } else { (l + s) - (l * s) };

    if v == 0.0 {
      return Hwb::<S>::new(h * 360.0, 0.0, 100.0).with_alpha(self.alpha);
    }

    let sv = 2.0 * (v - l) / v;
    let w = v * (1.0 - sv);
    let b = 1.0 - v;

    Hwb::<S>::new(h * 360.0, w * 100.0, b * 100.0).with_alpha(self.alpha)
  }

  /// Converts this HSL color to an [`Rgb`] color in the specified output space.
  pub fn to_rgb<OS>(&self) -> Rgb<OS>
  where
    OS: RgbSpec,
  {
    let h = self.h.0;
    let s = self.s.0;
    let l = self.l.0;

    if s <= 0.0 {
      return Rgb::<S>::from_normalized(l, l, l).to_rgb::<OS>().with_alpha(self.alpha);
    }
    if l <= 0.0 {
      return Rgb::<S>::from_normalized(0.0, 0.0, 0.0)
        .to_rgb::<OS>()
        .with_alpha(self.alpha);
    }
    if l >= 1.0 {
      return Rgb::<S>::from_normalized(1.0, 1.0, 1.0)
        .to_rgb::<OS>()
        .with_alpha(self.alpha);
    }

    let c = (1.0 - ((2.0 * l) - 1.0).abs()) * s;
    let h_prime = h * 6.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = l - (c / 2.0);

    let [r1, g1, b1] = match h_prime.floor().rem_euclid(6.0) {
      0.0 => [c, x, 0.0],
      1.0 => [x, c, 0.0],
      2.0 => [0.0, c, x],
      3.0 => [0.0, x, c],
      4.0 => [x, 0.0, c],
      5.0 => [c, 0.0, x],
      _ => unreachable!(),
    };

    Rgb::<S>::from_normalized(r1 + m, g1 + m, b1 + m)
      .to_rgb::<OS>()
      .with_alpha(self.alpha)
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
    let mut hsl = *self;
    hsl.decrement_h(amount);
    hsl
  }

  /// Returns a new color with the normalized hue increased by the given amount.
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.increment_h(amount);
    hsl
  }

  /// Returns a new color with the normalized hue scaled by the given factor.
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.scale_h(factor);
    hsl
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
    let mut hsl = *self;
    hsl.decrement_hue(amount);
    hsl
  }

  /// Returns a new color with the hue increased by the given degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.increment_hue(amount);
    hsl
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
    let mut hsl = *self;
    hsl.decrement_l(amount);
    hsl
  }

  /// Returns a new color with normalized lightness increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.increment_l(amount);
    hsl
  }

  /// Returns a new color with normalized lightness scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.scale_l(factor);
    hsl
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
    let mut hsl = *self;
    hsl.decrement_lightness(amount);
    hsl
  }

  /// Returns a new color with lightness increased by the given percentage points.
  pub fn with_lightness_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.increment_lightness(amount);
    hsl
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
    let mut hsl = *self;
    hsl.decrement_s(amount);
    hsl
  }

  /// Returns a new color with normalized saturation increased by the given amount.
  pub fn with_s_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.increment_s(amount);
    hsl
  }

  /// Returns a new color with normalized saturation scaled by the given factor.
  pub fn with_s_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.scale_s(factor);
    hsl
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
    let mut hsl = *self;
    hsl.decrement_saturation(amount);
    hsl
  }

  /// Returns a new color with saturation increased by the given percentage points.
  pub fn with_saturation_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsl = *self;
    hsl.increment_saturation(amount);
    hsl
  }

  /// Alias for [`Self::with_s_scaled_by`].
  pub fn with_saturation_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_s_scaled_by(factor)
  }
}

impl<S, T> Add<T> for Hsl<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() + rhs.into().to_rgb::<S>())
  }
}

impl<S> ColorSpace<3> for Hsl<S>
where
  S: RgbSpec,
{
  fn alpha(&self) -> f64 {
    self.alpha.0
  }

  fn components(&self) -> [f64; 3] {
    self.components()
  }

  fn set_alpha(&mut self, alpha: impl Into<Component>) {
    self.alpha = alpha.into().clamp(0.0, 1.0)
  }

  fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_components(components)
  }

  fn to_xyz(&self) -> Xyz {
    self.to_rgb::<S>().to_xyz()
  }
}

impl<S> Display for Hsl<S>
where
  S: RgbSpec,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "HSL({:.precision$}°, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.hue(),
        self.saturation(),
        self.lightness(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "HSL({:.precision$}°, {:.precision$}%, {:.precision$}%)",
        self.hue(),
        self.saturation(),
        self.lightness()
      )
    }
  }
}

impl<S, T> Div<T> for Hsl<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() / rhs.into().to_rgb::<S>())
  }
}

impl<S, T> From<[T; 3]> for Hsl<S>
where
  S: RgbSpec,
  T: Into<Component>,
{
  fn from([h, s, l]: [T; 3]) -> Self {
    Self::new(h, s, l)
  }
}

#[cfg(feature = "space-cmy")]
impl<OS, S> From<Cmy<OS>> for Hsl<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmy: Cmy<OS>) -> Self {
    cmy.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-cmyk")]
impl<OS, S> From<Cmyk<OS>> for Hsl<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<OS>) -> Self {
    cmyk.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-hsv")]
impl<OS, S> From<Hsv<OS>> for Hsl<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hsv: Hsv<OS>) -> Self {
    hsv.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-hwb")]
impl<OS, S> From<Hwb<OS>> for Hsl<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hwb: Hwb<OS>) -> Self {
    hwb.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-lab")]
impl<S> From<Lab> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(lab: Lab) -> Self {
    lab.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-lch")]
impl<S> From<Lch> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(lch: Lch) -> Self {
    lch.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-luv")]
impl<S> From<Luv> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(luv: Luv) -> Self {
    luv.to_rgb::<S>().to_hsl()
  }
}

impl<S> From<Lms> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(lms: Lms) -> Self {
    lms.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-okhsl")]
impl<S> From<Okhsl> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-okhsv")]
impl<S> From<Okhsv> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-okhwb")]
impl<S> From<Okhwb> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-oklab")]
impl<S> From<Oklab> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(oklab: Oklab) -> Self {
    oklab.to_rgb::<S>().to_hsl()
  }
}

#[cfg(feature = "space-oklch")]
impl<S> From<Oklch> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(oklch: Oklch) -> Self {
    oklch.to_rgb::<S>().to_hsl()
  }
}

impl<OS, S> From<Rgb<OS>> for Hsl<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(rgb: Rgb<OS>) -> Self {
    rgb.to_rgb::<S>().to_hsl()
  }
}

impl<S> From<Xyz> for Hsl<S>
where
  S: RgbSpec,
{
  fn from(xyz: Xyz) -> Self {
    xyz.to_rgb::<S>().to_hsl()
  }
}

impl<S, T> Mul<T> for Hsl<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() * rhs.into().to_rgb::<S>())
  }
}

impl<S, T> PartialEq<T> for Hsl<S>
where
  S: RgbSpec,
  T: Into<Hsl<S>> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.h == other.h && self.s == other.s && self.l == other.l
  }
}

impl<S, T> Sub<T> for Hsl<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() - rhs.into().to_rgb::<S>())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_hsl_values_via_rgb() {
      let a = Hsl::<Srgb>::new(0.0, 50.0, 25.0);
      let b = Hsl::<Srgb>::new(0.0, 50.0, 25.0);
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
      let mut hsl = Hsl::<Srgb>::new(180.0, 50.0, 50.0);
      hsl.decrement_h(0.25);

      assert_eq!(hsl.h(), 0.25);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hsl = Hsl::<Srgb>::new(36.0, 50.0, 50.0);
      hsl.decrement_h(0.2);

      assert!((hsl.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_subtracts_degrees_from_hue() {
      let mut hsl = Hsl::<Srgb>::new(180.0, 50.0, 50.0);
      hsl.decrement_hue(90.0);

      assert!((hsl.hue() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hsl = Hsl::<Srgb>::new(30.0, 50.0, 50.0);
      hsl.decrement_hue(60.0);

      assert!((hsl.hue() - 330.0).abs() < 1e-10);
    }
  }

  mod decrement_l {
    use super::*;

    #[test]
    fn it_subtracts_from_l() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      hsl.decrement_l(0.2);

      assert!((hsl.l() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_lightness {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_lightness() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      hsl.decrement_lightness(20.0);

      assert!((hsl.lightness() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_s {
    use super::*;

    #[test]
    fn it_subtracts_from_s() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      hsl.decrement_s(0.2);

      assert!((hsl.s() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_saturation {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_saturation() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      hsl.decrement_saturation(20.0);

      assert!((hsl.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let hsl = Hsl::<Srgb>::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", hsl), "HSL(120.00°, 50.00%, 75.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let hsl = Hsl::<Srgb>::new(120.12345, 50.6789, 75.4321);

      assert_eq!(format!("{:.4}", hsl), "HSL(120.1235°, 50.6789%, 75.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let hsl = Hsl::<Srgb>::new(120.0, 50.0, 75.0).with_alpha(0.5);

      assert_eq!(format!("{}", hsl), "HSL(120.00°, 50.00%, 75.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let hsl = Hsl::<Srgb>::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", hsl), "HSL(120.00°, 50.00%, 75.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_hsl_values_via_rgb() {
      let a = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      let result = a / b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  #[cfg(feature = "space-cmyk")]
  mod from_cmyk {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let cmyk = Cmyk::<Srgb>::new(100.0, 0.0, 0.0, 0.0);
      let hsl: Hsl<Srgb> = cmyk.into();

      assert!((hsl.hue() - 180.0).abs() < 1.0);
      assert!((hsl.saturation() - 100.0).abs() < 1.0);
      assert!((hsl.lightness() - 50.0).abs() < 1.0);
    }
  }

  mod from_lms {
    use super::*;

    #[test]
    fn it_converts_from_lms_via_rgb() {
      let lms = Lms::new(0.5, 0.5, 0.5);
      let hsl: Hsl<Srgb> = lms.into();

      assert!(hsl.h().is_finite());
      assert!(hsl.s().is_finite());
      assert!(hsl.l().is_finite());
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 0.0, 0.0);
      let hsl: Hsl<Srgb> = rgb.into();

      assert!((hsl.hue() - 0.0).abs() < 1e-10);
      assert!((hsl.saturation() - 100.0).abs() < 1e-10);
      assert!((hsl.lightness() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_green() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 1.0, 0.0);
      let hsl: Hsl<Srgb> = rgb.into();

      assert!((hsl.hue() - 120.0).abs() < 1e-10);
      assert!((hsl.saturation() - 100.0).abs() < 1e-10);
      assert!((hsl.lightness() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_blue() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 1.0);
      let hsl: Hsl<Srgb> = rgb.into();

      assert!((hsl.hue() - 240.0).abs() < 1e-10);
      assert!((hsl.saturation() - 100.0).abs() < 1e-10);
      assert!((hsl.lightness() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_white_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 1.0, 1.0);
      let hsl: Hsl<Srgb> = rgb.into();

      assert!((hsl.saturation()).abs() < 1e-10);
      assert!((hsl.lightness() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_black_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 0.0);
      let hsl: Hsl<Srgb> = rgb.into();

      assert!((hsl.saturation()).abs() < 1e-10);
      assert!((hsl.lightness()).abs() < 1e-10);
    }

    #[test]
    fn it_converts_gray_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let hsl: Hsl<Srgb> = rgb.into();

      assert!((hsl.saturation()).abs() < 1e-10);
      assert!((hsl.lightness() - 50.0).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz_via_rgb() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let hsl: Hsl<Srgb> = xyz.into();

      assert!(hsl.h().is_finite());
      assert!(hsl.s().is_finite());
      assert!(hsl.l().is_finite());
    }
  }

  mod increment_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_h() {
      let mut hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      hsl.increment_h(0.25);

      assert_eq!(hsl.h(), 0.5);
    }

    #[test]
    fn it_wraps_above_one() {
      let mut hsl = Hsl::<Srgb>::new(324.0, 50.0, 50.0);
      hsl.increment_h(0.2);

      assert!((hsl.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_adds_degrees_to_hue() {
      let mut hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      hsl.increment_hue(90.0);

      assert!((hsl.hue() - 180.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_above_360() {
      let mut hsl = Hsl::<Srgb>::new(300.0, 50.0, 50.0);
      hsl.increment_hue(90.0);

      assert!((hsl.hue() - 30.0).abs() < 1e-10);
    }
  }

  mod increment_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_l() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 50.0, 25.0);
      hsl.increment_l(0.25);

      assert_eq!(hsl.l(), 0.5);
    }
  }

  mod increment_lightness {
    use super::*;

    #[test]
    fn it_adds_percentage_to_lightness() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 50.0, 25.0);
      hsl.increment_lightness(25.0);

      assert!((hsl.lightness() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_s() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 25.0, 50.0);
      hsl.increment_s(0.25);

      assert_eq!(hsl.s(), 0.5);
    }
  }

  mod increment_saturation {
    use super::*;

    #[test]
    fn it_adds_percentage_to_saturation() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 25.0, 50.0);
      hsl.increment_saturation(25.0);

      assert!((hsl.saturation() - 50.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_hsl_values_via_rgb() {
      let a = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
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
      const HSL: Hsl<Srgb> = Hsl::new_const(270.0, 50.0, 50.0);

      assert_eq!(HSL.h(), 0.75);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      const HSL: Hsl<Srgb> = Hsl::new_const(-90.0, 50.0, 50.0);

      assert_eq!(HSL.h(), 0.75);
    }

    #[test]
    fn it_normalizes_saturation_to_0_1() {
      const HSL: Hsl<Srgb> = Hsl::new_const(0.0, 75.0, 50.0);

      assert_eq!(HSL.s(), 0.75);
    }

    #[test]
    fn it_normalizes_lightness_to_0_1() {
      const HSL: Hsl<Srgb> = Hsl::new_const(0.0, 50.0, 75.0);

      assert_eq!(HSL.l(), 0.75);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_hsl_values() {
      let a = Hsl::<Srgb>::new(180.0, 50.0, 50.0);
      let b = Hsl::<Srgb>::new(180.0, 50.0, 50.0);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_hsl_values() {
      let a = Hsl::<Srgb>::new(180.0, 50.0, 50.0);
      let b = Hsl::<Srgb>::new(180.0, 50.0, 60.0);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Hsl::<Srgb>::new(180.0, 50.0, 50.0).with_alpha(0.5);
      let b = Hsl::<Srgb>::new(180.0, 50.0, 50.0);

      assert_ne!(a, b);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_multiplies_h_by_factor() {
      let mut hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      hsl.scale_h(2.0);

      assert!((hsl.h() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_when_exceeding_one() {
      let mut hsl = Hsl::<Srgb>::new(270.0, 50.0, 50.0);
      hsl.scale_h(2.0);

      assert!((hsl.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_l_by_factor() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 50.0, 25.0);
      hsl.scale_l(2.0);

      assert_eq!(hsl.l(), 0.5);
    }
  }

  mod scale_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_s_by_factor() {
      let mut hsl = Hsl::<Srgb>::new(0.0, 25.0, 50.0);
      hsl.scale_s(2.0);

      assert_eq!(hsl.s(), 0.5);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_hsl_values_via_rgb() {
      let a = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsl::<Srgb>::new(0.0, 25.0, 25.0);
      let result = a - b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.l().is_finite());
    }
  }

  mod to_rgb {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let hsl = Hsl::<Srgb>::new(0.0, 100.0, 50.0);
      let rgb: Rgb<Srgb> = hsl.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_pure_green() {
      let hsl = Hsl::<Srgb>::new(120.0, 100.0, 50.0);
      let rgb: Rgb<Srgb> = hsl.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_pure_blue() {
      let hsl = Hsl::<Srgb>::new(240.0, 100.0, 50.0);
      let rgb: Rgb<Srgb> = hsl.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_white() {
      let hsl = Hsl::<Srgb>::new(0.0, 0.0, 100.0);
      let rgb: Rgb<Srgb> = hsl.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_black() {
      let hsl = Hsl::<Srgb>::new(0.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = hsl.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_achromatic_gray() {
      let hsl = Hsl::<Srgb>::new(0.0, 0.0, 50.0);
      let rgb: Rgb<Srgb> = hsl.to_rgb();

      assert_eq!(rgb.red(), rgb.green());
      assert_eq!(rgb.green(), rgb.blue());
    }

    #[test]
    fn it_roundtrips_with_from_rgb() {
      let original = Hsl::<Srgb>::new(210.0, 80.0, 40.0);
      let rgb: Rgb<Srgb> = original.to_rgb();
      let back: Hsl<Srgb> = rgb.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.lightness() - original.lightness()).abs() < 1.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let hsl = Hsl::<Srgb>::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let rgb: Rgb<Srgb> = hsl.to_rgb();

      assert!((rgb.alpha() - 0.3).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-hwb")]
  mod to_hwb {
    use super::*;

    #[test]
    fn it_converts_black() {
      let hsl = Hsl::<Srgb>::new(0.0, 0.0, 0.0);
      let hwb = hsl.to_hwb();

      assert!((hwb.whiteness()).abs() < 1e-10);
      assert!((hwb.blackness() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_color() {
      let hsl = Hsl::<Srgb>::new(120.0, 100.0, 50.0);
      let hwb = hsl.to_hwb();

      assert!((hwb.hue() - 120.0).abs() < 1.0);
      assert!((hwb.whiteness()).abs() < 1.0);
      assert!((hwb.blackness()).abs() < 1.0);
    }

    #[test]
    fn it_converts_white() {
      let hsl = Hsl::<Srgb>::new(0.0, 0.0, 100.0);
      let hwb = hsl.to_hwb();

      assert!((hwb.whiteness() - 100.0).abs() < 1e-10);
      assert!((hwb.blackness()).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_with_from_hwb() {
      let original = Hsl::<Srgb>::new(210.0, 80.0, 40.0);
      let hwb = original.to_hwb();
      let back: Hsl<Srgb> = hwb.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.lightness() - original.lightness()).abs() < 1.0);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz_via_rgb() {
      let hsl = Hsl::<Srgb>::new(120.0, 100.0, 50.0);
      let xyz = hsl.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_roundtrips_with_from_xyz() {
      let original = Hsl::<Srgb>::new(210.0, 80.0, 40.0);
      let xyz = original.to_xyz();
      let back: Hsl<Srgb> = xyz.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.lightness() - original.lightness()).abs() < 1.0);
    }
  }

  mod with_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_new_h() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_h(0.75);

      assert_eq!(result.h(), 0.75);
      assert_eq!(result.s(), hsl.s());
      assert_eq!(result.l(), hsl.l());
    }
  }

  mod with_h_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_decremented_h() {
      let hsl = Hsl::<Srgb>::new(180.0, 50.0, 50.0);
      let result = hsl.with_h_decremented_by(0.25);

      assert_eq!(result.h(), 0.25);
      assert_eq!(hsl.h(), 0.5);
    }
  }

  mod with_h_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_incremented_h() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_h_incremented_by(0.25);

      assert_eq!(result.h(), 0.5);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_hsl_with_scaled_h() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_hsl_with_new_hue_in_degrees() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
      assert_eq!(result.s(), hsl.s());
      assert_eq!(result.l(), hsl.l());
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsl_with_decremented_hue() {
      let hsl = Hsl::<Srgb>::new(180.0, 50.0, 50.0);
      let result = hsl.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
      assert!((hsl.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsl_with_incremented_hue() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_hue_incremented_by(90.0);

      assert!((result.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_new_l() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_l(0.75);

      assert_eq!(result.l(), 0.75);
      assert_eq!(result.h(), hsl.h());
      assert_eq!(result.s(), hsl.s());
    }
  }

  mod with_l_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_decremented_l() {
      let hsl = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsl.with_l_decremented_by(0.2);

      assert!((result.l() - 0.3).abs() < 1e-10);
      assert_eq!(hsl.l(), 0.5);
    }
  }

  mod with_l_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_incremented_l() {
      let hsl = Hsl::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsl.with_l_incremented_by(0.25);

      assert_eq!(result.l(), 0.5);
    }
  }

  mod with_l_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_scaled_l() {
      let hsl = Hsl::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsl.with_l_scaled_by(2.0);

      assert_eq!(result.l(), 0.5);
    }
  }

  mod with_lightness {
    use super::*;

    #[test]
    fn it_returns_hsl_with_new_lightness_in_percent() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_lightness(75.0);

      assert!((result.lightness() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hsl.h());
      assert_eq!(result.s(), hsl.s());
    }
  }

  mod with_lightness_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsl_with_decremented_lightness() {
      let hsl = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsl.with_lightness_decremented_by(20.0);

      assert!((result.lightness() - 30.0).abs() < 1e-10);
    }
  }

  mod with_lightness_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsl_with_incremented_lightness() {
      let hsl = Hsl::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsl.with_lightness_incremented_by(25.0);

      assert!((result.lightness() - 50.0).abs() < 1e-10);
    }
  }

  mod with_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_new_s() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_s(0.75);

      assert_eq!(result.s(), 0.75);
      assert_eq!(result.h(), hsl.h());
      assert_eq!(result.l(), hsl.l());
    }
  }

  mod with_s_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_decremented_s() {
      let hsl = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsl.with_s_decremented_by(0.2);

      assert!((result.s() - 0.3).abs() < 1e-10);
      assert_eq!(hsl.s(), 0.5);
    }
  }

  mod with_s_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_incremented_s() {
      let hsl = Hsl::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsl.with_s_incremented_by(0.25);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_s_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsl_with_scaled_s() {
      let hsl = Hsl::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsl.with_s_scaled_by(2.0);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_saturation {
    use super::*;

    #[test]
    fn it_returns_hsl_with_new_saturation_in_percent() {
      let hsl = Hsl::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsl.with_saturation(75.0);

      assert!((result.saturation() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hsl.h());
      assert_eq!(result.l(), hsl.l());
    }
  }

  mod with_saturation_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsl_with_decremented_saturation() {
      let hsl = Hsl::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsl.with_saturation_decremented_by(20.0);

      assert!((result.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod with_saturation_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsl_with_incremented_saturation() {
      let hsl = Hsl::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsl.with_saturation_incremented_by(25.0);

      assert!((result.saturation() - 50.0).abs() < 1e-10);
    }
  }
}
