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
#[cfg(feature = "space-okhwb")]
use crate::space::Okhwb;
#[cfg(feature = "space-oklab")]
use crate::space::Oklab;
#[cfg(feature = "space-oklch")]
use crate::space::Oklch;
use crate::{
  ColorimetricContext,
  component::Component,
  space::{ColorSpace, Rgb, RgbSpec, Srgb, Xyz},
};

/// LMS cone response color space.
///
/// Represents color as responses of the three types of cone cells in the human eye:
/// Long (L), Medium (M), and Short (S) wavelength-sensitive.
#[derive(Clone, Copy, Debug)]
pub struct Lms {
  alpha: Component,
  context: ColorimetricContext,
  l: Component,
  m: Component,
  s: Component,
}

impl Lms {
  /// Creates a new LMS color with the default viewing context.
  pub fn new(l: impl Into<Component>, m: impl Into<Component>, s: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: ColorimetricContext::default(),
      l: l.into(),
      m: m.into(),
      s: s.into(),
    }
  }

  /// Creates a new LMS color in a const context.
  pub const fn new_const(l: f64, m: f64, s: f64) -> Self {
    Self {
      alpha: Component::new_const(1.0),
      context: ColorimetricContext::DEFAULT,
      l: Component::new_const(l),
      m: Component::new_const(m),
      s: Component::new_const(s),
    }
  }

  /// Adapts this color to a different viewing context via XYZ.
  pub fn adapt_to(&self, context: ColorimetricContext) -> Self {
    let reference_white = self.context.reference_white();
    let target_white = context.reference_white();

    if reference_white == target_white {
      return self.with_context(context);
    }

    self.to_xyz().adapt_to(context).to_lms()
  }

  /// Returns the [L, M, S] components as an array.
  pub fn components(&self) -> [f64; 3] {
    [self.l.0, self.m.0, self.s.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Decreases the L component by the given amount.
  pub fn decrement_l(&mut self, amount: impl Into<Component>) {
    self.l -= amount.into();
  }

  /// Alias for [`Self::decrement_l`].
  pub fn decrement_long(&mut self, amount: impl Into<Component>) {
    self.decrement_l(amount)
  }

  /// Decreases the M component by the given amount.
  pub fn decrement_m(&mut self, amount: impl Into<Component>) {
    self.m -= amount.into();
  }

  /// Alias for [`Self::decrement_m`].
  pub fn decrement_medium(&mut self, amount: impl Into<Component>) {
    self.decrement_m(amount)
  }

  /// Decreases the S component by the given amount.
  pub fn decrement_s(&mut self, amount: impl Into<Component>) {
    self.s -= amount.into();
  }

  /// Alias for [`Self::decrement_s`].
  pub fn decrement_short(&mut self, amount: impl Into<Component>) {
    self.decrement_s(amount)
  }

  /// Increases the L component by the given amount.
  pub fn increment_l(&mut self, amount: impl Into<Component>) {
    self.l += amount.into();
  }

  /// Alias for [`Self::increment_l`].
  pub fn increment_long(&mut self, amount: impl Into<Component>) {
    self.increment_l(amount)
  }

  /// Increases the M component by the given amount.
  pub fn increment_m(&mut self, amount: impl Into<Component>) {
    self.m += amount.into();
  }

  /// Alias for [`Self::increment_m`].
  pub fn increment_medium(&mut self, amount: impl Into<Component>) {
    self.increment_m(amount)
  }

  /// Increases the S component by the given amount.
  pub fn increment_s(&mut self, amount: impl Into<Component>) {
    self.s += amount.into();
  }

  /// Alias for [`Self::increment_s`].
  pub fn increment_short(&mut self, amount: impl Into<Component>) {
    self.increment_s(amount)
  }

  /// Returns the L (long) cone response.
  pub fn l(&self) -> f64 {
    self.l.0
  }

  /// Alias for [`Self::l`].
  pub fn long(&self) -> f64 {
    self.l()
  }

  /// Returns the M (medium) cone response.
  pub fn m(&self) -> f64 {
    self.m.0
  }

  /// Alias for [`Self::m`].
  pub fn medium(&self) -> f64 {
    self.m()
  }

  /// Returns the S (short) cone response.
  pub fn s(&self) -> f64 {
    self.s.0
  }

  /// Scales the L component by the given factor.
  pub fn scale_l(&mut self, factor: impl Into<Component>) {
    self.l *= factor.into();
  }

  /// Alias for [`Self::scale_l`].
  pub fn scale_long(&mut self, factor: impl Into<Component>) {
    self.scale_l(factor)
  }

  /// Scales the M component by the given factor.
  pub fn scale_m(&mut self, factor: impl Into<Component>) {
    self.m *= factor.into();
  }

  /// Alias for [`Self::scale_m`].
  pub fn scale_medium(&mut self, factor: impl Into<Component>) {
    self.scale_m(factor)
  }

  /// Scales the S component by the given factor.
  pub fn scale_s(&mut self, factor: impl Into<Component>) {
    self.s *= factor.into();
  }

  /// Alias for [`Self::scale_s`].
  pub fn scale_short(&mut self, factor: impl Into<Component>) {
    self.scale_s(factor)
  }

  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_l(components[0].clone());
    self.set_m(components[1].clone());
    self.set_s(components[2].clone());
  }

  /// Sets the L component to the given value.
  pub fn set_l(&mut self, l: impl Into<Component>) {
    self.l = l.into();
  }

  /// Alias for [`Self::set_l`].
  pub fn set_long(&mut self, l: impl Into<Component>) {
    self.set_l(l)
  }

  /// Sets the M component to the given value.
  pub fn set_m(&mut self, m: impl Into<Component>) {
    self.m = m.into();
  }

  /// Alias for [`Self::set_m`].
  pub fn set_medium(&mut self, m: impl Into<Component>) {
    self.set_m(m)
  }

  /// Sets the S component to the given value.
  pub fn set_s(&mut self, s: impl Into<Component>) {
    self.s = s.into();
  }

  /// Alias for [`Self::set_s`].
  pub fn set_short(&mut self, s: impl Into<Component>) {
    self.set_s(s)
  }

  /// Alias for [`Self::s`].
  pub fn short(&self) -> f64 {
    self.s()
  }

  /// Converts to CIE XYZ using the inverse of the context's CAT matrix.
  pub fn to_xyz(&self) -> Xyz {
    Xyz::from(self.context.cat().inverse() * *self)
      .with_context(self.context)
      .with_alpha(self.alpha)
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

  /// Alias for [`Self::with_l`].
  pub fn with_long(&self, l: impl Into<Component>) -> Self {
    self.with_l(l)
  }

  /// Returns a new color with L decreased by the given amount.
  pub fn with_l_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.decrement_l(amount);
    lms
  }

  /// Alias for [`Self::with_l_decremented_by`].
  pub fn with_long_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_l_decremented_by(amount)
  }

  /// Returns a new color with L increased by the given amount.
  pub fn with_l_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.increment_l(amount);
    lms
  }

  /// Alias for [`Self::with_l_incremented_by`].
  pub fn with_long_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_l_incremented_by(amount)
  }

  /// Returns a new color with L scaled by the given factor.
  pub fn with_l_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.scale_l(factor);
    lms
  }

  /// Alias for [`Self::with_l_scaled_by`].
  pub fn with_long_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_l_scaled_by(factor)
  }

  /// Returns a new color with the given M value.
  pub fn with_m(&self, m: impl Into<Component>) -> Self {
    Self {
      m: m.into(),
      ..*self
    }
  }

  /// Alias for [`Self::with_m`].
  pub fn with_medium(&self, m: impl Into<Component>) -> Self {
    self.with_m(m)
  }

  /// Returns a new color with M decreased by the given amount.
  pub fn with_m_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.decrement_m(amount);
    lms
  }

  /// Alias for [`Self::with_m_decremented_by`].
  pub fn with_medium_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_m_decremented_by(amount)
  }

  /// Returns a new color with M increased by the given amount.
  pub fn with_m_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.increment_m(amount);
    lms
  }

  /// Alias for [`Self::with_m_incremented_by`].
  pub fn with_medium_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_m_incremented_by(amount)
  }

  /// Returns a new color with M scaled by the given factor.
  pub fn with_m_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.scale_m(factor);
    lms
  }

  /// Alias for [`Self::with_m_scaled_by`].
  pub fn with_medium_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_m_scaled_by(factor)
  }

  /// Returns a new color with the given S value.
  pub fn with_s(&self, s: impl Into<Component>) -> Self {
    Self {
      s: s.into(),
      ..*self
    }
  }

  /// Alias for [`Self::with_s`].
  pub fn with_short(&self, s: impl Into<Component>) -> Self {
    self.with_s(s)
  }

  /// Returns a new color with S decreased by the given amount.
  pub fn with_s_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.decrement_s(amount);
    lms
  }

  /// Alias for [`Self::with_s_decremented_by`].
  pub fn with_short_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_s_decremented_by(amount)
  }

  /// Returns a new color with S increased by the given amount.
  pub fn with_s_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.increment_s(amount);
    lms
  }

  /// Alias for [`Self::with_s_incremented_by`].
  pub fn with_short_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_s_incremented_by(amount)
  }

  /// Returns a new color with S scaled by the given factor.
  pub fn with_s_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut lms = *self;
    lms.scale_s(factor);
    lms
  }

  /// Alias for [`Self::with_s_scaled_by`].
  pub fn with_short_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_s_scaled_by(factor)
  }
}

