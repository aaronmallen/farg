use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, Div, Mul, Sub},
};

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
#[cfg(feature = "space-okhsv")]
use crate::space::Okhsv;
#[cfg(feature = "space-okhwb")]
use crate::space::Okhwb;
#[cfg(feature = "space-xyy")]
use crate::space::Xyy;
use crate::{
  ColorimetricContext, Illuminant, Observer,
  component::Component,
  space::{ColorSpace, Lms, Oklab, Rgb, RgbSpec, Srgb, Xyz},
};

/// Chroma threshold below which a color is considered achromatic (hueless).
const ACHROMATIC_THRESHOLD: f64 = 1e-4;

/// Oklch perceptual color space (cylindrical form of Oklab).
///
/// A cylindrical representation of the Oklab perceptual color space where L represents
/// perceived lightness (0.0-1.0), C represents chroma (colorfulness), and H represents
/// hue stored internally as a 0.0-1.0 fraction (0-360°). Designed for intuitive color
/// manipulation with perceptual uniformity.
#[derive(Clone, Copy, Debug)]
pub struct Oklch {
  alpha: Component,
  c: Component,
  context: ColorimetricContext,
  h: Component,
  l: Component,
}

impl Oklch {
  /// The default viewing context for Oklch (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new Oklch color from lightness (0.0-1.0), chroma, and hue (0-360°).
  pub fn new(l: impl Into<Component>, c: impl Into<Component>, h: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      c: c.into(),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      l: l.into(),
    }
  }

  /// Creates a new Oklch color in a const context from lightness, chroma, and hue (0-360°).
  pub const fn new_const(l: f64, c: f64, h: f64) -> Self {
    let r = (h / 360.0) % 1.0;

    Self {
      alpha: Component::new_const(1.0),
      c: Component::new_const(c),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new_const(if r < 0.0 { r + 1.0 } else { r }),
      l: Component::new_const(l),
    }
  }

  /// Returns the C (chroma) component.
  pub fn c(&self) -> f64 {
    self.c.0
  }

  /// Returns the chroma value (alias for [`Self::c`]).
  pub fn chroma(&self) -> f64 {
    self.c.0
  }

  /// Returns the [L, C, H] components as an array (hue normalized to 0.0-1.0).
  pub fn components(&self) -> [f64; 3] {
    [self.l.0, self.c.0, self.h.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases the chroma by the given amount.
  pub fn decrement_c(&mut self, amount: impl Into<Component>) {
    self.c -= amount.into();
  }

  /// Alias for [`Self::decrement_c`].
  pub fn decrement_chroma(&mut self, amount: impl Into<Component>) {
    self.decrement_c(amount)
  }

  /// Decreases the normalized hue by the given amount (wraps around 0.0-1.0).
  pub fn decrement_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 - amount.into().0).rem_euclid(1.0));
  }

  /// Decreases the hue by the given amount in degrees (wraps around 0-360°).
  pub fn decrement_hue(&mut self, amount: impl Into<Component>) {
    self.decrement_h(amount.into() / 360.0)
  }

  /// Decreases the L component by the given amount.
  pub fn decrement_l(&mut self, amount: impl Into<Component>) {
    self.l -= amount.into();
  }

  /// Generates a sequence of evenly-spaced colors between `self` and `other`.
  ///
  /// Returns `steps` colors including both endpoints, interpolated in the Oklch color space
  /// for perceptually uniform results. When `steps` is 0 the result is empty. When `steps`
  /// is 1 the result contains only `self`.
  ///
  /// Accepts any color type that can be converted to [`Xyz`].
  pub fn gradient(&self, other: impl Into<Xyz>, steps: usize) -> Vec<Self> {
    if steps == 0 {
      return Vec::new();
    }
    let other = other.into();
    if steps == 1 {
      return vec![self.mix(other, 0.0)];
    }
    let divisor = (steps - 1) as f64;
    (0..steps).map(|i| self.mix(other, i as f64 / divisor)).collect()
  }

  /// Returns the normalized hue component (0.0-1.0).
  pub fn h(&self) -> f64 {
    self.h.0
  }

  /// Returns the hue in degrees (0-360°).
  pub fn hue(&self) -> f64 {
    self.h.0 * 360.0
  }

  /// Increases the chroma by the given amount.
  pub fn increment_c(&mut self, amount: impl Into<Component>) {
    self.c += amount.into();
  }

  /// Alias for [`Self::increment_c`].
  pub fn increment_chroma(&mut self, amount: impl Into<Component>) {
    self.increment_c(amount)
  }

  /// Increases the normalized hue by the given amount (wraps around 0.0-1.0).
  pub fn increment_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 + amount.into().0).rem_euclid(1.0));
  }

  /// Increases the hue by the given amount in degrees (wraps around 0-360°).
  pub fn increment_hue(&mut self, amount: impl Into<Component>) {
    self.increment_h(amount.into() / 360.0)
  }

  /// Increases the L component by the given amount.
  pub fn increment_l(&mut self, amount: impl Into<Component>) {
    self.l += amount.into();
  }

  /// Returns the L (lightness) component.
  pub fn l(&self) -> f64 {
    self.l.0
  }

  /// Interpolates between `self` and `other` at parameter `t`, returning a new color.
  ///
  /// When `t` is 0.0 the result matches `self`, when 1.0 it matches `other`.
  /// Values outside 0.0–1.0 extrapolate beyond the endpoints. Interpolation is
  /// performed in the Oklch color space with shortest-arc hue and achromatic handling
  /// per the CSS Color Level 4 specification.
  ///
  /// Accepts any color type that can be converted to [`Xyz`].
  pub fn mix(&self, other: impl Into<Xyz>, t: f64) -> Self {
    let other = Self::from(other.into());

    let l = Component::new(self.l()).lerp(other.l(), t);
    let c = Component::new(self.c()).lerp(other.c(), t);
    let h = mix_hue(self.hue(), self.c(), other.hue(), other.c(), t);
    let alpha = Component::new(self.alpha()).lerp(other.alpha(), t);

    Self::new(l, c, h).with_alpha(alpha)
  }

  /// Interpolates `self` toward `other` at parameter `t`, mutating in place.
  ///
  /// See [`mix`](Self::mix) for details on the interpolation behavior.
  pub fn mixed_with(&mut self, other: impl Into<Xyz>, t: f64) {
    let result = self.mix(other, t);
    self.l = result.l;
    self.c = result.c;
    self.h = result.h;
    self.alpha = result.alpha;
  }

  /// Scales the chroma by the given factor.
  pub fn scale_c(&mut self, factor: impl Into<Component>) {
    self.c *= factor.into();
  }

  /// Alias for [`Self::scale_c`].
  pub fn scale_chroma(&mut self, factor: impl Into<Component>) {
    self.scale_c(factor)
  }

  /// Scales the normalized hue by the given factor (wraps around 0.0-1.0).
  pub fn scale_h(&mut self, factor: impl Into<Component>) {
    self.h = Component::new((self.h.0 * factor.into().0).rem_euclid(1.0));
  }

  /// Alias for [`Self::scale_h`].
  pub fn scale_hue(&mut self, factor: impl Into<Component>) {
    self.scale_h(factor)
  }

  /// Scales the L component by the given factor.
  pub fn scale_l(&mut self, factor: impl Into<Component>) {
    self.l *= factor.into();
  }

  /// Sets the C component.
  pub fn set_c(&mut self, c: impl Into<Component>) {
    self.c = c.into();
  }

  /// Alias for [`Self::set_c`].
  pub fn set_chroma(&mut self, chroma: impl Into<Component>) {
    self.set_c(chroma)
  }

  /// Sets the [L, C, H] components from an array.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_l(components[0].clone());
    self.set_c(components[1].clone());
    self.set_h(components[2].clone());
  }

  /// Sets the normalized hue component (0.0-1.0).
  pub fn set_h(&mut self, h: impl Into<Component>) {
    self.h = h.into();
  }

  /// Sets the hue from a value in degrees (0-360°).
  pub fn set_hue(&mut self, hue: impl Into<Component>) {
    self.h = Component::new((hue.into().0 / 360.0).rem_euclid(1.0));
  }

  /// Sets the L component.
  pub fn set_l(&mut self, l: impl Into<Component>) {
    self.l = l.into();
  }

  /// Returns this color as a CSS Color Level 4 `oklch(...)` string.
  ///
  /// L is 0-1, C is chroma, H is hue in degrees. Alpha is appended only
  /// when less than 1.0.
  ///
  /// ```
  /// use farg::space::{ColorSpace, Oklch};
  ///
  /// let color = Oklch::new(0.7, 0.15, 145.0);
  /// assert_eq!(color.to_css(), "oklch(0.7 0.15 145)");
  /// ```
  pub fn to_css(&self) -> String {
    fn f(v: f64) -> String {
      format!("{:.6}", v)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
    }

    let a = self.alpha.0;
    if a < 1.0 {
      format!("oklch({} {} {} / {})", f(self.l()), f(self.c()), f(self.hue()), f(a))
    } else {
      format!("oklch({} {} {})", f(self.l()), f(self.c()), f(self.hue()))
    }
  }

  /// Converts to the Oklab perceptual color space.
  pub fn to_oklab(&self) -> Oklab {
    let h_rad = self.h.0 * 2.0 * std::f64::consts::PI;
    let a = self.c.0 * h_rad.cos();
    let b = self.c.0 * h_rad.sin();

    Oklab::new(self.l, a, b).with_alpha(self.alpha)
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

  /// Returns a new color with the given C value.
  pub fn with_c(&self, c: impl Into<Component>) -> Self {
    Self {
      c: c.into(),
      ..*self
    }
  }

  /// Returns a new color with C decreased by the given amount.
  pub fn with_c_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.decrement_c(amount);
    oklch
  }

  /// Returns a new color with C increased by the given amount.
  pub fn with_c_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.increment_c(amount);
    oklch
  }

  /// Returns a new color with C scaled by the given factor.
  pub fn with_c_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.scale_c(factor);
    oklch
  }

  /// Alias for [`Self::with_c`].
  pub fn with_chroma(&self, chroma: impl Into<Component>) -> Self {
    self.with_c(chroma)
  }

  /// Alias for [`Self::with_c_decremented_by`].
  pub fn with_chroma_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_c_decremented_by(amount)
  }

  /// Alias for [`Self::with_c_incremented_by`].
  pub fn with_chroma_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_c_incremented_by(amount)
  }

  /// Alias for [`Self::with_c_scaled_by`].
  pub fn with_chroma_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_c_scaled_by(factor)
  }

  /// Returns this color with a different viewing context (without adaptation).
  pub fn with_context(&self, context: ColorimetricContext) -> Self {
    Self {
      context,
      ..*self
    }
  }

  /// Returns a new color with the given normalized hue (0.0-1.0).
  pub fn with_h(&self, h: impl Into<Component>) -> Self {
    Self {
      h: h.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized hue decreased by the given amount (wraps around 0.0-1.0).
  pub fn with_h_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.decrement_h(amount);
    oklch
  }

  /// Returns a new color with normalized hue increased by the given amount (wraps around 0.0-1.0).
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.increment_h(amount);
    oklch
  }

  /// Returns a new color with normalized hue scaled by the given factor (wraps around 0.0-1.0).
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.scale_h(factor);
    oklch
  }

  /// Returns a new color with the given hue in degrees (0-360°).
  pub fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self {
      h: Component::new((hue.into().0 / 360.0).rem_euclid(1.0)),
      ..*self
    }
  }

  /// Returns a new color with hue decreased by the given amount in degrees.
  pub fn with_hue_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.decrement_hue(amount);
    oklch
  }

  /// Returns a new color with hue increased by the given amount in degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.increment_hue(amount);
    oklch
  }

  /// Returns a new color with hue scaled by the given factor.
  pub fn with_hue_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.scale_hue(factor);
    oklch
  }

  /// Returns a new color with the given L value.
  pub fn with_l(&self, l: impl Into<Component>) -> Self {
    Self {
      l: l.into(),
      ..*self
    }
  }

  /// Returns a new color with L decreased by the given amount.
  pub fn with_l_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.decrement_l(amount);
    oklch
  }

  /// Returns a new color with L increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.increment_l(amount);
    oklch
  }

  /// Returns a new color with L scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut oklch = *self;
    oklch.scale_l(factor);
    oklch
  }
}

