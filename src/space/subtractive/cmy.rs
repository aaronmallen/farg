use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  marker::PhantomData,
  ops::{Add, Div, Mul, Sub},
};

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
#[cfg(feature = "space-okhsl")]
use crate::space::Okhsl;
#[cfg(feature = "space-okhsv")]
use crate::space::Okhsv;
#[cfg(feature = "space-oklab")]
use crate::space::Oklab;
#[cfg(feature = "space-oklch")]
use crate::space::Oklch;
use crate::{
  ColorimetricContext,
  component::Component,
  space::{ColorSpace, Lms, Rgb, RgbSpec, Srgb, Xyz},
};

/// CMY (Cyan, Magenta, Yellow) subtractive color space.
///
/// A subtractive color model parameterized by an [`RgbSpec`] that determines the
/// underlying RGB space. Defaults to [`Srgb`] when not specified.
/// Components are stored normalized in 0.0-1.0 (representing 0-100%).
///
/// CMY is the complement of RGB: each component represents the amount of its
/// corresponding primary subtracted from white. Converting to RGB is simply
/// `R = 1 - C`, `G = 1 - M`, `B = 1 - Y`.
#[derive(Clone, Copy, Debug)]
pub struct Cmy<S = Srgb>
where
  S: RgbSpec,
{
  alpha: Component,
  context: ColorimetricContext,
  c: Component,
  m: Component,
  y: Component,
  _spec: PhantomData<S>,
}

