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
  ColorimetricContext, Illuminant, Observer,
  component::Component,
  space::{ColorSpace, Lab, Lms, Rgb, RgbSpec, Srgb, Xyz},
};

/// Chroma threshold below which a color is considered achromatic (hueless).
const ACHROMATIC_THRESHOLD: f64 = 1e-4;

/// CIE LCh color space (cylindrical form of CIE L*a*b*).
///
/// A cylindrical representation of the CIE L\*a\*b\* color space where L\* represents
/// lightness (0–100), C\* represents chroma (colorfulness), and H represents
/// hue stored internally as a 0.0–1.0 fraction (0–360°). Uses the same L\* lightness
/// axis as Lab but replaces the rectangular a\*/b\* axes with polar coordinates.
#[derive(Clone, Copy, Debug)]
pub struct Lch {
  alpha: Component,
  c: Component,
  context: ColorimetricContext,
  h: Component,
  l: Component,
}

impl Lch {
  /// The default viewing context for Lch (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new Lch color from lightness (0–100), chroma, and hue (0–360°).
  pub fn new(l: impl Into<Component>, c: impl Into<Component>, h: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      c: c.into(),
      context: Self::DEFAULT_CONTEXT,
      h: Component::new((h.into().0 / 360.0).rem_euclid(1.0)),
      l: l.into(),
    }
  }

  /// Creates a new Lch color in a const context from lightness, chroma, and hue (0–360°).
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

  /// Adapts this color to a different viewing context via Lab and XYZ.
  pub fn adapt_to(&self, context: ColorimetricContext) -> Self {
    let reference_white = self.context.reference_white();
    let target_white = context.reference_white();

    if reference_white == target_white {
      return self.with_context(context);
    }

    Self::from(self.to_lab().adapt_to(context))
  }

  /// Returns the C\* (chroma) component.
  pub fn c(&self) -> f64 {
    self.c.0
  }

  /// Returns the chroma value (alias for [`Self::c`]).
  pub fn chroma(&self) -> f64 {
    self.c.0
  }

  /// Returns the [L\*, C\*, H] components as an array (hue normalized to 0.0–1.0).
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

  /// Decreases the normalized hue by the given amount (wraps around 0.0–1.0).
  pub fn decrement_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 - amount.into().0).rem_euclid(1.0));
  }

  /// Decreases the hue by the given amount in degrees (wraps around 0–360°).
  pub fn decrement_hue(&mut self, amount: impl Into<Component>) {
    self.decrement_h(amount.into() / 360.0)
  }

  /// Decreases the L\* component by the given amount.
  pub fn decrement_l(&mut self, amount: impl Into<Component>) {
    self.l -= amount.into();
  }

  /// Generates a sequence of evenly-spaced colors between `self` and `other`.
  ///
  /// Returns `steps` colors including both endpoints, interpolated in the LCh color space.
  /// When `steps` is 0 the result is empty. When `steps` is 1 the result contains only `self`.
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

  /// Returns the normalized hue component (0.0–1.0).
  pub fn h(&self) -> f64 {
    self.h.0
  }

  /// Returns the hue in degrees (0–360°).
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

  /// Increases the normalized hue by the given amount (wraps around 0.0–1.0).
  pub fn increment_h(&mut self, amount: impl Into<Component>) {
    self.h = Component::new((self.h.0 + amount.into().0).rem_euclid(1.0));
  }

  /// Increases the hue by the given amount in degrees (wraps around 0–360°).
  pub fn increment_hue(&mut self, amount: impl Into<Component>) {
    self.increment_h(amount.into() / 360.0)
  }

  /// Increases the L\* component by the given amount.
  pub fn increment_l(&mut self, amount: impl Into<Component>) {
    self.l += amount.into();
  }

  /// Returns the L\* (lightness) component.
  pub fn l(&self) -> f64 {
    self.l.0
  }

  /// Interpolates between `self` and `other` at parameter `t`, returning a new color.
  ///
  /// When `t` is 0.0 the result matches `self`, when 1.0 it matches `other`.
  /// Values outside 0.0–1.0 extrapolate beyond the endpoints. Interpolation is
  /// performed in the LCh color space with shortest-arc hue and achromatic handling
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

  /// Scales the normalized hue by the given factor (wraps around 0.0–1.0).
  pub fn scale_h(&mut self, factor: impl Into<Component>) {
    self.h = Component::new((self.h.0 * factor.into().0).rem_euclid(1.0));
  }

  /// Alias for [`Self::scale_h`].
  pub fn scale_hue(&mut self, factor: impl Into<Component>) {
    self.scale_h(factor)
  }

  /// Scales the L\* component by the given factor.
  pub fn scale_l(&mut self, factor: impl Into<Component>) {
    self.l *= factor.into();
  }

  /// Sets the C\* component.
  pub fn set_c(&mut self, c: impl Into<Component>) {
    self.c = c.into();
  }

  /// Alias for [`Self::set_c`].
  pub fn set_chroma(&mut self, chroma: impl Into<Component>) {
    self.set_c(chroma)
  }

  /// Sets the [L\*, C\*, H] components from an array.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_l(components[0].clone());
    self.set_c(components[1].clone());
    self.set_h(components[2].clone());
  }

  /// Sets the normalized hue component (0.0–1.0).
  pub fn set_h(&mut self, h: impl Into<Component>) {
    self.h = h.into();
  }

  /// Sets the hue from a value in degrees (0–360°).
  pub fn set_hue(&mut self, hue: impl Into<Component>) {
    self.h = Component::new((hue.into().0 / 360.0).rem_euclid(1.0));
  }

  /// Sets the L\* component.
  pub fn set_l(&mut self, l: impl Into<Component>) {
    self.l = l.into();
  }

  /// Converts to the CIE L\*a\*b\* color space.
  pub fn to_lab(&self) -> Lab {
    let h_rad = self.h.0 * 2.0 * std::f64::consts::PI;
    let a = self.c.0 * h_rad.cos();
    let b = self.c.0 * h_rad.sin();

    Lab::new(self.l, a, b).with_context(self.context).with_alpha(self.alpha)
  }

  /// Converts to the specified RGB color space.
  pub fn to_rgb<S>(&self) -> Rgb<S>
  where
    S: RgbSpec,
  {
    self.to_lab().to_rgb::<S>()
  }

  /// Converts to the CIE XYZ color space.
  pub fn to_xyz(&self) -> Xyz {
    self.to_lab().to_xyz()
  }

  /// Returns a new color with the given C\* value.
  pub fn with_c(&self, c: impl Into<Component>) -> Self {
    Self {
      c: c.into(),
      ..*self
    }
  }

  /// Returns a new color with C\* decreased by the given amount.
  pub fn with_c_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.decrement_c(amount);
    lch
  }

  /// Returns a new color with C\* increased by the given amount.
  pub fn with_c_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.increment_c(amount);
    lch
  }

  /// Returns a new color with C\* scaled by the given factor.
  pub fn with_c_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.scale_c(factor);
    lch
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

  /// Returns a new color with the given normalized hue (0.0–1.0).
  pub fn with_h(&self, h: impl Into<Component>) -> Self {
    Self {
      h: h.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized hue decreased by the given amount (wraps around 0.0–1.0).
  pub fn with_h_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.decrement_h(amount);
    lch
  }

  /// Returns a new color with normalized hue increased by the given amount (wraps around 0.0–1.0).
  pub fn with_h_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.increment_h(amount);
    lch
  }

  /// Returns a new color with normalized hue scaled by the given factor (wraps around 0.0–1.0).
  pub fn with_h_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.scale_h(factor);
    lch
  }

  /// Returns a new color with the given hue in degrees (0–360°).
  pub fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self {
      h: Component::new((hue.into().0 / 360.0).rem_euclid(1.0)),
      ..*self
    }
  }

  /// Returns a new color with hue decreased by the given amount in degrees.
  pub fn with_hue_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.decrement_hue(amount);
    lch
  }

  /// Returns a new color with hue increased by the given amount in degrees.
  pub fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.increment_hue(amount);
    lch
  }

  /// Returns a new color with hue scaled by the given factor.
  pub fn with_hue_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.scale_hue(factor);
    lch
  }

  /// Returns a new color with the given L\* value.
  pub fn with_l(&self, l: impl Into<Component>) -> Self {
    Self {
      l: l.into(),
      ..*self
    }
  }

  /// Returns a new color with L\* decreased by the given amount.
  pub fn with_l_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.decrement_l(amount);
    lch
  }

  /// Returns a new color with L\* increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.increment_l(amount);
    lch
  }

  /// Returns a new color with L\* scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lch = *self;
    lch.scale_l(factor);
    lch
  }
}