impl<T> Add<T> for Oklch
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Oklch {
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

impl Display for Oklch {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(4);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Oklch({:.precision$}, {:.precision$}, {:.precision$}°, {:.0}%)",
        self.l,
        self.c,
        self.hue(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "Oklch({:.precision$}, {:.precision$}, {:.precision$}°)",
        self.l,
        self.c,
        self.hue()
      )
    }
  }
}

impl<T> Div<T> for Oklch
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Oklch
where
  T: Into<Component>,
{
  fn from([l, c, h]: [T; 3]) -> Self {
    Self::new(l, c, h)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Oklch
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_oklch()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Oklch
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_oklch()
  }
}

#[cfg(feature = "space-hpluv")]
impl From<Hpluv> for Oklch {
  fn from(hpluv: Hpluv) -> Self {
    hpluv.to_oklch()
  }
}

#[cfg(feature = "space-hsi")]
impl<S> From<Hsi<S>> for Oklch
where
  S: RgbSpec,
{
  fn from(hsi: Hsi<S>) -> Self {
    hsi.to_oklch()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Oklch
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_oklch()
  }
}

#[cfg(feature = "space-hsluv")]
impl From<Hsluv> for Oklch {
  fn from(hsluv: Hsluv) -> Self {
    hsluv.to_oklch()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Oklch
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_oklch()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Oklch
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_oklch()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Oklch {
  fn from(lab: Lab) -> Self {
    lab.to_oklch()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Oklch {
  fn from(lch: Lch) -> Self {
    lch.to_oklch()
  }
}

#[cfg(feature = "space-lchuv")]
impl From<Lchuv> for Oklch {
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_oklch()
  }
}

impl From<Lms> for Oklch {
  fn from(lms: Lms) -> Self {
    lms.to_oklch()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Oklch {
  fn from(luv: Luv) -> Self {
    luv.to_oklch()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Oklch {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_oklch()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Oklch {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_oklch()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Oklch {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_oklch()
  }
}

impl From<Oklab> for Oklch {
  fn from(oklab: Oklab) -> Self {
    oklab.to_oklch()
  }
}

impl<S> From<Rgb<S>> for Oklch
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_oklch()
  }
}

#[cfg(feature = "space-xyy")]
impl From<Xyy> for Oklch {
  fn from(xyy: Xyy) -> Self {
    xyy.to_oklch()
  }
}

impl From<Xyz> for Oklch {
  fn from(xyz: Xyz) -> Self {
    xyz.to_oklch()
  }
}

impl<T> Mul<T> for Oklch
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Oklch
where
  T: Into<Oklch> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.l == other.l && self.c == other.c && self.h == other.h
  }
}

impl<T> Sub<T> for Oklch
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Oklch {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Oklch {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

/// Interpolates hue along the shortest arc with achromatic handling.
///
/// When either color is achromatic (chroma below [`ACHROMATIC_THRESHOLD`]), its hue is
/// treated as "powerless" and the other color's hue is used. When both are achromatic,
/// hue is 0. This follows the CSS Color Level 4 specification for hue interpolation.
fn mix_hue(h1: f64, c1: f64, h2: f64, c2: f64, t: f64) -> f64 {
  let achromatic1 = c1 < ACHROMATIC_THRESHOLD;
  let achromatic2 = c2 < ACHROMATIC_THRESHOLD;

  if achromatic1 && achromatic2 {
    return 0.0;
  }
  if achromatic1 {
    return h2;
  }
  if achromatic2 {
    return h1;
  }

  let mut diff = h2 - h1;
  if diff > 180.0 {
    diff -= 360.0;
  } else if diff < -180.0 {
    diff += 360.0;
  }

  (h1 + diff * t).rem_euclid(360.0)
}

#[cfg(test)]
mod test {
  use super::*;

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_oklch_colors() {
      let a = Oklch::new(0.5, 0.15, 180.0);
      let b = Oklch::new(0.3, 0.1, 90.0);
      let result = a + b;

      assert!(result.l() > 0.0);
    }
  }

  mod c {
    use super::*;

    #[test]
    fn it_returns_c_component() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert!((oklch.c() - 0.15).abs() < 1e-10);
    }
  }

  mod chroma {
    use super::*;

    #[test]
    fn it_returns_chroma_as_alias() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert!((oklch.chroma() - 0.15).abs() < 1e-10);
    }
  }

  mod components {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_components_as_array() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let [l, c, h] = oklch.components();

      assert_eq!(l, 0.5);
      assert_eq!(c, 0.15);
      assert_eq!(h, 0.5);
    }
  }

  mod decrement_c {
    use super::*;

    #[test]
    fn it_decreases_c_component() {
      let mut oklch = Oklch::new(0.5, 0.15, 180.0);
      oklch.decrement_c(0.05);

      assert!((oklch.c() - 0.1).abs() < 1e-10);
    }
  }

  mod decrement_h {
    use super::*;

    #[test]
    fn it_decreases_h_with_wrapping() {
      let mut oklch = Oklch::new(0.5, 0.15, 36.0);
      oklch.decrement_h(0.2);

      assert!((oklch.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_decreases_hue_in_degrees() {
      let mut oklch = Oklch::new(0.5, 0.15, 180.0);
      oklch.decrement_hue(90.0);

      assert!((oklch.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod decrement_l {
    use super::*;

    #[test]
    fn it_decreases_l_component() {
      let mut oklch = Oklch::new(0.5, 0.15, 180.0);
      oklch.decrement_l(0.1);

      assert!((oklch.l() - 0.4).abs() < 1e-10);
    }
  }

  mod gradient {
    use super::*;

    #[test]
    fn zero_steps_is_empty() {
      let c1 = Oklch::new(0.5, 0.15, 180.0);
      let c2 = Oklch::new(0.8, 0.10, 90.0);
      assert!(c1.gradient(c2.to_xyz(), 0).is_empty());
    }

    #[test]
    fn one_step_returns_self() {
      let c1 = Oklch::new(0.5, 0.15, 180.0);
      let c2 = Oklch::new(0.8, 0.10, 90.0);
      let steps = c1.gradient(c2.to_xyz(), 1);
      assert_eq!(steps.len(), 1);
      assert!((steps[0].l() - c1.l()).abs() < 1e-4);
    }

    #[test]
    fn two_steps_returns_endpoints() {
      let c1 = Oklch::new(0.5, 0.15, 180.0);
      let c2 = Oklch::new(0.8, 0.10, 90.0);
      let steps = c1.gradient(c2.to_xyz(), 2);
      assert_eq!(steps.len(), 2);
      assert!((steps[0].l() - c1.l()).abs() < 1e-4);
      assert!((steps[1].l() - c2.l()).abs() < 1e-4);
    }

    #[test]
    fn five_steps_correct_count() {
      let c1 = Oklch::new(0.2, 0.1, 0.0);
      let c2 = Oklch::new(0.9, 0.05, 180.0);
      assert_eq!(c1.gradient(c2.to_xyz(), 5).len(), 5);
    }

    #[test]
    fn monotonic_lightness_dark_to_light() {
      let dark = Oklch::new(0.1, 0.0, 0.0);
      let light = Oklch::new(0.9, 0.0, 0.0);
      let steps = dark.gradient(light.to_xyz(), 5);
      let lightnesses: Vec<f64> = steps.iter().map(|c| c.l()).collect();
      for i in 1..lightnesses.len() {
        assert!(
          lightnesses[i] >= lightnesses[i - 1],
          "Lightness should be monotonically increasing: {lightnesses:?}"
        );
      }
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert_eq!(format!("{}", oklch), "Oklch(0.5000, 0.1500, 180.0000°)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert_eq!(format!("{:.2}", oklch), "Oklch(0.50, 0.15, 180.00°)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let oklch = Oklch::new(0.5, 0.15, 180.0).with_alpha(0.5);

      assert_eq!(format!("{}", oklch), "Oklch(0.5000, 0.1500, 180.0000°, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert_eq!(format!("{}", oklch), "Oklch(0.5000, 0.1500, 180.0000°)");
    }
  }

  mod from_array {
    use super::*;

    #[test]
    fn it_creates_from_f64_array() {
      let oklch = Oklch::from([0.5, 0.15, 180.0]);

      assert!((oklch.l() - 0.5).abs() < 1e-10);
      assert!((oklch.c() - 0.15).abs() < 1e-10);
      assert!((oklch.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod from_oklab {
    use super::*;

    #[test]
    fn it_converts_from_oklab() {
      let oklab = Oklab::new(0.5, 0.0, 0.15);
      let oklch = Oklch::from(oklab);

      assert!((oklch.l() - 0.5).abs() < 1e-10);
      assert!((oklch.c() - 0.15).abs() < 1e-10);
      assert!((oklch.hue() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.5);
      let oklch = Oklch::from(oklab);

      assert!((oklch.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let oklch = Oklch::from(rgb);

      assert!((oklch.l() - 1.0).abs() < 1e-3);
      assert!(oklch.c().abs() < 1e-3);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let oklch = Oklch::from(rgb);

      assert!(oklch.l().abs() < 1e-3);
      assert!(oklch.c().abs() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let oklch = Oklch::from(rgb);

      assert!((oklch.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let oklch = Oklch::from(xyz);

      assert!(oklch.l() > 0.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let oklch = Oklch::from(xyz);

      assert!((oklch.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod h {
    use super::*;

    #[test]
    fn it_returns_normalized_hue() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert!((oklch.h() - 0.5).abs() < 1e-10);
    }
  }

  mod hue {
    use super::*;

    #[test]
    fn it_returns_hue_in_degrees() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert!((oklch.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod increment_c {
    use super::*;

    #[test]
    fn it_increases_c_component() {
      let mut oklch = Oklch::new(0.5, 0.15, 180.0);
      oklch.increment_c(0.05);

      assert!((oklch.c() - 0.2).abs() < 1e-10);
    }
  }

  mod increment_h {
    use super::*;

    #[test]
    fn it_increases_h_with_wrapping() {
      let mut oklch = Oklch::new(0.5, 0.15, 324.0);
      oklch.increment_h(0.2);

      assert!((oklch.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_increases_hue_in_degrees() {
      let mut oklch = Oklch::new(0.5, 0.15, 180.0);
      oklch.increment_hue(90.0);

      assert!((oklch.hue() - 270.0).abs() < 1e-10);
    }
  }

  mod increment_l {
    use super::*;

    #[test]
    fn it_increases_l_component() {
      let mut oklch = Oklch::new(0.5, 0.15, 180.0);
      oklch.increment_l(0.1);

      assert!((oklch.l() - 0.6).abs() < 1e-10);
    }
  }

  mod l {
    use super::*;

    #[test]
    fn it_returns_l_component() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert!((oklch.l() - 0.5).abs() < 1e-10);
    }
  }

  mod mix {
    use super::*;

    const EPSILON: f64 = 1e-4;

    #[test]
    fn at_zero_returns_self() {
      let c1 = Oklch::new(0.6, 0.2, 30.0);
      let c2 = Oklch::new(0.4, 0.1, 270.0);
      let result = c1.mix(c2.to_xyz(), 0.0);
      assert!((result.l() - c1.l()).abs() < EPSILON);
      assert!((result.c() - c1.c()).abs() < EPSILON);
    }

    #[test]
    fn at_one_returns_other() {
      let c1 = Oklch::new(0.6, 0.2, 30.0);
      let c2 = Oklch::new(0.4, 0.1, 270.0);
      let result = c1.mix(c2.to_xyz(), 1.0);
      assert!((result.l() - c2.l()).abs() < EPSILON);
      assert!((result.c() - c2.c()).abs() < EPSILON);
    }

    #[test]
    fn midpoint_is_between() {
      let c1 = Oklch::new(0.2, 0.0, 0.0);
      let c2 = Oklch::new(0.8, 0.0, 0.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!(mid.l() > 0.3 && mid.l() < 0.7);
    }

    #[test]
    fn alpha_interpolation() {
      let c1 = Oklch::new(0.5, 0.1, 180.0).with_alpha(0.0);
      let c2 = Oklch::new(0.5, 0.1, 180.0).with_alpha(1.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!((mid.alpha() - 0.5).abs() < EPSILON);
    }

    #[test]
    fn extrapolation() {
      let c1 = Oklch::new(0.2, 0.0, 0.0);
      let c2 = Oklch::new(0.8, 0.0, 0.0);
      let beyond = c1.mix(c2.to_xyz(), 1.5);
      assert!(beyond.l() > c2.l());
    }

    #[test]
    fn cross_type() {
      let oklch = Oklch::new(0.6, 0.2, 30.0);
      let xyz = Xyz::new(0.18048, 0.07219, 0.95030);
      let _result = oklch.mix(xyz, 0.5);
    }

    #[test]
    fn shortest_arc_hue() {
      let c1 = Oklch::new(0.6, 0.2, 350.0);
      let c2 = Oklch::new(0.6, 0.2, 10.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      let hue = mid.hue();
      assert!(hue < 20.0 || hue > 340.0, "Hue {hue} should be near 0°/360°");
    }

    #[test]
    fn both_achromatic() {
      let c1 = Oklch::new(0.2, 0.0, 0.0);
      let c2 = Oklch::new(0.8, 0.0, 0.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!(mid.c() < 0.01);
    }

    #[test]
    fn one_achromatic() {
      let grey = Oklch::new(0.5, 0.0, 0.0);
      let red = Oklch::new(0.6, 0.2, 30.0);
      let result = grey.mix(red.to_xyz(), 0.5);
      let result_hue = result.hue();
      assert!((result_hue - 30.0).abs() < 5.0);
    }
  }

  mod mix_hue_fn {
    use super::super::mix_hue;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn shortest_arc_forward() {
      let h = mix_hue(10.0, 0.1, 50.0, 0.1, 0.5);
      assert!((h - 30.0).abs() < EPSILON);
    }

    #[test]
    fn shortest_arc_crosses_zero() {
      let h = mix_hue(350.0, 0.1, 10.0, 0.1, 0.5);
      assert!((h - 0.0).abs() < EPSILON);
    }

    #[test]
    fn shortest_arc_backward() {
      let h = mix_hue(10.0, 0.1, 350.0, 0.1, 0.5);
      assert!((h - 0.0).abs() < EPSILON);
    }

    #[test]
    fn both_achromatic() {
      let h = mix_hue(90.0, 0.0, 270.0, 0.0, 0.5);
      assert!((h - 0.0).abs() < EPSILON);
    }

    #[test]
    fn first_achromatic_uses_second() {
      let h = mix_hue(90.0, 0.0, 200.0, 0.1, 0.5);
      assert!((h - 200.0).abs() < EPSILON);
    }

    #[test]
    fn second_achromatic_uses_first() {
      let h = mix_hue(90.0, 0.1, 200.0, 0.0, 0.5);
      assert!((h - 90.0).abs() < EPSILON);
    }

    #[test]
    fn same_hemisphere() {
      let h = mix_hue(100.0, 0.1, 140.0, 0.1, 0.5);
      assert!((h - 120.0).abs() < EPSILON);
    }
  }

  mod mixed_with {
    use super::*;

    #[test]
    fn it_mutates_in_place() {
      let c1 = Oklch::new(0.6, 0.2, 30.0);
      let c2 = Oklch::new(0.4, 0.1, 270.0);
      let expected = c1.mix(c2.to_xyz(), 0.5);
      let mut color = c1;
      color.mixed_with(c2.to_xyz(), 0.5);
      assert!((color.l() - expected.l()).abs() < 1e-10);
      assert!((color.c() - expected.c()).abs() < 1e-10);
      assert!((color.h() - expected.h()).abs() < 1e-10);
      assert!((color.alpha() - expected.alpha()).abs() < 1e-10);
    }
  }

  mod new {
    use super::*;

    #[test]
    fn it_creates_with_default_alpha() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert!((oklch.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);

      assert_eq!(oklch.context().illuminant().name(), "D65");
    }

    #[test]
    fn it_normalizes_hue_to_zero_one() {
      let oklch = Oklch::new(0.5, 0.15, 450.0);

      assert!((oklch.h() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      let oklch = Oklch::new(0.5, 0.15, -90.0);

      assert!((oklch.h() - 0.75).abs() < 1e-10);
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Oklch::new(0.5, 0.15, 180.0);
      let b = Oklch::new(0.5, 0.15, 180.0);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Oklch::new(0.5, 0.15, 180.0);
      let b = Oklch::new(0.6, 0.15, 180.0);

      assert!(a != b);
    }
  }

  mod scale_c {
    use super::*;

    #[test]
    fn it_scales_c_component() {
      let mut oklch = Oklch::new(0.5, 0.15, 180.0);
      oklch.scale_c(2.0);

      assert!((oklch.c() - 0.3).abs() < 1e-10);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_scales_h_with_wrapping() {
      let mut oklch = Oklch::new(0.5, 0.15, 270.0);
      oklch.scale_h(2.0);

      assert!((oklch.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_l {
    use super::*;

    #[test]
    fn it_scales_l_component() {
      let mut oklch = Oklch::new(0.5, 0.15, 180.0);
      oklch.scale_l(2.0);

      assert!((oklch.l() - 1.0).abs() < 1e-10);
    }
  }

  mod to_css {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_outputs_opaque_oklch() {
      let color = Oklch::new(0.7, 0.15, 145.0);
      assert_eq!(color.to_css(), "oklch(0.7 0.15 145)");
    }

    #[test]
    fn it_outputs_translucent_oklch() {
      let color = Oklch::new(0.7, 0.15, 145.0).with_alpha(0.5);
      assert_eq!(color.to_css(), "oklch(0.7 0.15 145 / 0.5)");
    }
  }

  mod to_oklab {
    use super::*;

    #[test]
    fn it_converts_to_oklab() {
      let oklch = Oklch::new(0.5, 0.15, 90.0);
      let oklab = oklch.to_oklab();

      assert!((oklab.l() - 0.5).abs() < 1e-10);
      assert!(oklab.a().abs() < 1e-10);
      assert!((oklab.b() - 0.15).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_through_oklab() {
      let original = Oklch::new(0.5, 0.15, 180.0);
      let roundtrip = Oklch::from(original.to_oklab());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.c() - roundtrip.c()).abs() < 1e-10);
      assert!((original.h() - roundtrip.h()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklch = Oklch::new(0.5, 0.15, 180.0).with_alpha(0.7);
      let oklab = oklch.to_oklab();

      assert!((oklab.alpha() - 0.7).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-okhsl")]
  mod to_okhsl {
    use super::*;

    #[test]
    fn it_converts_to_okhsl() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let okhsl = oklch.to_okhsl();

      assert!(okhsl.l() > 0.0);
      assert!(okhsl.h().is_finite());
    }

    #[test]
    fn it_converts_black() {
      let oklch = Oklch::new(0.0, 0.0, 0.0);
      let okhsl = oklch.to_okhsl();

      assert!(okhsl.l().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let oklch = Oklch::new(1.0, 0.0, 0.0);
      let okhsl = oklch.to_okhsl();

      assert!((okhsl.l() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklch = Oklch::new(0.5, 0.15, 180.0).with_alpha(0.3);
      let okhsl = oklch.to_okhsl();

      assert!((okhsl.alpha() - 0.3).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-okhsv")]
  mod to_okhsv {
    use super::*;

    #[test]
    fn it_converts_to_okhsv() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let okhsv = oklch.to_okhsv();

      assert!(okhsv.v() > 0.0);
      assert!(okhsv.h().is_finite());
    }

    #[test]
    fn it_converts_black() {
      let oklch = Oklch::new(0.0, 0.0, 0.0);
      let okhsv = oklch.to_okhsv();

      assert!(okhsv.v().abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklch = Oklch::new(0.5, 0.15, 180.0).with_alpha(0.3);
      let okhsv = oklch.to_okhsv();

      assert!((okhsv.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let oklch = Oklch::new(0.5, 0.0, 0.0);
      let rgb = oklch.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_roundtrips_through_rgb() {
      let original = Oklch::from(Rgb::<Srgb>::new(128, 64, 200));
      let roundtrip = Oklch::from(original.to_rgb::<Srgb>());

      assert!((original.l() - roundtrip.l()).abs() < 1e-3);
      assert!((original.c() - roundtrip.c()).abs() < 1e-3);
      assert!((original.h() - roundtrip.h()).abs() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklch = Oklch::new(0.5, 0.15, 180.0).with_alpha(0.7);
      let rgb = oklch.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let oklch = Oklch::new(0.5, 0.0, 0.0);
      let xyz = oklch.to_xyz();

      assert!(xyz.y() > 0.0);
    }

    #[test]
    fn it_roundtrips_through_xyz() {
      let original = Oklch::new(0.5, 0.15, 180.0);
      let roundtrip = Oklch::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.c() - roundtrip.c()).abs() < 1e-10);
      assert!((original.h() - roundtrip.h()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklch = Oklch::new(0.5, 0.15, 180.0).with_alpha(0.3);
      let xyz = oklch.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let oklch = Oklch::try_from("#FF5733").unwrap();

      assert!(oklch.l() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Oklch::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.l() - 0.5).abs() < 1e-10);
    }
  }

  mod with_c {
    use super::*;

    #[test]
    fn it_returns_new_color_with_c() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_c(0.2);

      assert!((result.c() - 0.2).abs() < 1e-10);
      assert!((result.l() - 0.5).abs() < 1e-10);
      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_c_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_c_decremented() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_c_decremented_by(0.05);

      assert!((result.c() - 0.1).abs() < 1e-10);
    }
  }

  mod with_c_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_c_incremented() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_c_incremented_by(0.05);

      assert!((result.c() - 0.2).abs() < 1e-10);
    }
  }

  mod with_c_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_c_scaled() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_c_scaled_by(2.0);

      assert!((result.c() - 0.3).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let context = ColorimetricContext::default();
      let result = oklch.with_context(context);

      assert!((result.l() - 0.5).abs() < 1e-10);
    }
  }

  mod with_h {
    use super::*;

    #[test]
    fn it_returns_new_color_with_h() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_h(0.75);

      assert!((result.h() - 0.75).abs() < 1e-10);
      assert!((result.l() - 0.5).abs() < 1e-10);
      assert!((result.c() - 0.15).abs() < 1e-10);
    }
  }

  mod with_h_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_h_decremented() {
      let oklch = Oklch::new(0.5, 0.15, 36.0);
      let result = oklch.with_h_decremented_by(0.2);

      assert!((result.h() - 0.9).abs() < 1e-10);
    }
  }

  mod with_h_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_h_incremented() {
      let oklch = Oklch::new(0.5, 0.15, 324.0);
      let result = oklch.with_h_incremented_by(0.2);

      assert!((result.h() - 0.1).abs() < 1e-10);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_h_scaled() {
      let oklch = Oklch::new(0.5, 0.15, 270.0);
      let result = oklch.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_new_color_with_hue_in_degrees() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_hue_decremented() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_hue_incremented() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_hue_incremented_by(90.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
    }
  }

  mod with_l {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_l(0.8);

      assert!((result.l() - 0.8).abs() < 1e-10);
      assert!((result.c() - 0.15).abs() < 1e-10);
      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_l_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_decremented() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_l_decremented_by(0.1);

      assert!((result.l() - 0.4).abs() < 1e-10);
    }
  }

  mod with_l_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_incremented() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_l_incremented_by(0.1);

      assert!((result.l() - 0.6).abs() < 1e-10);
    }
  }

  mod with_l_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_scaled() {
      let oklch = Oklch::new(0.5, 0.15, 180.0);
      let result = oklch.with_l_scaled_by(2.0);

      assert!((result.l() - 1.0).abs() < 1e-10);
    }
  }
}
