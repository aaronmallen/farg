use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  marker::PhantomData,
  ops::{Add, Div, Mul, Sub},
};

#[cfg(feature = "space-cmy")]
use crate::space::Cmy;
#[cfg(feature = "space-cmyk")]
use crate::space::Cmyk;
#[cfg(feature = "space-hpluv")]
use crate::space::Hpluv;
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
  ColorimetricContext,
  component::Component,
  space::{ColorSpace, Lms, Rgb, RgbSpec, Srgb, Xyz},
};

/// HSI (Hue, Saturation, Intensity) color space.
///
/// A cylindrical representation of RGB colors where intensity is the arithmetic mean
/// of the R, G, B channels. Parameterized by an [`RgbSpec`] that determines the
/// underlying RGB space. Defaults to [`Srgb`] when not specified.
///
/// Unlike HSL and HSV, HSI treats all three channels symmetrically in its intensity
/// calculation, making it useful in image processing and computer vision where
/// equal-weight channel averaging is desirable.
///
/// Components are stored normalized: hue in 0.0-1.0 (representing 0-360°),
/// saturation and intensity in 0.0-1.0 (representing 0-100%).
#[derive(Clone, Copy, Debug)]
pub struct Hsi<S = Srgb>
where
  S: RgbSpec,
{
  alpha: Component,
  context: ColorimetricContext,
  h: Component,
  i: Component,
  s: Component,
  _spec: PhantomData<S>,
}