impl<T> Add<T> for Lms
where
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() + rhs.into().to_rgb::<Srgb>())
  }
}

impl ColorSpace<3> for Lms {
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

  fn to_lms(&self) -> Self {
    *self
  }

  fn to_xyz(&self) -> Xyz {
    self.to_xyz()
  }
}

impl Display for Lms {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(4);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "LMS({:.precision$}, {:.precision$}, {:.precision$}, {:.0}%)",
        self.l,
        self.m,
        self.s,
        self.opacity()
      )
    } else {
      write!(
        f,
        "LMS({:.precision$}, {:.precision$}, {:.precision$})",
        self.l, self.m, self.s
      )
    }
  }
}

impl<T> Div<T> for Lms
where
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() / rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> From<[T; 3]> for Lms
where
  T: Into<Component>,
{
  fn from([l, m, s]: [T; 3]) -> Self {
    Self::new(l, m, s)
  }
}

#[cfg(feature = "space-cmy")]
impl<S> From<Cmy<S>> for Lms
where
  S: RgbSpec,
{
  fn from(cmy: Cmy<S>) -> Self {
    cmy.to_lms()
  }
}

#[cfg(feature = "space-cmyk")]
impl<S> From<Cmyk<S>> for Lms
where
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<S>) -> Self {
    cmyk.to_lms()
  }
}

