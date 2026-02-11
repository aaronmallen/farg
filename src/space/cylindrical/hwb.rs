use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  marker::PhantomData,
  ops::{Add, Div, Mul, Sub},
};

#[cfg(feature = "space-cmy")]
use crate::space::Cmy;
#[cfg(feature = "space-cmyk")]
use crate::space::Cmyk;
#[cfg(feature = "space-hsl")]
use crate::space::Hsl;
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

/// HWB (Hue, Whiteness, Blackness) color space.
///
/// A cylindrical representation of RGB colors, parameterized by an [`RgbSpec`] that
/// determines the underlying RGB space. Defaults to [`Srgb`] when not specified.
/// Components are stored normalized: hue in 0.0-1.0 (representing 0-360°),
/// whiteness and blackness in 0.0-1.0 (representing 0-100%).
#[derive(Clone, Copy, Debug)]
pub struct Hwb<S = Srgb>
where
  S: RgbSpec,
{
  alpha: Component,
  context: ColorimetricContext,
  h: Component,
  w: Component,
  b: Component,
  _spec: PhantomData<S>,
}

impl<S> Hwb<S>
where
  S: RgbSpec,
{
  /// Creates a new HWB color from hue (0-360°), whiteness (0-100%), and blackness (0-100%).
  pub fn new(h: impl Into<Component>, w: impl Into<Component>, b: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: S::CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      w: w.into() / 100.0,
      b: b.into() / 100.0,
      _spec: PhantomData,
    }
  }

  /// Creates a new HWB color in a const context from hue (0-360°), whiteness (0-100%), and blackness (0-100%).
  pub const fn new_const(h: f64, w: f64, b: f64) -> Self {
    let r = (h / 360.0) % 1.0;

    Self {
      alpha: Component::new_const(1.0),
      context: S::CONTEXT,
      h: Component::new_const(if r < 0.0 { r + 1.0 } else { r }),
      w: Component::new_const(w / 100.0),
      b: Component::new_const(b / 100.0),
      _spec: PhantomData,
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

  /// Returns the [H, W, B] components as normalized values.
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

  #[cfg(feature = "space-hsv")]
  /// Converts this HWB color to an [`Hsb`] color in the specified RGB color space. Alias for [`Self::to_hsv`].
  pub fn to_hsb(&self) -> Hsb<S> {
    self.to_hsv()
  }

  #[cfg(feature = "space-hsl")]
  /// Converts this HWB color to an [`Hsl`] color in the specified RGB color space.
  pub fn to_hsl(&self) -> Hsl<S> {
    let [h, w, b] = self.components();

    let (s, v) = if w + b >= 1.0 {
      (0.0, w / (w + b))
    } else {
      let v = 1.0 - b;
      (1.0 - (w / v), v)
    };

    let l = v * (1.0 - (s / 2.0));
    let sl = if l == 0.0 || l == 1.0 {
      0.0
    } else {
      (v - l) / l.min(1.0 - l)
    };

    Hsl::<S>::new(h * 360.0, sl * 100.0, l * 100.0).with_alpha(self.alpha)
  }

  #[cfg(feature = "space-hsv")]
  /// Converts this HWB color to an [`Hsv`] color in the specified RGB color space.
  pub fn to_hsv(&self) -> Hsv<S> {
    let [h, w, b] = self.components();

    let v = 1.0 - b;
    let s = if v == 0.0 { 0.0 } else { 1.0 - (w / v) };

    Hsv::<S>::new(h * 360.0, s * 100.0, v * 100.0).with_alpha(self.alpha)
  }

  /// Converts this HWB color to an [`Rgb`] color in the specified output space.
  pub fn to_rgb<OS>(&self) -> Rgb<OS>
  where
    OS: RgbSpec,
  {
    let h = self.h.0;
    let w = self.w.0;
    let b = self.b.0;

    // When W + B >= 1, the color is a shade of gray
    if w + b >= 1.0 {
      let gray = w / (w + b);
      return Rgb::<S>::from_normalized(gray, gray, gray)
        .to_rgb::<OS>()
        .with_alpha(self.alpha);
    }

    // Compute the pure hue RGB values
    let h_prime = h * 6.0;
    let c = 1.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());

    let [r1, g1, b1] = match h_prime.floor().rem_euclid(6.0) {
      0.0 => [c, x, 0.0],
      1.0 => [x, c, 0.0],
      2.0 => [0.0, c, x],
      3.0 => [0.0, x, c],
      4.0 => [x, 0.0, c],
      5.0 => [c, 0.0, x],
      _ => unreachable!(),
    };

    // Scale by (1 - W - B) and add W
    let scale = 1.0 - w - b;
    Rgb::<S>::from_normalized(r1 * scale + w, g1 * scale + w, b1 * scale + w)
      .to_rgb::<OS>()
      .with_alpha(self.alpha)
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
    let mut hwb = *self;
    hwb.decrement_b(amount);
    hwb
  }

  /// Returns a new color with normalized blackness increased by the given amount.
  pub fn with_b_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.increment_b(amount);
    hwb
  }

  /// Returns a new color with normalized blackness scaled by the given factor.
  pub fn with_b_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.scale_b(factor);
    hwb
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
    let mut hwb = *self;
    hwb.decrement_blackness(amount);
    hwb
  }

  /// Returns a new color with blackness increased by the given percentage points.
  pub fn with_blackness_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.increment_blackness(amount);
    hwb
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
    let mut hwb = *self;
    hwb.decrement_h(amount);
    hwb
  }

  /// Returns a new color with the normalized hue increased by the given amount.
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.increment_h(amount);
    hwb
  }

  /// Returns a new color with the normalized hue scaled by the given factor.
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.scale_h(factor);
    hwb
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
    let mut hwb = *self;
    hwb.decrement_hue(amount);
    hwb
  }

  /// Returns a new color with the hue increased by the given degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.increment_hue(amount);
    hwb
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
    let mut hwb = *self;
    hwb.decrement_w(amount);
    hwb
  }

  /// Returns a new color with normalized whiteness increased by the given amount.
  pub fn with_w_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.increment_w(amount);
    hwb
  }

  /// Returns a new color with normalized whiteness scaled by the given factor.
  pub fn with_w_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.scale_w(factor);
    hwb
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
    let mut hwb = *self;
    hwb.decrement_whiteness(amount);
    hwb
  }

  /// Returns a new color with whiteness increased by the given percentage points.
  pub fn with_whiteness_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hwb = *self;
    hwb.increment_whiteness(amount);
    hwb
  }

  /// Alias for [`Self::with_w_scaled_by`].
  pub fn with_whiteness_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_w_scaled_by(factor)
  }
}

