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
#[cfg(feature = "space-hwb")]
use crate::space::Hwb;
use crate::{
  ColorimetricContext,
  component::Component,
  space::{ColorSpace, Lms, Rgb, RgbSpec, Srgb, Xyz},
};

/// Type alias for [`Hsv`] using the HSB (Hue, Saturation, Brightness) naming convention.
pub type Hsb<S = Srgb> = Hsv<S>;

/// HSV (Hue, Saturation, Value) color space, also known as HSB (Hue, Saturation, Brightness).
///
/// A cylindrical representation of RGB colors, parameterized by an [`RgbSpec`] that
/// determines the underlying RGB space. Defaults to [`Srgb`] when not specified.
/// Components are stored normalized: hue in 0.0-1.0 (representing 0-360°),
/// saturation and value in 0.0-1.0 (representing 0-100%).
#[derive(Clone, Copy, Debug)]
pub struct Hsv<S = Srgb>
where
  S: RgbSpec,
{
  alpha: Component,
  context: ColorimetricContext,
  h: Component,
  s: Component,
  v: Component,
  _spec: PhantomData<S>,
}

impl<S> Hsv<S>
where
  S: RgbSpec,
{
  /// Creates a new HSV color from hue (0-360°), saturation (0-100%), and value (0-100%).
  pub fn new(h: impl Into<Component>, s: impl Into<Component>, v: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: S::CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      s: s.into() / 100.0,
      v: v.into() / 100.0,
      _spec: PhantomData,
    }
  }

  /// Creates a new HSV color in a const context from hue (0-360°), saturation (0-100%), and value (0-100%).
  pub const fn new_const(h: f64, s: f64, v: f64) -> Self {
    let r = (h / 360.0) % 1.0;

    Self {
      alpha: Component::new_const(1.0),
      context: S::CONTEXT,
      h: Component::new_const(if r < 0.0 { r + 1.0 } else { r }),
      s: Component::new_const(s / 100.0),
      v: Component::new_const(v / 100.0),
      _spec: PhantomData,
    }
  }

  /// Returns the normalized brightness component (0.0-1.0). Alias for [`Self::v`].
  pub fn b(&self) -> f64 {
    self.v.0
  }

  /// Returns the brightness as a percentage (0-100%). Alias for [`Self::value`].
  pub fn brightness(&self) -> f64 {
    self.v.0 * 100.0
  }

  /// Returns the [H, S, V] components as normalized values.
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

  /// Decreases the normalized brightness by the given amount. Alias for [`Self::decrement_v`].
  pub fn decrement_b(&mut self, amount: impl Into<Component>) {
    self.decrement_v(amount)
  }

  /// Decreases the value by the given amount in percentage points.
  pub fn decrement_value(&mut self, amount: impl Into<Component>) {
    self.decrement_v(amount.into() / 100.0)
  }

  /// Decreases the brightness by the given amount in percentage points. Alias for [`Self::decrement_value`].
  pub fn decrement_brightness(&mut self, amount: impl Into<Component>) {
    self.decrement_value(amount)
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

  /// Increases the normalized brightness by the given amount. Alias for [`Self::increment_v`].
  pub fn increment_b(&mut self, amount: impl Into<Component>) {
    self.increment_v(amount)
  }

  /// Increases the value by the given amount in percentage points.
  pub fn increment_value(&mut self, amount: impl Into<Component>) {
    self.increment_v(amount.into() / 100.0)
  }

  /// Increases the brightness by the given amount in percentage points. Alias for [`Self::increment_value`].
  pub fn increment_brightness(&mut self, amount: impl Into<Component>) {
    self.increment_value(amount)
  }

  /// Returns the normalized saturation component (0.0-1.0).
  pub fn s(&self) -> f64 {
    self.s.0
  }

  /// Returns the saturation as a percentage (0-100%).
  pub fn saturation(&self) -> f64 {
    self.s.0 * 100.0
  }

  /// Returns the normalized value component (0.0-1.0).
  pub fn v(&self) -> f64 {
    self.v.0
  }

  /// Returns the value as a percentage (0-100%).
  pub fn value(&self) -> f64 {
    self.v.0 * 100.0
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

  /// Scales the normalized brightness by the given factor. Alias for [`Self::scale_v`].
  pub fn scale_b(&mut self, factor: impl Into<Component>) {
    self.scale_v(factor)
  }

  /// Alias for [`Self::scale_v`].
  pub fn scale_brightness(&mut self, factor: impl Into<Component>) {
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

  /// Sets the normalized brightness component (0.0-1.0). Alias for [`Self::set_v`].
  pub fn set_b(&mut self, b: impl Into<Component>) {
    self.set_v(b)
  }

  /// Sets the value from a percentage value (0-100%).
  pub fn set_value(&mut self, value: impl Into<Component>) {
    self.v = value.into() / 100.0;
  }

  /// Sets the brightness from a percentage value (0-100%). Alias for [`Self::set_value`].
  pub fn set_brightness(&mut self, brightness: impl Into<Component>) {
    self.set_value(brightness)
  }

  #[cfg(feature = "space-hsl")]
  /// Converts this HSV color to an [`Hsl`] color in the specified RGB color space.
  pub fn to_hsl(&self) -> Hsl<S> {
    let [h, s, v] = self.components();

    let nl = v * (1.0 - (s / 2.0));
    let ns = if nl == 0.0 || nl == 1.0 {
      0.0
    } else {
      (v - nl) / nl.min(1.0 - nl)
    };

    Hsl::<S>::new(h, ns, nl).with_alpha(self.alpha)
  }

  #[cfg(feature = "space-hwb")]
  /// Converts this HSV color to an [`Hwb`] color in the specified RGB color space.
  pub fn to_hwb(&self) -> Hwb<S> {
    let [h, s, v] = self.components();

    let nw = (1.0 - s) * v;
    let nb = 1.0 - v;

    Hwb::<S>::new(h * 360.0, nw * 100.0, nb * 100.0).with_alpha(self.alpha)
  }

  /// Converts this HSV color to an [`Rgb`] color in the specified output space.
  pub fn to_rgb<OS>(&self) -> Rgb<OS>
  where
    OS: RgbSpec,
  {
    let h = self.h.0;
    let s = self.s.0;
    let v = self.v.0;

    if s <= 0.0 {
      return Rgb::<S>::from_normalized(v, v, v).to_rgb::<OS>().with_alpha(self.alpha);
    }
    if v <= 0.0 {
      return Rgb::<S>::from_normalized(0.0, 0.0, 0.0)
        .to_rgb::<OS>()
        .with_alpha(self.alpha);
    }

    let c = v * s;
    let h_prime = h * 6.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

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
    let mut hsv = *self;
    hsv.decrement_h(amount);
    hsv
  }

  /// Returns a new color with the normalized hue increased by the given amount.
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.increment_h(amount);
    hsv
  }

  /// Returns a new color with the normalized hue scaled by the given factor.
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.scale_h(factor);
    hsv
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
    let mut hsv = *self;
    hsv.decrement_hue(amount);
    hsv
  }

  /// Returns a new color with the hue increased by the given degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.increment_hue(amount);
    hsv
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
    let mut hsv = *self;
    hsv.decrement_s(amount);
    hsv
  }

  /// Returns a new color with normalized saturation increased by the given amount.
  pub fn with_s_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.increment_s(amount);
    hsv
  }

  /// Returns a new color with normalized saturation scaled by the given factor.
  pub fn with_s_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.scale_s(factor);
    hsv
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
    let mut hsv = *self;
    hsv.decrement_saturation(amount);
    hsv
  }

  /// Returns a new color with saturation increased by the given percentage points.
  pub fn with_saturation_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.increment_saturation(amount);
    hsv
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

  /// Returns a new color with the given normalized brightness. Alias for [`Self::with_v`].
  pub fn with_b(&self, b: impl Into<Component>) -> Self {
    self.with_v(b)
  }

  /// Returns a new color with normalized value decreased by the given amount.
  pub fn with_v_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.decrement_v(amount);
    hsv
  }

  /// Returns a new color with normalized brightness decreased by the given amount. Alias for [`Self::with_v_decremented_by`].
  pub fn with_b_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_v_decremented_by(amount)
  }

  /// Returns a new color with normalized value increased by the given amount.
  pub fn with_v_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.increment_v(amount);
    hsv
  }

  /// Returns a new color with normalized brightness increased by the given amount. Alias for [`Self::with_v_incremented_by`].
  pub fn with_b_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_v_incremented_by(amount)
  }

  /// Returns a new color with normalized value scaled by the given factor.
  pub fn with_v_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.scale_v(factor);
    hsv
  }

  /// Returns a new color with normalized brightness scaled by the given factor. Alias for [`Self::with_v_scaled_by`].
  pub fn with_b_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_v_scaled_by(factor)
  }

  /// Returns a new color with the given value in percentage (0-100%).
  pub fn with_value(&self, value: impl Into<Component>) -> Self {
    Self {
      v: value.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with the given brightness in percentage (0-100%). Alias for [`Self::with_value`].
  pub fn with_brightness(&self, brightness: impl Into<Component>) -> Self {
    self.with_value(brightness)
  }

  /// Returns a new color with value decreased by the given percentage points.
  pub fn with_value_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.decrement_value(amount);
    hsv
  }

  /// Returns a new color with brightness decreased by the given percentage points. Alias for [`Self::with_value_decremented_by`].
  pub fn with_brightness_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_value_decremented_by(amount)
  }

  /// Returns a new color with value increased by the given percentage points.
  pub fn with_value_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsv = *self;
    hsv.increment_value(amount);
    hsv
  }

  /// Returns a new color with brightness increased by the given percentage points. Alias for [`Self::with_value_incremented_by`].
  pub fn with_brightness_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_value_incremented_by(amount)
  }

  /// Alias for [`Self::with_v_scaled_by`].
  pub fn with_value_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_v_scaled_by(factor)
  }

  /// Alias for [`Self::with_v_scaled_by`].
  pub fn with_brightness_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_v_scaled_by(factor)
  }
}