impl<S> Cmy<S>
where
  S: RgbSpec,
{
  /// Creates a new CMY color from cyan (0-100%), magenta (0-100%), and yellow (0-100%).
  pub fn new(c: impl Into<Component>, m: impl Into<Component>, y: impl Into<Component>) -> Self {
    Self {
      alpha: Component::new(1.0),
      context: S::CONTEXT,
      c: c.into() / 100.0,
      m: m.into() / 100.0,
      y: y.into() / 100.0,
      _spec: PhantomData,
    }
  }

  /// Creates a new CMY color in a const context from cyan (0-100%), magenta (0-100%), and yellow (0-100%).
  pub const fn new_const(c: f64, m: f64, y: f64) -> Self {
    Self {
      alpha: Component::new_const(1.0),
      context: S::CONTEXT,
      c: Component::new_const(c / 100.0),
      m: Component::new_const(m / 100.0),
      y: Component::new_const(y / 100.0),
      _spec: PhantomData,
    }
  }

  /// Returns the normalized cyan component (0.0-1.0).
  pub fn c(&self) -> f64 {
    self.c.0
  }

  /// Returns the [C, M, Y] components as normalized values.
  pub fn components(&self) -> [f64; 3] {
    [self.c.0, self.m.0, self.y.0]
  }

  /// Returns the viewing context for this color.
  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  /// Returns the cyan as a percentage (0-100%).
  pub fn cyan(&self) -> f64 {
    self.c.0 * 100.0
  }

  /// Decreases the normalized cyan by the given amount.
  pub fn decrement_c(&mut self, amount: impl Into<Component>) {
    self.c -= amount.into();
  }

  /// Decreases the cyan by the given amount in percentage points.
  pub fn decrement_cyan(&mut self, amount: impl Into<Component>) {
    self.decrement_c(amount.into() / 100.0)
  }

  /// Decreases the normalized magenta by the given amount.
  pub fn decrement_m(&mut self, amount: impl Into<Component>) {
    self.m -= amount.into();
  }

  /// Decreases the magenta by the given amount in percentage points.
  pub fn decrement_magenta(&mut self, amount: impl Into<Component>) {
    self.decrement_m(amount.into() / 100.0)
  }

  /// Decreases the normalized yellow by the given amount.
  pub fn decrement_y(&mut self, amount: impl Into<Component>) {
    self.y -= amount.into();
  }

  /// Decreases the yellow by the given amount in percentage points.
  pub fn decrement_yellow(&mut self, amount: impl Into<Component>) {
    self.decrement_y(amount.into() / 100.0)
  }

  /// Increases the normalized cyan by the given amount.
  pub fn increment_c(&mut self, amount: impl Into<Component>) {
    self.c += amount.into();
  }

  /// Increases the cyan by the given amount in percentage points.
  pub fn increment_cyan(&mut self, amount: impl Into<Component>) {
    self.increment_c(amount.into() / 100.0)
  }

  /// Increases the normalized magenta by the given amount.
  pub fn increment_m(&mut self, amount: impl Into<Component>) {
    self.m += amount.into();
  }

  /// Increases the magenta by the given amount in percentage points.
  pub fn increment_magenta(&mut self, amount: impl Into<Component>) {
    self.increment_m(amount.into() / 100.0)
  }

  /// Increases the normalized yellow by the given amount.
  pub fn increment_y(&mut self, amount: impl Into<Component>) {
    self.y += amount.into();
  }

  /// Increases the yellow by the given amount in percentage points.
  pub fn increment_yellow(&mut self, amount: impl Into<Component>) {
    self.increment_y(amount.into() / 100.0)
  }

  /// Returns the normalized magenta component (0.0-1.0).
  pub fn m(&self) -> f64 {
    self.m.0
  }

  /// Returns the magenta as a percentage (0-100%).
  pub fn magenta(&self) -> f64 {
    self.m.0 * 100.0
  }

  /// Scales the normalized cyan by the given factor.
  pub fn scale_c(&mut self, factor: impl Into<Component>) {
    self.c *= factor.into();
  }

  /// Alias for [`Self::scale_c`].
  pub fn scale_cyan(&mut self, factor: impl Into<Component>) {
    self.scale_c(factor)
  }

  /// Scales the normalized magenta by the given factor.
  pub fn scale_m(&mut self, factor: impl Into<Component>) {
    self.m *= factor.into();
  }

  /// Alias for [`Self::scale_m`].
  pub fn scale_magenta(&mut self, factor: impl Into<Component>) {
    self.scale_m(factor)
  }

  /// Scales the normalized yellow by the given factor.
  pub fn scale_y(&mut self, factor: impl Into<Component>) {
    self.y *= factor.into();
  }

  /// Alias for [`Self::scale_y`].
  pub fn scale_yellow(&mut self, factor: impl Into<Component>) {
    self.scale_y(factor)
  }

  /// Sets all three components from normalized values.
  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_c(components[0].clone());
    self.set_m(components[1].clone());
    self.set_y(components[2].clone());
  }

  /// Sets the normalized cyan component (0.0-1.0).
  pub fn set_c(&mut self, c: impl Into<Component>) {
    self.c = c.into();
  }

  /// Sets the cyan from a percentage value (0-100%).
  pub fn set_cyan(&mut self, cyan: impl Into<Component>) {
    self.c = cyan.into() / 100.0;
  }

  /// Sets the normalized magenta component (0.0-1.0).
  pub fn set_m(&mut self, m: impl Into<Component>) {
    self.m = m.into();
  }

  /// Sets the magenta from a percentage value (0-100%).
  pub fn set_magenta(&mut self, magenta: impl Into<Component>) {
    self.m = magenta.into() / 100.0;
  }

  /// Sets the normalized yellow component (0.0-1.0).
  pub fn set_y(&mut self, y: impl Into<Component>) {
    self.y = y.into();
  }

  /// Sets the yellow from a percentage value (0-100%).
  pub fn set_yellow(&mut self, yellow: impl Into<Component>) {
    self.y = yellow.into() / 100.0;
  }

  #[cfg(feature = "space-cmyk")]
  /// Converts this CMY color to a [`Cmyk`] color in the specified output space.
  pub fn to_cmyk<OS>(&self) -> Cmyk<OS>
  where
    OS: RgbSpec,
  {
    let nc = self.c.0;
    let nm = self.m.0;
    let ny = self.y.0;
    let k = nc.min(nm).min(ny);

    if (k - 1.0).abs() < f64::EPSILON {
      return Cmyk::<OS>::new(0.0, 0.0, 0.0, 100.0).with_alpha(self.alpha);
    }

    Cmyk::<OS>::new(
      ((nc - k) / (1.0 - k)) * 100.0,
      ((nm - k) / (1.0 - k)) * 100.0,
      ((ny - k) / (1.0 - k)) * 100.0,
      k * 100.0,
    )
    .with_alpha(self.alpha)
  }

  /// Converts this CMY color to an [`Rgb`] color in the specified output space.
  pub fn to_rgb<OS>(&self) -> Rgb<OS>
  where
    OS: RgbSpec,
  {
    Rgb::<S>::from_normalized(1.0 - self.c.0, 1.0 - self.m.0, 1.0 - self.y.0)
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

  /// Returns a new color with the given normalized cyan value.
  pub fn with_c(&self, c: impl Into<Component>) -> Self {
    Self {
      c: c.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized cyan decreased by the given amount.
  pub fn with_c_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.decrement_c(amount);
    cmy
  }

  /// Returns a new color with normalized cyan increased by the given amount.
  pub fn with_c_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.increment_c(amount);
    cmy
  }

  /// Returns a new color with normalized cyan scaled by the given factor.
  pub fn with_c_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.scale_c(factor);
    cmy
  }

  /// Returns a new color with the given cyan in percentage (0-100%).
  pub fn with_cyan(&self, cyan: impl Into<Component>) -> Self {
    Self {
      c: cyan.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with cyan decreased by the given percentage points.
  pub fn with_cyan_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.decrement_cyan(amount);
    cmy
  }

  /// Returns a new color with cyan increased by the given percentage points.
  pub fn with_cyan_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.increment_cyan(amount);
    cmy
  }

  /// Alias for [`Self::with_c_scaled_by`].
  pub fn with_cyan_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_c_scaled_by(factor)
  }

  /// Returns a new color with the given normalized magenta value.
  pub fn with_m(&self, m: impl Into<Component>) -> Self {
    Self {
      m: m.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized magenta decreased by the given amount.
  pub fn with_m_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.decrement_m(amount);
    cmy
  }

  /// Returns a new color with normalized magenta increased by the given amount.
  pub fn with_m_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.increment_m(amount);
    cmy
  }

  /// Returns a new color with normalized magenta scaled by the given factor.
  pub fn with_m_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.scale_m(factor);
    cmy
  }

  /// Returns a new color with the given magenta in percentage (0-100%).
  pub fn with_magenta(&self, magenta: impl Into<Component>) -> Self {
    Self {
      m: magenta.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with magenta decreased by the given percentage points.
  pub fn with_magenta_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.decrement_magenta(amount);
    cmy
  }

  /// Returns a new color with magenta increased by the given percentage points.
  pub fn with_magenta_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.increment_magenta(amount);
    cmy
  }

  /// Alias for [`Self::with_m_scaled_by`].
  pub fn with_magenta_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_m_scaled_by(factor)
  }

  /// Returns a new color with the given normalized yellow value.
  pub fn with_y(&self, y: impl Into<Component>) -> Self {
    Self {
      y: y.into(),
      ..*self
    }
  }

  /// Returns a new color with normalized yellow decreased by the given amount.
  pub fn with_y_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.decrement_y(amount);
    cmy
  }

  /// Returns a new color with normalized yellow increased by the given amount.
  pub fn with_y_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.increment_y(amount);
    cmy
  }

  /// Returns a new color with normalized yellow scaled by the given factor.
  pub fn with_y_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.scale_y(factor);
    cmy
  }

  /// Returns a new color with the given yellow in percentage (0-100%).
  pub fn with_yellow(&self, yellow: impl Into<Component>) -> Self {
    Self {
      y: yellow.into() / 100.0,
      ..*self
    }
  }

  /// Returns a new color with yellow decreased by the given percentage points.
  pub fn with_yellow_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.decrement_yellow(amount);
    cmy
  }

  /// Returns a new color with yellow increased by the given percentage points.
  pub fn with_yellow_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut cmy = *self;
    cmy.increment_yellow(amount);
    cmy
  }

  /// Alias for [`Self::with_y_scaled_by`].
  pub fn with_yellow_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_y_scaled_by(factor)
  }

  /// Returns the normalized yellow component (0.0-1.0).
  pub fn y(&self) -> f64 {
    self.y.0
  }

  /// Returns the yellow as a percentage (0-100%).
  pub fn yellow(&self) -> f64 {
    self.y.0 * 100.0
  }
}