impl<S, T> Add<T> for Hwb<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() + rhs.into().to_rgb::<S>())
  }
}

impl<S> ColorSpace<3> for Hwb<S>
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
    self.alpha = alpha.into().clamp(0.0, 1.0);
  }

  fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_components(components)
  }

  fn to_xyz(&self) -> Xyz {
    self.to_rgb::<S>().to_xyz()
  }
}

impl<S> Display for Hwb<S>
where
  S: RgbSpec,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "HWB({:.precision$}°, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.hue(),
        self.whiteness(),
        self.blackness(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "HWB({:.precision$}°, {:.precision$}%, {:.precision$}%)",
        self.hue(),
        self.whiteness(),
        self.blackness()
      )
    }
  }
}

impl<S, T> Div<T> for Hwb<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() / rhs.into().to_rgb::<S>())
  }
}

impl<S, T> From<[T; 3]> for Hwb<S>
where
  S: RgbSpec,
  T: Into<Component>,
{
  fn from([h, w, b]: [T; 3]) -> Self {
    Self::new(h, w, b)
  }
}

#[cfg(feature = "space-cmy")]
impl<OS, S> From<Cmy<OS>> for Hwb<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmy: Cmy<OS>) -> Self {
    cmy.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-cmyk")]
impl<OS, S> From<Cmyk<OS>> for Hwb<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<OS>) -> Self {
    cmyk.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-hsl")]
impl<OS, S> From<Hsl<OS>> for Hwb<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hsl: Hsl<OS>) -> Self {
    hsl.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-hsv")]
impl<OS, S> From<Hsv<OS>> for Hwb<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hsv: Hsv<OS>) -> Self {
    hsv.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-lab")]
impl<S> From<Lab> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(lab: Lab) -> Self {
    lab.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-lch")]
impl<S> From<Lch> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(lch: Lch) -> Self {
    lch.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-luv")]
impl<S> From<Luv> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(luv: Luv) -> Self {
    luv.to_rgb::<S>().to_hwb()
  }
}

impl<S> From<Lms> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(lms: Lms) -> Self {
    lms.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-okhsl")]
impl<S> From<Okhsl> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-okhsv")]
impl<S> From<Okhsv> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-oklab")]
impl<S> From<Oklab> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(oklab: Oklab) -> Self {
    oklab.to_rgb::<S>().to_hwb()
  }
}

#[cfg(feature = "space-oklch")]
impl<S> From<Oklch> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(oklch: Oklch) -> Self {
    oklch.to_rgb::<S>().to_hwb()
  }
}

impl<OS, S> From<Rgb<OS>> for Hwb<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(rgb: Rgb<OS>) -> Self {
    rgb.to_rgb::<S>().to_hwb()
  }
}

