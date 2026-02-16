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
#[cfg(feature = "space-oklch")]
use crate::space::Oklch;
#[cfg(feature = "space-xyy")]
use crate::space::Xyy;
use crate::{
  ColorimetricContext, Illuminant, Observer,
  component::Component,
  matrix::Matrix3,
  space::{ColorSpace, LinearRgb, Lms, Rgb, RgbSpec, Srgb, Xyz},
};

/// Ok* toe-function constants.
const K1: f64 = 0.206;
const K2: f64 = 0.03;
const K3: f64 = (1.0 + K1) / (1.0 + K2);

/// Oklab perceptual color space.
///
/// A perceptual color space designed for image processing, where L represents
/// perceived lightness (0.0-1.0), a represents green-red chromaticity, and b
/// represents blue-yellow chromaticity. Designed to be perceptually uniform.
#[derive(Clone, Copy, Debug)]
pub struct Oklab {
  a: Component,
  alpha: Component,
  b: Component,
  context: ColorimetricContext,
  l: Component,
}

impl Oklab {
  /// The default viewing context for Oklab (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  /// Matrix for converting cube-root LMS to Oklab L, a, b.
  pub const LINEAR_LMS_MATRIX: Matrix3 = Matrix3::new([
    [0.2104542553, 0.7936177850, -0.0040720468],
    [1.9779984951, -2.4285922050, 0.4505937099],
    [0.0259040371, 0.7827717662, -0.8086757660],
  ]);
  /// Matrix for converting linear sRGB to linear LMS.
  pub const LINEAR_RGB_MATRIX: Matrix3 = Matrix3::new([
    [0.4122214708, 0.5363325363, 0.0514459929],
    [0.2119034982, 0.6806995451, 0.1073969566],
    [0.0883024619, 0.2817188376, 0.6299787005],
  ]);
  /// Matrix for converting XYZ to linear LMS.
  pub const LINEAR_XYZ_MATRIX: Matrix3 = Matrix3::new([
    [0.8189330101, 0.3618667424, -0.1288597137],
    [0.0329845436, 0.9293118715, 0.0361456387],
    [0.0482003018, 0.2643662691, 0.6338517070],
  ]);

  /// Creates a new Oklab color with the default viewing context.
  pub fn new(l: impl Into<Component>, a: impl Into<Component>, b: impl Into<Component>) -> Self {
    Self {
      a: a.into(),
      alpha: Component::new(1.0),
      b: b.into(),
      context: Self::DEFAULT_CONTEXT,
      l: l.into(),
    }
  }

  /// Creates a new Oklab color in a const context.
  pub const fn new_const(l: f64, a: f64, b: f64) -> Self {
    Self {
      a: Component::new_const(a),
      alpha: Component::new_const(1.0),
      b: Component::new_const(b),
      context: Self::DEFAULT_CONTEXT,
      l: Component::new_const(l),
    }
  }

  /// Returns the a (green-red) component.
  pub fn a(&self) -> f64 {
    self.a.0
  }

  /// Returns the b (blue-yellow) component.
  pub fn b(&self) -> f64 {
    self.b.0
  }