impl<S, T> Add<T> for Cmy<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn add(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() + rhs.into().to_rgb::<S>())
  }
}

impl<S> ColorSpace<3> for Cmy<S>
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

impl<S> Display for Cmy<S>
where
  S: RgbSpec,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let precision = f.precision().unwrap_or(2);
    if self.alpha.0 < 1.0 {
      write!(
        f,
        "CMY({:.precision$}%, {:.precision$}%, {:.precision$}%, {:.0}%)",
        self.cyan(),
        self.magenta(),
        self.yellow(),
        self.opacity()
      )
    } else {
      write!(
        f,
        "CMY({:.precision$}%, {:.precision$}%, {:.precision$}%)",
        self.cyan(),
        self.magenta(),
        self.yellow()
      )
    }
  }
}

impl<S, T> Div<T> for Cmy<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() / rhs.into().to_rgb::<S>())
  }
}

impl<S, T> From<[T; 3]> for Cmy<S>
where
  S: RgbSpec,
  T: Into<Component>,
{
  fn from([c, m, y]: [T; 3]) -> Self {
    Self::new(c, m, y)
  }
}

#[cfg(feature = "space-cmyk")]
impl<OS, S> From<Cmyk<OS>> for Cmy<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(cmyk: Cmyk<OS>) -> Self {
    cmyk.to_cmy()
  }
}

#[cfg(feature = "space-hsl")]
impl<OS, S> From<Hsl<OS>> for Cmy<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hsl: Hsl<OS>) -> Self {
    hsl.to_rgb::<S>().to_cmy()
  }
}

#[cfg(feature = "space-hsv")]
impl<OS, S> From<Hsv<OS>> for Cmy<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hsv: Hsv<OS>) -> Self {
    hsv.to_rgb::<S>().to_cmy()
  }
}

