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

/// CIE 1976 L\*u\*v\* threshold: (6/29)³.
pub(crate) const EPSILON: f64 = (6.0 / 29.0) * (6.0 / 29.0) * (6.0 / 29.0);

/// CIE 1976 L\*u\*v\* scaling factor: (29/3)³.
pub(crate) const KAPPA: f64 = (29.0 / 3.0) * (29.0 / 3.0) * (29.0 / 3.0);

/// CIE 1976 L\*u\*v\* (CIELUV) color space.
///
/// A perceptually-uniform color space standardized by the CIE in 1976. L\* represents
/// lightness (0–100), u\* and v\* encode chromaticity relative to a reference white point
/// using the CIE 1976 UCS (u', v') diagram. Values are computed relative to a reference
/// white point (default: D65 / CIE 1931 2°).
#[derive(Clone, Copy, Debug)]
pub struct Luv {
  alpha: Component,
  context: ColorimetricContext,
  l: Component,
  u: Component,
  v: Component,
}

impl Luv {
  /// The default viewing context for Luv (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new Luv color with the default viewing context.
  pub fn new(l: impl Into<Component>, u: impl Into<Component>, v: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: Self::DEFAULT_CONTEXT,
      l: l.into(),
      u: u.into(),
      v: v.into(),
    }
  }

  /// Creates a new Luv color in a const context.
  pub const fn new_const(l: f64, u: f64, v: f64) -> Self {
    Self {
      alpha: Component::new_const(1.0),
      context: Self::DEFAULT_CONTEXT,
      l: Component::new_const(l),
      u: Component::new_const(u),
      v: Component::new_const(v),
    }
  }

  /// Adapts this color to a different viewing context via XYZ.
  pub fn adapt_to(&self, context: ColorimetricContext) -> Self {
    let reference_white = self.context.reference_white();
    let target_white = context.reference_white();

    if reference_white == target_white {
      return self.with_context(context);
    }

    self.to_xyz().adapt_to(context).to_luv()
  }

  /// Returns the [L\*, u\*, v\*] components as an array.
  pub fn components(&self) -> [f64; 3] {
    [self.l.0, self.u.0, self.v.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases the L\* component by the given amount.
  pub fn decrement_l(&mut self, amount: impl Into<Component>) {
    self.l -= amount.into();
  }

  /// Decreases the u\* component by the given amount.
  pub fn decrement_u(&mut self, amount: impl Into<Component>) {
    self.u -= amount.into();
  }

  /// Decreases the v\* component by the given amount.
  pub fn decrement_v(&mut self, amount: impl Into<Component>) {
    self.v -= amount.into();
  }

  /// Increases the L\* component by the given amount.
  pub fn increment_l(&mut self, amount: impl Into<Component>) {
    self.l += amount.into();
  }

  /// Increases the u\* component by the given amount.
  pub fn increment_u(&mut self, amount: impl Into<Component>) {
    self.u += amount.into();
  }

  /// Increases the v\* component by the given amount.
  pub fn increment_v(&mut self, amount: impl Into<Component>) {
    self.v += amount.into();
  }

  /// Returns the L\* (lightness) component.
  pub fn l(&self) -> f64 {
    self.l.0
  }

  /// Scales the L\* component by the given factor.
  pub fn scale_l(&mut self, factor: impl Into<Component>) {
    self.l *= factor.into();
  }

  /// Scales the u\* component by the given factor.
  pub fn scale_u(&mut self, factor: impl Into<Component>) {
    self.u *= factor.into();
  }

  /// Scales the v\* component by the given factor.
  pub fn scale_v(&mut self, factor: impl Into<Component>) {
    self.v *= factor.into();
  }

  /// Sets the [L\*, u\*, v\*] components from an array.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_l(components[0].clone());
    self.set_u(components[1].clone());
    self.set_v(components[2].clone());
  }

  /// Sets the L\* component.
  pub fn set_l(&mut self, l: impl Into<Component>) {
    self.l = l.into();
  }

  /// Sets the u\* component.
  pub fn set_u(&mut self, u: impl Into<Component>) {
    self.u = u.into();
  }

  /// Sets the v\* component.
  pub fn set_v(&mut self, v: impl Into<Component>) {
    self.v = v.into();
  }

  /// Converts to the CIE LCh(uv) color space (cylindrical form).
  #[cfg(feature = "space-lchuv")]
  pub fn to_lchuv(&self) -> crate::space::Lchuv {
    let [l, u, v] = self.components();
    let c = (u * u + v * v).sqrt();
    let h = v.atan2(u).to_degrees();

    crate::space::Lchuv::new(l, c, h)
      .with_context(self.context)
      .with_alpha(self.alpha)
  }

  /// Converts to the CIE XYZ color space.
  pub fn to_xyz(&self) -> Xyz {
    let [l, u_star, v_star] = self.components();
    let [xn, yn, zn] = self.context.reference_white().components();

    let u_prime_n = luv_u_prime(xn, yn, zn);
    let v_prime_n = luv_v_prime(xn, yn, zn);

    if l == 0.0 {
      return Xyz::new(0.0, 0.0, 0.0)
        .with_context(self.context)
        .with_alpha(self.alpha);
    }

    let u_prime = u_star / (13.0 * l) + u_prime_n;
    let v_prime = v_star / (13.0 * l) + v_prime_n;

    let y = if l > 8.0 {
      yn * ((l + 16.0) / 116.0).powi(3)
    } else {
      yn * l / KAPPA
    };

    let x = y * 9.0 * u_prime / (4.0 * v_prime);
    let z = y * (12.0 - 3.0 * u_prime - 20.0 * v_prime) / (4.0 * v_prime);

    Xyz::new(x, y, z).with_context(self.context).with_alpha(self.alpha)
  }

  /// Returns the u\* component.
  pub fn u(&self) -> f64 {
    self.u.0
  }

  /// Returns the v\* component.
  pub fn v(&self) -> f64 {
    self.v.0
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
    let mut luv = *self;
    luv.decrement_l(amount);
    luv
  }

  /// Returns a new color with L\* increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut luv = *self;
    luv.increment_l(amount);
    luv
  }

  /// Returns a new color with L\* scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut luv = *self;
    luv.scale_l(factor);
    luv
  }

  /// Returns a new color with the given u\* value.
  pub fn with_u(&self, u: impl Into<Component>) -> Self {
    Self {
      u: u.into(),
      ..*self
    }
  }

  /// Returns a new color with u\* decreased by the given amount.
  pub fn with_u_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut luv = *self;
    luv.decrement_u(amount);
    luv
  }

  /// Returns a new color with u\* increased by the given amount.
  pub fn with_u_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut luv = *self;
    luv.increment_u(amount);
    luv
  }

  /// Returns a new color with u\* scaled by the given factor.
  pub fn with_u_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut luv = *self;
    luv.scale_u(factor);
    luv
  }

  /// Returns a new color with the given v\* value.
  pub fn with_v(&self, v: impl Into<Component>) -> Self {
    Self {
      v: v.into(),
      ..*self
    }
  }

  /// Returns a new color with v\* decreased by the given amount.
  pub fn with_v_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut luv = *self;
    luv.decrement_v(amount);
    luv
  }

  /// Returns a new color with v\* increased by the given amount.
  pub fn with_v_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut luv = *self;
    luv.increment_v(amount);
    luv
  }

  /// Returns a new color with v\* scaled by the given factor.
  pub fn with_v_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut luv = *self;
    luv.scale_v(factor);
    luv
  }
}