impl<S> Hsi<S>
where
  S: RgbSpec,
{
  /// Creates a new HSI color from hue (0-360°), saturation (0-100%), and intensity (0-100%).
  pub fn new(h: impl Into<Component>, s: impl Into<Component>, i: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: S::CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      i: i.into() / 100.0,
      s: s.into() / 100.0,
      _spec: PhantomData,
    }
  }

  /// Creates a new HSI color in a const context from hue (0-360°), saturation (0-100%), and intensity (0-100%).
  pub const fn new_const(h: f64, s: f64, i: f64) -> Self {
    let r = (h / 360.0) % 1.0;

    Self {
      alpha: Component::new_const(1.0),
      context: S::CONTEXT,
      h: Component::new_const(if r < 0.0 { r + 1.0 } else { r }),
      i: Component::new_const(i / 100.0),
      s: Component::new_const(s / 100.0),
      _spec: PhantomData,
    }
  }

  /// Returns the [H, S, I] components as normalized values.
  pub fn components(&self) -> [f64; 3] {
    [self.h.0, self.s.0, self.i.0]
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

  /// Decreases the normalized intensity by the given amount.
  pub fn decrement_i(&mut self, amount: impl Into<Component>) {
    self.i -= amount.into();
  }

  /// Decreases the intensity by the given amount in percentage points.
  pub fn decrement_intensity(&mut self, amount: impl Into<Component>) {
    self.decrement_i(amount.into() / 100.0)
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

  /// Returns the normalized intensity component (0.0-1.0).
  pub fn i(&self) -> f64 {
    self.i.0
  }

  /// Increases the normalized hue by the given amount (wraps around 0.0-1.0).
  pub fn increment_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 + amount.into().0).rem_euclid(1.0));
  }

  /// Increases the hue by the given amount in degrees (wraps around 0-360°).
  pub fn increment_hue(&mut self, amount: impl Into<Component>) {
    self.increment_h(amount.into() / 360.0)
  }

  /// Increases the normalized intensity by the given amount.
  pub fn increment_i(&mut self, amount: impl Into<Component>) {
    self.i += amount.into();
  }

  /// Increases the intensity by the given amount in percentage points.
  pub fn increment_intensity(&mut self, amount: impl Into<Component>) {
    self.increment_i(amount.into() / 100.0)
  }

  /// Increases the normalized saturation by the given amount.
  pub fn increment_s(&mut self, amount: impl Into<Component>) {
    self.s += amount.into();
  }

  /// Increases the saturation by the given amount in percentage points.
  pub fn increment_saturation(&mut self, amount: impl Into<Component>) {
    self.increment_s(amount.into() / 100.0)
  }

  /// Returns the intensity as a percentage (0-100%).
  pub fn intensity(&self) -> f64 {
    self.i.0 * 100.0
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

  /// Scales the normalized intensity by the given factor.
  pub fn scale_i(&mut self, factor: impl Into<Component>) {
    self.i *= factor.into();
  }

  /// Alias for [`Self::scale_i`].
  pub fn scale_intensity(&mut self, factor: impl Into<Component>) {
    self.scale_i(factor)
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
    self.set_i(components[2].clone());
  }

  /// Sets the normalized hue component (0.0-1.0).
  pub fn set_h(&mut self, h: impl Into<Component>) {
    self.h = h.into();
  }

  /// Sets the hue from a value in degrees (0-360°).
  pub fn set_hue(&mut self, hue: impl Into<Component>) {
    self.h = Component::new((hue.into().0 / 360.0).rem_euclid(1.0));
  }

  /// Sets the normalized intensity component (0.0-1.0).
  pub fn set_i(&mut self, i: impl Into<Component>) {
    self.i = i.into();
  }

  /// Sets the intensity from a percentage value (0-100%).
  pub fn set_intensity(&mut self, intensity: impl Into<Component>) {
    self.i = intensity.into() / 100.0;
  }

  /// Sets the normalized saturation component (0.0-1.0).
  pub fn set_s(&mut self, s: impl Into<Component>) {
    self.s = s.into();
  }

  /// Sets the saturation from a percentage value (0-100%).
  pub fn set_saturation(&mut self, saturation: impl Into<Component>) {
    self.s = saturation.into() / 100.0;
  }

  /// Converts this HSI color to an [`Rgb`] color in the specified output space.
  pub fn to_rgb<OS>(&self) -> Rgb<OS>
  where
    OS: RgbSpec,
  {
    let h = self.h.0 * 360.0;
    let s = self.s.0;
    let i = self.i.0;

    if s <= 0.0 {
      return Rgb::<S>::from_normalized(i, i, i).to_rgb::<OS>().with_alpha(self.alpha);
    }
    if i <= 0.0 {
      return Rgb::<S>::from_normalized(0.0, 0.0, 0.0)
        .to_rgb::<OS>()
        .with_alpha(self.alpha);
    }

    let pi_3 = std::f64::consts::FRAC_PI_3;

    let (r, g, b) = if h < 120.0 {
      let h_rad = h.to_radians();
      let b = i * (1.0 - s);
      let r = i * (1.0 + s * h_rad.cos() / (pi_3 - h_rad).cos());
      let g = 3.0 * i - r - b;
      (r, g, b)
    } else if h < 240.0 {
      let h_rad = (h - 120.0).to_radians();
      let r = i * (1.0 - s);
      let g = i * (1.0 + s * h_rad.cos() / (pi_3 - h_rad).cos());
      let b = 3.0 * i - r - g;
      (r, g, b)
    } else {
      let h_rad = (h - 240.0).to_radians();
      let g = i * (1.0 - s);
      let b = i * (1.0 + s * h_rad.cos() / (pi_3 - h_rad).cos());
      let r = 3.0 * i - g - b;
      (r, g, b)
    };

    Rgb::<S>::from_normalized(r, g, b).to_rgb::<OS>().with_alpha(self.alpha)
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
    let mut hsi = *self;
    hsi.decrement_h(amount);
    hsi
  }

  /// Returns a new color with the normalized hue increased by the given amount.
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.increment_h(amount);
    hsi
  }

  /// Returns a new color with the normalized hue scaled by the given factor.
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.scale_h(factor);
    hsi
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
    let mut hsi = *self;
    hsi.decrement_hue(amount);
    hsi
  }

  /// Returns a new color with the hue increased by the given degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.increment_hue(amount);
    hsi
  }

  /// Alias for [`Self::with_h_scaled_by`].
  pub fn with_hue_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_h_scaled_by(factor)
  }

  /// Returns a new color with the given normalized intensity value.
  pub fn with_i(&self, i: impl Into<Component>) -> Self {
    Self {
      i: i.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized intensity decreased by the given amount.
  pub fn with_i_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.decrement_i(amount);
    hsi
  }

  /// Returns a new color with normalized intensity increased by the given amount.
  pub fn with_i_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.increment_i(amount);
    hsi
  }

  /// Returns a new color with normalized intensity scaled by the given factor.
  pub fn with_i_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.scale_i(factor);
    hsi
  }

  /// Returns a new color with the given intensity in percentage (0-100%).
  pub fn with_intensity(&self, intensity: impl Into<Component>) -> Self {
    Self {
      i: intensity.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with intensity decreased by the given percentage points.
  pub fn with_intensity_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.decrement_intensity(amount);
    hsi
  }

  /// Returns a new color with intensity increased by the given percentage points.
  pub fn with_intensity_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.increment_intensity(amount);
    hsi
  }

  /// Alias for [`Self::with_i_scaled_by`].
  pub fn with_intensity_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_i_scaled_by(factor)
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
    let mut hsi = *self;
    hsi.decrement_s(amount);
    hsi
  }

  /// Returns a new color with normalized saturation increased by the given amount.
  pub fn with_s_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.increment_s(amount);
    hsi
  }

  /// Returns a new color with normalized saturation scaled by the given factor.
  pub fn with_s_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.scale_s(factor);
    hsi
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
    let mut hsi = *self;
    hsi.decrement_saturation(amount);
    hsi
  }

  /// Returns a new color with saturation increased by the given percentage points.
  pub fn with_saturation_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut hsi = *self;
    hsi.increment_saturation(amount);
    hsi
  }

  /// Alias for [`Self::with_s_scaled_by`].
  pub fn with_saturation_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_s_scaled_by(factor)
  }
}

