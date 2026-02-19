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
#[cfg(feature = "space-oklab")]
use crate::space::Oklab;
#[cfg(feature = "space-oklch")]
use crate::space::Oklch;
use crate::{
  ColorimetricContext, Illuminant, Observer,
  component::Component,
  space::{ColorSpace, Lms, Rgb, RgbSpec, Srgb, Xyz},
};

/// CIE xyY chromaticity + luminance color space.
///
/// Separates chromaticity (x, y) from luminance (Y), making it a direct
/// reparameterization of CIE XYZ. Widely used for chromaticity diagrams
/// and specifying color in terms of dominant wavelength and purity.
/// Values are computed relative to a reference white point (default: D65 / CIE 1931 2°).
#[derive(Clone, Copy, Debug)]
pub struct Xyy {
  alpha: Component,
  context: ColorimetricContext,
  x_chrom: Component,
  y_chrom: Component,
  big_y: Component,
}

impl Xyy {
  /// The default viewing context for xyY (D65 illuminant, CIE 1931 2° observer).
  pub const DEFAULT_CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);

  /// Creates a new xyY color with the default viewing context.
  pub fn new(x: impl Into<Component>, y: impl Into<Component>, big_y: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: Self::DEFAULT_CONTEXT,
      x_chrom: x.into(),
      y_chrom: y.into(),
      big_y: big_y.into(),
    }
  }

  /// Creates a new xyY color in a const context.
  pub const fn new_const(x: f64, y: f64, big_y: f64) -> Self {
    Self {
      alpha: Component::new_const(1.0),
      context: Self::DEFAULT_CONTEXT,
      x_chrom: Component::new_const(x),
      y_chrom: Component::new_const(y),
      big_y: Component::new_const(big_y),
    }
  }

  /// Adapts this color to a different viewing context via XYZ.
  pub fn adapt_to(&self, context: ColorimetricContext) -> Self {
    let reference_white = self.context.reference_white();
    let target_white = context.reference_white();

    if reference_white == target_white {
      return self.with_context(context);
    }

    self.to_xyz().adapt_to(context).to_xyy()
  }

  /// Returns the Y (luminance) component.
  pub fn big_y(&self) -> f64 {
    self.big_y.0
  }

  /// Returns the [x, y, Y] components as an array.
  pub fn components(&self) -> [f64; 3] {
    [self.x_chrom.0, self.y_chrom.0, self.big_y.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases the Y (luminance) component by the given amount.
  pub fn decrement_big_y(&mut self, amount: impl Into<Component>) {
    self.big_y -= amount.into();
  }

  /// Decreases the x chromaticity component by the given amount.
  pub fn decrement_x(&mut self, amount: impl Into<Component>) {
    self.x_chrom -= amount.into();
  }

  /// Decreases the y chromaticity component by the given amount.
  pub fn decrement_y(&mut self, amount: impl Into<Component>) {
    self.y_chrom -= amount.into();
  }

  /// Increases the Y (luminance) component by the given amount.
  pub fn increment_big_y(&mut self, amount: impl Into<Component>) {
    self.big_y += amount.into();
  }

  /// Increases the x chromaticity component by the given amount.
  pub fn increment_x(&mut self, amount: impl Into<Component>) {
    self.x_chrom += amount.into();
  }

  /// Increases the y chromaticity component by the given amount.
  pub fn increment_y(&mut self, amount: impl Into<Component>) {
    self.y_chrom += amount.into();
  }

  /// Scales the Y (luminance) component by the given factor.
  pub fn scale_big_y(&mut self, factor: impl Into<Component>) {
    self.big_y *= factor.into();
  }

  /// Scales the x chromaticity component by the given factor.
  pub fn scale_x(&mut self, factor: impl Into<Component>) {
    self.x_chrom *= factor.into();
  }

  /// Scales the y chromaticity component by the given factor.
  pub fn scale_y(&mut self, factor: impl Into<Component>) {
    self.y_chrom *= factor.into();
  }

  /// Sets the [x, y, Y] components from an array.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_x(components[0].clone());
    self.set_y(components[1].clone());
    self.set_big_y(components[2].clone());
  }

  /// Sets the Y (luminance) component.
  pub fn set_big_y(&mut self, big_y: impl Into<Component>) {
    self.big_y = big_y.into();
  }

  /// Sets the x chromaticity component.
  pub fn set_x(&mut self, x: impl Into<Component>) {
    self.x_chrom = x.into();
  }

  /// Sets the y chromaticity component.
  pub fn set_y(&mut self, y: impl Into<Component>) {
    self.y_chrom = y.into();
  }

  /// Converts to the CIE XYZ color space.
  pub fn to_xyz(&self) -> Xyz {
    let [x_c, y_c, big_y] = self.components();

    if y_c == 0.0 {
      return Xyz::new(0.0, 0.0, 0.0)
        .with_context(self.context)
        .with_alpha(self.alpha);
    }

    let ratio = big_y / y_c;
    let x = ratio * x_c;
    let z = ratio * (1.0 - x_c - y_c);

    Xyz::new(x, big_y, z).with_context(self.context).with_alpha(self.alpha)
  }

  /// Returns the x chromaticity component.
  pub fn x(&self) -> f64 {
    self.x_chrom.0
  }

  /// Returns the y chromaticity component.
  pub fn y(&self) -> f64 {
    self.y_chrom.0
  }

  /// Returns this color with a different viewing context (without adaptation).
  pub fn with_context(&self, context: ColorimetricContext) -> Self {
    Self {
      context,
      ..*self
    }
  }

  /// Returns a new color with the given Y (luminance) value.
  pub fn with_big_y(&self, big_y: impl Into<Component>) -> Self {
    Self {
      big_y: big_y.into(),
      ..*self
    }
  }

  /// Returns a new color with Y decreased by the given amount.
  pub fn with_big_y_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.decrement_big_y(amount);
    xyy
  }

  /// Returns a new color with Y increased by the given amount.
  pub fn with_big_y_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.increment_big_y(amount);
    xyy
  }

  /// Returns a new color with Y scaled by the given factor.
  pub fn with_big_y_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.scale_big_y(factor);
    xyy
  }

  /// Returns a new color with the given x chromaticity value.
  pub fn with_x(&self, x: impl Into<Component>) -> Self {
    Self {
      x_chrom: x.into(),
      ..*self
    }
  }

  /// Returns a new color with x decreased by the given amount.
  pub fn with_x_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.decrement_x(amount);
    xyy
  }

  /// Returns a new color with x increased by the given amount.
  pub fn with_x_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.increment_x(amount);
    xyy
  }

  /// Returns a new color with x scaled by the given factor.
  pub fn with_x_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.scale_x(factor);
    xyy
  }

  /// Returns a new color with the given y chromaticity value.
  pub fn with_y(&self, y: impl Into<Component>) -> Self {
    Self {
      y_chrom: y.into(),
      ..*self
    }
  }

  /// Returns a new color with y decreased by the given amount.
  pub fn with_y_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.decrement_y(amount);
    xyy
  }

  /// Returns a new color with y increased by the given amount.
  pub fn with_y_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.increment_y(amount);
    xyy
  }

  /// Returns a new color with y scaled by the given factor.
  pub fn with_y_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyy = *self;
    xyy.scale_y(factor);
    xyy
  }
}

