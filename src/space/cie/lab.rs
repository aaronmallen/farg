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
  ColorimetricContext, Illuminant, Observer,
  component::Component,
  space::{ColorSpace, Lms, Rgb, RgbSpec, Srgb, Xyz},
};

/// CIE 1976 L\*a\*b\* threshold: δ³ = (6/29)³.
const DELTA_CUBED: f64 = (6.0 / 29.0) * (6.0 / 29.0) * (6.0 / 29.0);

/// CIE 1976 L\*a\*b\* threshold: δ = 6/29.
const DELTA: f64 = 6.0 / 29.0;

/// CIE 1976 L\*a\*b\* scaling factor: 3δ² = 3·(6/29)².
const THREE_DELTA_SQ: f64 = 3.0 * (6.0 / 29.0) * (6.0 / 29.0);

/// CIE 1976 L\*a\*b\* (CIELAB) color space.
///
/// A perceptually-motivated color space standardized by the CIE in 1976. L\* represents
/// lightness (0–100), a\* represents the green–red chromaticity axis, and b\* represents
/// the blue–yellow chromaticity axis. Values are computed relative to a reference white
/// point (default: D65 / CIE 1931 2°).
#[derive(Clone, Copy, Debug)]
pub struct Lab {
  a: Component,
  alpha: Component,
  b: Component,
  context: ColorimetricContext,
  l: Component,
}

impl Lab {
  /// The default viewing context for Lab (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new Lab color with the default viewing context.
  pub fn new(l: impl Into<Component>, a: impl Into<Component>, b: impl Into<Component>) -> Self {
    Self {
      a: a.into(),
      alpha: Component::new(1.0),
      b: b.into(),
      context: Self::DEFAULT_CONTEXT,
      l: l.into(),
    }
  }

  /// Creates a new Lab color in a const context.
  pub const fn new_const(l: f64, a: f64, b: f64) -> Self {
    Self {
      a: Component::new_const(a),
      alpha: Component::new_const(1.0),
      b: Component::new_const(b),
      context: Self::DEFAULT_CONTEXT,
      l: Component::new_const(l),
    }
  }

  /// Returns the a\* (green–red) component.
  pub fn a(&self) -> f64 {
    self.a.0
  }

  /// Adapts this color to a different viewing context via XYZ.
  pub fn adapt_to(&self, context: ColorimetricContext) -> Self {
    let reference_white = self.context.reference_white();
    let target_white = context.reference_white();

    if reference_white == target_white {
      return self.with_context(context);
    }

    self.to_xyz().adapt_to(context).to_lab()
  }

  /// Returns the b\* (blue–yellow) component.
  pub fn b(&self) -> f64 {
    self.b.0
  }

