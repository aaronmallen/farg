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
use crate::{
  ColorimetricContext,
  chromaticity::Xy,
  component::Component,
  space::{ColorSpace, LinearRgb, Lms, Rgb, RgbSpec, Srgb},
};

/// CIE 1931 XYZ tristimulus color space.
///
/// The device-independent reference space through which all conversions flow.
/// Y represents relative luminance, while X and Z carry chromaticity information.
#[derive(Clone, Copy, Debug)]
pub struct Xyz {
  alpha: Component,
  context: ColorimetricContext,
  x: Component,
  y: Component,
  z: Component,
}

impl Xyz {
  /// Creates a new XYZ color with the default viewing context.
  pub fn new(x: impl Into<Component>, y: impl Into<Component>, z: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: ColorimetricContext::default(),
      x: x.into(),
      y: y.into(),
      z: z.into(),
    }
  }

  /// Creates a new XYZ color in a const context.
  pub const fn new_const(x: f64, y: f64, z: f64) -> Self {
    Self {
      alpha: Component::new_const(1.0),
      context: ColorimetricContext::DEFAULT,
      x: Component::new_const(x),
      y: Component::new_const(y),
      z: Component::new_const(z),
    }
  }

  /// Adapts this color to a different viewing context using chromatic adaptation.
  pub fn adapt_to(&self, context: ColorimetricContext) -> Self {
    let reference_white = self.context.reference_white();
    let target_white = context.reference_white();

    if reference_white == target_white {
      return self.with_context(context);
    }

    context
      .cat()
      .adapt(*self, reference_white, target_white)
      .with_context(context)
  }

  /// Returns a new color with all components scaled by the given factor.
  pub fn amplified_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.amplify(factor);
    xyz
  }

  /// Scales all components in place by the given factor.
  pub fn amplify(&mut self, factor: impl Into<Component>) {
    let factor = factor.into();
    self.scale_x(factor);
    self.scale_y(factor);
    self.scale_z(factor);
  }

  /// Divides all components in place by the given factor.
  pub fn attenuate(&mut self, factor: impl Into<Component>) {
    let factor = factor.into();
    self.x /= factor;
    self.y /= factor;
    self.z /= factor;
  }

  /// Returns a new color with all components divided by the given factor.
  pub fn attenuated_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.attenuate(factor);
    xyz
  }

  /// Returns the CIE 1931 xy chromaticity coordinates.
  pub fn chromaticity(&self) -> Xy {
    let [x, y, z] = self.components();
    let sum = x + y + z;

    if sum == 0.0 {
      Xy::new(0.0, 0.0)
    } else {
      Xy::new(x / sum, y / sum)
    }
  }

  /// Returns the [X, Y, Z] components as an array.
  pub fn components(&self) -> [f64; 3] {
    [self.x.0, self.y.0, self.z.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases luminance (Y) while proportionally scaling X and Z to preserve chromaticity.
  pub fn decrement_luminance(&mut self, amount: impl Into<Component>) {
    let luminance = self.y - amount.into();

    if self.y.0 != 0.0 {
      let factor = luminance / self.y;
      self.scale_x(factor);
      self.scale_z(factor);
    }

    self.y = luminance;
  }

  /// Decreases the X component by the given amount.
  pub fn decrement_x(&mut self, amount: impl Into<Component>) {
    self.x -= amount.into();
  }

  /// Decreases the Y component by the given amount.
  pub fn decrement_y(&mut self, amount: impl Into<Component>) {
    self.y -= amount.into();
  }

  /// Decreases the Z component by the given amount.
  pub fn decrement_z(&mut self, amount: impl Into<Component>) {
    self.z -= amount.into();
  }

  /// Increases luminance (Y) while proportionally scaling X and Z to preserve chromaticity.
  pub fn increment_luminance(&mut self, amount: impl Into<Component>) {
    let luminance = self.y + amount.into();

    if self.y.0 != 0.0 {
      let factor = luminance / self.y;
      self.scale_x(factor);
      self.scale_z(factor);
    }

    self.y = luminance;
  }

  /// Increases the X component by the given amount.
  pub fn increment_x(&mut self, amount: impl Into<Component>) {
    self.x += amount.into();
  }

  /// Increases the Y component by the given amount.
  pub fn increment_y(&mut self, amount: impl Into<Component>) {
    self.y += amount.into();
  }

  /// Increases the Z component by the given amount.
  pub fn increment_z(&mut self, amount: impl Into<Component>) {
    self.z += amount.into();
  }

  /// Returns the relative luminance (Y component).
  pub fn luminance(&self) -> f64 {
    self.y()
  }

  /// Scales luminance by the given factor while proportionally scaling X and Z.
  pub fn scale_luminance(&mut self, factor: impl Into<Component>) {
    self.amplify(factor)
  }

  /// Scales the X component by the given factor.
  pub fn scale_x(&mut self, factor: impl Into<Component>) {
    self.x *= factor.into();
  }

  /// Scales the Y component by the given factor.
  pub fn scale_y(&mut self, factor: impl Into<Component>) {
    self.y *= factor.into();
  }

  /// Scales the Z component by the given factor.
  pub fn scale_z(&mut self, factor: impl Into<Component>) {
    self.z *= factor.into();
  }

  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_x(components[0].clone());
    self.set_y(components[1].clone());
    self.set_z(components[2].clone());
  }

  pub fn set_luminance(&mut self, luminance: impl Into<Component>) {
    self.set_y(luminance);
  }

  pub fn set_x(&mut self, x: impl Into<Component>) {
    self.x = x.into();
  }

  pub fn set_y(&mut self, y: impl Into<Component>) {
    self.y = y.into();
  }

  pub fn set_z(&mut self, z: impl Into<Component>) {
    self.z = z.into();
  }

  /// Converts to the LMS cone response space using the context's CAT matrix.
  pub fn to_lms(&self) -> Lms {
    Lms::from(self.context.cat().matrix() * self.components())
      .with_context(self.context)
      .with_alpha(self.alpha)
  }

  /// Converts to the specified RGB color space.
  pub fn to_rgb<S>(&self) -> Rgb<S>
  where
    S: RgbSpec,
  {
    let adapted = self.adapt_to(S::CONTEXT);
    let [r, g, b] = *S::inversed_xyz_matrix() * adapted.components();
    LinearRgb::<S>::from_normalized(r, g, b)
      .to_encoded()
      .with_alpha(self.alpha)
  }

  /// Returns this color with a different viewing context (without adaptation).
  pub fn with_context(&self, context: ColorimetricContext) -> Self {
    Self {
      context,
      ..*self
    }
  }

  /// Returns a new color with the given luminance, scaling X and Z proportionally.
  pub fn with_luminance(&self, luminance: impl Into<Component>) -> Self {
    let luminance = luminance.into();

    if self.y.0 == 0.0 {
      return Self {
        y: luminance,
        ..*self
      };
    }

    let factor = luminance / self.y;

    Self {
      x: self.x * factor,
      y: luminance,
      z: self.z * factor,
      ..*self
    }
  }

  /// Returns a new color with luminance decreased by the given amount.
  pub fn with_luminance_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.decrement_luminance(amount);
    xyz
  }

  /// Returns a new color with luminance increased by the given amount.
  pub fn with_luminance_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.increment_luminance(amount);
    xyz
  }

  /// Returns a new color with luminance scaled by the given factor.
  pub fn with_luminance_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.scale_luminance(factor);
    xyz
  }

  /// Returns a new color with the given X value.
  pub fn with_x(&self, x: impl Into<Component>) -> Self {
    Self {
      x: x.into(),
      ..*self
    }
  }

  /// Returns a new color with the given Y value.
  pub fn with_y(&self, y: impl Into<Component>) -> Self {
    Self {
      y: y.into(),
      ..*self
    }
  }

  /// Returns a new color with the given Z value.
  pub fn with_z(&self, z: impl Into<Component>) -> Self {
    Self {
      z: z.into(),
      ..*self
    }
  }

  /// Returns a new color with X decreased by the given amount.
  pub fn with_x_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.decrement_x(amount);
    xyz
  }

  /// Returns a new color with X increased by the given amount.
  pub fn with_x_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.increment_x(amount);
    xyz
  }

  /// Returns a new color with X scaled by the given factor.
  pub fn with_x_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.scale_x(factor);
    xyz
  }

  /// Returns a new color with Y decreased by the given amount.
  pub fn with_y_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.decrement_y(amount);
    xyz
  }

  /// Returns a new color with Y increased by the given amount.
  pub fn with_y_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.increment_y(amount);
    xyz
  }

  /// Returns a new color with Y scaled by the given factor.
  pub fn with_y_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.scale_y(factor);
    xyz
  }

  /// Returns a new color with Z decreased by the given amount.
  pub fn with_z_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.decrement_z(amount);
    xyz
  }

  /// Returns a new color with Z increased by the given amount.
  pub fn with_z_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.increment_z(amount);
    xyz
  }

  /// Returns a new color with Z scaled by the given factor.
  pub fn with_z_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.scale_z(factor);
    xyz
  }

  /// Returns the X component.
  pub fn x(&self) -> f64 {
    self.x.0
  }

  /// Returns the Y component.
  pub fn y(&self) -> f64 {
    self.y.0
  }

  /// Returns the Z component.
  pub fn z(&self) -> f64 {
    self.z.0
  }
}