#[cfg(feature = "space-hwb")]
impl<OS, S> From<Hwb<OS>> for Cmy<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(hwb: Hwb<OS>) -> Self {
    hwb.to_rgb::<S>().to_cmy()
  }
}

#[cfg(feature = "space-lab")]
impl<S> From<Lab> for Cmy<S>
where
  S: RgbSpec,
{
  fn from(lab: Lab) -> Self {
    lab.to_rgb::<S>().to_cmy()
  }
}

#[cfg(feature = "space-lch")]
impl<S> From<Lch> for Cmy<S>
where
  S: RgbSpec,
{
  fn from(lch: Lch) -> Self {
    lch.to_rgb::<S>().to_cmy()
  }
}

impl<S> From<Lms> for Cmy<S>
where
  S: RgbSpec,
{
  fn from(lms: Lms) -> Self {
    lms.to_rgb::<S>().to_cmy()
  }
}

#[cfg(feature = "space-okhsl")]
impl<S> From<Okhsl> for Cmy<S>
where
  S: RgbSpec,
{
  fn from(okhsl: Okhsl) -> Self {
    okhsl.to_rgb::<S>().to_cmy()
  }
}

#[cfg(feature = "space-okhsv")]
impl<S> From<Okhsv> for Cmy<S>
where
  S: RgbSpec,
{
  fn from(okhsv: Okhsv) -> Self {
    okhsv.to_rgb::<S>().to_cmy()
  }
}

#[cfg(feature = "space-oklab")]
impl<S> From<Oklab> for Cmy<S>
where
  S: RgbSpec,
{
  fn from(oklab: Oklab) -> Self {
    oklab.to_rgb::<S>().to_cmy()
  }
}

#[cfg(feature = "space-oklch")]
impl<S> From<Oklch> for Cmy<S>
where
  S: RgbSpec,
{
  fn from(oklch: Oklch) -> Self {
    oklch.to_rgb::<S>().to_cmy()
  }
}

impl<OS, S> From<Rgb<OS>> for Cmy<S>
where
  OS: RgbSpec,
  S: RgbSpec,
{
  fn from(rgb: Rgb<OS>) -> Self {
    rgb.to_rgb::<S>().to_cmy()
  }
}

impl<S> From<Xyz> for Cmy<S>
where
  S: RgbSpec,
{
  fn from(xyz: Xyz) -> Self {
    xyz.to_rgb::<S>().to_cmy()
  }
}

impl<S, T> Mul<T> for Cmy<S>
where
  S: RgbSpec,
  T: Into<Self>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self {
    Self::from(self.to_rgb::<S>() * rhs.into().to_rgb::<S>())
  }
}

impl<S, T> PartialEq<T> for Cmy<S>
where
  S: RgbSpec,
  T: Into<Cmy<S>> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.alpha == other.alpha && self.c == other.c && self.m == other.m && self.y == other.y
  }
}