impl<S, T> Add<T> for Hsv<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() + rhs.into().to_rgb::<S>())
  }
}

impl<S> ColorSpace<3> for Hsv<S>
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

impl<S> Display for Hsv<S>
where
  S: RgbSpec,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "HSV({:.precision$}°, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.hue(),
        self.saturation(),
        self.value(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "HSV({:.precision$}°, {:.precision$}%, {:.precision$}%)",
        self.hue(),
        self.saturation(),
        self.value()
      )
    }
  }
}

impl<S, T> Div<T> for Hsv<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() / rhs.into().to_rgb::<S>())
  }
}

impl<S, T> From<[T; 3]> for Hsv<S>
where
  S: RgbSpec,
  T: Into<Component>,
{
  fn from([h, s, v]: [T; 3]) -> Self {
    Self::new(h, s, v)
  }
}

#[cfg(feature = "space-cmy")]
impl<OS, S> From<Cmy<OS>> for Hsv<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmy: Cmy<OS>) -> Self {
    cmy.to_rgb::<S>().to_hsv()
  }
}

#[cfg(feature = "space-cmyk")]
impl<OS, S> From<Cmyk<OS>> for Hsv<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<OS>) -> Self {
    cmyk.to_rgb::<S>().to_hsv()
  }
}

#[cfg(feature = "space-hsl")]
impl<OS, S> From<Hsl<OS>> for Hsv<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hsl: Hsl<OS>) -> Self {
    hsl.to_rgb::<S>().to_hsv()
  }
}