impl<T> Add<T> for Xyz
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Xyz {
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

  fn to_xyz(&self) -> Self {
    *self
  }
}

impl Display for Xyz {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(4);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "XYZ({:.precision$}, {:.precision$}, {:.precision$}, {:.0}%)",
        self.x,
        self.y,
        self.z,
        self.opacity()
      )
    } else {
      write!(
        f,
        "XYZ({:.precision$}, {:.precision$}, {:.precision$})",
        self.x, self.y, self.z
      )
    }
  }
}

impl<T> Div<T> for Xyz
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Xyz
where
  T: Into<Component>,
{
  fn from([x, y, z]: [T; 3]) -> Self {
    Self::new(x, y, z)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Xyz
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_xyz()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Xyz
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_xyz()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Xyz
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_xyz()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Xyz
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_xyz()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Xyz
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_xyz()
  }
}

impl From<Lms> for Xyz {
  fn from(lms: Lms) -> Self {
    lms.to_xyz()
  }
}

impl<S> From<Rgb<S>> for Xyz
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_xyz().with_context(*rgb.context())
  }
}

impl<T> Mul<T> for Xyz
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Xyz
where
  T: Into<Xyz> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.x == other.x && self.y == other.y && self.z == other.z
  }
}

impl<T> Sub<T> for Xyz
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Xyz {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Rgb::<Srgb>::try_from(value)?.to_xyz())
  }
}