#[cfg(feature = "space-hsl")]
impl<S> From<Hsl<S>> for Lms
where
  S: RgbSpec,
{
  fn from(hsl: Hsl<S>) -> Self {
    hsl.to_lms()
  }
}

#[cfg(feature = "space-hsv")]
impl<S> From<Hsv<S>> for Lms
where
  S: RgbSpec,
{
  fn from(hsv: Hsv<S>) -> Self {
    hsv.to_lms()
  }
}

#[cfg(feature = "space-hwb")]
impl<S> From<Hwb<S>> for Lms
where
  S: RgbSpec,
{
  fn from(hwb: Hwb<S>) -> Self {
    hwb.to_lms()
  }
}

#[cfg(feature = "space-lab")]
impl From<Lab> for Lms {
  fn from(lab: Lab) -> Self {
    lab.to_lms()
  }
}

#[cfg(feature = "space-lch")]
impl From<Lch> for Lms {
  fn from(lch: Lch) -> Self {
    lch.to_lms()
  }
}

#[cfg(feature = "space-luv")]
impl From<Luv> for Lms {
  fn from(luv: Luv) -> Self {
    luv.to_lms()
  }
}

#[cfg(feature = "space-okhsl")]
impl From<Okhsl> for Lms {
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_lms()
  }
}