impl<T> Add<T> for Lch
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Lch {
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

impl Display for Lch {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(4);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Lch({:.precision$}, {:.precision$}, {:.precision$}°, {:.0}%)",
        self.l,
        self.c,
        self.hue(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "Lch({:.precision$}, {:.precision$}, {:.precision$}°)",
        self.l,
        self.c,
        self.hue()
      )
    }
  }
}

impl<T> Div<T> for Lch
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Lch
where
  T: Into<Component>,
{
  fn from([l, c, h]: [T; 3]) -> Self {
    Self::new(l, c, h)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Lch
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_lch()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Lch
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_lch()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Lch
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_lch()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Lch
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_lch()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Lch
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_lch()
  }
}

impl From<Lab> for Lch {
  fn from(lab: Lab) -> Self {
    lab.to_lch()
  }
}

impl From<Lms> for Lch {
  fn from(lms: Lms) -> Self {
    lms.to_lch()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Lch {
  fn from(luv: Luv) -> Self {
    luv.to_lch()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Lch {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_lch()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Lch {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_lch()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Lch {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_lch()
  }
}

#[cfg(feature = "space-oklab")]
impl From<Oklab> for Lch {
  fn from(oklab: Oklab) -> Self {
    oklab.to_lch()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Lch {
  fn from(oklch: Oklch) -> Self {
    oklch.to_lch()
  }
}

impl<S> From<Rgb<S>> for Lch
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_lch()
  }
}

#[cfg(feature = "space-xyy")]
impl From<Xyy> for Lch {
  fn from(xyy: Xyy) -> Self {
    xyy.to_lch()
  }
}

impl From<Xyz> for Lch {
  fn from(xyz: Xyz) -> Self {
    xyz.to_lch()
  }
}

impl<T> Mul<T> for Lch
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Lch
where
  T: Into<Lch> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.l == other.l && self.c == other.c && self.h == other.h
  }
}

impl<T> Sub<T> for Lch
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Lch {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Lch {
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
    fn it_adds_two_lch_colors() {
      let a = Lch::new(50.0, 30.0, 180.0);
      let b = Lch::new(30.0, 20.0, 90.0);
      let result = a + b;

      assert!(result.l() > 0.0);
    }
  }

  mod adapt_to {
    use super::*;
    use crate::{Illuminant, illuminant::IlluminantType, spectral::Spd};

    static TEST_SPD_A: &[(u32, f64)] = &[
      (380, 9.80),
      (400, 14.71),
      (420, 20.99),
      (440, 28.70),
      (460, 37.81),
      (480, 48.24),
      (500, 59.86),
      (520, 72.50),
      (540, 85.95),
      (560, 100.00),
      (580, 114.44),
      (600, 129.04),
      (620, 143.62),
      (640, 157.98),
      (660, 171.96),
      (680, 185.43),
      (700, 198.26),
      (720, 210.36),
      (740, 221.67),
      (760, 232.12),
      (780, 241.68),
    ];

    #[test]
    fn it_returns_same_values_when_white_points_match() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let same_context = Lch::DEFAULT_CONTEXT;
      let adapted = lch.adapt_to(same_context);

      assert!((adapted.l() - lch.l()).abs() < 1e-10);
      assert!((adapted.c() - lch.c()).abs() < 1e-10);
      assert!((adapted.h() - lch.h()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_values_for_non_d65_source() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let context_a = ColorimetricContext::new().with_illuminant(illuminant_a);
      let lch = Lch::new(50.0, 30.0, 180.0).with_context(context_a);
      let adapted = lch.adapt_to(Lch::DEFAULT_CONTEXT);

      assert!(
        (adapted.l() - lch.l()).abs() > 0.01
          || (adapted.c() - lch.c()).abs() > 0.01
          || (adapted.h() - lch.h()).abs() > 0.001
      );
    }

    #[test]
    fn it_preserves_alpha() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let target_context = ColorimetricContext::new().with_illuminant(illuminant_a);
      let lch = Lch::new(50.0, 30.0, 180.0).with_alpha(0.5);
      let adapted = lch.adapt_to(target_context);

      assert!((adapted.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod c {
    use super::*;

    #[test]
    fn it_returns_c_component() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert!((lch.c() - 30.0).abs() < 1e-10);
    }
  }

  mod chroma {
    use super::*;

    #[test]
    fn it_returns_chroma_as_alias() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert!((lch.chroma() - 30.0).abs() < 1e-10);
    }
  }

  mod components {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_components_as_array() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let [l, c, h] = lch.components();

      assert_eq!(l, 50.0);
      assert_eq!(c, 30.0);
      assert_eq!(h, 0.5);
    }
  }

  mod decrement_c {
    use super::*;

    #[test]
    fn it_decreases_c_component() {
      let mut lch = Lch::new(50.0, 30.0, 180.0);
      lch.decrement_c(10.0);

      assert!((lch.c() - 20.0).abs() < 1e-10);
    }
  }

  mod decrement_h {
    use super::*;

    #[test]
    fn it_decreases_h_with_wrapping() {
      let mut lch = Lch::new(50.0, 30.0, 36.0);
      lch.decrement_h(0.2);

      assert!((lch.h() - 0.9).abs() < 1e-10);
    }
  }

  mod decrement_hue {
    use super::*;

    #[test]
    fn it_decreases_hue_in_degrees() {
      let mut lch = Lch::new(50.0, 30.0, 180.0);
      lch.decrement_hue(90.0);

      assert!((lch.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod decrement_l {
    use super::*;

    #[test]
    fn it_decreases_l_component() {
      let mut lch = Lch::new(50.0, 30.0, 180.0);
      lch.decrement_l(10.0);

      assert!((lch.l() - 40.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert_eq!(format!("{}", lch), "Lch(50.0000, 30.0000, 180.0000°)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert_eq!(format!("{:.2}", lch), "Lch(50.00, 30.00, 180.00°)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let lch = Lch::new(50.0, 30.0, 180.0).with_alpha(0.5);

      assert_eq!(format!("{}", lch), "Lch(50.0000, 30.0000, 180.0000°, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert_eq!(format!("{}", lch), "Lch(50.0000, 30.0000, 180.0000°)");
    }
  }

  mod from_array {
    use super::*;

    #[test]
    fn it_creates_from_f64_array() {
      let lch = Lch::from([50.0, 30.0, 180.0]);

      assert!((lch.l() - 50.0).abs() < 1e-10);
      assert!((lch.c() - 30.0).abs() < 1e-10);
      assert!((lch.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod from_lab {
    use super::*;

    #[test]
    fn it_converts_from_lab() {
      let lab = Lab::new(50.0, 0.0, 30.0);
      let lch = Lch::from(lab);

      assert!((lch.l() - 50.0).abs() < 1e-10);
      assert!((lch.c() - 30.0).abs() < 1e-10);
      assert!((lch.hue() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let lab = Lab::new(50.0, 20.0, -30.0).with_alpha(0.5);
      let lch = Lch::from(lab);

      assert!((lch.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let lch = Lch::from(rgb);

      assert!((lch.l() - 100.0).abs() < 0.01);
      assert!(lch.c().abs() < 0.01);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let lch = Lch::from(rgb);

      assert!(lch.l().abs() < 1e-10);
      assert!(lch.c().abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let lch = Lch::from(rgb);

      assert!((lch.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let lch = Lch::from(xyz);

      assert!(lch.l() > 0.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let lch = Lch::from(xyz);

      assert!((lch.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod gradient {
    use super::*;

    #[test]
    fn zero_steps_is_empty() {
      let c1 = Lch::new(50.0, 30.0, 180.0);
      let c2 = Lch::new(80.0, 10.0, 90.0);
      assert!(c1.gradient(c2.to_xyz(), 0).is_empty());
    }

    #[test]
    fn one_step_returns_self() {
      let c1 = Lch::new(50.0, 30.0, 180.0);
      let c2 = Lch::new(80.0, 10.0, 90.0);
      let steps = c1.gradient(c2.to_xyz(), 1);
      assert_eq!(steps.len(), 1);
      assert!((steps[0].l() - c1.l()).abs() < 1e-4);
    }

    #[test]
    fn two_steps_returns_endpoints() {
      let c1 = Lch::new(50.0, 30.0, 180.0);
      let c2 = Lch::new(80.0, 10.0, 90.0);
      let steps = c1.gradient(c2.to_xyz(), 2);
      assert_eq!(steps.len(), 2);
      assert!((steps[0].l() - c1.l()).abs() < 1e-4);
      assert!((steps[1].l() - c2.l()).abs() < 1e-4);
    }

    #[test]
    fn five_steps_correct_count() {
      let c1 = Lch::new(20.0, 10.0, 0.0);
      let c2 = Lch::new(90.0, 5.0, 180.0);
      assert_eq!(c1.gradient(c2.to_xyz(), 5).len(), 5);
    }

    #[test]
    fn monotonic_lightness_dark_to_light() {
      let dark = Lch::new(10.0, 0.0, 0.0);
      let light = Lch::new(90.0, 0.0, 0.0);
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

  mod h {
    use super::*;

    #[test]
    fn it_returns_normalized_hue() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert!((lch.h() - 0.5).abs() < 1e-10);
    }
  }

  mod hue {
    use super::*;

    #[test]
    fn it_returns_hue_in_degrees() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert!((lch.hue() - 180.0).abs() < 1e-10);
    }
  }

  mod increment_c {
    use super::*;

    #[test]
    fn it_increases_c_component() {
      let mut lch = Lch::new(50.0, 30.0, 180.0);
      lch.increment_c(10.0);

      assert!((lch.c() - 40.0).abs() < 1e-10);
    }
  }

  mod increment_h {
    use super::*;

    #[test]
    fn it_increases_h_with_wrapping() {
      let mut lch = Lch::new(50.0, 30.0, 324.0);
      lch.increment_h(0.2);

      assert!((lch.h() - 0.1).abs() < 1e-10);
    }
  }

  mod increment_hue {
    use super::*;

    #[test]
    fn it_increases_hue_in_degrees() {
      let mut lch = Lch::new(50.0, 30.0, 180.0);
      lch.increment_hue(90.0);

      assert!((lch.hue() - 270.0).abs() < 1e-10);
    }
  }

  mod increment_l {
    use super::*;

    #[test]
    fn it_increases_l_component() {
      let mut lch = Lch::new(50.0, 30.0, 180.0);
      lch.increment_l(10.0);

      assert!((lch.l() - 60.0).abs() < 1e-10);
    }
  }

  mod l {
    use super::*;

    #[test]
    fn it_returns_l_component() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert!((lch.l() - 50.0).abs() < 1e-10);
    }
  }

  mod mix {
    use super::*;

    const EPSILON: f64 = 1e-4;

    #[test]
    fn at_zero_returns_self() {
      let c1 = Lch::new(60.0, 40.0, 30.0);
      let c2 = Lch::new(40.0, 20.0, 270.0);
      let result = c1.mix(c2.to_xyz(), 0.0);
      assert!((result.l() - c1.l()).abs() < EPSILON);
      assert!((result.c() - c1.c()).abs() < EPSILON);
    }

    #[test]
    fn at_one_returns_other() {
      let c1 = Lch::new(60.0, 40.0, 30.0);
      let c2 = Lch::new(40.0, 20.0, 270.0);
      let result = c1.mix(c2.to_xyz(), 1.0);
      assert!((result.l() - c2.l()).abs() < EPSILON);
      assert!((result.c() - c2.c()).abs() < EPSILON);
    }

    #[test]
    fn midpoint_is_between() {
      let c1 = Lch::new(20.0, 0.0, 0.0);
      let c2 = Lch::new(80.0, 0.0, 0.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!(mid.l() > 30.0 && mid.l() < 70.0);
    }

    #[test]
    fn alpha_interpolation() {
      let c1 = Lch::new(50.0, 10.0, 180.0).with_alpha(0.0);
      let c2 = Lch::new(50.0, 10.0, 180.0).with_alpha(1.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!((mid.alpha() - 0.5).abs() < EPSILON);
    }

    #[test]
    fn extrapolation() {
      let c1 = Lch::new(20.0, 0.0, 0.0);
      let c2 = Lch::new(80.0, 0.0, 0.0);
      let beyond = c1.mix(c2.to_xyz(), 1.5);
      assert!(beyond.l() > c2.l());
    }

    #[test]
    fn cross_type() {
      let lch = Lch::new(60.0, 40.0, 30.0);
      let xyz = Xyz::new(0.18048, 0.07219, 0.95030);
      let _result = lch.mix(xyz, 0.5);
    }

    #[test]
    fn shortest_arc_hue() {
      let c1 = Lch::new(60.0, 40.0, 350.0);
      let c2 = Lch::new(60.0, 40.0, 10.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      let hue = mid.hue();
      assert!(hue < 20.0 || hue > 340.0, "Hue {hue} should be near 0°/360°");
    }

    #[test]
    fn both_achromatic() {
      let c1 = Lch::new(20.0, 0.0, 0.0);
      let c2 = Lch::new(80.0, 0.0, 0.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!(mid.c() < 0.01);
    }

    #[test]
    fn one_achromatic() {
      let grey = Lch::new(50.0, 0.0, 0.0);
      let red = Lch::new(60.0, 40.0, 30.0);
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
      let c1 = Lch::new(60.0, 40.0, 30.0);
      let c2 = Lch::new(40.0, 20.0, 270.0);
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
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert!((lch.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let lch = Lch::new(50.0, 30.0, 180.0);

      assert_eq!(lch.context().illuminant().name(), "D65");
    }

    #[test]
    fn it_normalizes_hue_to_zero_one() {
      let lch = Lch::new(50.0, 30.0, 450.0);

      assert!((lch.h() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn it_normalizes_negative_hue() {
      let lch = Lch::new(50.0, 30.0, -90.0);

      assert!((lch.h() - 0.75).abs() < 1e-10);
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Lch::new(50.0, 30.0, 180.0);
      let b = Lch::new(50.0, 30.0, 180.0);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Lch::new(50.0, 30.0, 180.0);
      let b = Lch::new(60.0, 30.0, 180.0);

      assert!(a != b);
    }
  }

  mod scale_c {
    use super::*;

    #[test]
    fn it_scales_c_component() {
      let mut lch = Lch::new(50.0, 30.0, 180.0);
      lch.scale_c(2.0);

      assert!((lch.c() - 60.0).abs() < 1e-10);
    }
  }

  mod scale_h {
    use super::*;

    #[test]
    fn it_scales_h_with_wrapping() {
      let mut lch = Lch::new(50.0, 30.0, 270.0);
      lch.scale_h(2.0);

      assert!((lch.h() - 0.5).abs() < 1e-10);
    }
  }

  mod scale_l {
    use super::*;

    #[test]
    fn it_scales_l_component() {
      let mut lch = Lch::new(50.0, 30.0, 180.0);
      lch.scale_l(2.0);

      assert!((lch.l() - 100.0).abs() < 1e-10);
    }
  }

  mod to_lab {
    use super::*;

    #[test]
    fn it_converts_to_lab() {
      let lch = Lch::new(50.0, 30.0, 90.0);
      let lab = lch.to_lab();

      assert!((lab.l() - 50.0).abs() < 1e-10);
      assert!(lab.a().abs() < 1e-10);
      assert!((lab.b() - 30.0).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_through_lab() {
      let original = Lch::new(50.0, 30.0, 180.0);
      let roundtrip = Lch::from(original.to_lab());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.c() - roundtrip.c()).abs() < 1e-10);
      assert!((original.h() - roundtrip.h()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let lch = Lch::new(50.0, 30.0, 180.0).with_alpha(0.7);
      let lab = lch.to_lab();

      assert!((lab.alpha() - 0.7).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_context() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let lab = lch.to_lab();

      assert_eq!(lab.context().illuminant().name(), "D65");
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let lch = Lch::new(50.0, 0.0, 0.0);
      let rgb = lch.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_roundtrips_through_rgb() {
      let original = Lch::from(Rgb::<Srgb>::new(128, 64, 200));
      let roundtrip = Lch::from(original.to_rgb::<Srgb>());

      assert!((original.l() - roundtrip.l()).abs() < 0.5);
      assert!((original.c() - roundtrip.c()).abs() < 0.5);
      assert!((original.h() - roundtrip.h()).abs() < 0.01);
    }

    #[test]
    fn it_preserves_alpha() {
      let lch = Lch::new(50.0, 30.0, 180.0).with_alpha(0.7);
      let rgb = lch.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let lch = Lch::new(50.0, 0.0, 0.0);
      let xyz = lch.to_xyz();

      assert!(xyz.y() > 0.0);
    }

    #[test]
    fn it_roundtrips_through_xyz() {
      let original = Lch::new(50.0, 30.0, 180.0);
      let roundtrip = Lch::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.c() - roundtrip.c()).abs() < 1e-10);
      assert!((original.h() - roundtrip.h()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let lch = Lch::new(50.0, 30.0, 180.0).with_alpha(0.3);
      let xyz = lch.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let lch = Lch::try_from("#FF5733").unwrap();

      assert!(lch.l() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Lch::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
    }
  }

  mod with_c {
    use super::*;

    #[test]
    fn it_returns_new_color_with_c() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_c(40.0);

      assert!((result.c() - 40.0).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_c_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_c_decremented() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_c_decremented_by(10.0);

      assert!((result.c() - 20.0).abs() < 1e-10);
    }
  }

  mod with_c_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_c_incremented() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_c_incremented_by(10.0);

      assert!((result.c() - 40.0).abs() < 1e-10);
    }
  }

  mod with_c_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_c_scaled() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_c_scaled_by(2.0);

      assert!((result.c() - 60.0).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let context = ColorimetricContext::default();
      let result = lch.with_context(context);

      assert!((result.l() - 50.0).abs() < 1e-10);
    }
  }

  mod with_h {
    use super::*;

    #[test]
    fn it_returns_new_color_with_h() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_h(0.75);

      assert!((result.h() - 0.75).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
      assert!((result.c() - 30.0).abs() < 1e-10);
    }
  }

  mod with_h_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_h_decremented() {
      let lch = Lch::new(50.0, 30.0, 36.0);
      let result = lch.with_h_decremented_by(0.2);

      assert!((result.h() - 0.9).abs() < 1e-10);
    }
  }

  mod with_h_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_h_incremented() {
      let lch = Lch::new(50.0, 30.0, 324.0);
      let result = lch.with_h_incremented_by(0.2);

      assert!((result.h() - 0.1).abs() < 1e-10);
    }
  }

  mod with_h_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_h_scaled() {
      let lch = Lch::new(50.0, 30.0, 270.0);
      let result = lch.with_h_scaled_by(2.0);

      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_hue {
    use super::*;

    #[test]
    fn it_returns_new_color_with_hue_in_degrees() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_hue(270.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
    }
  }

  mod with_hue_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_hue_decremented() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_hue_decremented_by(90.0);

      assert!((result.hue() - 90.0).abs() < 1e-10);
    }
  }

  mod with_hue_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_hue_incremented() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_hue_incremented_by(90.0);

      assert!((result.hue() - 270.0).abs() < 1e-10);
    }
  }

  mod with_l {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_l(80.0);

      assert!((result.l() - 80.0).abs() < 1e-10);
      assert!((result.c() - 30.0).abs() < 1e-10);
      assert!((result.h() - 0.5).abs() < 1e-10);
    }
  }

  mod with_l_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_decremented() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_l_decremented_by(10.0);

      assert!((result.l() - 40.0).abs() < 1e-10);
    }
  }

  mod with_l_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_incremented() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_l_incremented_by(10.0);

      assert!((result.l() - 60.0).abs() < 1e-10);
    }
  }

  mod with_l_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_scaled() {
      let lch = Lch::new(50.0, 30.0, 180.0);
      let result = lch.with_l_scaled_by(2.0);

      assert!((result.l() - 100.0).abs() < 1e-10);
    }
  }
}