impl TryFrom<String> for Xyz {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Rgb::<Srgb>::try_from(value)?.to_xyz())
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
      let illuminant = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_context(ColorimetricContext::new().with_illuminant(illuminant));
      let same_context = ColorimetricContext::new().with_illuminant(illuminant);
      let adapted = xyz.adapt_to(same_context);

      assert!((adapted.x() - xyz.x()).abs() < 1e-10);
      assert!((adapted.y() - xyz.y()).abs() < 1e-10);
      assert!((adapted.z() - xyz.z()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_values_when_adapting_to_different_illuminant() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let illuminant_b = Illuminant::new("Test B", IlluminantType::Custom, Spd::new(TEST_SPD_B));
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_context(ColorimetricContext::new().with_illuminant(illuminant_a));
      let target_context = ColorimetricContext::new().with_illuminant(illuminant_b);
      let adapted = xyz.adapt_to(target_context);

      assert!(adapted.x() != xyz.x() || adapted.z() != xyz.z());
    }

    #[test]
    fn it_updates_context_after_adaptation() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let illuminant_b = Illuminant::new("Test B", IlluminantType::Custom, Spd::new(TEST_SPD_B));
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_context(ColorimetricContext::new().with_illuminant(illuminant_a));
      let target_context = ColorimetricContext::new().with_illuminant(illuminant_b);
      let adapted = xyz.adapt_to(target_context);

      assert_eq!(adapted.context().illuminant().name(), "Test B");
    }
  }

  mod amplified_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_scales_all_components_by_factor() {
      let xyz = Xyz::new(0.2, 0.4, 0.1);
      let result = xyz.amplified_by(2.0);

      assert_eq!(result.x(), 0.4);
      assert_eq!(result.y(), 0.8);
      assert_eq!(result.z(), 0.2);
    }

    #[test]
    fn it_accepts_integer_factor() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.amplified_by(3);

      assert_eq!(result.x(), 0.30000000000000004);
      assert_eq!(result.y(), 0.6000000000000001);
      assert_eq!(result.z(), 0.8999999999999999);
    }
  }

  mod amplify {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_mutates_in_place() {
      let mut xyz = Xyz::new(0.2, 0.4, 0.1);
      xyz.amplify(2.0);

      assert_eq!(xyz.x(), 0.4);
      assert_eq!(xyz.y(), 0.8);
      assert_eq!(xyz.z(), 0.2);
    }
  }

  mod attenuate {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_divides_all_components_by_factor() {
      let mut xyz = Xyz::new(0.4, 0.8, 0.2);
      xyz.attenuate(2.0);

      assert_eq!(xyz.x(), 0.2);
      assert_eq!(xyz.y(), 0.4);
      assert_eq!(xyz.z(), 0.1);
    }

    #[test]
    fn it_accepts_integer_factor() {
      let mut xyz = Xyz::new(1.0, 0.5, 0.25);
      xyz.attenuate(2);

      assert_eq!(xyz.x(), 0.5);
      assert_eq!(xyz.y(), 0.25);
      assert_eq!(xyz.z(), 0.125);
    }
  }

  mod attenuated_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_new_attenuated_xyz() {
      let xyz = Xyz::new(0.4, 0.8, 0.2);
      let result = xyz.attenuated_by(2.0);

      assert_eq!(result.x(), 0.2);
      assert_eq!(result.y(), 0.4);
      assert_eq!(result.z(), 0.1);
      assert_eq!(xyz.x(), 0.4);
    }
  }

  mod chromaticity {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::chromaticity::Xy;

    #[test]
    fn it_computes_chromaticity_coordinates() {
      let xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let xy = xyz.chromaticity();
      let sum = 0.95047 + 1.0 + 1.08883;

      assert_eq!(xy.x(), 0.95047 / sum);
      assert_eq!(xy.y(), 1.0 / sum);
    }

    #[test]
    fn it_handles_zero_sum() {
      let xyz = Xyz::new(0.0, 0.0, 0.0);
      let xy = xyz.chromaticity();

      assert_eq!(xy, Xy::new(0.0, 0.0));
    }

    #[test]
    fn it_preserves_chromaticity_at_different_luminances() {
      let xyz1 = Xyz::new(0.95047, 1.0, 1.08883);
      let xyz2 = Xyz::new(0.95047 * 0.5, 1.0 * 0.5, 1.08883 * 0.5);

      assert_eq!(xyz1.chromaticity(), xyz2.chromaticity());
    }
  }

  mod decrement_luminance {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_decrements_y_and_scales_x_z_proportionally() {
      let mut xyz = Xyz::new(0.2, 0.4, 0.1);
      xyz.decrement_luminance(0.2);

      assert_eq!(xyz.y(), 0.2);
      assert_eq!(xyz.x(), 0.1);
      assert_eq!(xyz.z(), 0.05);
    }

    #[test]
    fn it_handles_zero_luminance_without_scaling() {
      let mut xyz = Xyz::new(0.0, 0.0, 0.0);
      xyz.decrement_luminance(0.1);

      assert_eq!(xyz.y(), -0.1);
      assert_eq!(xyz.x(), 0.0);
      assert_eq!(xyz.z(), 0.0);
    }
  }

  mod decrement_x {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_x() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.decrement_x(0.2);

      assert_eq!(xyz.x(), 0.3);
    }
  }

  mod decrement_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_y() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.decrement_y(0.1);

      assert_eq!(xyz.y(), 0.19999999999999998);
    }
  }

  mod decrement_z {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_z() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.decrement_z(0.1);

      assert_eq!(xyz.z(), 0.1);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let xyz = Xyz::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{}", xyz), "XYZ(0.1235, 0.2346, 0.3457)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let xyz = Xyz::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{:.2}", xyz), "XYZ(0.12, 0.23, 0.35)");
      assert_eq!(format!("{:.6}", xyz), "XYZ(0.123457, 0.234568, 0.345679)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.5);

      assert_eq!(format!("{}", xyz), "XYZ(0.5000, 0.5000, 0.5000, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);

      assert_eq!(format!("{}", xyz), "XYZ(0.5000, 0.5000, 0.5000)");
    }
  }

  mod increment_luminance {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_increments_y_and_scales_x_z_proportionally() {
      let mut xyz = Xyz::new(0.1, 0.2, 0.05);
      xyz.increment_luminance(0.2);

      assert_eq!(xyz.y(), 0.4);
      assert_eq!(xyz.x(), 0.2);
      assert_eq!(xyz.z(), 0.1);
    }

    #[test]
    fn it_handles_zero_luminance_without_scaling() {
      let mut xyz = Xyz::new(0.0, 0.0, 0.0);
      xyz.increment_luminance(0.1);

      assert_eq!(xyz.y(), 0.1);
      assert_eq!(xyz.x(), 0.0);
      assert_eq!(xyz.z(), 0.0);
    }
  }

  mod increment_x {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_x() {
      let mut xyz = Xyz::new(0.1, 0.3, 0.2);
      xyz.increment_x(0.2);

      assert_eq!(xyz.x(), 0.30000000000000004);
    }
  }

  mod increment_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_y() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.increment_y(0.1);

      assert_eq!(xyz.y(), 0.4);
    }
  }

  mod increment_z {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_z() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.increment_z(0.1);

      assert_eq!(xyz.z(), 0.30000000000000004);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_xyz_values() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.1, 0.2, 0.3);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_xyz_values() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.1, 0.2, 0.4);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_with_array() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);

      assert_eq!(xyz, [0.1, 0.2, 0.3]);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Xyz::new(0.1, 0.2, 0.3).with_alpha(0.5);
      let b = Xyz::new(0.1, 0.2, 0.3);

      assert_ne!(a, b);
    }
  }

  mod scale_x {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_x_by_factor() {
      let mut xyz = Xyz::new(0.2, 0.3, 0.4);
      xyz.scale_x(2.0);

      assert_eq!(xyz.x(), 0.4);
    }
  }

  mod scale_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_y_by_factor() {
      let mut xyz = Xyz::new(0.2, 0.3, 0.4);
      xyz.scale_y(2.0);

      assert_eq!(xyz.y(), 0.6);
    }
  }

  mod scale_z {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_z_by_factor() {
      let mut xyz = Xyz::new(0.2, 0.3, 0.4);
      xyz.scale_z(2.0);

      assert_eq!(xyz.z(), 0.8);
    }
  }

  mod with_luminance {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_scaled_components() {
      let xyz = Xyz::new(0.1, 0.2, 0.05);
      let result = xyz.with_luminance(0.4);

      assert_eq!(result.y(), 0.4);
      assert_eq!(result.x(), 0.2);
      assert_eq!(result.z(), 0.1);
    }

    #[test]
    fn it_handles_zero_luminance_by_setting_y_only() {
      let xyz = Xyz::new(0.1, 0.0, 0.05);
      let result = xyz.with_luminance(0.4);

      assert_eq!(result.y(), 0.4);
      assert_eq!(result.x(), 0.1);
      assert_eq!(result.z(), 0.05);
    }
  }

  mod with_luminance_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_decremented_xyz() {
      let xyz = Xyz::new(0.2, 0.4, 0.1);
      let result = xyz.with_luminance_decremented_by(0.2);

      assert_eq!(result.y(), 0.2);
      assert_eq!(xyz.y(), 0.4);
    }
  }

  mod with_luminance_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_incremented_xyz() {
      let xyz = Xyz::new(0.1, 0.2, 0.05);
      let result = xyz.with_luminance_incremented_by(0.2);

      assert_eq!(result.y(), 0.4);
      assert_eq!(xyz.y(), 0.2);
    }
  }

  mod with_luminance_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_scaled_xyz() {
      let xyz = Xyz::new(0.1, 0.2, 0.05);
      let result = xyz.with_luminance_scaled_by(2.0);

      assert_eq!(result.x(), 0.2);
      assert_eq!(result.y(), 0.4);
      assert_eq!(result.z(), 0.1);
    }
  }

  mod with_x {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_new_x() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_x(0.5);

      assert_eq!(result.x(), 0.5);
      assert_eq!(result.y(), 0.2);
      assert_eq!(result.z(), 0.3);
    }
  }

  mod with_x_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_decremented_x() {
      let xyz = Xyz::new(0.5, 0.2, 0.3);
      let result = xyz.with_x_decremented_by(0.2);

      assert_eq!(result.x(), 0.3);
      assert_eq!(xyz.x(), 0.5);
    }
  }

  mod with_x_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_incremented_x() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_x_incremented_by(0.2);

      assert_eq!(result.x(), 0.30000000000000004);
    }
  }

  mod with_x_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_scaled_x() {
      let xyz = Xyz::new(0.2, 0.2, 0.3);
      let result = xyz.with_x_scaled_by(2.0);

      assert_eq!(result.x(), 0.4);
    }
  }

  mod with_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_new_y() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_y(0.5);

      assert_eq!(result.x(), 0.1);
      assert_eq!(result.y(), 0.5);
      assert_eq!(result.z(), 0.3);
    }
  }

  mod with_y_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_decremented_y() {
      let xyz = Xyz::new(0.1, 0.5, 0.3);
      let result = xyz.with_y_decremented_by(0.2);

      assert_eq!(result.y(), 0.3);
    }
  }

  mod with_y_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_incremented_y() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_y_incremented_by(0.2);

      assert_eq!(result.y(), 0.4);
    }
  }

  mod with_y_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_scaled_y() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_y_scaled_by(2.0);

      assert_eq!(result.y(), 0.4);
    }
  }

  mod with_z {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_new_z() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_z(0.5);

      assert_eq!(result.x(), 0.1);
      assert_eq!(result.y(), 0.2);
      assert_eq!(result.z(), 0.5);
    }
  }

  mod with_z_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_decremented_z() {
      let xyz = Xyz::new(0.1, 0.2, 0.5);
      let result = xyz.with_z_decremented_by(0.2);

      assert_eq!(result.z(), 0.3);
    }
  }

  mod with_z_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_incremented_z() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_z_incremented_by(0.2);

      assert_eq!(result.z(), 0.5);
    }
  }

  mod with_z_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_scaled_z() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_z_scaled_by(2.0);

      assert_eq!(result.z(), 0.6);
    }
  }

  #[cfg(feature = "space-cmyk")]
  mod from_cmyk {
    use super::*;

    #[test]
    fn it_converts_from_cmyk_via_rgb() {
      let cmyk = Cmyk::<Srgb>::new(25.0, 50.0, 75.0, 10.0);
      let xyz: Xyz = cmyk.into();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }
  }

  mod to_lms {
    use super::*;

    #[test]
    fn it_converts_to_lms() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let lms = xyz.to_lms();

      assert!(lms.l().is_finite());
      assert!(lms.m().is_finite());
      assert!(lms.s().is_finite());
    }

    #[test]
    fn it_preserves_context() {
      use crate::Cat;

      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let ctx = xyz.context().with_cat(Cat::XYZ_SCALING);
      let xyz_with_ctx = xyz.with_context(ctx);
      let lms = xyz_with_ctx.to_lms();

      assert_eq!(lms.context().cat().name(), "XYZ Scaling");
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let lms = xyz.to_lms();

      assert!((lms.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_rgb {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_white_xyz_to_white_rgb() {
      let white_xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let rgb: Rgb<Srgb> = white_xyz.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_black_xyz_to_black_rgb() {
      let black_xyz = Xyz::new(0.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = black_xyz.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_roundtrips_with_rgb_to_xyz() {
      let original = Rgb::<Srgb>::new(200, 100, 50);
      let xyz = original.to_xyz();
      let back: Rgb<Srgb> = xyz.to_rgb();

      assert_eq!(back.red(), original.red());
      assert_eq!(back.green(), original.green());
      assert_eq!(back.blue(), original.blue());
    }

    #[test]
    fn it_preserves_alpha() {
      let xyz = Xyz::new(0.5, 0.5, 0.5).with_alpha(0.7);
      let rgb: Rgb<Srgb> = xyz.to_rgb();

      assert!((rgb.alpha() - 0.7).abs() < 1e-10);
    }
  }

  mod with_context {
    use super::*;
    use crate::Cat;

    #[test]
    fn it_returns_xyz_with_new_context() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let new_ctx = xyz.context().with_cat(Cat::XYZ_SCALING);
      let result = xyz.with_context(new_ctx);

      assert_eq!(result.context().cat().name(), "XYZ Scaling");
      assert_eq!(result.x(), 0.1);
      assert_eq!(result.y(), 0.2);
      assert_eq!(result.z(), 0.3);
    }
  }
}