#[cfg(feature = "space-okhsv")]
impl From<Okhsv> for Lms {
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_lms()
  }
}

#[cfg(feature = "space-okhwb")]
impl From<Okhwb> for Lms {
  fn from(okhwb: Okhwb) -> Self {
    okhwb.to_lms()
  }
}

#[cfg(feature = "space-oklab")]
impl From<Oklab> for Lms {
  fn from(oklab: Oklab) -> Self {
    oklab.to_lms()
  }
}

#[cfg(feature = "space-oklch")]
impl From<Oklch> for Lms {
  fn from(oklch: Oklch) -> Self {
    oklch.to_lms()
  }
}

impl<S> From<Rgb<S>> for Lms
where
  S: RgbSpec,
{
  fn from(rgb: Rgb<S>) -> Self {
    rgb.to_xyz().to_lms().with_context(*rgb.context())
  }
}

impl From<Xyz> for Lms {
  fn from(xyz: Xyz) -> Self {
    xyz.to_lms()
  }
}

impl<T> Mul<T> for Lms
where
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() * rhs.into().to_rgb::<Srgb>())
  }
}

impl<T> PartialEq<T> for Lms
where
  T: Into<Self> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.l == other.l && self.m == other.m && self.s == other.s
  }
}

impl<T> Sub<T> for Lms
where
  T: Into<Self>,
{
  type Output = Self;

  fn sub(self, rhs: T) -> Self::Output {
    Self::from(self.to_rgb::<Srgb>() - rhs.into().to_rgb::<Srgb>())
  }
}

impl TryFrom<&str> for Lms {
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Ok(Rgb::<Srgb>::try_from(value)?.to_lms())
  }
}

impl TryFrom<String> for Lms {
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Ok(Rgb::<Srgb>::try_from(value)?.to_lms())
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
      let lms = Lms::new(0.5, 0.5, 0.5).with_context(ColorimetricContext::new().with_illuminant(illuminant));
      let same_context = ColorimetricContext::new().with_illuminant(illuminant);
      let adapted = lms.adapt_to(same_context);

      assert!((adapted.l() - lms.l()).abs() < 1e-10);
      assert!((adapted.m() - lms.m()).abs() < 1e-10);
      assert!((adapted.s() - lms.s()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_values_when_adapting_to_different_illuminant() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let illuminant_b = Illuminant::new("Test B", IlluminantType::Custom, Spd::new(TEST_SPD_B));
      let lms = Lms::new(0.5, 0.5, 0.5).with_context(ColorimetricContext::new().with_illuminant(illuminant_a));
      let target_context = ColorimetricContext::new().with_illuminant(illuminant_b);
      let adapted = lms.adapt_to(target_context);

      assert!(adapted.l() != lms.l() || adapted.s() != lms.s());
    }

    #[test]
    fn it_updates_context_after_adaptation() {
      let illuminant_a = Illuminant::new("Test A", IlluminantType::Custom, Spd::new(TEST_SPD_A));
      let illuminant_b = Illuminant::new("Test B", IlluminantType::Custom, Spd::new(TEST_SPD_B));
      let lms = Lms::new(0.5, 0.5, 0.5).with_context(ColorimetricContext::new().with_illuminant(illuminant_a));
      let target_context = ColorimetricContext::new().with_illuminant(illuminant_b);
      let adapted = lms.adapt_to(target_context);

      assert_eq!(adapted.context().illuminant().name(), "Test B");
    }
  }

  mod decrement_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_l() {
      let mut lms = Lms::new(0.5, 0.3, 0.2);
      lms.decrement_l(0.2);