  /// Returns the [L\*, a\*, b\*] components as an array.
  pub fn components(&self) -> [f64; 3] {
    [self.l.0, self.a.0, self.b.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases the a\* component by the given amount.
  pub fn decrement_a(&mut self, amount: impl Into<Component>) {
    self.a -= amount.into();
  }

  /// Decreases the b\* component by the given amount.
  pub fn decrement_b(&mut self, amount: impl Into<Component>) {
    self.b -= amount.into();
  }

  /// Decreases the L\* component by the given amount.
  pub fn decrement_l(&mut self, amount: impl Into<Component>) {
    self.l -= amount.into();
  }

  /// Generates a sequence of evenly-spaced colors between `self` and `other` in rectangular L\*a\*b\*.
  ///
  /// Returns `steps` colors including both endpoints, interpolated directly in L\*/a\*/b\*
  /// coordinates. When `steps` is 0 the result is empty. When `steps` is 1 the result
  /// contains only `self`.
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

  /// Increases the a\* component by the given amount.
  pub fn increment_a(&mut self, amount: impl Into<Component>) {
    self.a += amount.into();
  }

  /// Increases the b\* component by the given amount.
  pub fn increment_b(&mut self, amount: impl Into<Component>) {
    self.b += amount.into();
  }

  /// Increases the L\* component by the given amount.
  pub fn increment_l(&mut self, amount: impl Into<Component>) {
    self.l += amount.into();
  }

  /// Returns the L\* (lightness) component.
  pub fn l(&self) -> f64 {
    self.l.0
  }

  /// Interpolates between `self` and `other` at parameter `t` in rectangular L\*a\*b\*.
  ///
  /// When `t` is 0.0 the result matches `self`, when 1.0 it matches `other`.
  /// Values outside 0.0–1.0 extrapolate beyond the endpoints. Interpolation is
  /// performed directly in L\*/a\*/b\* rectangular coordinates, which avoids
  /// hue-interpolation desaturation and handles neutrals naturally.
  ///
  /// Accepts any color type that can be converted to [`Xyz`].
  pub fn mix(&self, other: impl Into<Xyz>, t: f64) -> Self {
    let other = Self::from(other.into());

    let l = Component::new(self.l()).lerp(other.l(), t);
    let a = Component::new(self.a()).lerp(other.a(), t);
    let b = Component::new(self.b()).lerp(other.b(), t);
    let alpha = Component::new(self.alpha()).lerp(other.alpha(), t);

    Self::new(l, a, b).with_alpha(alpha)
  }

  /// Interpolates `self` toward `other` at parameter `t` in rectangular L\*a\*b\*, mutating in place.
  ///
  /// See [`mix`](Self::mix) for details on the interpolation behavior.
  pub fn mixed_with(&mut self, other: impl Into<Xyz>, t: f64) {
    let result = self.mix(other, t);
    self.l = result.l;
    self.a = result.a;
    self.b = result.b;
    self.alpha = result.alpha;
  }

  /// Scales the a\* component by the given factor.
  pub fn scale_a(&mut self, factor: impl Into<Component>) {
    self.a *= factor.into();
  }

  /// Scales the b\* component by the given factor.
  pub fn scale_b(&mut self, factor: impl Into<Component>) {
    self.b *= factor.into();
  }

  /// Scales the L\* component by the given factor.
  pub fn scale_l(&mut self, factor: impl Into<Component>) {
    self.l *= factor.into();
  }

  /// Sets the a\* component.
  pub fn set_a(&mut self, a: impl Into<Component>) {
    self.a = a.into();
  }

  /// Sets the b\* component.
  pub fn set_b(&mut self, b: impl Into<Component>) {
    self.b = b.into();
  }

  /// Sets the [L\*, a\*, b\*] components from an array.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_l(components[0].clone());
    self.set_a(components[1].clone());
    self.set_b(components[2].clone());
  }

  /// Sets the L\* component.
  pub fn set_l(&mut self, l: impl Into<Component>) {
    self.l = l.into();
  }

  /// Converts to the CIE LCh color space (cylindrical form).
  #[cfg(feature = "space-lch")]
  pub fn to_lch(&self) -> Lch {
    let [l, a, b] = self.components();
    let c = (a * a + b * b).sqrt();
    let h = b.atan2(a).to_degrees();

    Lch::new(l, c, h).with_context(self.context).with_alpha(self.alpha)
  }

  /// Converts to the CIE XYZ color space.
  pub fn to_xyz(&self) -> Xyz {
    let [l, a, b] = self.components();
    let [xn, yn, zn] = self.context.reference_white().components();

    let fy = (l + 16.0) / 116.0;
    let fx = fy + a / 500.0;
    let fz = fy - b / 200.0;

    let x = xn * lab_f_inv(fx);
    let y = yn * lab_f_inv(fy);
    let z = zn * lab_f_inv(fz);

    Xyz::new(x, y, z).with_context(self.context).with_alpha(self.alpha)
  }

  /// Returns a new color with the given a\* value.
  pub fn with_a(&self, a: impl Into<Component>) -> Self {
    Self {
      a: a.into(),
      ..*self
    }
  }

  /// Returns a new color with a\* decreased by the given amount.
  pub fn with_a_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lab = *self;
    lab.decrement_a(amount);
    lab
  }

  /// Returns a new color with a\* increased by the given amount.
  pub fn with_a_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lab = *self;
    lab.increment_a(amount);
    lab
  }

  /// Returns a new color with a\* scaled by the given factor.
  pub fn with_a_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lab = *self;
    lab.scale_a(factor);
    lab
  }

  /// Returns a new color with the given b\* value.
  pub fn with_b(&self, b: impl Into<Component>) -> Self {
    Self {
      b: b.into(),
      ..*self
    }
  }

  /// Returns a new color with b\* decreased by the given amount.
  pub fn with_b_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lab = *self;
    lab.decrement_b(amount);
    lab
  }

  /// Returns a new color with b\* increased by the given amount.
  pub fn with_b_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lab = *self;
    lab.increment_b(amount);
    lab
  }

  /// Returns a new color with b\* scaled by the given factor.
  pub fn with_b_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lab = *self;
    lab.scale_b(factor);
    lab
  }

  /// Returns this color with a different viewing context (without adaptation).
  pub fn with_context(&self, context: ColorimetricContext) -> Self {
    Self {
      context,
      ..*self
    }
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
    let mut lab = *self;
    lab.decrement_l(amount);
    lab
  }

  /// Returns a new color with L\* increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lab = *self;
    lab.increment_l(amount);
    lab
  }

  /// Returns a new color with L\* scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lab = *self;
    lab.scale_l(factor);
    lab
  }
}