impl<T> Add<T> for Xyy
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Xyy {
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
impl<'de> serde::Deserialize<'de> for Xyy {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    #[derive(serde::Deserialize)]
    struct XyyData {
      x: Component,
      y: Component,
      #[serde(rename = "Y")]
      big_y: Component,
      #[serde(default = "crate::component::default_alpha")]
      alpha: Component,
    }

    let data = XyyData::deserialize(deserializer)?;
    Ok(Self {
      x_chrom: data.x,
      y_chrom: data.y,
      big_y: data.big_y,
      alpha: data.alpha,
      context: Self::DEFAULT_CONTEXT,
    })
  }
}

impl Display for Xyy {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(4);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "xyY({:.precision$}, {:.precision$}, {:.precision$}, {:.0}%)",
        self.x_chrom,
        self.y_chrom,
        self.big_y,
        self.opacity()
      )
    } else {
      write!(
        f,
        "xyY({:.precision$}, {:.precision$}, {:.precision$})",
        self.x_chrom, self.y_chrom, self.big_y
      )
    }
  }
}

impl<T> Div<T> for Xyy
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Xyy
where
  T: Into<Component>,
{
  fn from([x, y, big_y]: [T; 3]) -> Self {
    Self::new(x, y, big_y)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Xyy
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_xyy()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Xyy
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_xyy()
  }
}