impl<T> Add<T> for Luv
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Luv {
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

impl Display for Luv {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(4);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "Luv({:.precision$}, {:.precision$}, {:.precision$}, {:.0}%)",
        self.l,
        self.u,
        self.v,
        self.opacity()
      )
    } else {
      write!(
        f,
        "Luv({:.precision$}, {:.precision$}, {:.precision$})",
        self.l, self.u, self.v
      )
    }
  }
}

impl<T> Div<T> for Luv
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Luv
where
  T: Into<Component>,
{
  fn from([l, u, v]: [T; 3]) -> Self {
    Self::new(l, u, v)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Luv
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_luv()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Luv
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_luv()
  }
}

#[cfg(feature = "space-hsi")]
impl<S> From<Hsi<S>> for Luv
where
  S: RgbSpec,
{
  fn from(hsi: Hsi<S>) -> Self {
    hsi.to_luv()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Luv
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_luv()
  }
}

#[cfg(feature = "space-hsluv")]
impl From<Hsluv> for Luv {
  fn from(hsluv: Hsluv) -> Self {
    hsluv.to_luv()
  }
}

#[cfg(feature = "space-hpluv")]
impl From<Hpluv> for Luv {
  fn from(hpluv: Hpluv) -> Self {
    hpluv.to_luv()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Luv
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_luv()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Luv
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_luv()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Luv {
  fn from(lab: Lab) -> Self {
    lab.to_luv()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Luv {
  fn from(lch: Lch) -> Self {
    lch.to_luv()
  }
}

#[cfg(feature = "space-lchuv")]
impl From<Lchuv> for Luv {
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_luv()
  }
}

impl From<Lms> for Luv {
  fn from(lms: Lms) -> Self {
    lms.to_luv()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Luv {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_luv()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Luv {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_luv()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Luv {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_luv()
  }
}

#[cfg(feature = "space-oklab")]
impl From<Oklab> for Luv {
  fn from(oklab: Oklab) -> Self {
    oklab.to_luv()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Luv {
  fn from(oklch: Oklch) -> Self {
    oklch.to_luv()
  }
}

impl<S> From<Rgb<S>> for Luv
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_luv()
  }
}

#[cfg(feature = "space-xyy")]
impl From<Xyy> for Luv {
  fn from(xyy: Xyy) -> Self {
    xyy.to_luv()
  }
}

impl From<Xyz> for Luv {
  fn from(xyz: Xyz) -> Self {
    xyz.to_luv()
  }
}

impl<T> Mul<T> for Luv
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Luv
where
  T: Into<Luv> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.l == other.l && self.u == other.u && self.v == other.v
  }
}

impl<T> Sub<T> for Luv
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Luv {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Luv {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

/// Computes the CIE 1976 u' chromaticity coordinate from XYZ.
pub(crate) fn luv_u_prime(x: f64, y: f64, z: f64) -> f64 {
  let denom = x + 15.0 * y + 3.0 * z;
  if denom == 0.0 { 0.0 } else { 4.0 * x / denom }
}

/// Computes the CIE 1976 v' chromaticity coordinate from XYZ.
pub(crate) fn luv_v_prime(x: f64, y: f64, z: f64) -> f64 {
  let denom = x + 15.0 * y + 3.0 * z;
  if denom == 0.0 { 0.0 } else { 9.0 * y / denom }
}

#[cfg(test)]
mod test {
  use super::*;

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
      let luv = Luv::new(50.0, 20.0, -30.0);
      let same_context = Luv::DEFAULT_CONTEXT;
      let adapted = luv.adapt_to(same_context);

      assert!((adapted.l() - luv.l()).abs() < 1e-10);
      assert!((adapted.u() - luv.u()).abs() < 1e-10);
      assert!((adapted.v() - luv.v()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_values_for_non_d65_source() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let context_a = ColorimetricContext::new().with_illuminant(illuminant_a);
      let luv = Luv::new(50.0, 20.0, -30.0).with_context(context_a);
      let adapted = luv.adapt_to(Luv::DEFAULT_CONTEXT);

      assert!((adapted.l() - luv.l()).abs() > 0.01 || (adapted.u() - luv.u()).abs() > 0.01);
    }

    #[test]
    fn it_preserves_alpha() {
      let illuminant_b = Illuminant::new("Test B", IlluminantType::Custom, Spd::new(TEST_SPD_B));
      let luv = Luv::new(50.0, 20.0, -30.0).with_alpha(0.5);
      let target_context = ColorimetricContext::new().with_illuminant(illuminant_b);
      let adapted = luv.adapt_to(target_context);

      assert!((adapted.alpha() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_updates_context_when_white_points_match() {
      let context = Luv::DEFAULT_CONTEXT;
      let luv = Luv::new(50.0, 20.0, -30.0);
      let adapted = luv.adapt_to(context);

      assert_eq!(adapted.context().illuminant().name(), "D65");
    }
  }

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_luv_colors() {
      let a = Luv::new(50.0, 20.0, -30.0);
      let b = Luv::new(30.0, -10.0, 15.0);
      let result = a + b;

      assert!(result.l() > 0.0);
    }
  }

  mod components {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_components_as_array() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let [l, u, v] = luv.components();

      assert_eq!(l, 50.0);
      assert_eq!(u, 20.0);
      assert_eq!(v, -30.0);
    }
  }

  mod decrement_l {
    use super::*;

    #[test]
    fn it_decreases_l_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.decrement_l(10.0);

      assert!((luv.l() - 40.0).abs() < 1e-10);
    }
  }

  mod decrement_u {
    use super::*;

    #[test]
    fn it_decreases_u_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.decrement_u(5.0);

      assert!((luv.u() - 15.0).abs() < 1e-10);
    }
  }

  mod decrement_v {
    use super::*;

    #[test]
    fn it_decreases_v_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.decrement_v(5.0);

      assert!((luv.v() - -35.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let luv = Luv::new(50.0, 20.0, -30.0);

      assert_eq!(format!("{}", luv), "Luv(50.0000, 20.0000, -30.0000)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let luv = Luv::new(50.0, 20.0, -30.0);

      assert_eq!(format!("{:.2}", luv), "Luv(50.00, 20.00, -30.00)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let luv = Luv::new(50.0, 20.0, -30.0).with_alpha(0.5);

      assert_eq!(format!("{}", luv), "Luv(50.0000, 20.0000, -30.0000, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let luv = Luv::new(50.0, 20.0, -30.0);

      assert_eq!(format!("{}", luv), "Luv(50.0000, 20.0000, -30.0000)");
    }
  }

  mod from_array {
    use super::*;

    #[test]
    fn it_creates_from_f64_array() {
      let luv = Luv::from([50.0, 20.0, -30.0]);

      assert!((luv.l() - 50.0).abs() < 1e-10);
      assert!((luv.u() - 20.0).abs() < 1e-10);
      assert!((luv.v() - -30.0).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let luv = Luv::from(rgb);

      assert!((luv.l() - 100.0).abs() < 0.01);
      assert!(luv.u().abs() < 0.01);
      assert!(luv.v().abs() < 0.01);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let luv = Luv::from(rgb);

      assert!(luv.l().abs() < 1e-10);
      assert!(luv.u().abs() < 1e-10);
      assert!(luv.v().abs() < 1e-10);
    }

    #[test]
    fn it_converts_red_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 0, 0);
      let luv = Luv::from(rgb);

      assert!((luv.l() - 53.2408).abs() < 0.01);
      assert!((luv.u() - 175.015).abs() < 0.1);
      assert!((luv.v() - 37.756).abs() < 0.1);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let luv = Luv::from(rgb);

      assert!((luv.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let luv = Luv::from(xyz);

      assert!(luv.l() > 0.0);
    }

    #[test]
    fn it_converts_d65_white_to_l100() {
      let xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let luv = Luv::from(xyz);

      assert!((luv.l() - 100.0).abs() < 0.01);
      assert!(luv.u().abs() < 0.01);
      assert!(luv.v().abs() < 0.01);
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let luv = Luv::from(xyz);

      assert!((luv.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod increment_l {
    use super::*;

    #[test]
    fn it_increases_l_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.increment_l(10.0);

      assert!((luv.l() - 60.0).abs() < 1e-10);
    }
  }

  mod increment_u {
    use super::*;

    #[test]
    fn it_increases_u_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.increment_u(5.0);

      assert!((luv.u() - 25.0).abs() < 1e-10);
    }
  }

  mod increment_v {
    use super::*;

    #[test]
    fn it_increases_v_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.increment_v(5.0);

      assert!((luv.v() - -25.0).abs() < 1e-10);
    }
  }

  mod l {
    use super::*;

    #[test]
    fn it_returns_l_component() {
      let luv = Luv::new(50.0, 20.0, -30.0);

      assert!((luv.l() - 50.0).abs() < 1e-10);
    }
  }

  mod new {
    use super::*;

    #[test]
    fn it_creates_with_default_alpha() {
      let luv = Luv::new(50.0, 20.0, -30.0);

      assert!((luv.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let luv = Luv::new(50.0, 20.0, -30.0);

      assert_eq!(luv.context().illuminant().name(), "D65");
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Luv::new(50.0, 20.0, -30.0);
      let b = Luv::new(50.0, 20.0, -30.0);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Luv::new(50.0, 20.0, -30.0);
      let b = Luv::new(60.0, 20.0, -30.0);

      assert!(a != b);
    }
  }

  mod roundtrip {
    use super::*;

    #[test]
    fn it_roundtrips_through_xyz() {
      let original = Luv::new(50.0, 20.0, -30.0);
      let roundtrip = Luv::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.u() - roundtrip.u()).abs() < 1e-10);
      assert!((original.v() - roundtrip.v()).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_white() {
      let original = Luv::new(100.0, 0.0, 0.0);
      let roundtrip = Luv::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.u() - roundtrip.u()).abs() < 1e-10);
      assert!((original.v() - roundtrip.v()).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_black() {
      let original = Luv::new(0.0, 0.0, 0.0);
      let roundtrip = Luv::from(original.to_xyz());

      assert!((original.l() - roundtrip.l()).abs() < 1e-10);
      assert!((original.u() - roundtrip.u()).abs() < 1e-10);
      assert!((original.v() - roundtrip.v()).abs() < 1e-10);
    }
  }

  mod scale_l {
    use super::*;

    #[test]
    fn it_scales_l_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.scale_l(2.0);

      assert!((luv.l() - 100.0).abs() < 1e-10);
    }
  }

  mod scale_u {
    use super::*;

    #[test]
    fn it_scales_u_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.scale_u(2.0);

      assert!((luv.u() - 40.0).abs() < 1e-10);
    }
  }

  mod scale_v {
    use super::*;

    #[test]
    fn it_scales_v_component() {
      let mut luv = Luv::new(50.0, 20.0, -30.0);
      luv.scale_v(2.0);

      assert!((luv.v() - -60.0).abs() < 1e-10);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let luv = Luv::new(50.0, 0.0, 0.0);
      let rgb = luv.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_roundtrips_through_rgb() {
      let original = Luv::from(Rgb::<Srgb>::new(128, 64, 200));
      let roundtrip = Luv::from(original.to_rgb::<Srgb>());

      assert!((original.l() - roundtrip.l()).abs() < 0.5);
      assert!((original.u() - roundtrip.u()).abs() < 0.5);
      assert!((original.v() - roundtrip.v()).abs() < 0.5);
    }

    #[test]
    fn it_preserves_alpha() {
      let luv = Luv::new(50.0, 0.0, 0.0).with_alpha(0.7);
      let rgb = luv.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let luv = Luv::new(50.0, 0.0, 0.0);
      let xyz = luv.to_xyz();

      assert!(xyz.y() > 0.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let luv = Luv::new(50.0, 0.0, 0.0).with_alpha(0.3);
      let xyz = luv.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let luv = Luv::try_from("#FF5733").unwrap();

      assert!(luv.l() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Luv::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod u {
    use super::*;

    #[test]
    fn it_returns_u_component() {
      let luv = Luv::new(50.0, 20.0, -30.0);

      assert!((luv.u() - 20.0).abs() < 1e-10);
    }
  }

  mod v {
    use super::*;

    #[test]
    fn it_returns_v_component() {
      let luv = Luv::new(50.0, 20.0, -30.0);

      assert!((luv.v() - -30.0).abs() < 1e-10);
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let context = ColorimetricContext::default();
      let result = luv.with_context(context);

      assert!((result.l() - 50.0).abs() < 1e-10);
    }
  }

  mod with_l {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_l(80.0);

      assert!((result.l() - 80.0).abs() < 1e-10);
      assert!((result.u() - 20.0).abs() < 1e-10);
      assert!((result.v() - -30.0).abs() < 1e-10);
    }
  }

  mod with_l_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_decremented() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_l_decremented_by(10.0);

      assert!((result.l() - 40.0).abs() < 1e-10);
    }
  }

  mod with_l_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_incremented() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_l_incremented_by(10.0);

      assert!((result.l() - 60.0).abs() < 1e-10);
    }
  }

  mod with_l_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_l_scaled() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_l_scaled_by(2.0);

      assert!((result.l() - 100.0).abs() < 1e-10);
    }
  }

  mod with_u {
    use super::*;

    #[test]
    fn it_returns_new_color_with_u() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_u(40.0);

      assert!((result.u() - 40.0).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
      assert!((result.v() - -30.0).abs() < 1e-10);
    }
  }

  mod with_u_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_u_decremented() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_u_decremented_by(5.0);

      assert!((result.u() - 15.0).abs() < 1e-10);
    }
  }

  mod with_u_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_u_incremented() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_u_incremented_by(5.0);

      assert!((result.u() - 25.0).abs() < 1e-10);
    }
  }

  mod with_u_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_u_scaled() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_u_scaled_by(2.0);

      assert!((result.u() - 40.0).abs() < 1e-10);
    }
  }

  mod with_v {
    use super::*;

    #[test]
    fn it_returns_new_color_with_v() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_v(40.0);

      assert!((result.v() - 40.0).abs() < 1e-10);
      assert!((result.l() - 50.0).abs() < 1e-10);
      assert!((result.u() - 20.0).abs() < 1e-10);
    }
  }

  mod with_v_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_v_decremented() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_v_decremented_by(5.0);

      assert!((result.v() - -35.0).abs() < 1e-10);
    }
  }

  mod with_v_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_v_incremented() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_v_incremented_by(5.0);

      assert!((result.v() - -25.0).abs() < 1e-10);
    }
  }

  mod with_v_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_v_scaled() {
      let luv = Luv::new(50.0, 20.0, -30.0);
      let result = luv.with_v_scaled_by(2.0);

      assert!((result.v() - -60.0).abs() < 1e-10);
    }
  }
}