impl<S, T> Sub<T> for Cmy<S>
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
    fn it_adds_two_cmy_values_via_rgb() {
      let a = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let b = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let result = a + b;

      assert!(result.c().is_finite());
      assert!(result.m().is_finite());
      assert!(result.y().is_finite());
    }
  }

  mod decrement_c {
    use super::*;

    #[test]
    fn it_subtracts_from_c() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      cmy.decrement_c(0.2);

      assert!((cmy.c() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_cyan {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_cyan() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      cmy.decrement_cyan(20.0);

      assert!((cmy.cyan() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_m {
    use super::*;

    #[test]
    fn it_subtracts_from_m() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      cmy.decrement_m(0.2);

      assert!((cmy.m() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_magenta {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_magenta() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      cmy.decrement_magenta(20.0);

      assert!((cmy.magenta() - 30.0).abs() < 1e-10);
    }
  }

  mod decrement_y {
    use super::*;

    #[test]
    fn it_subtracts_from_y() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      cmy.decrement_y(0.2);

      assert!((cmy.y() - 0.3).abs() < 1e-10);
    }
  }

  mod decrement_yellow {
    use super::*;

    #[test]
    fn it_subtracts_percentage_from_yellow() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      cmy.decrement_yellow(20.0);

      assert!((cmy.yellow() - 30.0).abs() < 1e-10);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 75.0);

      assert_eq!(format!("{}", cmy), "CMY(25.00%, 50.00%, 75.00%)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let cmy = Cmy::<Srgb>::new(25.6789, 50.1234, 75.4321);

      assert_eq!(format!("{:.4}", cmy), "CMY(25.6789%, 50.1234%, 75.4321%)");
    }

    #[test]
    fn it_includes_opacity_when_alpha_below_one() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 75.0).with_alpha(0.5);

      assert_eq!(format!("{}", cmy), "CMY(25.00%, 50.00%, 75.00%, 50%)");
    }

    #[test]
    fn it_omits_opacity_when_fully_opaque() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 75.0);

      assert_eq!(format!("{}", cmy), "CMY(25.00%, 50.00%, 75.00%)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_cmy_values_via_rgb() {
      let a = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let b = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let result = a / b;

      assert!(result.c().is_finite());
      assert!(result.m().is_finite());
      assert!(result.y().is_finite());
    }
  }

  #[cfg(feature = "space-cmyk")]
  mod from_cmyk {
    use super::*;

    #[test]
    fn it_converts_via_to_cmy() {
      let cmyk = Cmyk::<Srgb>::new(100.0, 0.0, 100.0, 0.0);
      let cmy: Cmy<Srgb> = cmyk.into();

      assert!((cmy.cyan() - 100.0).abs() < 1.0);
      assert!((cmy.magenta()).abs() < 1.0);
      assert!((cmy.yellow() - 100.0).abs() < 1.0);
    }
  }

  #[cfg(feature = "space-hsl")]
  mod from_hsl {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let hsl = Hsl::<Srgb>::new(120.0, 100.0, 50.0);
      let cmy: Cmy<Srgb> = hsl.into();

      assert!((cmy.cyan() - 100.0).abs() < 1.0);
      assert!((cmy.magenta()).abs() < 1.0);
      assert!((cmy.yellow() - 100.0).abs() < 1.0);
    }
  }

  #[cfg(feature = "space-hsv")]
  mod from_hsv {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let hsv = Hsv::<Srgb>::new(120.0, 100.0, 100.0);
      let cmy: Cmy<Srgb> = hsv.into();

      assert!((cmy.cyan() - 100.0).abs() < 1.0);
      assert!((cmy.magenta()).abs() < 1.0);
      assert!((cmy.yellow() - 100.0).abs() < 1.0);
    }
  }

  #[cfg(feature = "space-hwb")]
  mod from_hwb {
    use super::*;

    #[test]
    fn it_converts_via_rgb() {
      let hwb = Hwb::<Srgb>::new(120.0, 0.0, 0.0);
      let cmy: Cmy<Srgb> = hwb.into();

      assert!((cmy.cyan() - 100.0).abs() < 1.0);
      assert!((cmy.magenta()).abs() < 1.0);
      assert!((cmy.yellow() - 100.0).abs() < 1.0);
    }
  }

  mod from_lms {
    use super::*;

    #[test]
    fn it_converts_from_lms_via_rgb() {
      let lms = Lms::new(0.5, 0.5, 0.5);
      let cmy: Cmy<Srgb> = lms.into();

      assert!(cmy.c().is_finite());
      assert!(cmy.m().is_finite());
      assert!(cmy.y().is_finite());
    }
  }

  mod from_rgb {
    use super::*;

    #[test]
    fn it_converts_pure_red() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 0.0, 0.0);
      let cmy: Cmy<Srgb> = rgb.into();

      assert!((cmy.cyan()).abs() < 1e-10);
      assert!((cmy.magenta() - 100.0).abs() < 1e-10);
      assert!((cmy.yellow() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_green() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 1.0, 0.0);
      let cmy: Cmy<Srgb> = rgb.into();

      assert!((cmy.cyan() - 100.0).abs() < 1e-10);
      assert!((cmy.magenta()).abs() < 1e-10);
      assert!((cmy.yellow() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_pure_blue() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 1.0);
      let cmy: Cmy<Srgb> = rgb.into();

      assert!((cmy.cyan() - 100.0).abs() < 1e-10);
      assert!((cmy.magenta() - 100.0).abs() < 1e-10);
      assert!((cmy.yellow()).abs() < 1e-10);
    }

    #[test]
    fn it_converts_white() {
      let rgb = Rgb::<Srgb>::from_normalized(1.0, 1.0, 1.0);
      let cmy: Cmy<Srgb> = rgb.into();

      assert!((cmy.cyan()).abs() < 1e-10);
      assert!((cmy.magenta()).abs() < 1e-10);
      assert!((cmy.yellow()).abs() < 1e-10);
    }

    #[test]
    fn it_converts_black() {
      let rgb = Rgb::<Srgb>::from_normalized(0.0, 0.0, 0.0);
      let cmy: Cmy<Srgb> = rgb.into();

      assert!((cmy.cyan() - 100.0).abs() < 1e-10);
      assert!((cmy.magenta() - 100.0).abs() < 1e-10);
      assert!((cmy.yellow() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_converts_gray_50_percent() {
      let rgb = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let cmy: Cmy<Srgb> = rgb.into();

      assert!((cmy.cyan() - 50.0).abs() < 1e-10);
      assert!((cmy.magenta() - 50.0).abs() < 1e-10);
      assert!((cmy.yellow() - 50.0).abs() < 1e-10);
    }
  }

  mod from_xyz {
    use super::*;

    #[test]
    fn it_converts_from_xyz_via_rgb() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let cmy: Cmy<Srgb> = xyz.into();

      assert!(cmy.c().is_finite());
      assert!(cmy.m().is_finite());
      assert!(cmy.y().is_finite());
    }
  }

  mod increment_c {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_c() {
      let mut cmy = Cmy::<Srgb>::new(25.0, 50.0, 50.0);
      cmy.increment_c(0.25);

      assert_eq!(cmy.c(), 0.5);
    }
  }

  mod increment_cyan {
    use super::*;

    #[test]
    fn it_adds_percentage_to_cyan() {
      let mut cmy = Cmy::<Srgb>::new(25.0, 50.0, 50.0);
      cmy.increment_cyan(25.0);

      assert!((cmy.cyan() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_m {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_m() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 25.0, 50.0);
      cmy.increment_m(0.25);

      assert_eq!(cmy.m(), 0.5);
    }
  }

  mod increment_magenta {
    use super::*;

    #[test]
    fn it_adds_percentage_to_magenta() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 25.0, 50.0);
      cmy.increment_magenta(25.0);

      assert!((cmy.magenta() - 50.0).abs() < 1e-10);
    }
  }

  mod increment_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_y() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 25.0);
      cmy.increment_y(0.25);

      assert_eq!(cmy.y(), 0.5);
    }
  }

  mod increment_yellow {
    use super::*;

    #[test]
    fn it_adds_percentage_to_yellow() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 25.0);
      cmy.increment_yellow(25.0);

      assert!((cmy.yellow() - 50.0).abs() < 1e-10);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_cmy_values_via_rgb() {
      let a = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let b = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let result = a * b;

      assert!(result.c().is_finite());
      assert!(result.m().is_finite());
      assert!(result.y().is_finite());
    }
  }

  mod new_const {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_cyan_to_0_1() {
      const CMY: Cmy<Srgb> = Cmy::new_const(75.0, 50.0, 25.0);

      assert_eq!(CMY.c(), 0.75);
    }

    #[test]
    fn it_normalizes_magenta_to_0_1() {
      const CMY: Cmy<Srgb> = Cmy::new_const(50.0, 75.0, 25.0);

      assert_eq!(CMY.m(), 0.75);
    }

    #[test]
    fn it_normalizes_yellow_to_0_1() {
      const CMY: Cmy<Srgb> = Cmy::new_const(50.0, 25.0, 75.0);

      assert_eq!(CMY.y(), 0.75);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_cmy_values() {
      let a = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let b = Cmy::<Srgb>::new(25.0, 50.0, 75.0);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_cmy_values() {
      let a = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let b = Cmy::<Srgb>::new(25.0, 50.0, 80.0);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_unequal_when_alpha_differs() {
      let a = Cmy::<Srgb>::new(25.0, 50.0, 75.0).with_alpha(0.5);
      let b = Cmy::<Srgb>::new(25.0, 50.0, 75.0);

      assert_ne!(a, b);
    }
  }

  mod scale_c {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_c_by_factor() {
      let mut cmy = Cmy::<Srgb>::new(25.0, 50.0, 50.0);
      cmy.scale_c(2.0);

      assert_eq!(cmy.c(), 0.5);
    }
  }

  mod scale_m {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_m_by_factor() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 25.0, 50.0);
      cmy.scale_m(2.0);

      assert_eq!(cmy.m(), 0.5);
    }
  }

  mod scale_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_y_by_factor() {
      let mut cmy = Cmy::<Srgb>::new(50.0, 50.0, 25.0);
      cmy.scale_y(2.0);

      assert_eq!(cmy.y(), 0.5);
    }
  }

  mod sub {
    use super::*;

    #[test]
    fn it_subtracts_two_cmy_values_via_rgb() {
      let a = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      let b = Cmy::<Srgb>::new(25.0, 25.0, 25.0);
      let result = a - b;

      assert!(result.c().is_finite());
      assert!(result.m().is_finite());
      assert!(result.y().is_finite());
    }
  }

  #[cfg(feature = "space-cmyk")]
  mod to_cmyk {
    use super::*;

    #[test]
    fn it_converts_pure_cyan() {
      let cmy = Cmy::<Srgb>::new(100.0, 0.0, 0.0);
      let cmyk: Cmyk<Srgb> = cmy.to_cmyk();

      assert!((cmyk.cyan() - 100.0).abs() < 1e-10);
      assert!((cmyk.magenta()).abs() < 1e-10);
      assert!((cmyk.yellow()).abs() < 1e-10);
      assert!((cmyk.key()).abs() < 1e-10);
    }

    #[test]
    fn it_converts_full_black() {
      let cmy = Cmy::<Srgb>::new(100.0, 100.0, 100.0);
      let cmyk: Cmyk<Srgb> = cmy.to_cmyk();

      assert!((cmyk.cyan()).abs() < 1e-10);
      assert!((cmyk.magenta()).abs() < 1e-10);
      assert!((cmyk.yellow()).abs() < 1e-10);
      assert!((cmyk.key() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_preserving_color() {
      let original = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let cmyk: Cmyk<Srgb> = original.to_cmyk();
      let back: Cmy<Srgb> = cmyk.into();
      let original_rgb: Rgb<Srgb> = original.to_rgb();
      let back_rgb: Rgb<Srgb> = back.to_rgb();

      assert_eq!(original_rgb.red(), back_rgb.red());
      assert_eq!(original_rgb.green(), back_rgb.green());
      assert_eq!(original_rgb.blue(), back_rgb.blue());
    }
  }

  mod to_rgb {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_pure_cyan() {
      let cmy = Cmy::<Srgb>::new(100.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = cmy.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_pure_magenta() {
      let cmy = Cmy::<Srgb>::new(0.0, 100.0, 0.0);
      let rgb: Rgb<Srgb> = cmy.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_pure_yellow() {
      let cmy = Cmy::<Srgb>::new(0.0, 0.0, 100.0);
      let rgb: Rgb<Srgb> = cmy.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_white() {
      let cmy = Cmy::<Srgb>::new(0.0, 0.0, 0.0);
      let rgb: Rgb<Srgb> = cmy.to_rgb();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 255);
      assert_eq!(rgb.blue(), 255);
    }

    #[test]
    fn it_converts_black() {
      let cmy = Cmy::<Srgb>::new(100.0, 100.0, 100.0);
      let rgb: Rgb<Srgb> = cmy.to_rgb();

      assert_eq!(rgb.red(), 0);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_converts_achromatic_gray() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      let rgb: Rgb<Srgb> = cmy.to_rgb();

      assert_eq!(rgb.red(), rgb.green());
      assert_eq!(rgb.green(), rgb.blue());
    }

    #[test]
    fn it_roundtrips_with_from_rgb() {
      let original = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let rgb: Rgb<Srgb> = original.to_rgb();
      let back: Cmy<Srgb> = rgb.into();

      assert!((back.cyan() - original.cyan()).abs() < 1.0);
      assert!((back.magenta() - original.magenta()).abs() < 1.0);
      assert!((back.yellow() - original.yellow()).abs() < 1.0);
    }

    #[test]
    fn it_preserves_alpha() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 75.0).with_alpha(0.3);
      let rgb: Rgb<Srgb> = cmy.to_rgb();

      assert!((rgb.alpha() - 0.3).abs() < 1e-10);
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_to_xyz_via_rgb() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let xyz = cmy.to_xyz();

      assert!(xyz.x().is_finite());
      assert!(xyz.y().is_finite());
      assert!(xyz.z().is_finite());
    }

    #[test]
    fn it_roundtrips_with_from_xyz() {
      let original = Cmy::<Srgb>::new(25.0, 50.0, 75.0);
      let xyz = original.to_xyz();
      let back: Cmy<Srgb> = xyz.into();

      assert!((back.cyan() - original.cyan()).abs() < 1.0);
      assert!((back.magenta() - original.magenta()).abs() < 1.0);
      assert!((back.yellow() - original.yellow()).abs() < 1.0);
    }
  }

  mod with_c {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_new_c() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 50.0);
      let result = cmy.with_c(0.75);

      assert_eq!(result.c(), 0.75);
      assert_eq!(result.m(), cmy.m());
      assert_eq!(result.y(), cmy.y());
    }
  }

  mod with_c_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_decremented_c() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      let result = cmy.with_c_decremented_by(0.2);

      assert!((result.c() - 0.3).abs() < 1e-10);
      assert_eq!(cmy.c(), 0.5);
    }
  }

  mod with_c_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_incremented_c() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 50.0);
      let result = cmy.with_c_incremented_by(0.25);

      assert_eq!(result.c(), 0.5);
    }
  }

  mod with_c_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_scaled_c() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 50.0);
      let result = cmy.with_c_scaled_by(2.0);

      assert_eq!(result.c(), 0.5);
    }
  }

  mod with_cyan {
    use super::*;

    #[test]
    fn it_returns_cmy_with_new_cyan_in_percent() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 50.0);
      let result = cmy.with_cyan(75.0);

      assert!((result.cyan() - 75.0).abs() < 1e-10);
      assert_eq!(result.m(), cmy.m());
      assert_eq!(result.y(), cmy.y());
    }
  }

  mod with_cyan_decremented_by {
    use super::*;

    #[test]
    fn it_returns_cmy_with_decremented_cyan() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      let result = cmy.with_cyan_decremented_by(20.0);

      assert!((result.cyan() - 30.0).abs() < 1e-10);
    }
  }

  mod with_cyan_incremented_by {
    use super::*;

    #[test]
    fn it_returns_cmy_with_incremented_cyan() {
      let cmy = Cmy::<Srgb>::new(25.0, 50.0, 50.0);
      let result = cmy.with_cyan_incremented_by(25.0);

      assert!((result.cyan() - 50.0).abs() < 1e-10);
    }
  }

  mod with_m {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_new_m() {
      let cmy = Cmy::<Srgb>::new(50.0, 25.0, 50.0);
      let result = cmy.with_m(0.75);

      assert_eq!(result.m(), 0.75);
      assert_eq!(result.c(), cmy.c());
      assert_eq!(result.y(), cmy.y());
    }
  }

  mod with_m_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_decremented_m() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      let result = cmy.with_m_decremented_by(0.2);

      assert!((result.m() - 0.3).abs() < 1e-10);
      assert_eq!(cmy.m(), 0.5);
    }
  }

  mod with_m_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_incremented_m() {
      let cmy = Cmy::<Srgb>::new(50.0, 25.0, 50.0);
      let result = cmy.with_m_incremented_by(0.25);

      assert_eq!(result.m(), 0.5);
    }
  }

  mod with_m_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_scaled_m() {
      let cmy = Cmy::<Srgb>::new(50.0, 25.0, 50.0);
      let result = cmy.with_m_scaled_by(2.0);

      assert_eq!(result.m(), 0.5);
    }
  }

  mod with_magenta {
    use super::*;

    #[test]
    fn it_returns_cmy_with_new_magenta_in_percent() {
      let cmy = Cmy::<Srgb>::new(50.0, 25.0, 50.0);
      let result = cmy.with_magenta(75.0);

      assert!((result.magenta() - 75.0).abs() < 1e-10);
      assert_eq!(result.c(), cmy.c());
      assert_eq!(result.y(), cmy.y());
    }
  }

  mod with_magenta_decremented_by {
    use super::*;

    #[test]
    fn it_returns_cmy_with_decremented_magenta() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      let result = cmy.with_magenta_decremented_by(20.0);

      assert!((result.magenta() - 30.0).abs() < 1e-10);
    }
  }

  mod with_magenta_incremented_by {
    use super::*;

    #[test]
    fn it_returns_cmy_with_incremented_magenta() {
      let cmy = Cmy::<Srgb>::new(50.0, 25.0, 50.0);
      let result = cmy.with_magenta_incremented_by(25.0);

      assert!((result.magenta() - 50.0).abs() < 1e-10);
    }
  }

  mod with_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_new_y() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 25.0);
      let result = cmy.with_y(0.75);

      assert_eq!(result.y(), 0.75);
      assert_eq!(result.c(), cmy.c());
      assert_eq!(result.m(), cmy.m());
    }
  }

  mod with_y_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_decremented_y() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      let result = cmy.with_y_decremented_by(0.2);

      assert!((result.y() - 0.3).abs() < 1e-10);
      assert_eq!(cmy.y(), 0.5);
    }
  }

  mod with_y_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_incremented_y() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 25.0);
      let result = cmy.with_y_incremented_by(0.25);

      assert_eq!(result.y(), 0.5);
    }
  }

  mod with_y_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_cmy_with_scaled_y() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 25.0);
      let result = cmy.with_y_scaled_by(2.0);

      assert_eq!(result.y(), 0.5);
    }
  }

  mod with_yellow {
    use super::*;

    #[test]
    fn it_returns_cmy_with_new_yellow_in_percent() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 25.0);
      let result = cmy.with_yellow(75.0);

      assert!((result.yellow() - 75.0).abs() < 1e-10);
      assert_eq!(result.c(), cmy.c());
      assert_eq!(result.m(), cmy.m());
    }
  }

  mod with_yellow_decremented_by {
    use super::*;

    #[test]
    fn it_returns_cmy_with_decremented_yellow() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 50.0);
      let result = cmy.with_yellow_decremented_by(20.0);

      assert!((result.yellow() - 30.0).abs() < 1e-10);
    }
  }

  mod with_yellow_incremented_by {
    use super::*;

    #[test]
    fn it_returns_cmy_with_incremented_yellow() {
      let cmy = Cmy::<Srgb>::new(50.0, 50.0, 25.0);
      let result = cmy.with_yellow_incremented_by(25.0);

      assert!((result.yellow() - 50.0).abs() < 1e-10);
    }
  }
}