impl<T> Add<T> for Lab
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Lab {
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

impl Display for Lab {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(4);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Lab({:.precision$}, {:.precision$}, {:.precision$}, {:.0}%)",
        self.l,
        self.a,
        self.b,
        self.opacity()
      )
    } else {
      write!(
        f,
        "Lab({:.precision$}, {:.precision$}, {:.precision$})",
        self.l, self.a, self.b
      )
    }
  }
}

impl<T> Div<T> for Lab
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Lab
where
  T: Into<Component>,
{
  fn from([l, a, b]: [T; 3]) -> Self {
    Self::new(l, a, b)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Lab
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_lab()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Lab
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_lab()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Lab
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_lab()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Lab
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_lab()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Lab
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_lab()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Lab {
  fn from(lch: Lch) -> Self {
    lch.to_lab()
  }
}

#[cfg(feature = "space-lchuv")]
impl From<Lchuv> for Lab {
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_lab()
  }
}

impl From<Lms> for Lab {
  fn from(lms: Lms) -> Self {
    lms.to_lab()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Lab {
  fn from(luv: Luv) -> Self {
    luv.to_lab()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Lab {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_lab()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Lab {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_lab()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Lab {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_lab()
  }
}

#[cfg(feature = "space-oklab")]
impl From<Oklab> for Lab {
  fn from(oklab: Oklab) -> Self {
    oklab.to_lab()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Lab {
  fn from(oklch: Oklch) -> Self {
    oklch.to_lab()
  }
}

impl<S> From<Rgb<S>> for Lab
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_lab()
  }
}

#[cfg(feature = "space-xyy")]
impl From<Xyy> for Lab {
  fn from(xyy: Xyy) -> Self {
    xyy.to_lab()
  }
}

impl From<Xyz> for Lab {
  fn from(xyz: Xyz) -> Self {
    xyz.to_lab()
  }
}

impl<T> Mul<T> for Lab
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Lab
where
  T: Into<Lab> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.l == other.l && self.a == other.a && self.b == other.b
  }
}

impl<T> Sub<T> for Lab
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Lab {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Lab {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

/// CIE 1976 L\*a\*b\* forward companding function.
///
/// Maps a ratio `t` (component / reference white component) to a perceptually
/// uniform scale.
pub(crate) fn lab_f(t: f64) -> f64 {
  if t > DELTA_CUBED {
    t.cbrt()
  } else {
    t / THREE_DELTA_SQ + 4.0 / 29.0
  }
}

/// CIE 1976 L\*a\*b\* inverse companding function.
///
/// Inverts [`lab_f`] to recover the original ratio from the perceptual scale.
fn lab_f_inv(t: f64) -> f64 {
  if t > DELTA {
    t * t * t
  } else {
    THREE_DELTA_SQ * (t - 4.0 / 29.0)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod a {
    use super::*;

    #[test]
    fn it_returns_a_component() {
      let lab = Lab::new(50.0, 20.0, -30.0);

      assert!((lab.a() - 20.0).abs() < 1e-10);
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

    static TEST_SPD_B: &[(u32, f64)] = &[
      (380, 49.98),
      (400, 82.75),
      (420, 91.49),
      (440, 93.43),
      (460, 104.86),
      (480, 117.01),
      (500, 100.00),
      (520, 104.78),
      (540, 105.36),
      (560, 100.00),
      (580, 95.79),
      (600, 88.69),
      (620, 90.01),
      (640, 85.49),
      (660, 81.68),
      (680, 71.61),
      (700, 64.15),
      (720, 57.26),
      (740, 51.85),
      (760, 43.06),
      (780, 37.21),
    ];

    #[test]
    fn it_returns_same_values_when_white_points_match() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let same_context = Lab::DEFAULT_CONTEXT;
      let adapted = lab.adapt_to(same_context);

      assert!((adapted.l() - lab.l()).abs() < 1e-10);
      assert!((adapted.a() - lab.a()).abs() < 1e-10);
      assert!((adapted.b() - lab.b()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_values_for_non_d65_source() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let context_a = ColorimetricContext::new().with_illuminant(illuminant_a);
      let lab = Lab::new(50.0, 20.0, -30.0).with_context(context_a);
      let adapted = lab.adapt_to(Lab::DEFAULT_CONTEXT);

      assert!((adapted.l() - lab.l()).abs() > 0.01 || (adapted.a() - lab.a()).abs() > 0.01);
    }

    #[test]
    fn it_preserves_alpha() {
      let illuminant_b = Illuminant::new("Test B", IlluminantType::Custom, Spd::new(TEST_SPD_B));
      let lab = Lab::new(50.0, 20.0, -30.0).with_alpha(0.5);
      let target_context = ColorimetricContext::new().with_illuminant(illuminant_b);
      let adapted = lab.adapt_to(target_context);

      assert!((adapted.alpha() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_updates_context_when_white_points_match() {
      let context = Lab::DEFAULT_CONTEXT;
      let lab = Lab::new(50.0, 20.0, -30.0);
      let adapted = lab.adapt_to(context);

      assert_eq!(adapted.context().illuminant().name(), "D65");
    }
  }

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_lab_colors() {
      let a = Lab::new(50.0, 20.0, -30.0);
      let b = Lab::new(30.0, -10.0, 15.0);
      let result = a + b;

      assert!(result.l() > 0.0);
    }
  }

  mod b {
    use super::*;

    #[test]
    fn it_returns_b_component() {
      let lab = Lab::new(50.0, 20.0, -30.0);

      assert!((lab.b() - -30.0).abs() < 1e-10);
    }
  }

  mod components {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_components_as_array() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let [l, a, b] = lab.components();

      assert_eq!(l, 50.0);
      assert_eq!(a, 20.0);
      assert_eq!(b, -30.0);
    }
  }

  mod clip_to_gamut {
    use super::*;

    #[test]
    fn it_moves_out_of_gamut_lab_closer_to_srgb_gamut() {
      let mut lab = Lab::new(50.0, 100.0, 100.0);
      lab.clip_to_gamut::<Srgb>();
      let rgb = lab.to_rgb::<Srgb>();

      assert!(rgb.r() >= -1e-6 && rgb.r() <= 1.0 + 1e-6);
      assert!(rgb.g() >= -1e-6 && rgb.g() <= 1.0 + 1e-6);
      assert!(rgb.b() >= -1e-6 && rgb.b() <= 1.0 + 1e-6);
    }

    #[test]
    fn it_leaves_in_gamut_color_approximately_unchanged() {
      let mut lab = Lab::new(50.0, 0.0, 0.0);
      let orig = lab;
      lab.clip_to_gamut::<Srgb>();

      assert!((lab.l() - orig.l()).abs() < 0.5);
    }
  }

  mod compress_to_gamut {
    use super::*;

    #[test]
    fn it_moves_out_of_gamut_lab_closer_to_srgb_gamut() {
      let mut lab = Lab::new(50.0, 100.0, 100.0);
      lab.compress_to_gamut::<Srgb>();
      let rgb = lab.to_rgb::<Srgb>();

      assert!(rgb.r() >= -1e-6 && rgb.r() <= 1.0 + 1e-6);
      assert!(rgb.g() >= -1e-6 && rgb.g() <= 1.0 + 1e-6);
      assert!(rgb.b() >= -1e-6 && rgb.b() <= 1.0 + 1e-6);
    }

    #[test]
    fn it_leaves_in_gamut_color_approximately_unchanged() {
      let mut lab = Lab::new(50.0, 0.0, 0.0);
      let orig = lab;
      lab.compress_to_gamut::<Srgb>();

      assert!((lab.l() - orig.l()).abs() < 0.5);
    }
  }

  mod decrement_a {
    use super::*;

    #[test]
    fn it_decreases_a_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.decrement_a(5.0);

      assert!((lab.a() - 15.0).abs() < 1e-10);
    }
  }

  mod decrement_b {
    use super::*;

    #[test]
    fn it_decreases_b_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.decrement_b(5.0);

      assert!((lab.b() - -35.0).abs() < 1e-10);
    }
  }

  mod decrement_l {
    use super::*;

    #[test]
    fn it_decreases_l_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.decrement_l(10.0);

      assert!((lab.l() - 40.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let lab = Lab::new(50.0, 20.0, -30.0);

      assert_eq!(format!("{}", lab), "Lab(50.0000, 20.0000, -30.0000)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let lab = Lab::new(50.0, 20.0, -30.0);

      assert_eq!(format!("{:.2}", lab), "Lab(50.00, 20.00, -30.00)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let lab = Lab::new(50.0, 20.0, -30.0).with_alpha(0.5);

      assert_eq!(format!("{}", lab), "Lab(50.0000, 20.0000, -30.0000, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let lab = Lab::new(50.0, 20.0, -30.0);

      assert_eq!(format!("{}", lab), "Lab(50.0000, 20.0000, -30.0000)");
    }
  }

  mod from_array {
    use super::*;

    #[test]
    fn it_creates_from_f64_array() {
      let lab = Lab::from([50.0, 20.0, -30.0]);

      assert!((lab.l() - 50.0).abs() < 1e-10);
      assert!((lab.a() - 20.0).abs() < 1e-10);
      assert!((lab.b() - -30.0).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let lab = Lab::from(rgb);

      assert!((lab.l() - 100.0).abs() < 0.01);
      assert!(lab.a().abs() < 0.01);
      assert!(lab.b().abs() < 0.01);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let lab = Lab::from(rgb);

      assert!(lab.l().abs() < 1e-10);
      assert!(lab.a().abs() < 1e-10);
      assert!(lab.b().abs() < 1e-10);
    }

    #[test]
    fn it_converts_red_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 0, 0);
      let lab = Lab::from(rgb);

      assert!((lab.l() - 53.2408).abs() < 0.01);
      assert!((lab.a() - 80.0925).abs() < 0.02);
      assert!((lab.b() - 67.2032).abs() < 0.01);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let lab = Lab::from(rgb);

      assert!((lab.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let lab = Lab::from(xyz);

      assert!(lab.l() > 0.0);
    }

    #[test]
    fn it_converts_d65_white_to_l100() {
      let xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let lab = Lab::from(xyz);

      assert!((lab.l() - 100.0).abs() < 0.01);
      assert!(lab.a().abs() < 0.01);
      assert!(lab.b().abs() < 0.01);
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let lab = Lab::from(xyz);

      assert!((lab.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod gradient {
    use super::*;

    #[test]
    fn zero_steps_is_empty() {
      let c1 = Lab::new(50.0, 20.0, -30.0);
      let c2 = Lab::new(80.0, -10.0, 40.0);
      assert!(c1.gradient(c2.to_xyz(), 0).is_empty());
    }

    #[test]
    fn one_step_returns_self() {
      let c1 = Lab::new(50.0, 20.0, -30.0);
      let c2 = Lab::new(80.0, -10.0, 40.0);
      let steps = c1.gradient(c2.to_xyz(), 1);
      assert_eq!(steps.len(), 1);
      assert!((steps[0].l() - c1.l()).abs() < 1e-4);
    }

    #[test]
    fn two_steps_returns_endpoints() {
      let c1 = Lab::new(50.0, 20.0, -30.0);
      let c2 = Lab::new(80.0, -10.0, 40.0);
      let steps = c1.gradient(c2.to_xyz(), 2);
      assert_eq!(steps.len(), 2);
      assert!((steps[0].l() - c1.l()).abs() < 1e-4);
      assert!((steps[1].l() - c2.l()).abs() < 1e-4);
    }

    #[test]
    fn five_steps_correct_count() {
      let c1 = Lab::new(10.0, 0.0, 0.0);
      let c2 = Lab::new(90.0, 0.0, 0.0);
      assert_eq!(c1.gradient(c2.to_xyz(), 5).len(), 5);
    }

    #[test]
    fn monotonic_lightness_dark_to_light() {
      let dark = Lab::new(10.0, 0.0, 0.0);
      let light = Lab::new(90.0, 0.0, 0.0);
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

  mod increment_a {
    use super::*;

    #[test]
    fn it_increases_a_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.increment_a(5.0);

      assert!((lab.a() - 25.0).abs() < 1e-10);
    }
  }

  mod increment_b {
    use super::*;

    #[test]
    fn it_increases_b_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.increment_b(5.0);

      assert!((lab.b() - -25.0).abs() < 1e-10);
    }
  }

  mod increment_l {
    use super::*;

    #[test]
    fn it_increases_l_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.increment_l(10.0);

      assert!((lab.l() - 60.0).abs() < 1e-10);
    }
  }

  mod is_in_gamut {
    use super::*;

    #[test]
    fn it_returns_true_for_neutral_gray() {
      let lab = Lab::new(50.0, 0.0, 0.0);

      assert!(lab.is_in_gamut::<Srgb>());
    }

    #[test]
    fn it_returns_false_for_highly_saturated_color() {
      let lab = Lab::new(50.0, 100.0, 100.0);

      assert!(!lab.is_in_gamut::<Srgb>());
    }
  }

  mod is_realizable {
    use super::*;

    #[test]
    fn it_returns_true_for_neutral_gray() {
      let lab = Lab::new(50.0, 0.0, 0.0);

      assert!(lab.is_realizable());
    }

    #[test]
    fn it_returns_true_for_black() {
      let lab = Lab::new(0.0, 0.0, 0.0);

      assert!(lab.is_realizable());
    }
  }

  mod l {
    use super::*;

    #[test]
    fn it_returns_l_component() {
      let lab = Lab::new(50.0, 20.0, -30.0);

      assert!((lab.l() - 50.0).abs() < 1e-10);
    }
  }

  mod mix {
    use super::*;

    const EPSILON: f64 = 1e-4;

    #[test]
    fn at_zero_returns_self() {
      let c1 = Lab::new(50.0, 20.0, -30.0);
      let c2 = Lab::new(80.0, -10.0, 40.0);
      let result = c1.mix(c2.to_xyz(), 0.0);
      assert!((result.l() - c1.l()).abs() < EPSILON);
      assert!((result.a() - c1.a()).abs() < EPSILON);
      assert!((result.b() - c1.b()).abs() < EPSILON);
    }

    #[test]
    fn at_one_returns_other() {
      let c1 = Lab::new(50.0, 20.0, -30.0);
      let c2 = Lab::new(80.0, -10.0, 40.0);
      let result = c1.mix(c2.to_xyz(), 1.0);
      assert!((result.l() - c2.l()).abs() < EPSILON);
      assert!((result.a() - c2.a()).abs() < EPSILON);
      assert!((result.b() - c2.b()).abs() < EPSILON);
    }

    #[test]
    fn midpoint_is_between() {
      let c1 = Lab::new(20.0, 0.0, 0.0);
      let c2 = Lab::new(80.0, 0.0, 0.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!(mid.l() > 30.0 && mid.l() < 70.0);
    }

    #[test]
    fn alpha_interpolation() {
      let c1 = Lab::new(50.0, 0.0, 0.0).with_alpha(0.0);
      let c2 = Lab::new(50.0, 0.0, 0.0).with_alpha(1.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!((mid.alpha() - 0.5).abs() < EPSILON);
    }

    #[test]
    fn extrapolation() {
      let c1 = Lab::new(20.0, 0.0, 0.0);
      let c2 = Lab::new(80.0, 0.0, 0.0);
      let beyond = c1.mix(c2.to_xyz(), 1.5);
      assert!(beyond.l() > c2.l());
    }

    #[test]
    fn cross_type() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let xyz = Xyz::new(0.18048, 0.07219, 0.95030);
      let _result = lab.mix(xyz, 0.5);
    }
  }

  mod mixed_with {
    use super::*;

    #[test]
    fn it_mutates_in_place() {
      let c1 = Lab::new(50.0, 20.0, -30.0);
      let c2 = Lab::new(80.0, -10.0, 40.0);
      let expected = c1.mix(c2.to_xyz(), 0.5);
      let mut color = c1;
      color.mixed_with(c2.to_xyz(), 0.5);
      assert!((color.l() - expected.l()).abs() < 1e-10);
      assert!((color.a() - expected.a()).abs() < 1e-10);
      assert!((color.b() - expected.b()).abs() < 1e-10);
      assert!((color.alpha() - expected.alpha()).abs() < 1e-10);
    }
  }

  mod new {
    use super::*;

    #[test]
    fn it_creates_with_default_alpha() {
      let lab = Lab::new(50.0, 20.0, -30.0);

      assert!((lab.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let lab = Lab::new(50.0, 20.0, -30.0);

      assert_eq!(lab.context().illuminant().name(), "D65");
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Lab::new(50.0, 20.0, -30.0);
      let b = Lab::new(50.0, 20.0, -30.0);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Lab::new(50.0, 20.0, -30.0);
      let b = Lab::new(60.0, 20.0, -30.0);

      assert!(a != b);
    }
  }

  mod perceptually_map_to_gamut {
    use super::*;

    #[test]
    fn it_leaves_in_gamut_color_approximately_unchanged() {
      let mut lab = Lab::new(50.0, 0.0, 0.0);
      let orig = lab;
      lab.perceptually_map_to_gamut::<Srgb>();

      assert!((lab.l() - orig.l()).abs() < 0.5);
    }
  }

  mod roundtrip {
    use super::*;

    #[test]
    fn it_roundtrips_through_xyz() {
      let original = Lab::new(50.0, 20.0, -30.0);
      let roundtrip = Lab::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.a() - roundtrip.a()).abs() < 1e-10);
      assert!((original.b() - roundtrip.b()).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_white() {
      let original = Lab::new(100.0, 0.0, 0.0);
      let roundtrip = Lab::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.a() - roundtrip.a()).abs() < 1e-10);
      assert!((original.b() - roundtrip.b()).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_black() {
      let original = Lab::new(0.0, 0.0, 0.0);
      let roundtrip = Lab::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.a() - roundtrip.a()).abs() < 1e-10);
      assert!((original.b() - roundtrip.b()).abs() < 1e-10);
    }
  }

  mod scale_a {
    use super::*;

    #[test]
    fn it_scales_a_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.scale_a(2.0);

      assert!((lab.a() - 40.0).abs() < 1e-10);
    }
  }

  mod scale_b {
    use super::*;

    #[test]
    fn it_scales_b_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.scale_b(2.0);

      assert!((lab.b() - -60.0).abs() < 1e-10);
    }
  }

  mod scale_l {
    use super::*;

    #[test]
    fn it_scales_l_component() {
      let mut lab = Lab::new(50.0, 20.0, -30.0);
      lab.scale_l(2.0);

      assert!((lab.l() - 100.0).abs() < 1e-10);
    }
  }

  mod scale_to_gamut {
    use super::*;

    #[test]
    fn it_moves_out_of_gamut_lab_closer_to_srgb_gamut() {
      let mut lab = Lab::new(50.0, 100.0, 100.0);
      lab.scale_to_gamut::<Srgb>();
      let rgb = lab.to_rgb::<Srgb>();

      assert!(rgb.r() >= -1e-6 && rgb.r() <= 1.0 + 1e-6);
      assert!(rgb.g() >= -1e-6 && rgb.g() <= 1.0 + 1e-6);
      assert!(rgb.b() >= -1e-6 && rgb.b() <= 1.0 + 1e-6);
    }

    #[test]
    fn it_leaves_in_gamut_color_approximately_unchanged() {
      let mut lab = Lab::new(50.0, 0.0, 0.0);
      let orig = lab;
      lab.scale_to_gamut::<Srgb>();

      assert!((lab.l() - orig.l()).abs() < 0.5);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let lab = Lab::new(50.0, 0.0, 0.0);
      let rgb = lab.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_roundtrips_through_rgb() {
      let original = Lab::from(Rgb::<Srgb>::new(128, 64, 200));
      let roundtrip = Lab::from(original.to_rgb::<Srgb>());

      assert!((original.l() - roundtrip.l()).abs() < 0.5);
      assert!((original.a() - roundtrip.a()).abs() < 0.5);
      assert!((original.b() - roundtrip.b()).abs() < 0.5);
    }

    #[test]
    fn it_preserves_alpha() {
      let lab = Lab::new(50.0, 0.0, 0.0).with_alpha(0.7);
      let rgb = lab.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let lab = Lab::new(50.0, 0.0, 0.0);
      let xyz = lab.to_xyz();

      assert!(xyz.y() > 0.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let lab = Lab::new(50.0, 0.0, 0.0).with_alpha(0.3);
      let xyz = lab.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let lab = Lab::try_from("#FF5733").unwrap();

      assert!(lab.l() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Lab::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod with_a {
    use super::*;

    #[test]
    fn it_returns_new_color_with_a() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_a(40.0);

      assert!((result.a() - 40.0).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
      assert!((result.b() - -30.0).abs() < 1e-10);
    }
  }

  mod with_a_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_a_decremented() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_a_decremented_by(5.0);

      assert!((result.a() - 15.0).abs() < 1e-10);
    }
  }

  mod with_a_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_a_incremented() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_a_incremented_by(5.0);

      assert!((result.a() - 25.0).abs() < 1e-10);
    }
  }

  mod with_a_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_a_scaled() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_a_scaled_by(2.0);

      assert!((result.a() - 40.0).abs() < 1e-10);
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
    }
  }

  mod with_b {
    use super::*;

    #[test]
    fn it_returns_new_color_with_b() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_b(40.0);

      assert!((result.b() - 40.0).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
      assert!((result.a() - 20.0).abs() < 1e-10);
    }
  }

  mod with_b_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_b_decremented() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_b_decremented_by(5.0);

      assert!((result.b() - -35.0).abs() < 1e-10);
    }
  }

  mod with_b_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_b_incremented() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_b_incremented_by(5.0);

      assert!((result.b() - -25.0).abs() < 1e-10);
    }
  }

  mod with_b_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_b_scaled() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_b_scaled_by(2.0);

      assert!((result.b() - -60.0).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let context = ColorimetricContext::default();
      let result = lab.with_context(context);

      assert!((result.l() - 50.0).abs() < 1e-10);
    }
  }

  mod with_gamut_clipped {
    use super::*;

    #[test]
    fn it_returns_approximately_in_gamut_color() {
      let lab = Lab::new(50.0, 100.0, 100.0);
      let result = lab.with_gamut_clipped::<Srgb>();
      let rgb = result.to_rgb::<Srgb>();

      assert!(rgb.r() >= -1e-6 && rgb.r() <= 1.0 + 1e-6);
      assert!(rgb.g() >= -1e-6 && rgb.g() <= 1.0 + 1e-6);
      assert!(rgb.b() >= -1e-6 && rgb.b() <= 1.0 + 1e-6);
    }

    #[test]
    fn it_does_not_mutate_original() {
      let lab = Lab::new(50.0, 100.0, 100.0);
      let _ = lab.with_gamut_clipped::<Srgb>();

      assert!((lab.a() - 100.0).abs() < 1e-10);
    }
  }

  mod with_gamut_compressed {
    use super::*;

    #[test]
    fn it_returns_approximately_in_gamut_color() {
      let lab = Lab::new(50.0, 100.0, 100.0);
      let result = lab.with_gamut_compressed::<Srgb>();
      let rgb = result.to_rgb::<Srgb>();

      assert!(rgb.r() >= -1e-6 && rgb.r() <= 1.0 + 1e-6);
      assert!(rgb.g() >= -1e-6 && rgb.g() <= 1.0 + 1e-6);
      assert!(rgb.b() >= -1e-6 && rgb.b() <= 1.0 + 1e-6);
    }

    #[test]
    fn it_does_not_mutate_original() {
      let lab = Lab::new(50.0, 100.0, 100.0);
      let _ = lab.with_gamut_compressed::<Srgb>();

      assert!((lab.a() - 100.0).abs() < 1e-10);
    }
  }

  mod with_gamut_perceptually_mapped {
    use super::*;

    #[test]
    fn it_does_not_mutate_original() {
      let lab = Lab::new(50.0, 100.0, 100.0);
      let _ = lab.with_gamut_perceptually_mapped::<Srgb>();

      assert!((lab.a() - 100.0).abs() < 1e-10);
    }
  }

  mod with_gamut_scaled {
    use super::*;

    #[test]
    fn it_returns_approximately_in_gamut_color() {
      let lab = Lab::new(50.0, 100.0, 100.0);
      let result = lab.with_gamut_scaled::<Srgb>();
      let rgb = result.to_rgb::<Srgb>();

      assert!(rgb.r() >= -1e-6 && rgb.r() <= 1.0 + 1e-6);
      assert!(rgb.g() >= -1e-6 && rgb.g() <= 1.0 + 1e-6);
      assert!(rgb.b() >= -1e-6 && rgb.b() <= 1.0 + 1e-6);
    }

    #[test]
    fn it_does_not_mutate_original() {
      let lab = Lab::new(50.0, 100.0, 100.0);
      let _ = lab.with_gamut_scaled::<Srgb>();

      assert!((lab.a() - 100.0).abs() < 1e-10);
    }
  }

  mod with_l {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_l(80.0);

      assert!((result.l() - 80.0).abs() < 1e-10);
      assert!((result.a() - 20.0).abs() < 1e-10);
      assert!((result.b() - -30.0).abs() < 1e-10);
    }
  }

  mod with_l_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_decremented() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_l_decremented_by(10.0);

      assert!((result.l() - 40.0).abs() < 1e-10);
    }
  }

  mod with_l_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_incremented() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_l_incremented_by(10.0);

      assert!((result.l() - 60.0).abs() < 1e-10);
    }
  }

  mod with_l_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_scaled() {
      let lab = Lab::new(50.0, 20.0, -30.0);
      let result = lab.with_l_scaled_by(2.0);

      assert!((result.l() - 100.0).abs() < 1e-10);
    }
  }
}