#[cfg(feature = "space-hwb")]
impl<OS, S> From<Hwb<OS>> for Hsv<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hwb: Hwb<OS>) -> Self {
    hwb.to_rgb::<S>().to_hsv()
  }
}

impl<S> From<Lms> for Hsv<S>
where
  S: RgbSpec,
{
  fn from(lms: Lms) -> Self {
    lms.to_rgb::<S>().to_hsv()
  }
}

impl<OS, S> From<Rgb<OS>> for Hsv<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(rgb: Rgb<OS>) -> Self {
    rgb.to_rgb::<S>().to_hsv()
  }
}

impl<S> From<Xyz> for Hsv<S>
where
  S: RgbSpec,
{
  fn from(xyz: Xyz) -> Self {
    xyz.to_rgb::<S>().to_hsv()
  }
}

impl<S, T> Mul<T> for Hsv<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() * rhs.into().to_rgb::<S>())
  }
}

impl<S, T> PartialEq<T> for Hsv<S>
where
  S: RgbSpec,
  T: Into<Hsv<S>> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.h == other.h && self.s == other.s && self.v == other.v
  }
}

impl<S, T> Sub<T> for Hsv<S>
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
    fn it_adds_two_hsv_values_via_rgb() {
      let a = Hsv::<Srgb>::new(0.0, 50.0, 25.0);
      let b = Hsv::<Srgb>::new(0.0, 50.0, 25.0);
      let result = a + b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.v().is_finite());
    }
  }

  mod decrement_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_h() {
      let mut hsv = Hsv::<Srgb>::new(180.0, 50.0, 50.0);
      hsv.decrement_h(0.25);

      assert_eq!(hsv.h(), 0.25);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hsv = Hsv::<Srgb>::new(36.0, 50.0, 50.0);
      hsv.decrement_h(0.2);

      assert!((hsv.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_subtracts_degrees_from_hue() {
      let mut hsv = Hsv::<Srgb>::new(180.0, 50.0, 50.0);
      hsv.decrement_hue(90.0);

      assert!((hsv.hue() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hsv = Hsv::<Srgb>::new(30.0, 50.0, 50.0);
      hsv.decrement_hue(60.0);

      assert!((hsv.hue() - 330.0).abs() < 1e-10);
    }
  }

  mod decrement_s {
    use super::*;

    #[test]
    fn it_subtracts_from_s() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      hsv.decrement_s(0.2);

      assert!((hsv.s() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_saturation {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_saturation() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      hsv.decrement_saturation(20.0);

      assert!((hsv.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_v {
    use super::*;

    #[test]
    fn it_subtracts_from_v() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      hsv.decrement_v(0.2);

      assert!((hsv.v() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_value {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_value() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      hsv.decrement_value(20.0);

      assert!((hsv.value() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let hsv = Hsv::<Srgb>::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", hsv), "HSV(120.00°, 50.00%, 75.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let hsv = Hsv::<Srgb>::new(120.12345, 50.6789, 75.4321);

      assert_eq!(format!("{:.4}", hsv), "HSV(120.1235°, 50.6789%, 75.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let hsv = Hsv::<Srgb>::new(120.0, 50.0, 75.0).with_alpha(0.5);

      assert_eq!(format!("{}", hsv), "HSV(120.00°, 50.00%, 75.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let hsv = Hsv::<Srgb>::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", hsv), "HSV(120.00°, 50.00%, 75.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_hsv_values_via_rgb() {
      let a = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let result = a / b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.v().is_finite());
    }
  }

  #[cfg(feature = "space-cmyk")]
  mod from_cmyk {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let cmyk = Cmyk::<Srgb>::new(100.0, 0.0, 0.0, 0.0);
      let hsv: Hsv<Srgb> = cmyk.into();

      assert!((hsv.hue() - 180.0).abs() < 1.0);
      assert!((hsv.saturation() - 100.0).abs() < 1.0);
      assert!((hsv.value() - 100.0).abs() < 1.0);
    }
  }

  mod from_lms {
    use super::*;

    #[test]
    fn it_converts_from_lms_via_rgb() {
      let lms = Lms::new(0.5, 0.5, 0.5);
      let hsv: Hsv<Srgb> = lms.into();

      assert!(hsv.h().is_finite());
      assert!(hsv.s().is_finite());
      assert!(hsv.v().is_finite());
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 0.0, 0.0);
      let hsv: Hsv<Srgb> = rgb.into();

      assert!((hsv.hue() - 0.0).abs() < 1e-10);
      assert!((hsv.saturation() - 100.0).abs() < 1e-10);
      assert!((hsv.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_green() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 1.0, 0.0);
      let hsv: Hsv<Srgb> = rgb.into();

      assert!((hsv.hue() - 120.0).abs() < 1e-10);
      assert!((hsv.saturation() - 100.0).abs() < 1e-10);
      assert!((hsv.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_blue() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 1.0);
      let hsv: Hsv<Srgb> = rgb.into();

      assert!((hsv.hue() - 240.0).abs() < 1e-10);
      assert!((hsv.saturation() - 100.0).abs() < 1e-10);
      assert!((hsv.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_white_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 1.0, 1.0);
      let hsv: Hsv<Srgb> = rgb.into();

      assert!((hsv.saturation()).abs() < 1e-10);
      assert!((hsv.value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_black_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 0.0);
      let hsv: Hsv<Srgb> = rgb.into();

      assert!((hsv.saturation()).abs() < 1e-10);
      assert!((hsv.value()).abs() < 1e-10);
    }

    #[test]
    fn it_converts_gray_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let hsv: Hsv<Srgb> = rgb.into();

      assert!((hsv.saturation()).abs() < 1e-10);
      assert!((hsv.value() - 50.0).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz_via_rgb() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let hsv: Hsv<Srgb> = xyz.into();

      assert!(hsv.h().is_finite());
      assert!(hsv.s().is_finite());
      assert!(hsv.v().is_finite());
    }
  }

  mod increment_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_h() {
      let mut hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      hsv.increment_h(0.25);

      assert_eq!(hsv.h(), 0.5);
    }

    #[test]
    fn it_wraps_above_one() {
      let mut hsv = Hsv::<Srgb>::new(324.0, 50.0, 50.0);
      hsv.increment_h(0.2);

      assert!((hsv.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_adds_degrees_to_hue() {
      let mut hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      hsv.increment_hue(90.0);

      assert!((hsv.hue() - 180.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_above_360() {
      let mut hsv = Hsv::<Srgb>::new(300.0, 50.0, 50.0);
      hsv.increment_hue(90.0);

      assert!((hsv.hue() - 30.0).abs() < 1e-10);
    }
  }

  mod increment_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_s() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 25.0, 50.0);
      hsv.increment_s(0.25);

      assert_eq!(hsv.s(), 0.5);
    }
  }

  mod increment_saturation {
    use super::*;

    #[test]
    fn it_adds_percentage_to_saturation() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 25.0, 50.0);
      hsv.increment_saturation(25.0);

      assert!((hsv.saturation() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_v {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_v() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 50.0, 25.0);
      hsv.increment_v(0.25);

      assert_eq!(hsv.v(), 0.5);
    }
  }

  mod increment_value {
    use super::*;

    #[test]
    fn it_adds_percentage_to_value() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 50.0, 25.0);
      hsv.increment_value(25.0);

      assert!((hsv.value() - 50.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_hsv_values_via_rgb() {
      let a = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let result = a * b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.v().is_finite());
    }
  }

  mod new_const {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_hue_to_0_1() {
      const HSV: Hsv<Srgb> = Hsv::new_const(270.0, 50.0, 50.0);

      assert_eq!(HSV.h(), 0.75);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      const HSV: Hsv<Srgb> = Hsv::new_const(-90.0, 50.0, 50.0);

      assert_eq!(HSV.h(), 0.75);
    }

    #[test]
    fn it_normalizes_saturation_to_0_1() {
      const HSV: Hsv<Srgb> = Hsv::new_const(0.0, 75.0, 50.0);

      assert_eq!(HSV.s(), 0.75);
    }

    #[test]
    fn it_normalizes_value_to_0_1() {
      const HSV: Hsv<Srgb> = Hsv::new_const(0.0, 50.0, 75.0);

      assert_eq!(HSV.v(), 0.75);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_hsv_values() {
      let a = Hsv::<Srgb>::new(180.0, 50.0, 50.0);
      let b = Hsv::<Srgb>::new(180.0, 50.0, 50.0);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_hsv_values() {
      let a = Hsv::<Srgb>::new(180.0, 50.0, 50.0);
      let b = Hsv::<Srgb>::new(180.0, 50.0, 60.0);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Hsv::<Srgb>::new(180.0, 50.0, 50.0).with_alpha(0.5);
      let b = Hsv::<Srgb>::new(180.0, 50.0, 50.0);

      assert_ne!(a, b);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_multiplies_h_by_factor() {
      let mut hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      hsv.scale_h(2.0);

      assert!((hsv.h() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_when_exceeding_one() {
      let mut hsv = Hsv::<Srgb>::new(270.0, 50.0, 50.0);
      hsv.scale_h(2.0);

      assert!((hsv.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_s_by_factor() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 25.0, 50.0);
      hsv.scale_s(2.0);

      assert_eq!(hsv.s(), 0.5);
    }
  }

  mod scale_v {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_v_by_factor() {
      let mut hsv = Hsv::<Srgb>::new(0.0, 50.0, 25.0);
      hsv.scale_v(2.0);

      assert_eq!(hsv.v(), 0.5);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_hsv_values_via_rgb() {
      let a = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsv::<Srgb>::new(0.0, 25.0, 25.0);
      let result = a - b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.v().is_finite());
    }
  }

  mod to_rgb {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let hsv = Hsv::<Srgb>::new(0.0, 100.0, 100.0);
      let rgb: Rgb<Srgb> = hsv.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_pure_green() {
      let hsv = Hsv::<Srgb>::new(120.0, 100.0, 100.0);
      let rgb: Rgb<Srgb> = hsv.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_pure_blue() {
      let hsv = Hsv::<Srgb>::new(240.0, 100.0, 100.0);
      let rgb: Rgb<Srgb> = hsv.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_white() {
      let hsv = Hsv::<Srgb>::new(0.0, 0.0, 100.0);
      let rgb: Rgb<Srgb> = hsv.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_black() {
      let hsv = Hsv::<Srgb>::new(0.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = hsv.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_achromatic_gray() {
      let hsv = Hsv::<Srgb>::new(0.0, 0.0, 50.0);
      let rgb: Rgb<Srgb> = hsv.to_rgb();

      assert_eq!(rgb.red(), rgb.green());
      assert_eq!(rgb.green(), rgb.blue());
    }

    #[test]
    fn it_roundtrips_with_from_rgb() {
      let original = Hsv::<Srgb>::new(210.0, 80.0, 40.0);
      let rgb: Rgb<Srgb> = original.to_rgb();
      let back: Hsv<Srgb> = rgb.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.value() - original.value()).abs() < 1.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let hsv = Hsv::<Srgb>::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let rgb: Rgb<Srgb> = hsv.to_rgb();

      assert!((rgb.alpha() - 0.3).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-hwb")]
  mod to_hwb {
    use super::*;

    #[test]
    fn it_converts_black() {
      let hsv = Hsv::<Srgb>::new(0.0, 0.0, 0.0);
      let hwb = hsv.to_hwb();

      assert!((hwb.whiteness()).abs() < 1e-10);
      assert!((hwb.blackness() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_color() {
      let hsv = Hsv::<Srgb>::new(120.0, 100.0, 100.0);
      let hwb = hsv.to_hwb();

      assert!((hwb.hue() - 120.0).abs() < 1.0);
      assert!((hwb.whiteness()).abs() < 1.0);
      assert!((hwb.blackness()).abs() < 1.0);
    }

    #[test]
    fn it_converts_white() {
      let hsv = Hsv::<Srgb>::new(0.0, 0.0, 100.0);
      let hwb = hsv.to_hwb();

      assert!((hwb.whiteness() - 100.0).abs() < 1e-10);
      assert!((hwb.blackness()).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_with_from_hwb() {
      let original = Hsv::<Srgb>::new(210.0, 80.0, 40.0);
      let hwb = original.to_hwb();
      let back: Hsv<Srgb> = hwb.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.value() - original.value()).abs() < 1.0);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz_via_rgb() {
      let hsv = Hsv::<Srgb>::new(120.0, 100.0, 100.0);
      let xyz = hsv.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_roundtrips_with_from_xyz() {
      let original = Hsv::<Srgb>::new(210.0, 80.0, 40.0);
      let xyz = original.to_xyz();
      let back: Hsv<Srgb> = xyz.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.value() - original.value()).abs() < 1.0);
    }
  }

  mod with_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_new_h() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_h(0.75);

      assert_eq!(result.h(), 0.75);
      assert_eq!(result.s(), hsv.s());
      assert_eq!(result.v(), hsv.v());
    }
  }

  mod with_h_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_decremented_h() {
      let hsv = Hsv::<Srgb>::new(180.0, 50.0, 50.0);
      let result = hsv.with_h_decremented_by(0.25);

      assert_eq!(result.h(), 0.25);
      assert_eq!(hsv.h(), 0.5);
    }
  }

  mod with_h_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_incremented_h() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_h_incremented_by(0.25);

      assert_eq!(result.h(), 0.5);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_hsv_with_scaled_h() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_hsv_with_new_hue_in_degrees() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
      assert_eq!(result.s(), hsv.s());
      assert_eq!(result.v(), hsv.v());
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsv_with_decremented_hue() {
      let hsv = Hsv::<Srgb>::new(180.0, 50.0, 50.0);
      let result = hsv.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
      assert!((hsv.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsv_with_incremented_hue() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_hue_incremented_by(90.0);

      assert!((result.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_new_s() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_s(0.75);

      assert_eq!(result.s(), 0.75);
      assert_eq!(result.h(), hsv.h());
      assert_eq!(result.v(), hsv.v());
    }
  }

  mod with_s_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_decremented_s() {
      let hsv = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsv.with_s_decremented_by(0.2);

      assert!((result.s() - 0.3).abs() < 1e-10);
      assert_eq!(hsv.s(), 0.5);
    }
  }

  mod with_s_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_incremented_s() {
      let hsv = Hsv::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsv.with_s_incremented_by(0.25);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_s_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_scaled_s() {
      let hsv = Hsv::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsv.with_s_scaled_by(2.0);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_saturation {
    use super::*;

    #[test]
    fn it_returns_hsv_with_new_saturation_in_percent() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_saturation(75.0);

      assert!((result.saturation() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hsv.h());
      assert_eq!(result.v(), hsv.v());
    }
  }

  mod with_saturation_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsv_with_decremented_saturation() {
      let hsv = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsv.with_saturation_decremented_by(20.0);

      assert!((result.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod with_saturation_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsv_with_incremented_saturation() {
      let hsv = Hsv::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsv.with_saturation_incremented_by(25.0);

      assert!((hsv.saturation() - 25.0).abs() < 1e-10);
      assert!((result.saturation() - 50.0).abs() < 1e-10);
    }
  }

  mod with_v {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_new_v() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_v(0.75);

      assert_eq!(result.v(), 0.75);
      assert_eq!(result.h(), hsv.h());
      assert_eq!(result.s(), hsv.s());
    }
  }

  mod with_v_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_decremented_v() {
      let hsv = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsv.with_v_decremented_by(0.2);

      assert!((result.v() - 0.3).abs() < 1e-10);
      assert_eq!(hsv.v(), 0.5);
    }
  }

  mod with_v_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_incremented_v() {
      let hsv = Hsv::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsv.with_v_incremented_by(0.25);

      assert_eq!(result.v(), 0.5);
    }
  }

  mod with_v_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsv_with_scaled_v() {
      let hsv = Hsv::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsv.with_v_scaled_by(2.0);

      assert_eq!(result.v(), 0.5);
    }
  }

  mod with_value {
    use super::*;

    #[test]
    fn it_returns_hsv_with_new_value_in_percent() {
      let hsv = Hsv::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsv.with_value(75.0);

      assert!((result.value() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hsv.h());
      assert_eq!(result.s(), hsv.s());
    }
  }

  mod with_value_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsv_with_decremented_value() {
      let hsv = Hsv::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsv.with_value_decremented_by(20.0);

      assert!((result.value() - 30.0).abs() < 1e-10);
    }
  }

  mod with_value_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsv_with_incremented_value() {
      let hsv = Hsv::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsv.with_value_incremented_by(25.0);

      assert!((result.value() - 50.0).abs() < 1e-10);
    }
  }
}