impl<S, T> Add<T> for Hsi<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() + rhs.into().to_rgb::<S>())
  }
}

impl<S> ColorSpace<3> for Hsi<S>
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

impl<S> Display for Hsi<S>
where
  S: RgbSpec,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "HSI({:.precision$}°, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.hue(),
        self.saturation(),
        self.intensity(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "HSI({:.precision$}°, {:.precision$}%, {:.precision$}%)",
        self.hue(),
        self.saturation(),
        self.intensity()
      )
    }
  }
}

impl<S, T> Div<T> for Hsi<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() / rhs.into().to_rgb::<S>())
  }
}

impl<S, T> From<[T; 3]> for Hsi<S>
where
  S: RgbSpec,
  T: Into<Component>,
{
  fn from([h, s, i]: [T; 3]) -> Self {
    Self::new(h, s, i)
  }
}

#[cfg(feature = "space-cmy")]
impl<OS, S> From<Cmy<OS>> for Hsi<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmy: Cmy<OS>) -> Self {
    cmy.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-cmyk")]
impl<OS, S> From<Cmyk<OS>> for Hsi<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<OS>) -> Self {
    cmyk.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-hsl")]
impl<OS, S> From<Hsl<OS>> for Hsi<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hsl: Hsl<OS>) -> Self {
    hsl.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-hsluv")]
impl<S> From<Hsluv> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(hsluv: Hsluv) -> Self {
    hsluv.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-hpluv")]
impl<S> From<Hpluv> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(hpluv: Hpluv) -> Self {
    hpluv.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-hsv")]
impl<OS, S> From<Hsv<OS>> for Hsi<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hsv: Hsv<OS>) -> Self {
    hsv.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-hwb")]
impl<OS, S> From<Hwb<OS>> for Hsi<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hwb: Hwb<OS>) -> Self {
    hwb.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-lab")]
impl<S> From<Lab> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(lab: Lab) -> Self {
    lab.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-lch")]
impl<S> From<Lch> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(lch: Lch) -> Self {
    lch.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-lchuv")]
impl<S> From<Lchuv> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_rgb::<S>().to_hsi()
  }
}

impl<S> From<Lms> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(lms: Lms) -> Self {
    lms.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-luv")]
impl<S> From<Luv> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(luv: Luv) -> Self {
    luv.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-okhsl")]
impl<S> From<Okhsl> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-okhsv")]
impl<S> From<Okhsv> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-okhwb")]
impl<S> From<Okhwb> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-oklab")]
impl<S> From<Oklab> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(oklab: Oklab) -> Self {
    oklab.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-oklch")]
impl<S> From<Oklch> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(oklch: Oklch) -> Self {
    oklch.to_rgb::<S>().to_hsi()
  }
}

impl<OS, S> From<Rgb<OS>> for Hsi<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(rgb: Rgb<OS>) -> Self {
    rgb.to_rgb::<S>().to_hsi()
  }
}

#[cfg(feature = "space-xyy")]
impl<S> From<Xyy> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(xyy: Xyy) -> Self {
    xyy.to_rgb::<S>().to_hsi()
  }
}

impl<S> From<Xyz> for Hsi<S>
where
  S: RgbSpec,
{
  fn from(xyz: Xyz) -> Self {
    xyz.to_rgb::<S>().to_hsi()
  }
}

impl<S, T> Mul<T> for Hsi<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() * rhs.into().to_rgb::<S>())
  }
}