  /// Returns the [L, a, b] components as an array.
  pub fn components(&self) -> [f64; 3] {
    [self.l.0, self.a.0, self.b.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases the a component by the given amount.
  pub fn decrement_a(&mut self, amount: impl Into<Component>) {
    self.a -= amount.into();
  }

  /// Decreases the b component by the given amount.
  pub fn decrement_b(&mut self, amount: impl Into<Component>) {
    self.b -= amount.into();
  }

  /// Decreases the L component by the given amount.
  pub fn decrement_l(&mut self, amount: impl Into<Component>) {
    self.l -= amount.into();
  }

  /// Generates a sequence of evenly-spaced colors between `self` and `other` in rectangular Oklab.
  ///
  /// Returns `steps` colors including both endpoints, interpolated directly in L/a/b
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

  /// Increases the a component by the given amount.
  pub fn increment_a(&mut self, amount: impl Into<Component>) {
    self.a += amount.into();
  }

  /// Increases the b component by the given amount.
  pub fn increment_b(&mut self, amount: impl Into<Component>) {
    self.b += amount.into();
  }

  /// Increases the L component by the given amount.
  pub fn increment_l(&mut self, amount: impl Into<Component>) {
    self.l += amount.into();
  }

  /// Returns the L (lightness) component.
  pub fn l(&self) -> f64 {
    self.l.0
  }

  /// Interpolates between `self` and `other` at parameter `t` in rectangular Oklab.
  ///
  /// When `t` is 0.0 the result matches `self`, when 1.0 it matches `other`.
  /// Values outside 0.0–1.0 extrapolate beyond the endpoints. Interpolation is
  /// performed directly in L/a/b rectangular coordinates, which avoids hue-interpolation
  /// desaturation and handles neutrals naturally.
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

  /// Interpolates `self` toward `other` at parameter `t` in rectangular Oklab, mutating in place.
  ///
  /// See [`mix`](Self::mix) for details on the interpolation behavior.
  pub fn mixed_with(&mut self, other: impl Into<Xyz>, t: f64) {
    let result = self.mix(other, t);
    self.l = result.l;
    self.a = result.a;
    self.b = result.b;
    self.alpha = result.alpha;
  }

  /// Scales the a component by the given factor.
  pub fn scale_a(&mut self, factor: impl Into<Component>) {
    self.a *= factor.into();
  }

  /// Scales the b component by the given factor.
  pub fn scale_b(&mut self, factor: impl Into<Component>) {
    self.b *= factor.into();
  }

  /// Scales the L component by the given factor.
  pub fn scale_l(&mut self, factor: impl Into<Component>) {
    self.l *= factor.into();
  }

  /// Sets the a component.
  pub fn set_a(&mut self, a: impl Into<Component>) {
    self.a = a.into();
  }

  /// Sets the b component.
  pub fn set_b(&mut self, b: impl Into<Component>) {
    self.b = b.into();
  }

  /// Sets the [L, a, b] components from an array.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_l(components[0].clone());
    self.set_a(components[1].clone());
    self.set_b(components[2].clone());
  }

  /// Sets the L component.
  pub fn set_l(&mut self, l: impl Into<Component>) {
    self.l = l.into();
  }

  /// Converts to the Okhsl perceptual color space.
  #[cfg(feature = "space-okhsl")]
  pub fn to_okhsl(&self) -> Okhsl {
    let [l, a, b] = self.components();
    let h_rad = b.atan2(a);
    let h = (h_rad / (2.0 * std::f64::consts::PI)).rem_euclid(1.0);
    let c = (a * a + b * b).sqrt();
    let okhsl_l = toe(l);
    let s = if c < 1e-4 {
      0.0
    } else {
      let cusp = cusp_for_hue(h);
      let max_c = max_chroma_at_lightness(cusp, l);
      if max_c < 1e-10 { 0.0 } else { (c / max_c).min(1.0) }
    };

    Okhsl::new(h * 360.0, s * 100.0, okhsl_l * 100.0).with_alpha(self.alpha)
  }

  /// Converts to the Okhsv perceptual color space (HSV form).
  ///
  /// Uses an HSV cone model where V=1, S=1 is the cusp (maximum chroma)
  /// and V=1, S=0 is white. The inverse of `Okhsv::to_oklab`.
  #[cfg(feature = "space-okhsv")]
  pub fn to_okhsv(&self) -> Okhsv {
    let [l, a, b] = self.components();
    let h_rad = b.atan2(a);
    let h = (h_rad / (2.0 * std::f64::consts::PI)).rem_euclid(1.0);
    let c = (a * a + b * b).sqrt();

    let cusp = cusp_for_hue(h);
    let (l_cusp, c_cusp) = cusp;

    if c_cusp < 1e-10 || l < 1e-10 {
      let v = toe(l);
      return Okhsv::new(h * 360.0, 0.0, v * 100.0).with_alpha(self.alpha);
    }

    let tv = l + c * (1.0 - l_cusp) / c_cusp;
    let v = toe(tv);
    let s = if tv > 1e-10 { (c / (tv * c_cusp)).min(1.0) } else { 0.0 };

    Okhsv::new(h * 360.0, s * 100.0, v * 100.0).with_alpha(self.alpha)
  }

  /// Converts to the Okhwb perceptual color space (HWB form).
  #[cfg(feature = "space-okhwb")]
  pub fn to_okhwb(&self) -> Okhwb {
    self.to_okhsv().to_okhwb()
  }

  /// Converts to the Oklch perceptual color space (cylindrical form).
  #[cfg(feature = "space-oklch")]
  pub fn to_oklch(&self) -> Oklch {
    let [l, a, b] = self.components();
    let c = (a * a + b * b).sqrt();
    let h = b.atan2(a).to_degrees();

    Oklch::new(l, c, h).with_alpha(self.alpha)
  }

  /// Converts to the specified RGB color space.
  pub fn to_rgb<S>(&self) -> Rgb<S>
  where
    S: RgbSpec,
  {
    let [l, a, b] = self.components();

    let ll = l + (0.3963377774 * a) + (0.2158037573 * b);
    let lm = l - (0.1055613458 * a) - (0.0638541728 * b);
    let ls = l - (0.0894841775 * a) - (1.2914855480 * b);

    let linear_lms = [ll * ll * ll, lm * lm * lm, ls * ls * ls];
    let [r, g, b] = Self::LINEAR_RGB_MATRIX.inverse() * linear_lms;

    LinearRgb::<S>::from_normalized(r, g, b)
      .with_alpha(self.alpha)
      .to_encoded()
  }

  /// Converts to the CIE XYZ color space.
  pub fn to_xyz(&self) -> Xyz {
    let lab = self.components();

    let lms = Self::LINEAR_LMS_MATRIX.inverse() * lab;
    let linear = [
      lms[0] * lms[0] * lms[0],
      lms[1] * lms[1] * lms[1],
      lms[2] * lms[2] * lms[2],
    ];
    let [x, y, z] = Self::LINEAR_XYZ_MATRIX.inverse() * linear;

    Xyz::new(x, y, z).with_context(self.context).with_alpha(self.alpha)
  }

  /// Returns a new color with the given a value.
  pub fn with_a(&self, a: impl Into<Component>) -> Self {
    Self {
      a: a.into(),
      ..*self
    }
  }

  /// Returns a new color with a decreased by the given amount.
  pub fn with_a_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklab = *self;
    oklab.decrement_a(amount);
    oklab
  }