impl<S> From<Xyz> for Hwb<S>
where
  S: RgbSpec,
{
  fn from(xyz: Xyz) -> Self {
    xyz.to_rgb::<S>().to_hwb()
  }
}

impl<S, T> Mul<T> for Hwb<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() * rhs.into().to_rgb::<S>())
  }
}

impl<S, T> PartialEq<T> for Hwb<S>
where
  S: RgbSpec,
  T: Into<Hwb<S>> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.h == other.h && self.w == other.w && self.b == other.b
  }
}

impl<S, T> Sub<T> for Hwb<S>
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
    fn it_adds_two_hwb_values_via_rgb() {
      let a = Hwb::<Srgb>::new(0.0, 25.0, 25.0);
      let b = Hwb::<Srgb>::new(0.0, 25.0, 25.0);
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
      let mut hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      hwb.decrement_b(0.2);

      assert!((hwb.b() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_blackness {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_blackness() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      hwb.decrement_blackness(20.0);

      assert!((hwb.blackness() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_h() {
      let mut hwb = Hwb::<Srgb>::new(180.0, 50.0, 50.0);
      hwb.decrement_h(0.25);

      assert_eq!(hwb.h(), 0.25);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hwb = Hwb::<Srgb>::new(36.0, 50.0, 50.0);
      hwb.decrement_h(0.2);

      assert!((hwb.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_subtracts_degrees_from_hue() {
      let mut hwb = Hwb::<Srgb>::new(180.0, 50.0, 50.0);
      hwb.decrement_hue(90.0);

      assert!((hwb.hue() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hwb = Hwb::<Srgb>::new(30.0, 50.0, 50.0);
      hwb.decrement_hue(60.0);

      assert!((hwb.hue() - 330.0).abs() < 1e-10);
    }
  }

  mod decrement_w {
    use super::*;

    #[test]
    fn it_subtracts_from_w() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      hwb.decrement_w(0.2);

      assert!((hwb.w() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_whiteness {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_whiteness() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      hwb.decrement_whiteness(20.0);

      assert!((hwb.whiteness() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let hwb = Hwb::<Srgb>::new(120.0, 25.0, 50.0);

      assert_eq!(format!("{}", hwb), "HWB(120.00°, 25.00%, 50.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let hwb = Hwb::<Srgb>::new(120.12345, 25.6789, 50.4321);

      assert_eq!(format!("{:.4}", hwb), "HWB(120.1235°, 25.6789%, 50.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let hwb = Hwb::<Srgb>::new(120.0, 25.0, 50.0).with_alpha(0.5);

      assert_eq!(format!("{}", hwb), "HWB(120.00°, 25.00%, 50.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let hwb = Hwb::<Srgb>::new(120.0, 25.0, 50.0);

      assert_eq!(format!("{}", hwb), "HWB(120.00°, 25.00%, 50.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_hwb_values_via_rgb() {
      let a = Hwb::<Srgb>::new(0.0, 25.0, 25.0);
      let b = Hwb::<Srgb>::new(0.0, 25.0, 25.0);
      let result = a / b;

      assert!(result.h().is_finite());
      assert!(result.w().is_finite());
      assert!(result.b().is_finite());
    }
  }

  #[cfg(feature = "space-hsl")]
  mod from_hsl {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let hsl = Hsl::<Srgb>::new(120.0, 100.0, 50.0);
      let hwb: Hwb<Srgb> = hsl.into();

      assert!((hwb.hue() - 120.0).abs() < 1.0);
      assert!((hwb.whiteness() - 0.0).abs() < 1.0);
      assert!((hwb.blackness() - 0.0).abs() < 1.0);
    }
  }

  #[cfg(feature = "space-hsv")]
  mod from_hsv {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let hsv = Hsv::<Srgb>::new(120.0, 100.0, 100.0);
      let hwb: Hwb<Srgb> = hsv.into();

      assert!((hwb.hue() - 120.0).abs() < 1.0);
      assert!((hwb.whiteness() - 0.0).abs() < 1.0);
      assert!((hwb.blackness() - 0.0).abs() < 1.0);
    }
  }

  #[cfg(feature = "space-cmyk")]
  mod from_cmyk {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let cmyk = Cmyk::<Srgb>::new(100.0, 0.0, 0.0, 0.0);
      let hwb: Hwb<Srgb> = cmyk.into();

      assert!((hwb.hue() - 180.0).abs() < 1.0);
      assert!((hwb.whiteness()).abs() < 1.0);
      assert!((hwb.blackness()).abs() < 1.0);
    }
  }

  mod from_lms {
    use super::*;

    #[test]
    fn it_converts_from_lms_via_rgb() {
      let lms = Lms::new(0.5, 0.5, 0.5);
      let hwb: Hwb<Srgb> = lms.into();

      assert!(hwb.h().is_finite());
      assert!(hwb.w().is_finite());
      assert!(hwb.b().is_finite());
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 0.0, 0.0);
      let hwb: Hwb<Srgb> = rgb.into();

      assert!((hwb.hue() - 0.0).abs() < 1e-10);
      assert!((hwb.whiteness() - 0.0).abs() < 1e-10);
      assert!((hwb.blackness() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_green() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 1.0, 0.0);
      let hwb: Hwb<Srgb> = rgb.into();

      assert!((hwb.hue() - 120.0).abs() < 1e-10);
      assert!((hwb.whiteness() - 0.0).abs() < 1e-10);
      assert!((hwb.blackness() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_blue() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 1.0);
      let hwb: Hwb<Srgb> = rgb.into();

      assert!((hwb.hue() - 240.0).abs() < 1e-10);
      assert!((hwb.whiteness() - 0.0).abs() < 1e-10);
      assert!((hwb.blackness() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 1.0, 1.0);
      let hwb: Hwb<Srgb> = rgb.into();

      assert!((hwb.whiteness() - 100.0).abs() < 1e-10);
      assert!((hwb.blackness()).abs() < 1e-10);
    }

    #[test]
    fn it_converts_black() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 0.0);
      let hwb: Hwb<Srgb> = rgb.into();

      assert!((hwb.whiteness()).abs() < 1e-10);
      assert!((hwb.blackness() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_gray_50_percent() {
      let rgb = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let hwb: Hwb<Srgb> = rgb.into();

      assert!((hwb.whiteness() - 50.0).abs() < 1e-10);
      assert!((hwb.blackness() - 50.0).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz_via_rgb() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let hwb: Hwb<Srgb> = xyz.into();

      assert!(hwb.h().is_finite());
      assert!(hwb.w().is_finite());
      assert!(hwb.b().is_finite());
    }
  }

  mod increment_b {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_b() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 50.0, 25.0);
      hwb.increment_b(0.25);

      assert_eq!(hwb.b(), 0.5);
    }
  }

  mod increment_blackness {
    use super::*;

    #[test]
    fn it_adds_percentage_to_blackness() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 50.0, 25.0);
      hwb.increment_blackness(25.0);

      assert!((hwb.blackness() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_h() {
      let mut hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      hwb.increment_h(0.25);

      assert_eq!(hwb.h(), 0.5);
    }

    #[test]
    fn it_wraps_above_one() {
      let mut hwb = Hwb::<Srgb>::new(324.0, 50.0, 50.0);
      hwb.increment_h(0.2);

      assert!((hwb.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_adds_degrees_to_hue() {
      let mut hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      hwb.increment_hue(90.0);

      assert!((hwb.hue() - 180.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_above_360() {
      let mut hwb = Hwb::<Srgb>::new(300.0, 50.0, 50.0);
      hwb.increment_hue(90.0);

      assert!((hwb.hue() - 30.0).abs() < 1e-10);
    }
  }

  mod increment_w {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_w() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 25.0, 50.0);
      hwb.increment_w(0.25);

      assert_eq!(hwb.w(), 0.5);
    }
  }

  mod increment_whiteness {
    use super::*;

    #[test]
    fn it_adds_percentage_to_whiteness() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 25.0, 50.0);
      hwb.increment_whiteness(25.0);

      assert!((hwb.whiteness() - 50.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_hwb_values_via_rgb() {
      let a = Hwb::<Srgb>::new(0.0, 25.0, 25.0);
      let b = Hwb::<Srgb>::new(0.0, 25.0, 25.0);
      let result = a * b;

      assert!(result.h().is_finite());
      assert!(result.w().is_finite());
      assert!(result.b().is_finite());
    }
  }

  mod new_const {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_hue_to_0_1() {
      const HWB: Hwb<Srgb> = Hwb::new_const(270.0, 50.0, 50.0);

      assert_eq!(HWB.h(), 0.75);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      const HWB: Hwb<Srgb> = Hwb::new_const(-90.0, 50.0, 50.0);

      assert_eq!(HWB.h(), 0.75);
    }

    #[test]
    fn it_normalizes_whiteness_to_0_1() {
      const HWB: Hwb<Srgb> = Hwb::new_const(0.0, 75.0, 50.0);

      assert_eq!(HWB.w(), 0.75);
    }

    #[test]
    fn it_normalizes_blackness_to_0_1() {
      const HWB: Hwb<Srgb> = Hwb::new_const(0.0, 50.0, 75.0);

      assert_eq!(HWB.b(), 0.75);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_hwb_values() {
      let a = Hwb::<Srgb>::new(180.0, 25.0, 25.0);
      let b = Hwb::<Srgb>::new(180.0, 25.0, 25.0);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_hwb_values() {
      let a = Hwb::<Srgb>::new(180.0, 25.0, 25.0);
      let b = Hwb::<Srgb>::new(180.0, 25.0, 50.0);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Hwb::<Srgb>::new(180.0, 25.0, 25.0).with_alpha(0.5);
      let b = Hwb::<Srgb>::new(180.0, 25.0, 25.0);

      assert_ne!(a, b);
    }
  }

  mod scale_b {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_b_by_factor() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 50.0, 25.0);
      hwb.scale_b(2.0);

      assert_eq!(hwb.b(), 0.5);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_multiplies_h_by_factor() {
      let mut hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      hwb.scale_h(2.0);

      assert!((hwb.h() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_when_exceeding_one() {
      let mut hwb = Hwb::<Srgb>::new(270.0, 50.0, 50.0);
      hwb.scale_h(2.0);

      assert!((hwb.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_w {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_w_by_factor() {
      let mut hwb = Hwb::<Srgb>::new(0.0, 25.0, 50.0);
      hwb.scale_w(2.0);

      assert_eq!(hwb.w(), 0.5);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_hwb_values_via_rgb() {
      let a = Hwb::<Srgb>::new(0.0, 50.0, 25.0);
      let b = Hwb::<Srgb>::new(0.0, 25.0, 12.5);
      let result = a - b;

      assert!(result.h().is_finite());
      assert!(result.w().is_finite());
      assert!(result.b().is_finite());
    }
  }

  mod to_rgb {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let hwb = Hwb::<Srgb>::new(0.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = hwb.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_pure_green() {
      let hwb = Hwb::<Srgb>::new(120.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = hwb.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_pure_blue() {
      let hwb = Hwb::<Srgb>::new(240.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = hwb.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_white() {
      let hwb = Hwb::<Srgb>::new(0.0, 100.0, 0.0);
      let rgb: Rgb<Srgb> = hwb.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_black() {
      let hwb = Hwb::<Srgb>::new(0.0, 0.0, 100.0);
      let rgb: Rgb<Srgb> = hwb.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_achromatic_gray() {
      let hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      let rgb: Rgb<Srgb> = hwb.to_rgb();

      assert_eq!(rgb.red(), rgb.green());
      assert_eq!(rgb.green(), rgb.blue());
    }

    #[test]
    fn it_normalizes_when_w_plus_b_exceeds_100() {
      let hwb = Hwb::<Srgb>::new(0.0, 75.0, 75.0);
      let rgb: Rgb<Srgb> = hwb.to_rgb();

      // Should be 50% gray (75 / 150 = 0.5)
      assert_eq!(rgb.red(), rgb.green());
      assert_eq!(rgb.green(), rgb.blue());
    }

    #[test]
    fn it_roundtrips_with_from_rgb() {
      let original = Hwb::<Srgb>::new(210.0, 20.0, 40.0);
      let rgb: Rgb<Srgb> = original.to_rgb();
      let back: Hwb<Srgb> = rgb.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.whiteness() - original.whiteness()).abs() < 1.0);
      assert!((back.blackness() - original.blackness()).abs() < 1.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let hwb = Hwb::<Srgb>::new(120.0, 25.0, 25.0).with_alpha(0.3);
      let rgb: Rgb<Srgb> = hwb.to_rgb();

      assert!((rgb.alpha() - 0.3).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-hsl")]
  mod to_hsl {
    use super::*;

    #[test]
    fn it_converts_pure_green() {
      let hwb = Hwb::<Srgb>::new(120.0, 0.0, 0.0);
      let hsl = hwb.to_hsl();

      assert!((hsl.hue() - 120.0).abs() < 1.0);
      assert!((hsl.saturation() - 100.0).abs() < 1.0);
      assert!((hsl.lightness() - 50.0).abs() < 1.0);
    }

    #[test]
    fn it_roundtrips_with_from_hsl() {
      let original = Hwb::<Srgb>::new(210.0, 20.0, 40.0);
      let hsl = original.to_hsl();
      let back: Hwb<Srgb> = hsl.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.whiteness() - original.whiteness()).abs() < 1.0);
      assert!((back.blackness() - original.blackness()).abs() < 1.0);
    }
  }

  #[cfg(feature = "space-hsv")]
  mod to_hsv {
    use super::*;

    #[test]
    fn it_converts_pure_green() {
      let hwb = Hwb::<Srgb>::new(120.0, 0.0, 0.0);
      let hsv = hwb.to_hsv();

      assert!((hsv.hue() - 120.0).abs() < 1.0);
      assert!((hsv.saturation() - 100.0).abs() < 1.0);
      assert!((hsv.value() - 100.0).abs() < 1.0);
    }

    #[test]
    fn it_roundtrips_with_from_hsv() {
      let original = Hwb::<Srgb>::new(210.0, 20.0, 40.0);
      let hsv = original.to_hsv();
      let back: Hwb<Srgb> = hsv.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.whiteness() - original.whiteness()).abs() < 1.0);
      assert!((back.blackness() - original.blackness()).abs() < 1.0);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz_via_rgb() {
      let hwb = Hwb::<Srgb>::new(120.0, 0.0, 0.0);
      let xyz = hwb.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_roundtrips_with_from_xyz() {
      let original = Hwb::<Srgb>::new(210.0, 20.0, 40.0);
      let xyz = original.to_xyz();
      let back: Hwb<Srgb> = xyz.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.whiteness() - original.whiteness()).abs() < 1.0);
      assert!((back.blackness() - original.blackness()).abs() < 1.0);
    }
  }

  mod with_b {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_new_b() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_b(0.75);

      assert_eq!(result.b(), 0.75);
      assert_eq!(result.h(), hwb.h());
      assert_eq!(result.w(), hwb.w());
    }
  }

  mod with_b_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_decremented_b() {
      let hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hwb.with_b_decremented_by(0.2);

      assert!((result.b() - 0.3).abs() < 1e-10);
      assert_eq!(hwb.b(), 0.5);
    }
  }

  mod with_b_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_incremented_b() {
      let hwb = Hwb::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hwb.with_b_incremented_by(0.25);

      assert_eq!(result.b(), 0.5);
    }
  }

  mod with_b_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_scaled_b() {
      let hwb = Hwb::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hwb.with_b_scaled_by(2.0);

      assert_eq!(result.b(), 0.5);
    }
  }

  mod with_blackness {
    use super::*;

    #[test]
    fn it_returns_hwb_with_new_blackness_in_percent() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_blackness(75.0);

      assert!((result.blackness() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hwb.h());
      assert_eq!(result.w(), hwb.w());
    }
  }

  mod with_blackness_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hwb_with_decremented_blackness() {
      let hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hwb.with_blackness_decremented_by(20.0);

      assert!((result.blackness() - 30.0).abs() < 1e-10);
    }
  }

  mod with_blackness_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hwb_with_incremented_blackness() {
      let hwb = Hwb::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hwb.with_blackness_incremented_by(25.0);

      assert!((result.blackness() - 50.0).abs() < 1e-10);
    }
  }

  mod with_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_new_h() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_h(0.75);

      assert_eq!(result.h(), 0.75);
      assert_eq!(result.w(), hwb.w());
      assert_eq!(result.b(), hwb.b());
    }
  }

  mod with_h_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_decremented_h() {
      let hwb = Hwb::<Srgb>::new(180.0, 50.0, 50.0);
      let result = hwb.with_h_decremented_by(0.25);

      assert_eq!(result.h(), 0.25);
      assert_eq!(hwb.h(), 0.5);
    }
  }

  mod with_h_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_incremented_h() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_h_incremented_by(0.25);

      assert_eq!(result.h(), 0.5);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_hwb_with_scaled_h() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_hwb_with_new_hue_in_degrees() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
      assert_eq!(result.w(), hwb.w());
      assert_eq!(result.b(), hwb.b());
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hwb_with_decremented_hue() {
      let hwb = Hwb::<Srgb>::new(180.0, 50.0, 50.0);
      let result = hwb.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
      assert!((hwb.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hwb_with_incremented_hue() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_hue_incremented_by(90.0);

      assert!((result.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_w {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_new_w() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_w(0.75);

      assert_eq!(result.w(), 0.75);
      assert_eq!(result.h(), hwb.h());
      assert_eq!(result.b(), hwb.b());
    }
  }

  mod with_w_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_decremented_w() {
      let hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hwb.with_w_decremented_by(0.2);

      assert!((result.w() - 0.3).abs() < 1e-10);
      assert_eq!(hwb.w(), 0.5);
    }
  }

  mod with_w_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_incremented_w() {
      let hwb = Hwb::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hwb.with_w_incremented_by(0.25);

      assert_eq!(result.w(), 0.5);
    }
  }

  mod with_w_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hwb_with_scaled_w() {
      let hwb = Hwb::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hwb.with_w_scaled_by(2.0);

      assert_eq!(result.w(), 0.5);
    }
  }

  mod with_whiteness {
    use super::*;

    #[test]
    fn it_returns_hwb_with_new_whiteness_in_percent() {
      let hwb = Hwb::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hwb.with_whiteness(75.0);

      assert!((result.whiteness() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hwb.h());
      assert_eq!(result.b(), hwb.b());
    }
  }

  mod with_whiteness_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hwb_with_decremented_whiteness() {
      let hwb = Hwb::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hwb.with_whiteness_decremented_by(20.0);

      assert!((result.whiteness() - 30.0).abs() < 1e-10);
    }
  }

  mod with_whiteness_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hwb_with_incremented_whiteness() {
      let hwb = Hwb::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hwb.with_whiteness_incremented_by(25.0);

      assert!((result.whiteness() - 50.0).abs() < 1e-10);
    }
  }
}