impl<S, T> PartialEq<T> for Hsi<S>
where
  S: RgbSpec,
  T: Into<Hsi<S>> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.h == other.h && self.s == other.s && self.i == other.i
  }
}

impl<S, T> Sub<T> for Hsi<S>
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
    fn it_adds_two_hsi_values_via_rgb() {
      let a = Hsi::<Srgb>::new(0.0, 50.0, 25.0);
      let b = Hsi::<Srgb>::new(0.0, 50.0, 25.0);
      let result = a + b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.i().is_finite());
    }
  }

  mod decrement_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_h() {
      let mut hsi = Hsi::<Srgb>::new(180.0, 50.0, 50.0);
      hsi.decrement_h(0.25);

      assert_eq!(hsi.h(), 0.25);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hsi = Hsi::<Srgb>::new(36.0, 50.0, 50.0);
      hsi.decrement_h(0.2);

      assert!((hsi.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_subtracts_degrees_from_hue() {
      let mut hsi = Hsi::<Srgb>::new(180.0, 50.0, 50.0);
      hsi.decrement_hue(90.0);

      assert!((hsi.hue() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_below_zero() {
      let mut hsi = Hsi::<Srgb>::new(30.0, 50.0, 50.0);
      hsi.decrement_hue(60.0);

      assert!((hsi.hue() - 330.0).abs() < 1e-10);
    }
  }

  mod decrement_i {
    use super::*;

    #[test]
    fn it_subtracts_from_i() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      hsi.decrement_i(0.2);

      assert!((hsi.i() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_intensity {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_intensity() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      hsi.decrement_intensity(20.0);

      assert!((hsi.intensity() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_s {
    use super::*;

    #[test]
    fn it_subtracts_from_s() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      hsi.decrement_s(0.2);

      assert!((hsi.s() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_saturation {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_saturation() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      hsi.decrement_saturation(20.0);

      assert!((hsi.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let hsi = Hsi::<Srgb>::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", hsi), "HSI(120.00°, 50.00%, 75.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let hsi = Hsi::<Srgb>::new(120.12345, 50.6789, 75.4321);

      assert_eq!(format!("{:.4}", hsi), "HSI(120.1235°, 50.6789%, 75.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let hsi = Hsi::<Srgb>::new(120.0, 50.0, 75.0).with_alpha(0.5);

      assert_eq!(format!("{}", hsi), "HSI(120.00°, 50.00%, 75.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let hsi = Hsi::<Srgb>::new(120.0, 50.0, 75.0);

      assert_eq!(format!("{}", hsi), "HSI(120.00°, 50.00%, 75.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_hsi_values_via_rgb() {
      let a = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let result = a / b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.i().is_finite());
    }
  }

  #[cfg(feature = "space-cmyk")]
  mod from_cmyk {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let cmyk = Cmyk::<Srgb>::new(100.0, 0.0, 0.0, 0.0);
      let hsi: Hsi<Srgb> = cmyk.into();

      assert!((hsi.hue() - 180.0).abs() < 1.0);
    }
  }

  mod from_lms {
    use super::*;

    #[test]
    fn it_converts_from_lms_via_rgb() {
      let lms = Lms::new(0.5, 0.5, 0.5);
      let hsi: Hsi<Srgb> = lms.into();

      assert!(hsi.h().is_finite());
      assert!(hsi.s().is_finite());
      assert!(hsi.i().is_finite());
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 0.0, 0.0);
      let hsi: Hsi<Srgb> = rgb.into();

      assert!((hsi.hue() - 0.0).abs() < 1e-10);
      assert!((hsi.saturation() - 100.0).abs() < 1e-10);
      // I = (1+0+0)/3 ≈ 33.33%
      assert!((hsi.intensity() - 100.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_green() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 1.0, 0.0);
      let hsi: Hsi<Srgb> = rgb.into();

      assert!((hsi.hue() - 120.0).abs() < 1e-10);
      assert!((hsi.saturation() - 100.0).abs() < 1e-10);
      assert!((hsi.intensity() - 100.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_blue() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 1.0);
      let hsi: Hsi<Srgb> = rgb.into();

      assert!((hsi.hue() - 240.0).abs() < 1e-10);
      assert!((hsi.saturation() - 100.0).abs() < 1e-10);
      assert!((hsi.intensity() - 100.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_white_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 1.0, 1.0);
      let hsi: Hsi<Srgb> = rgb.into();

      assert!((hsi.saturation()).abs() < 1e-10);
      assert!((hsi.intensity() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_black_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 0.0);
      let hsi: Hsi<Srgb> = rgb.into();

      assert!((hsi.saturation()).abs() < 1e-10);
      assert!((hsi.intensity()).abs() < 1e-10);
    }

    #[test]
    fn it_converts_gray_to_achromatic() {
      let rgb = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let hsi: Hsi<Srgb> = rgb.into();

      assert!((hsi.saturation()).abs() < 1e-10);
      assert!((hsi.intensity() - 50.0).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz_via_rgb() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let hsi: Hsi<Srgb> = xyz.into();

      assert!(hsi.h().is_finite());
      assert!(hsi.s().is_finite());
      assert!(hsi.i().is_finite());
    }
  }

  mod increment_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_h() {
      let mut hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      hsi.increment_h(0.25);

      assert_eq!(hsi.h(), 0.5);
    }

    #[test]
    fn it_wraps_above_one() {
      let mut hsi = Hsi::<Srgb>::new(324.0, 50.0, 50.0);
      hsi.increment_h(0.2);

      assert!((hsi.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_adds_degrees_to_hue() {
      let mut hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      hsi.increment_hue(90.0);

      assert!((hsi.hue() - 180.0).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_above_360() {
      let mut hsi = Hsi::<Srgb>::new(300.0, 50.0, 50.0);
      hsi.increment_hue(90.0);

      assert!((hsi.hue() - 30.0).abs() < 1e-10);
    }
  }

  mod increment_i {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_i() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 50.0, 25.0);
      hsi.increment_i(0.25);

      assert_eq!(hsi.i(), 0.5);
    }
  }

  mod increment_intensity {
    use super::*;

    #[test]
    fn it_adds_percentage_to_intensity() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 50.0, 25.0);
      hsi.increment_intensity(25.0);

      assert!((hsi.intensity() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_s() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 25.0, 50.0);
      hsi.increment_s(0.25);

      assert_eq!(hsi.s(), 0.5);
    }
  }

  mod increment_saturation {
    use super::*;

    #[test]
    fn it_adds_percentage_to_saturation() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 25.0, 50.0);
      hsi.increment_saturation(25.0);

      assert!((hsi.saturation() - 50.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_hsi_values_via_rgb() {
      let a = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let result = a * b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.i().is_finite());
    }
  }

  mod new_const {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_hue_to_0_1() {
      const HSI: Hsi<Srgb> = Hsi::new_const(270.0, 50.0, 50.0);

      assert_eq!(HSI.h(), 0.75);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      const HSI: Hsi<Srgb> = Hsi::new_const(-90.0, 50.0, 50.0);

      assert_eq!(HSI.h(), 0.75);
    }

    #[test]
    fn it_normalizes_saturation_to_0_1() {
      const HSI: Hsi<Srgb> = Hsi::new_const(0.0, 75.0, 50.0);

      assert_eq!(HSI.s(), 0.75);
    }

    #[test]
    fn it_normalizes_intensity_to_0_1() {
      const HSI: Hsi<Srgb> = Hsi::new_const(0.0, 50.0, 75.0);

      assert_eq!(HSI.i(), 0.75);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_hsi_values() {
      let a = Hsi::<Srgb>::new(180.0, 50.0, 50.0);
      let b = Hsi::<Srgb>::new(180.0, 50.0, 50.0);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_hsi_values() {
      let a = Hsi::<Srgb>::new(180.0, 50.0, 50.0);
      let b = Hsi::<Srgb>::new(180.0, 50.0, 60.0);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Hsi::<Srgb>::new(180.0, 50.0, 50.0).with_alpha(0.5);
      let b = Hsi::<Srgb>::new(180.0, 50.0, 50.0);

      assert_ne!(a, b);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_multiplies_h_by_factor() {
      let mut hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      hsi.scale_h(2.0);

      assert!((hsi.h() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_wraps_when_exceeding_one() {
      let mut hsi = Hsi::<Srgb>::new(270.0, 50.0, 50.0);
      hsi.scale_h(2.0);

      assert!((hsi.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_i {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_i_by_factor() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 50.0, 25.0);
      hsi.scale_i(2.0);

      assert_eq!(hsi.i(), 0.5);
    }
  }

  mod scale_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_s_by_factor() {
      let mut hsi = Hsi::<Srgb>::new(0.0, 25.0, 50.0);
      hsi.scale_s(2.0);

      assert_eq!(hsi.s(), 0.5);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_hsi_values_via_rgb() {
      let a = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let b = Hsi::<Srgb>::new(0.0, 25.0, 25.0);
      let result = a - b;

      assert!(result.h().is_finite());
      assert!(result.s().is_finite());
      assert!(result.i().is_finite());
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_pure_red() {
      // Pure red: H=0, S=100%, I=33.33%
      let hsi = Hsi::<Srgb>::new(0.0, 100.0, 100.0 / 3.0);
      let rgb: Rgb<Srgb> = hsi.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert!(rgb.green() <= 1);
      assert!(rgb.blue() <= 1);
    }

    #[test]
    fn it_converts_pure_green() {
      // Pure green: H=120, S=100%, I=33.33%
      let hsi = Hsi::<Srgb>::new(120.0, 100.0, 100.0 / 3.0);
      let rgb: Rgb<Srgb> = hsi.to_rgb();

      assert!(rgb.red() <= 1);
      assert_eq!(rgb.green(), 255);
      assert!(rgb.blue() <= 1);
    }

    #[test]
    fn it_converts_pure_blue() {
      // Pure blue: H=240, S=100%, I=33.33%
      let hsi = Hsi::<Srgb>::new(240.0, 100.0, 100.0 / 3.0);
      let rgb: Rgb<Srgb> = hsi.to_rgb();

      assert!(rgb.red() <= 1);
      assert!(rgb.green() <= 1);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_white() {
      // White: S=0%, I=100%
      let hsi = Hsi::<Srgb>::new(0.0, 0.0, 100.0);
      let rgb: Rgb<Srgb> = hsi.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_black() {
      let hsi = Hsi::<Srgb>::new(0.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = hsi.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_achromatic_gray() {
      let hsi = Hsi::<Srgb>::new(0.0, 0.0, 50.0);
      let rgb: Rgb<Srgb> = hsi.to_rgb();

      assert_eq!(rgb.red(), rgb.green());
      assert_eq!(rgb.green(), rgb.blue());
    }

    #[test]
    fn it_converts_yellow() {
      // Yellow: H=60, S=100%, I=66.67%
      let hsi = Hsi::<Srgb>::new(60.0, 100.0, 200.0 / 3.0);
      let rgb: Rgb<Srgb> = hsi.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert!(rgb.blue() <= 1);
    }

    #[test]
    fn it_roundtrips_with_from_rgb() {
      let original = Hsi::<Srgb>::new(210.0, 60.0, 40.0);
      let rgb: Rgb<Srgb> = original.to_rgb();
      let back: Hsi<Srgb> = rgb.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.intensity() - original.intensity()).abs() < 1.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let hsi = Hsi::<Srgb>::new(120.0, 50.0, 50.0).with_alpha(0.3);
      let rgb: Rgb<Srgb> = hsi.to_rgb();

      assert!((rgb.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz_via_rgb() {
      let hsi = Hsi::<Srgb>::new(120.0, 100.0, 50.0);
      let xyz = hsi.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_roundtrips_with_from_xyz() {
      let original = Hsi::<Srgb>::new(210.0, 60.0, 40.0);
      let xyz = original.to_xyz();
      let back: Hsi<Srgb> = xyz.into();

      assert!((back.hue() - original.hue()).abs() < 1.0);
      assert!((back.saturation() - original.saturation()).abs() < 1.0);
      assert!((back.intensity() - original.intensity()).abs() < 1.0);
    }
  }

  mod with_h {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_new_h() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_h(0.75);

      assert_eq!(result.h(), 0.75);
      assert_eq!(result.s(), hsi.s());
      assert_eq!(result.i(), hsi.i());
    }
  }

  mod with_h_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_decremented_h() {
      let hsi = Hsi::<Srgb>::new(180.0, 50.0, 50.0);
      let result = hsi.with_h_decremented_by(0.25);

      assert_eq!(result.h(), 0.25);
      assert_eq!(hsi.h(), 0.5);
    }
  }

  mod with_h_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_incremented_h() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_h_incremented_by(0.25);

      assert_eq!(result.h(), 0.5);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_hsi_with_scaled_h() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_hsi_with_new_hue_in_degrees() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
      assert_eq!(result.s(), hsi.s());
      assert_eq!(result.i(), hsi.i());
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsi_with_decremented_hue() {
      let hsi = Hsi::<Srgb>::new(180.0, 50.0, 50.0);
      let result = hsi.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
      assert!((hsi.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsi_with_incremented_hue() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_hue_incremented_by(90.0);

      assert!((result.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod with_i {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_new_i() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_i(0.75);

      assert_eq!(result.i(), 0.75);
      assert_eq!(result.h(), hsi.h());
      assert_eq!(result.s(), hsi.s());
    }
  }

  mod with_i_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_decremented_i() {
      let hsi = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsi.with_i_decremented_by(0.2);

      assert!((result.i() - 0.3).abs() < 1e-10);
      assert_eq!(hsi.i(), 0.5);
    }
  }

  mod with_i_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_incremented_i() {
      let hsi = Hsi::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsi.with_i_incremented_by(0.25);

      assert_eq!(result.i(), 0.5);
    }
  }

  mod with_i_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_scaled_i() {
      let hsi = Hsi::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsi.with_i_scaled_by(2.0);

      assert_eq!(result.i(), 0.5);
    }
  }

  mod with_intensity {
    use super::*;

    #[test]
    fn it_returns_hsi_with_new_intensity_in_percent() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_intensity(75.0);

      assert!((result.intensity() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hsi.h());
      assert_eq!(result.s(), hsi.s());
    }
  }

  mod with_intensity_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsi_with_decremented_intensity() {
      let hsi = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsi.with_intensity_decremented_by(20.0);

      assert!((result.intensity() - 30.0).abs() < 1e-10);
    }
  }

  mod with_intensity_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsi_with_incremented_intensity() {
      let hsi = Hsi::<Srgb>::new(0.0, 50.0, 25.0);
      let result = hsi.with_intensity_incremented_by(25.0);

      assert!((result.intensity() - 50.0).abs() < 1e-10);
    }
  }

  mod with_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_new_s() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_s(0.75);

      assert_eq!(result.s(), 0.75);
      assert_eq!(result.h(), hsi.h());
      assert_eq!(result.i(), hsi.i());
    }
  }

  mod with_s_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_decremented_s() {
      let hsi = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsi.with_s_decremented_by(0.2);

      assert!((result.s() - 0.3).abs() < 1e-10);
      assert_eq!(hsi.s(), 0.5);
    }
  }

  mod with_s_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_incremented_s() {
      let hsi = Hsi::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsi.with_s_incremented_by(0.25);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_s_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_hsi_with_scaled_s() {
      let hsi = Hsi::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsi.with_s_scaled_by(2.0);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_saturation {
    use super::*;

    #[test]
    fn it_returns_hsi_with_new_saturation_in_percent() {
      let hsi = Hsi::<Srgb>::new(90.0, 50.0, 50.0);
      let result = hsi.with_saturation(75.0);

      assert!((result.saturation() - 75.0).abs() < 1e-10);
      assert_eq!(result.h(), hsi.h());
      assert_eq!(result.i(), hsi.i());
    }
  }

  mod with_saturation_decremented_by {
    use super::*;

    #[test]
    fn it_returns_hsi_with_decremented_saturation() {
      let hsi = Hsi::<Srgb>::new(0.0, 50.0, 50.0);
      let result = hsi.with_saturation_decremented_by(20.0);

      assert!((result.saturation() - 30.0).abs() < 1e-10);
    }
  }

  mod with_saturation_incremented_by {
    use super::*;

    #[test]
    fn it_returns_hsi_with_incremented_saturation() {
      let hsi = Hsi::<Srgb>::new(0.0, 25.0, 50.0);
      let result = hsi.with_saturation_incremented_by(25.0);

      assert!((result.saturation() - 50.0).abs() < 1e-10);
    }
  }
}