  /// Returns a new color with a increased by the given amount.
  pub fn with_a_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklab = *self;
    oklab.increment_a(amount);
    oklab
  }

  /// Returns a new color with a scaled by the given factor.
  pub fn with_a_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut oklab = *self;
    oklab.scale_a(factor);
    oklab
  }

  /// Returns a new color with the given b value.
  pub fn with_b(&self, b: impl Into<Component>) -> Self {
    Self {
      b: b.into(),
      ..*self
    }
  }

  /// Returns a new color with b decreased by the given amount.
  pub fn with_b_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklab = *self;
    oklab.decrement_b(amount);
    oklab
  }

  /// Returns a new color with b increased by the given amount.
  pub fn with_b_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklab = *self;
    oklab.increment_b(amount);
    oklab
  }

  /// Returns a new color with b scaled by the given factor.
  pub fn with_b_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut oklab = *self;
    oklab.scale_b(factor);
    oklab
  }

  /// Returns this color with a different viewing context (without adaptation).
  pub fn with_context(&self, context: ColorimetricContext) -> Self {
    Self {
      context,
      ..*self
    }
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
    let mut oklab = *self;
    oklab.decrement_l(amount);
    oklab
  }

  /// Returns a new color with L increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut oklab = *self;
    oklab.increment_l(amount);
    oklab
  }

  /// Returns a new color with L scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut oklab = *self;
    oklab.scale_l(factor);
    oklab
  }
}

impl<T> Add<T> for Oklab
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Oklab {
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

  #[cfg(feature = "space-okhsl")]
  fn to_okhsl(&self) -> Okhsl {
    self.to_okhsl()
  }

  #[cfg(feature = "space-okhsv")]
  fn to_okhsv(&self) -> Okhsv {
    self.to_okhsv()
  }

  #[cfg(feature = "space-okhwb")]
  fn to_okhwb(&self) -> Okhwb {
    self.to_okhwb()
  }

  fn to_xyz(&self) -> Xyz {
    self.to_xyz()
  }
}

impl Display for Oklab {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(4);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Oklab({:.precision$}, {:.precision$}, {:.precision$}, {:.0}%)",
        self.l,
        self.a,
        self.b,
        self.opacity()
      )
    } else {
      write!(
        f,
        "Oklab({:.precision$}, {:.precision$}, {:.precision$})",
        self.l, self.a, self.b
      )
    }
  }
}

impl<T> Div<T> for Oklab
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Oklab
where
  T: Into<Component>,
{
  fn from([l, a, b]: [T; 3]) -> Self {
    Self::new(l, a, b)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Oklab
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_oklab()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Oklab
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_oklab()
  }
}