      assert_eq!(lms.l(), 0.3);
    }
  }

  mod decrement_m {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_m() {
      let mut lms = Lms::new(0.5, 0.3, 0.2);
      lms.decrement_m(0.1);

      assert_eq!(lms.m(), 0.19999999999999998);
    }
  }

  mod decrement_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_s() {
      let mut lms = Lms::new(0.5, 0.3, 0.2);
      lms.decrement_s(0.1);

      assert_eq!(lms.s(), 0.1);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let lms = Lms::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{}", lms), "LMS(0.1235, 0.2346, 0.3457)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let lms = Lms::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{:.2}", lms), "LMS(0.12, 0.23, 0.35)");
      assert_eq!(format!("{:.6}", lms), "LMS(0.123457, 0.234568, 0.345679)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let lms = Lms::new(0.5, 0.5, 0.5).with_alpha(0.5);

      assert_eq!(format!("{}", lms), "LMS(0.5000, 0.5000, 0.5000, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let lms = Lms::new(0.5, 0.5, 0.5);

      assert_eq!(format!("{}", lms), "LMS(0.5000, 0.5000, 0.5000)");
    }
  }

  #[cfg(feature = "space-cmyk")]
  mod from_cmyk {
    use super::*;

    #[test]
    fn it_converts_from_cmyk_via_rgb() {
      let cmyk = Cmyk::<Srgb>::new(25.0, 50.0, 75.0, 10.0);
      let lms: Lms = cmyk.into();

      assert!(lms.l().is_finite());
      assert!(lms.m().is_finite());
      assert!(lms.s().is_finite());
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let lms: Lms = xyz.into();

      assert!(lms.l() != 0.5 || lms.m() != 0.5 || lms.s() != 0.5);
    }
  }

  mod increment_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_l() {
      let mut lms = Lms::new(0.1, 0.3, 0.2);
      lms.increment_l(0.2);

      assert_eq!(lms.l(), 0.30000000000000004);
    }
  }

  mod increment_m {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_m() {
      let mut lms = Lms::new(0.5, 0.3, 0.2);
      lms.increment_m(0.1);

      assert_eq!(lms.m(), 0.4);
    }
  }

  mod increment_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_s() {
      let mut lms = Lms::new(0.5, 0.3, 0.2);
      lms.increment_s(0.1);

      assert_eq!(lms.s(), 0.30000000000000004);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_lms_values() {
      let a = Lms::new(0.1, 0.2, 0.3);
      let b = Lms::new(0.1, 0.2, 0.3);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_lms_values() {
      let a = Lms::new(0.1, 0.2, 0.3);
      let b = Lms::new(0.1, 0.2, 0.4);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_with_array() {
      let lms = Lms::new(0.1, 0.2, 0.3);

      assert_eq!(lms, [0.1, 0.2, 0.3]);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Lms::new(0.1, 0.2, 0.3).with_alpha(0.5);
      let b = Lms::new(0.1, 0.2, 0.3);

      assert_ne!(a, b);
    }
  }

  mod scale_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_l_by_factor() {
      let mut lms = Lms::new(0.2, 0.3, 0.4);
      lms.scale_l(2.0);

      assert_eq!(lms.l(), 0.4);
    }
  }

  mod scale_m {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_m_by_factor() {
      let mut lms = Lms::new(0.2, 0.3, 0.4);
      lms.scale_m(2.0);

      assert_eq!(lms.m(), 0.6);
    }
  }

  mod scale_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_s_by_factor() {
      let mut lms = Lms::new(0.2, 0.3, 0.4);
      lms.scale_s(2.0);

      assert_eq!(lms.s(), 0.8);
    }
  }

  mod to_rgb {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_to_rgb_via_xyz() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let lms = xyz.to_lms();
      let rgb: Rgb<Srgb> = lms.to_rgb();
      let expected: Rgb<Srgb> = xyz.to_rgb();

      assert_eq!(rgb.red(), expected.red());
      assert_eq!(rgb.green(), expected.green());
      assert_eq!(rgb.blue(), expected.blue());
    }

    #[test]
    fn it_roundtrips_with_from_rgb() {
      let original = Rgb::<Srgb>::new(200, 100, 50);
      let lms = original.to_lms();
      let back: Rgb<Srgb> = lms.to_rgb();

      assert_eq!(back.red(), original.red());
      assert_eq!(back.green(), original.green());
      assert_eq!(back.blue(), original.blue());
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_roundtrips_with_xyz() {
      let original = Xyz::new(0.4, 0.5, 0.3);
      let lms = original.to_lms();
      let back = lms.to_xyz();

      assert!((back.x() - original.x()).abs() < 1e-10);
      assert!((back.y() - original.y()).abs() < 1e-10);
      assert!((back.z() - original.z()).abs() < 1e-10);
    }

    #[test]
    fn it_preserves_alpha() {
      let lms = Lms::new(0.5, 0.5, 0.5).with_alpha(0.3);
      let xyz = lms.to_xyz();

      assert!((xyz.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod with_l {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_new_l() {
      let lms = Lms::new(0.1, 0.2, 0.3);
      let result = lms.with_l(0.5);

      assert_eq!(result.l(), 0.5);
      assert_eq!(result.m(), 0.2);
      assert_eq!(result.s(), 0.3);
    }
  }

  mod with_l_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_decremented_l() {
      let lms = Lms::new(0.5, 0.2, 0.3);
      let result = lms.with_l_decremented_by(0.2);

      assert_eq!(result.l(), 0.3);
      assert_eq!(lms.l(), 0.5);
    }
  }

  mod with_l_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_incremented_l() {
      let lms = Lms::new(0.1, 0.2, 0.3);
      let result = lms.with_l_incremented_by(0.2);

      assert_eq!(result.l(), 0.30000000000000004);
    }
  }

  mod with_l_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_scaled_l() {
      let lms = Lms::new(0.2, 0.2, 0.3);
      let result = lms.with_l_scaled_by(2.0);

      assert_eq!(result.l(), 0.4);
    }
  }

  mod with_m {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_new_m() {
      let lms = Lms::new(0.1, 0.2, 0.3);
      let result = lms.with_m(0.5);

      assert_eq!(result.l(), 0.1);
      assert_eq!(result.m(), 0.5);
      assert_eq!(result.s(), 0.3);
    }
  }

  mod with_m_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_decremented_m() {
      let lms = Lms::new(0.1, 0.5, 0.3);
      let result = lms.with_m_decremented_by(0.2);

      assert_eq!(result.m(), 0.3);
    }
  }

  mod with_m_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_incremented_m() {
      let lms = Lms::new(0.1, 0.2, 0.3);
      let result = lms.with_m_incremented_by(0.2);

      assert_eq!(result.m(), 0.4);
    }
  }

  mod with_m_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_scaled_m() {
      let lms = Lms::new(0.1, 0.2, 0.3);
      let result = lms.with_m_scaled_by(2.0);

      assert_eq!(result.m(), 0.4);
    }
  }

  mod with_s {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_new_s() {
      let lms = Lms::new(0.1, 0.2, 0.3);
      let result = lms.with_s(0.5);

      assert_eq!(result.l(), 0.1);
      assert_eq!(result.m(), 0.2);
      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_s_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_decremented_s() {
      let lms = Lms::new(0.1, 0.2, 0.5);
      let result = lms.with_s_decremented_by(0.2);

      assert_eq!(result.s(), 0.3);
    }
  }

  mod with_s_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_incremented_s() {
      let lms = Lms::new(0.1, 0.2, 0.3);
      let result = lms.with_s_incremented_by(0.2);

      assert_eq!(result.s(), 0.5);
    }
  }

  mod with_s_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_lms_with_scaled_s() {
      let lms = Lms::new(0.1, 0.2, 0.3);
      let result = lms.with_s_scaled_by(2.0);

      assert_eq!(result.s(), 0.6);
    }
  }
}