#[cfg(feature = "space-hsi")]
impl<S> From<Hsi<S>> for Xyy
where
  S: RgbSpec,
{
  fn from(hsi: Hsi<S>) -> Self {
    hsi.to_xyy()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Xyy
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_xyy()
  }
}

#[cfg(feature = "space-hsluv")]
impl From<Hsluv> for Xyy {
  fn from(hsluv: Hsluv) -> Self {
    hsluv.to_xyy()
  }
}

#[cfg(feature = "space-hpluv")]
impl From<Hpluv> for Xyy {
  fn from(hpluv: Hpluv) -> Self {
    hpluv.to_xyy()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Xyy
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_xyy()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Xyy
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_xyy()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Xyy {
  fn from(lab: Lab) -> Self {
    lab.to_xyy()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Xyy {
  fn from(lch: Lch) -> Self {
    lch.to_xyy()
  }
}

#[cfg(feature = "space-lchuv")]
impl From<Lchuv> for Xyy {
  fn from(lchuv: Lchuv) -> Self {
    lchuv.to_xyy()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Xyy {
  fn from(luv: Luv) -> Self {
    luv.to_xyy()
  }
}

impl From<Lms> for Xyy {
  fn from(lms: Lms) -> Self {
    lms.to_xyy()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Xyy {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_xyy()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Xyy {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_xyy()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Xyy {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_xyy()
  }
}

#[cfg(feature = "space-oklab")]
impl From<Oklab> for Xyy {
  fn from(oklab: Oklab) -> Self {
    oklab.to_xyy()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Xyy {
  fn from(oklch: Oklch) -> Self {
    oklch.to_xyy()
  }
}

impl<S> From<Rgb<S>> for Xyy
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_xyy()
  }
}

impl From<Xyz> for Xyy {
  fn from(xyz: Xyz) -> Self {
    xyz.to_xyy()
  }
}

impl<T> Mul<T> for Xyy
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Xyy
where
  T: Into<Xyy> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha
      && self.x_chrom == other.x_chrom
      && self.y_chrom == other.y_chrom
      && self.big_y == other.big_y
  }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Xyy {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeStruct;

    let field_count = if self.alpha.0 < 1.0 { 4 } else { 3 };
    let mut state = serializer.serialize_struct("Xyy", field_count)?;
    state.serialize_field("x", &self.x_chrom)?;
    state.serialize_field("y", &self.y_chrom)?;
    state.serialize_field("Y", &self.big_y)?;
    if self.alpha.0 < 1.0 {
      state.serialize_field("alpha", &self.alpha)?;
    }
    state.end()
  }
}

impl<T> Sub<T> for Xyy
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Xyy {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
}

impl TryFrom<String> for Xyy {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Self::from(Rgb::<Srgb>::try_from(value)?.to_xyz()))
  }
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
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let same_context = Xyy::DEFAULT_CONTEXT;
      let adapted = xyy.adapt_to(same_context);

      assert!((adapted.x() - xyy.x()).abs() < 1e-10);
      assert!((adapted.y() - xyy.y()).abs() < 1e-10);
      assert!((adapted.big_y() - xyy.big_y()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_values_for_non_d65_source() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let context_a = ColorimetricContext::new().with_illuminant(illuminant_a);
      let xyy = Xyy::new(0.3127, 0.3290, 0.5).with_context(context_a);
      let adapted = xyy.adapt_to(Xyy::DEFAULT_CONTEXT);

      assert!(
        (adapted.x() - xyy.x()).abs() > 0.001
          || (adapted.y() - xyy.y()).abs() > 0.001
          || (adapted.big_y() - xyy.big_y()).abs() > 0.001
      );
    }

    #[test]
    fn it_preserves_alpha() {
      let illuminant_b = Illuminant::new("Test B", IlluminantType::Custom, Spd::new(TEST_SPD_B));
      let xyy = Xyy::new(0.3127, 0.3290, 0.5).with_alpha(0.5);
      let target_context = ColorimetricContext::new().with_illuminant(illuminant_b);
      let adapted = xyy.adapt_to(target_context);

      assert!((adapted.alpha() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_updates_context_when_white_points_match() {
      let context = Xyy::DEFAULT_CONTEXT;
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let adapted = xyy.adapt_to(context);

      assert_eq!(adapted.context().illuminant().name(), "D65");
    }
  }

  mod add {
    use super::*;

    #[test]
    fn it_adds_two_xyy_colors() {
      let a = Xyy::new(0.3127, 0.3290, 0.5);
      let b = Xyy::new(0.4, 0.3, 0.3);
      let result = a + b;

      assert!(result.big_y() > 0.0);
    }
  }

  mod components {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_components_as_array() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let [x, y, big_y] = xyy.components();

      assert_eq!(x, 0.3127);
      assert_eq!(y, 0.3290);
      assert_eq!(big_y, 0.5);
    }
  }

  mod decrement_big_y {
    use super::*;

    #[test]
    fn it_decreases_big_y_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.decrement_big_y(0.1);

      assert!((xyy.big_y() - 0.4).abs() < 1e-10);
    }
  }

  mod decrement_x {
    use super::*;

    #[test]
    fn it_decreases_x_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.decrement_x(0.1);

      assert!((xyy.x() - 0.2127).abs() < 1e-10);
    }
  }

  mod decrement_y {
    use super::*;

    #[test]
    fn it_decreases_y_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.decrement_y(0.1);

      assert!((xyy.y() - 0.2290).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let xyy = Xyy::new(0.3127, 0.3290, 1.0);

      assert_eq!(format!("{}", xyy), "xyY(0.3127, 0.3290, 1.0000)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let xyy = Xyy::new(0.3127, 0.3290, 1.0);

      assert_eq!(format!("{:.2}", xyy), "xyY(0.31, 0.33, 1.00)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let xyy = Xyy::new(0.3127, 0.3290, 1.0).with_alpha(0.5);

      assert_eq!(format!("{}", xyy), "xyY(0.3127, 0.3290, 1.0000, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let xyy = Xyy::new(0.3127, 0.3290, 1.0);

      assert_eq!(format!("{}", xyy), "xyY(0.3127, 0.3290, 1.0000)");
    }
  }

  mod from_array {
    use super::*;

    #[test]
    fn it_creates_from_f64_array() {
      let xyy = Xyy::from([0.3127, 0.3290, 1.0]);

      assert!((xyy.x() - 0.3127).abs() < 1e-10);
      assert!((xyy.y() - 0.3290).abs() < 1e-10);
      assert!((xyy.big_y() - 1.0).abs() < 1e-10);
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_white_correctly() {
      let rgb = Rgb::<Srgb>::new(255, 255, 255);
      let xyy = Xyy::from(rgb);

      assert!((xyy.big_y() - 1.0).abs() < 0.01);
      assert!((xyy.x() - 0.3127).abs() < 0.001);
      assert!((xyy.y() - 0.3290).abs() < 0.001);
    }

    #[test]
    fn it_converts_black_correctly() {
      let rgb = Rgb::<Srgb>::new(0, 0, 0);
      let xyy = Xyy::from(rgb);

      assert!(xyy.big_y().abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let rgb = Rgb::<Srgb>::new(128, 64, 32).with_alpha(0.5);
      let xyy = Xyy::from(rgb);

      assert!((xyy.alpha() - 0.5).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let xyy = Xyy::from(xyz);

      assert!(xyy.big_y() > 0.0);
    }

    #[test]
    fn it_converts_d65_white_correctly() {
      let xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let xyy = Xyy::from(xyz);

      assert!((xyy.big_y() - 1.0).abs() < 1e-10);
      assert!((xyy.x() - 0.3127).abs() < 0.001);
      assert!((xyy.y() - 0.3290).abs() < 0.001);
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let xyy = Xyy::from(xyz);

      assert!((xyy.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod increment_big_y {
    use super::*;

    #[test]
    fn it_increases_big_y_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.increment_big_y(0.1);

      assert!((xyy.big_y() - 0.6).abs() < 1e-10);
    }
  }

  mod increment_x {
    use super::*;

    #[test]
    fn it_increases_x_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.increment_x(0.1);

      assert!((xyy.x() - 0.4127).abs() < 1e-10);
    }
  }

  mod increment_y {
    use super::*;

    #[test]
    fn it_increases_y_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.increment_y(0.1);

      assert!((xyy.y() - 0.4290).abs() < 1e-10);
    }
  }

  mod new {
    use super::*;

    #[test]
    fn it_creates_with_default_alpha() {
      let xyy = Xyy::new(0.3127, 0.3290, 1.0);

      assert!((xyy.alpha() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn it_creates_with_default_context() {
      let xyy = Xyy::new(0.3127, 0.3290, 1.0);

      assert_eq!(xyy.context().illuminant().name(), "D65");
    }
  }

  mod partial_eq {
    use super::*;

    #[test]
    fn it_compares_equal_colors() {
      let a = Xyy::new(0.3127, 0.3290, 0.5);
      let b = Xyy::new(0.3127, 0.3290, 0.5);

      assert!(a == b);
    }

    #[test]
    fn it_compares_unequal_colors() {
      let a = Xyy::new(0.3127, 0.3290, 0.5);
      let b = Xyy::new(0.4, 0.3290, 0.5);

      assert!(a != b);
    }
  }

  mod roundtrip {
    use super::*;

    #[test]
    fn it_roundtrips_through_xyz() {
      let original = Xyy::new(0.3127, 0.3290, 0.5);
      let roundtrip = Xyy::from(original.to_xyz());

      assert!((original.x() - roundtrip.x()).abs() < 1e-10);
      assert!((original.y() - roundtrip.y()).abs() < 1e-10);
      assert!((original.big_y() - roundtrip.big_y()).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_white() {
      let original = Xyy::new(0.3127, 0.3290, 1.0);
      let roundtrip = Xyy::from(original.to_xyz());

      assert!((original.x() - roundtrip.x()).abs() < 1e-10);
      assert!((original.y() - roundtrip.y()).abs() < 1e-10);
      assert!((original.big_y() - roundtrip.big_y()).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_black() {
      let original = Xyy::new(0.3127, 0.3290, 0.0);
      let roundtrip = Xyy::from(original.to_xyz());

      // Black maps to Y=0, and chromaticity comes from reference white
      assert!(roundtrip.big_y().abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_saturated_color() {
      let original = Xyy::new(0.64, 0.33, 0.2126);
      let roundtrip = Xyy::from(original.to_xyz());

      assert!((original.x() - roundtrip.x()).abs() < 1e-10);
      assert!((original.y() - roundtrip.y()).abs() < 1e-10);
      assert!((original.big_y() - roundtrip.big_y()).abs() < 1e-10);
    }
  }

  mod scale_big_y {
    use super::*;

    #[test]
    fn it_scales_big_y_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.scale_big_y(2.0);

      assert!((xyy.big_y() - 1.0).abs() < 1e-10);
    }
  }

  mod scale_x {
    use super::*;

    #[test]
    fn it_scales_x_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.scale_x(2.0);

      assert!((xyy.x() - 0.6254).abs() < 1e-10);
    }
  }

  mod scale_y {
    use super::*;

    #[test]
    fn it_scales_y_component() {
      let mut xyy = Xyy::new(0.3127, 0.3290, 0.5);
      xyy.scale_y(2.0);

      assert!((xyy.y() - 0.6580).abs() < 1e-10);
    }
  }

  mod to_rgb {
    use super::*;

    #[test]
    fn it_converts_to_srgb() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let rgb = xyy.to_rgb::<Srgb>();

      assert!(rgb.red() > 0);
    }

    #[test]
    fn it_roundtrips_through_rgb() {
      let original = Xyy::from(Rgb::<Srgb>::new(128, 64, 200));
      let roundtrip = Xyy::from(original.to_rgb::<Srgb>());

      assert!((original.x() - roundtrip.x()).abs() < 0.001);
      assert!((original.y() - roundtrip.y()).abs() < 0.001);
      assert!((original.big_y() - roundtrip.big_y()).abs() < 0.001);
    }

    #[test]
    fn it_preserves_alpha() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5).with_alpha(0.7);
      let rgb = xyy.to_rgb::<Srgb>();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let xyz = xyy.to_xyz();

      assert!(xyz.y() > 0.0);
    }

    #[test]
    fn it_handles_zero_y_chromaticity() {
      let xyy = Xyy::new(0.3127, 0.0, 0.5);
      let xyz = xyy.to_xyz();

      assert!(xyz.x().abs() < 1e-10);
      assert!(xyz.y().abs() < 1e-10);
      assert!(xyz.z().abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5).with_alpha(0.3);
      let xyz = xyy.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod try_from_str {
    use super::*;

    #[test]
    fn it_parses_hex_string() {
      let xyy = Xyy::try_from("#FF5733").unwrap();

      assert!(xyy.big_y() > 0.0);
    }

    #[test]
    fn it_returns_error_for_invalid_hex() {
      let result = Xyy::try_from("not_a_color");

      assert!(result.is_err());
    }
  }

  mod x {
    use super::*;

    #[test]
    fn it_returns_x_component() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);

      assert!((xyy.x() - 0.3127).abs() < 1e-10);
    }
  }

  mod y {
    use super::*;

    #[test]
    fn it_returns_y_component() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);

      assert!((xyy.y() - 0.3290).abs() < 1e-10);
    }
  }

  mod big_y {
    use super::*;

    #[test]
    fn it_returns_big_y_component() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);

      assert!((xyy.big_y() - 0.5).abs() < 1e-10);
    }
  }

  mod with_alpha {
    use super::*;

    #[test]
    fn it_returns_new_color_with_alpha() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_alpha(0.5);

      assert!((result.alpha() - 0.5).abs() < 1e-10);
      assert!((result.x() - 0.3127).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;

    #[test]
    fn it_returns_new_color_with_context() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let context = ColorimetricContext::default();
      let result = xyy.with_context(context);

      assert!((result.x() - 0.3127).abs() < 1e-10);
    }
  }

  mod with_big_y {
    use super::*;

    #[test]
    fn it_returns_new_color_with_big_y() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_big_y(0.8);

      assert!((result.big_y() - 0.8).abs() < 1e-10);
      assert!((result.x() - 0.3127).abs() < 1e-10);
      assert!((result.y() - 0.3290).abs() < 1e-10);
    }
  }

  mod with_big_y_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_big_y_decremented() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_big_y_decremented_by(0.1);

      assert!((result.big_y() - 0.4).abs() < 1e-10);
    }
  }

  mod with_big_y_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_big_y_incremented() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_big_y_incremented_by(0.1);

      assert!((result.big_y() - 0.6).abs() < 1e-10);
    }
  }

  mod with_big_y_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_big_y_scaled() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_big_y_scaled_by(2.0);

      assert!((result.big_y() - 1.0).abs() < 1e-10);
    }
  }

  mod with_x {
    use super::*;

    #[test]
    fn it_returns_new_color_with_x() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_x(0.4);

      assert!((result.x() - 0.4).abs() < 1e-10);
      assert!((result.y() - 0.3290).abs() < 1e-10);
      assert!((result.big_y() - 0.5).abs() < 1e-10);
    }
  }

  mod with_x_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_x_decremented() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_x_decremented_by(0.1);

      assert!((result.x() - 0.2127).abs() < 1e-10);
    }
  }

  mod with_x_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_x_incremented() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_x_incremented_by(0.1);

      assert!((result.x() - 0.4127).abs() < 1e-10);
    }
  }

  mod with_x_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_x_scaled() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_x_scaled_by(2.0);

      assert!((result.x() - 0.6254).abs() < 1e-10);
    }
  }

  mod with_y {
    use super::*;

    #[test]
    fn it_returns_new_color_with_y() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_y(0.4);

      assert!((result.y() - 0.4).abs() < 1e-10);
      assert!((result.x() - 0.3127).abs() < 1e-10);
      assert!((result.big_y() - 0.5).abs() < 1e-10);
    }
  }

  mod with_y_decremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_y_decremented() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_y_decremented_by(0.1);

      assert!((result.y() - 0.2290).abs() < 1e-10);
    }
  }

  mod with_y_incremented_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_y_incremented() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_y_incremented_by(0.1);

      assert!((result.y() - 0.4290).abs() < 1e-10);
    }
  }

  mod with_y_scaled_by {
    use super::*;

    #[test]
    fn it_returns_new_color_with_y_scaled() {
      let xyy = Xyy::new(0.3127, 0.3290, 0.5);
      let result = xyy.with_y_scaled_by(2.0);

      assert!((result.y() - 0.6580).abs() < 1e-10);
    }
  }
}