#[cfg(feature = "space-hsi")]
impl<S> From<Hsi<S>> for Oklab
where
  S: RgbSpec,
{
  fn from(hsi: Hsi<S>) -> Self {
    hsi.to_oklab()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Oklab
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_oklab()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Oklab
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_oklab()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Oklab
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_oklab()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Oklab {
  fn from(lab: Lab) -> Self {
    lab.to_oklab()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Oklab {
  fn from(lch: Lch) -> Self {
    lch.to_oklab()
  }
}

#[cfg(feature = "space-lchuv")]
impl From<Lchuv> for Oklab {
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_oklab()
  }
}

impl From<Lms> for Oklab {
  fn from(lms: Lms) -> Self {
    lms.to_oklab()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Oklab {
  fn from(luv: Luv) -> Self {
    luv.to_oklab()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Oklab {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_oklab()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Oklab {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_oklab()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Oklab {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_oklab()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Oklab {
  fn from(oklch: Oklch) -> Self {
    oklch.to_oklab()
  }
}

impl<S> From<Rgb<S>> for Oklab
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_oklab()
  }
}

#[cfg(feature = "space-xyy")]
impl From<Xyy> for Oklab {
  fn from(xyy: Xyy) -> Self {
    xyy.to_oklab()
  }
}

impl From<Xyz> for Oklab {
  fn from(xyz: Xyz) -> Self {
    xyz.to_oklab()
  }
}

impl<T> Mul<T> for Oklab
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Oklab
where
  T: Into<Oklab> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.l == other.l && self.a == other.a && self.b == other.b
  }
}

impl<T> Sub<T> for Oklab
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Oklab {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Oklab {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

/// Finds the cusp (L, C) for a given normalized hue (0.0-1.0).
///
/// The cusp is the point of maximum chroma on the sRGB gamut boundary
/// for the given hue.
pub(crate) fn cusp_for_hue(h: f64) -> (f64, f64) {
  let h_rad = h * 2.0 * std::f64::consts::PI;
  let a = h_rad.cos();
  let b = h_rad.sin();

  let s_cusp = compute_max_saturation(a, b);
  let rgb = oklab_to_linear_srgb(1.0, s_cusp * a, s_cusp * b);
  let l_cusp = (1.0 / rgb[0].max(rgb[1]).max(rgb[2])).cbrt();
  let c_cusp = l_cusp * s_cusp;

  (l_cusp, c_cusp)
}

/// Maximum chroma at a given lightness for a cusp (L_cusp, C_cusp).
///
/// Returns the maximum chroma achievable at the given lightness within
/// the sRGB gamut for the hue defined by the cusp.
pub(crate) fn max_chroma_at_lightness(cusp: (f64, f64), l: f64) -> f64 {
  let (l_cusp, c_cusp) = cusp;

  if l <= l_cusp {
    if l_cusp <= 0.0 { 0.0 } else { c_cusp * l / l_cusp }
  } else if l_cusp >= 1.0 {
    0.0
  } else {
    c_cusp * (1.0 - l) / (1.0 - l_cusp)
  }
}

/// Maps Oklab lightness to Ok* perceived lightness via the toe function.
///
/// Improves perceptual uniformity at the dark end of the lightness range.
pub(crate) fn toe(x: f64) -> f64 {
  0.5 * ((K3 * x) - K1 + ((K3 * x - K1).powi(2) + 4.0 * K2 * K3 * x).sqrt())
}

/// Inverse toe function: maps Ok* perceived lightness to Oklab lightness.
pub(crate) fn toe_inv(x: f64) -> f64 {
  (x * x + K1 * x) / (K3 * (x + K2))
}

/// Computes the maximum saturation for a given hue direction (a_, b_) in Oklab.
///
/// Based on Björn Ottosson's reference implementation. Uses a polynomial
/// approximation refined by one step of Halley's method.
fn compute_max_saturation(a: f64, b: f64) -> f64 {
  let (k0, k1, k2, k3, k4, wl, wm, ws);

  if -1.88170328 * a - 0.80936493 * b > 1.0 {
    k0 = 1.19086277;
    k1 = 1.76576728;
    k2 = 0.59662641;
    k3 = 0.75515197;
    k4 = 0.56771245;
    wl = 4.0767416621;
    wm = -3.3077115913;
    ws = 0.2309699292;
  } else if 1.81444104 * a - 1.19445276 * b > 1.0 {
    k0 = 0.73956515;
    k1 = -0.45954404;
    k2 = 0.08285427;
    k3 = 0.12541070;
    k4 = -0.14503204;
    wl = -1.2684380046;
    wm = 2.6097574011;
    ws = -0.3413193965;
  } else {
    k0 = 1.35733652;
    k1 = -0.00915799;
    k2 = -1.15130210;
    k3 = -0.50559606;
    k4 = 0.00692167;
    wl = -0.0041960863;
    wm = -0.7034186147;
    ws = 1.7076147010;
  }

  let sat = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

  let k_l = 0.3963377774 * a + 0.2158037573 * b;
  let k_m = -0.1055613458 * a - 0.0638541728 * b;
  let k_s = -0.0894841775 * a - 1.2914855480 * b;

  let l_ = 1.0 + sat * k_l;
  let m_ = 1.0 + sat * k_m;
  let s_ = 1.0 + sat * k_s;

  let l = l_ * l_ * l_;
  let m = m_ * m_ * m_;
  let s = s_ * s_ * s_;

  let l_ds = 3.0 * k_l * l_ * l_;
  let m_ds = 3.0 * k_m * m_ * m_;
  let s_ds = 3.0 * k_s * s_ * s_;

  let l_ds2 = 6.0 * k_l * k_l * l_;
  let m_ds2 = 6.0 * k_m * k_m * m_;
  let s_ds2 = 6.0 * k_s * k_s * s_;

  let f = wl * l + wm * m + ws * s;
  let f1 = wl * l_ds + wm * m_ds + ws * s_ds;
  let f2 = wl * l_ds2 + wm * m_ds2 + ws * s_ds2;

  sat - f * f1 / (f1 * f1 - 0.5 * f * f2)
}

/// Converts Oklab L, a, b to linear sRGB components.
fn oklab_to_linear_srgb(l: f64, a: f64, b: f64) -> [f64; 3] {
  let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
  let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
  let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

  let l3 = l_ * l_ * l_;
  let m3 = m_ * m_ * m_;
  let s3 = s_ * s_ * s_;

  [
    4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3,
    -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3,
    -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3,
  ]
}

#[cfg(test)]
mod test {
  use super::*;

  mod a {
    use super::*;

    #[test]
    fn it_returns_a_component() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);

      assert!((oklab.a() - 0.1).abs() < 1e-10);
    }
  }

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_oklab_colors() {
      let a = Oklab::new(0.5, 0.1, -0.1);
      let b = Oklab::new(0.3, -0.05, 0.05);
      let result = a + b;

      assert!(result.l() > 0.0);
    }
  }

  mod b {
    use super::*;

    #[test]
    fn it_returns_b_component() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);

      assert!((oklab.b() - -0.1).abs() < 1e-10);
    }
  }

  mod components {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_components_as_array() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let [l, a, b] = oklab.components();

      assert_eq!(l, 0.5);
      assert_eq!(a, 0.1);
      assert_eq!(b, -0.1);
    }
  }

  mod decrement_a {
    use super::*;

    #[test]
    fn it_decreases_a_component() {
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.decrement_a(0.05);

      assert!((oklab.a() - 0.05).abs() < 1e-10);
    }
  }

  mod decrement_b {
    use super::*;

    #[test]
    fn it_decreases_b_component() {
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.decrement_b(0.05);

      assert!((oklab.b() - -0.15).abs() < 1e-10);
    }
  }

  mod decrement_l {
    use super::*;

    #[test]
    fn it_decreases_l_component() {
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.decrement_l(0.1);

      assert!((oklab.l() - 0.4).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);

      assert_eq!(format!("{}", oklab), "Oklab(0.5000, 0.1000, -0.1000)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);

      assert_eq!(format!("{:.2}", oklab), "Oklab(0.50, 0.10, -0.10)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.5);

      assert_eq!(format!("{}", oklab), "Oklab(0.5000, 0.1000, -0.1000, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);

      assert_eq!(format!("{}", oklab), "Oklab(0.5000, 0.1000, -0.1000)");
    }
  }

  mod from_array {
    use super::*;

    #[test]
    fn it_creates_from_f64_array() {
      let oklab = Oklab::from([0.5, 0.1, -0.1]);

      assert!((oklab.l() - 0.5).abs() < 1e-10);
      assert!((oklab.a() - 0.1).abs() < 1e-10);
      assert!((oklab.b() - -0.1).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let oklab = Oklab::from(rgb);

      assert!((oklab.l() - 1.0).abs() < 1e-3);
      assert!(oklab.a().abs() < 1e-3);
      assert!(oklab.b().abs() < 1e-3);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let oklab = Oklab::from(rgb);

      assert!(oklab.l().abs() < 1e-3);
      assert!(oklab.a().abs() < 1e-3);
      assert!(oklab.b().abs() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let oklab = Oklab::from(rgb);

      assert!((oklab.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let oklab = Oklab::from(xyz);

      assert!(oklab.l() > 0.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let oklab = Oklab::from(xyz);

      assert!((oklab.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod gradient {
    use super::*;

    #[test]
    fn zero_steps_is_empty() {
      let c1 = Oklab::new(0.5, 0.1, -0.1);
      let c2 = Oklab::new(0.8, -0.05, 0.1);
      assert!(c1.gradient(c2.to_xyz(), 0).is_empty());
    }

    #[test]
    fn one_step_returns_self() {
      let c1 = Oklab::new(0.5, 0.1, -0.1);
      let c2 = Oklab::new(0.8, -0.05, 0.1);
      let steps = c1.gradient(c2.to_xyz(), 1);
      assert_eq!(steps.len(), 1);
      assert!((steps[0].l() - c1.l()).abs() < 1e-4);
    }

    #[test]
    fn two_steps_returns_endpoints() {
      let c1 = Oklab::new(0.5, 0.1, -0.1);
      let c2 = Oklab::new(0.8, -0.05, 0.1);
      let steps = c1.gradient(c2.to_xyz(), 2);
      assert_eq!(steps.len(), 2);
      assert!((steps[0].l() - c1.l()).abs() < 1e-4);
      assert!((steps[1].l() - c2.l()).abs() < 1e-4);
    }

    #[test]
    fn five_steps_correct_count() {
      let c1 = Oklab::new(0.2, 0.0, 0.0);
      let c2 = Oklab::new(0.9, 0.0, 0.0);
      assert_eq!(c1.gradient(c2.to_xyz(), 5).len(), 5);
    }

    #[test]
    fn monotonic_lightness_dark_to_light() {
      let dark = Oklab::new(0.1, 0.0, 0.0);
      let light = Oklab::new(0.9, 0.0, 0.0);
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
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.increment_a(0.05);

      assert!((oklab.a() - 0.15).abs() < 1e-10);
    }
  }

  mod increment_b {
    use super::*;

    #[test]
    fn it_increases_b_component() {
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.increment_b(0.05);

      assert!((oklab.b() - -0.05).abs() < 1e-10);
    }
  }

  mod increment_l {
    use super::*;

    #[test]
    fn it_increases_l_component() {
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.increment_l(0.1);

      assert!((oklab.l() - 0.6).abs() < 1e-10);
    }
  }

  mod l {
    use super::*;

    #[test]
    fn it_returns_l_component() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);

      assert!((oklab.l() - 0.5).abs() < 1e-10);
    }
  }

  mod mix {
    use super::*;

    const EPSILON: f64 = 1e-4;

    #[test]
    fn at_zero_returns_self() {
      let c1 = Oklab::new(0.5, 0.1, -0.1);
      let c2 = Oklab::new(0.8, -0.05, 0.1);
      let result = c1.mix(c2.to_xyz(), 0.0);
      assert!((result.l() - c1.l()).abs() < EPSILON);
      assert!((result.a() - c1.a()).abs() < EPSILON);
      assert!((result.b() - c1.b()).abs() < EPSILON);
    }

    #[test]
    fn at_one_returns_other() {
      let c1 = Oklab::new(0.5, 0.1, -0.1);
      let c2 = Oklab::new(0.8, -0.05, 0.1);
      let result = c1.mix(c2.to_xyz(), 1.0);
      assert!((result.l() - c2.l()).abs() < EPSILON);
      assert!((result.a() - c2.a()).abs() < EPSILON);
      assert!((result.b() - c2.b()).abs() < EPSILON);
    }

    #[test]
    fn midpoint_is_between() {
      let c1 = Oklab::new(0.2, 0.0, 0.0);
      let c2 = Oklab::new(0.8, 0.0, 0.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!(mid.l() > 0.3 && mid.l() < 0.7);
    }

    #[test]
    fn alpha_interpolation() {
      let c1 = Oklab::new(0.5, 0.0, 0.0).with_alpha(0.0);
      let c2 = Oklab::new(0.5, 0.0, 0.0).with_alpha(1.0);
      let mid = c1.mix(c2.to_xyz(), 0.5);
      assert!((mid.alpha() - 0.5).abs() < EPSILON);
    }

    #[test]
    fn extrapolation() {
      let c1 = Oklab::new(0.2, 0.0, 0.0);
      let c2 = Oklab::new(0.8, 0.0, 0.0);
      let beyond = c1.mix(c2.to_xyz(), 1.5);
      assert!(beyond.l() > c2.l());
    }

    #[test]
    fn cross_type() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let xyz = Xyz::new(0.18048, 0.07219, 0.95030);
      let _result = oklab.mix(xyz, 0.5);
    }
  }

  mod mixed_with {
    use super::*;

    #[test]
    fn it_mutates_in_place() {
      let c1 = Oklab::new(0.5, 0.1, -0.1);
      let c2 = Oklab::new(0.8, -0.05, 0.1);
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
      let oklab = Oklab::new(0.5, 0.1, -0.1);

      assert!((oklab.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);

      assert_eq!(oklab.context().illuminant().name(), "D65");
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Oklab::new(0.5, 0.1, -0.1);
      let b = Oklab::new(0.5, 0.1, -0.1);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Oklab::new(0.5, 0.1, -0.1);
      let b = Oklab::new(0.6, 0.1, -0.1);

      assert!(a != b);
    }
  }

  mod scale_a {
    use super::*;

    #[test]
    fn it_scales_a_component() {
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.scale_a(2.0);

      assert!((oklab.a() - 0.2).abs() < 1e-10);
    }
  }

  mod scale_b {
    use super::*;

    #[test]
    fn it_scales_b_component() {
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.scale_b(2.0);

      assert!((oklab.b() - -0.2).abs() < 1e-10);
    }
  }

  mod scale_l {
    use super::*;

    #[test]
    fn it_scales_l_component() {
      let mut oklab = Oklab::new(0.5, 0.1, -0.1);
      oklab.scale_l(2.0);

      assert!((oklab.l() - 1.0).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-okhsl")]
  mod to_okhsl {
    use super::*;

    #[test]
    fn it_converts_achromatic_to_zero_saturation() {
      let oklab = Oklab::new(0.5, 0.0, 0.0);
      let okhsl = oklab.to_okhsl();

      assert!(okhsl.s() < 1e-3);
    }

    #[test]
    fn it_converts_black() {
      let oklab = Oklab::new(0.0, 0.0, 0.0);
      let okhsl = oklab.to_okhsl();

      assert!(okhsl.l().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let oklab = Oklab::new(1.0, 0.0, 0.0);
      let okhsl = oklab.to_okhsl();

      assert!((okhsl.l() - 1.0).abs() < 1e-10);
      assert!(okhsl.s() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.3);
      let okhsl = oklab.to_okhsl();

      assert!((okhsl.alpha() - 0.3).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-okhsv")]
  mod to_okhsv {
    use super::*;

    #[test]
    fn it_converts_achromatic_to_zero_saturation() {
      let oklab = Oklab::new(0.5, 0.0, 0.0);
      let okhsv = oklab.to_okhsv();

      assert!(okhsv.s() < 1e-3);
    }

    #[test]
    fn it_converts_black() {
      let oklab = Oklab::new(0.0, 0.0, 0.0);
      let okhsv = oklab.to_okhsv();

      assert!(okhsv.v().abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let oklab = Oklab::new(1.0, 0.0, 0.0);
      let okhsv = oklab.to_okhsv();

      assert!((okhsv.v() - 1.0).abs() < 1e-10);
      assert!(okhsv.s() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.3);
      let okhsv = oklab.to_okhsv();

      assert!((okhsv.alpha() - 0.3).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-okhwb")]
  mod to_okhwb {
    use super::*;

    #[test]
    fn it_converts_to_okhwb() {
      let oklab = Oklab::new(0.5, 0.0, 0.0);
      let okhwb = oklab.to_okhwb();

      assert!(okhwb.h().is_finite());
      assert!(okhwb.w().is_finite());
      assert!(okhwb.b().is_finite());
    }

    #[test]
    fn it_converts_black() {
      let oklab = Oklab::new(0.0, 0.0, 0.0);
      let okhwb = oklab.to_okhwb();

      assert!((okhwb.blackness() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let oklab = Oklab::new(1.0, 0.0, 0.0);
      let okhwb = oklab.to_okhwb();

      assert!((okhwb.whiteness() - 100.0).abs() < 1e-3);
      assert!(okhwb.blackness().abs() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.5);
      let okhwb = oklab.to_okhwb();

      assert!((okhwb.alpha() - 0.5).abs() < 1e-10);
    }
  }

  #[cfg(feature = "space-oklch")]
  mod to_oklch {
    use super::*;

    #[test]
    fn it_converts_to_oklch() {
      let oklab = Oklab::new(0.5, 0.0, 0.15);
      let oklch = oklab.to_oklch();

      assert!((oklch.l() - 0.5).abs() < 1e-10);
      assert!((oklch.c() - 0.15).abs() < 1e-10);
      assert!((oklch.hue() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_through_oklch() {
      let original = Oklab::new(0.5, 0.1, -0.1);
      let roundtrip = Oklch::from(original).to_oklab();

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.a() - roundtrip.a()).abs() < 1e-10);
      assert!((original.b() - roundtrip.b()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1).with_alpha(0.7);
      let oklch = oklab.to_oklch();

      assert!((oklch.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let oklab = Oklab::new(0.5, 0.0, 0.0);
      let rgb = oklab.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_roundtrips_through_rgb() {
      let original = Oklab::from(Rgb::<Srgb>::new(128, 64, 200));
      let roundtrip = Oklab::from(original.to_rgb::<Srgb>());

      assert!((original.l() - roundtrip.l()).abs() < 1e-3);
      assert!((original.a() - roundtrip.a()).abs() < 1e-3);
      assert!((original.b() - roundtrip.b()).abs() < 1e-3);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.0, 0.0).with_alpha(0.7);
      let rgb = oklab.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let oklab = Oklab::new(0.5, 0.0, 0.0);
      let xyz = oklab.to_xyz();

      assert!(xyz.y() > 0.0);
    }

    #[test]
    fn it_roundtrips_through_xyz() {
      let original = Oklab::new(0.5, 0.1, -0.1);
      let roundtrip = Oklab::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.a() - roundtrip.a()).abs() < 1e-10);
      assert!((original.b() - roundtrip.b()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let oklab = Oklab::new(0.5, 0.0, 0.0).with_alpha(0.3);
      let xyz = oklab.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let oklab = Oklab::try_from("#FF5733").unwrap();

      assert!(oklab.l() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Oklab::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod with_a {
    use super::*;

    #[test]
    fn it_returns_new_color_with_a() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_a(0.2);

      assert!((result.a() - 0.2).abs() < 1e-10);
      assert!((result.l() - 0.5).abs() < 1e-10);
      assert!((result.b() - -0.1).abs() < 1e-10);
    }
  }

  mod with_a_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_a_decremented() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_a_decremented_by(0.05);

      assert!((result.a() - 0.05).abs() < 1e-10);
    }
  }

  mod with_a_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_a_incremented() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_a_incremented_by(0.05);

      assert!((result.a() - 0.15).abs() < 1e-10);
    }
  }

  mod with_a_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_a_scaled() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_a_scaled_by(2.0);

      assert!((result.a() - 0.2).abs() < 1e-10);
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.l() - 0.5).abs() < 1e-10);
    }
  }

  mod with_b {
    use super::*;

    #[test]
    fn it_returns_new_color_with_b() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_b(0.2);

      assert!((result.b() - 0.2).abs() < 1e-10);
      assert!((result.l() - 0.5).abs() < 1e-10);
      assert!((result.a() - 0.1).abs() < 1e-10);
    }
  }

  mod with_b_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_b_decremented() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_b_decremented_by(0.05);

      assert!((result.b() - -0.15).abs() < 1e-10);
    }
  }

  mod with_b_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_b_incremented() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_b_incremented_by(0.05);

      assert!((result.b() - -0.05).abs() < 1e-10);
    }
  }

  mod with_b_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_b_scaled() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_b_scaled_by(2.0);

      assert!((result.b() - -0.2).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let context = ColorimetricContext::default();
      let result = oklab.with_context(context);

      assert!((result.l() - 0.5).abs() < 1e-10);
    }
  }

  mod with_l {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_l(0.8);

      assert!((result.l() - 0.8).abs() < 1e-10);
      assert!((result.a() - 0.1).abs() < 1e-10);
      assert!((result.b() - -0.1).abs() < 1e-10);
    }
  }

  mod with_l_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_decremented() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_l_decremented_by(0.1);

      assert!((result.l() - 0.4).abs() < 1e-10);
    }
  }

  mod with_l_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_incremented() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_l_incremented_by(0.1);

      assert!((result.l() - 0.6).abs() < 1e-10);
    }
  }

  mod with_l_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_scaled() {
      let oklab = Oklab::new(0.5, 0.1, -0.1);
      let result = oklab.with_l_scaled_by(2.0);

      assert!((result.l() - 1.0).abs() < 1e-10);
    }
  }
}
