#[cfg(feature = "space-cmy")]
mod cmy;
#[cfg(feature = "space-cmyk")]
mod cmyk;
#[cfg(feature = "space-hsl")]
mod hsl;
#[cfg(feature = "space-hsv")]
mod hsv;
#[cfg(feature = "space-hwb")]
mod hwb;
mod lms;
mod rgb;
mod xyz;

#[cfg(feature = "space-cmy")]
pub use cmy::Cmy;
#[cfg(feature = "space-cmyk")]
pub use cmyk::Cmyk;
#[cfg(feature = "space-hsl")]
pub use hsl::Hsl;
#[cfg(feature = "space-hsv")]
pub use hsv::{Hsb, Hsv};
#[cfg(feature = "space-hwb")]
pub use hwb::Hwb;
pub use lms::Lms;
pub use rgb::*;
pub use xyz::Xyz;

use crate::{chromaticity::Xy, component::Component};

/// Common interface for all color spaces.
///
/// Provides conversions between spaces, luminance operations, and component access.
/// All color spaces can convert to [`Xyz`], which serves as the universal hub.
pub trait ColorSpace<const N: usize>: Copy + Clone + From<Xyz> {
  /// Returns a new color with all components scaled by the given factor.
  fn amplified_by(&self, factor: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().amplified_by(factor))
  }

  /// Scales all components in place by the given factor.
  fn amplify(&mut self, factor: impl Into<Component>) {
    self.set_components(self.amplified_by(factor).components())
  }

  /// Divides all components in place by the given factor.
  fn attenuate(&mut self, factor: impl Into<Component>) {
    self.set_components(self.attenuated_by(factor).components())
  }

  /// Returns a new color with all components divided by the given factor.
  fn attenuated_by(&self, factor: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().attenuated_by(factor))
  }

  /// Returns the sRGB blue channel as a u8 (0-255).
  fn blue(&self) -> u8 {
    self.to_rgb::<Srgb>().blue()
  }

  /// Returns the CIE 1931 xy chromaticity coordinates.
  fn chromaticity(&self) -> Xy {
    self.to_xyz().chromaticity()
  }

  /// Returns the color's components as an array.
  fn components(&self) -> [f64; N];

  #[cfg(feature = "space-cmyk")]
  /// Returns the sRGB cyan component as a percentage (0-100%).
  fn cyan(&self) -> f64 {
    self.to_cmyk().cyan()
  }

  /// Decreases luminance in place by the given amount.
  fn decrement_luminance(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_luminance_decremented_by(amount).components())
  }

  /// Returns the sRGB green channel as a u8 (0-255).
  fn green(&self) -> u8 {
    self.to_rgb::<Srgb>().green()
  }

  /// Increases luminance in place by the given amount.
  fn increment_luminance(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_luminance_incremented_by(amount).components())
  }

  /// Returns the relative luminance (CIE Y).
  fn luminance(&self) -> f64 {
    self.to_xyz().luminance()
  }

  #[cfg(feature = "space-cmyk")]
  /// Returns the sRGB magenta component as a percentage (0-100%).
  fn magenta(&self) -> f64 {
    self.to_cmyk().magenta()
  }

  /// Returns the sRGB red channel as a u8 (0-255).
  fn red(&self) -> u8 {
    self.to_rgb::<Srgb>().red()
  }

  /// Scales luminance in place by the given factor.
  fn scale_luminance(&mut self, factor: impl Into<Component>) {
    self.set_components(self.with_luminance_scaled_by(factor).components())
  }

  /// Sets the color's components from an array.
  fn set_components(&mut self, components: [impl Into<Component> + Clone; N]);

  /// Sets the luminance to the given value in place.
  fn set_luminance(&mut self, luminance: impl Into<Component>) {
    self.set_components(self.with_luminance(luminance).components())
  }

  #[cfg(feature = "space-cmy")]
  /// Converts to the CMY color space with sRGB encoding.
  fn to_cmy(&self) -> Cmy<Srgb> {
    self.to_rgb::<Srgb>().to_cmy()
  }

  #[cfg(feature = "space-cmyk")]
  /// Converts to the CMYK color space with sRGB encoding.
  fn to_cmyk(&self) -> Cmyk<Srgb> {
    self.to_rgb::<Srgb>().to_cmyk()
  }

  #[cfg(feature = "space-hsv")]
  /// Converts to the HSB color space with sRGB encoding.
  fn to_hsb(&self) -> Hsb<Srgb> {
    self.to_rgb::<Srgb>().to_hsb()
  }

  #[cfg(feature = "space-hsl")]
  /// Converts to the HSL color space with sRGB encoding.
  fn to_hsl(&self) -> Hsl<Srgb> {
    self.to_rgb::<Srgb>().to_hsl()
  }

  #[cfg(feature = "space-hsv")]
  /// Converts to the HSV color space with sRGB encoding.
  fn to_hsv(&self) -> Hsv<Srgb> {
    self.to_rgb::<Srgb>().to_hsv()
  }

  #[cfg(feature = "space-hwb")]
  /// Converts to the HWB color space with sRGB encoding.
  fn to_hwb(&self) -> Hwb<Srgb> {
    self.to_rgb::<Srgb>().to_hwb()
  }

  /// Converts to the LMS cone response space.
  fn to_lms(&self) -> Lms {
    self.to_xyz().to_lms()
  }

  /// Converts to the specified RGB color space.
  fn to_rgb<S>(&self) -> Rgb<S>
  where
    S: RgbSpec,
  {
    self.to_xyz().to_rgb::<S>()
  }

  /// Converts to CIE XYZ.
  fn to_xyz(&self) -> Xyz;

  /// Returns a new color with the given luminance, preserving chromaticity.
  fn with_luminance(&self, luminance: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().with_luminance(luminance))
  }

  /// Returns a new color with luminance decreased by the given amount.
  fn with_luminance_decremented_by(&self, amount: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().with_luminance_decremented_by(amount))
  }

  /// Returns a new color with luminance increased by the given amount.
  fn with_luminance_incremented_by(&self, amount: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().with_luminance_incremented_by(amount))
  }

  /// Returns a new color with luminance scaled by the given factor.
  fn with_luminance_scaled_by(&self, factor: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().with_luminance_scaled_by(factor))
  }

  #[cfg(feature = "space-cmyk")]
  /// Returns the sRGB yellow component as a percentage (0-100%).
  fn yellow(&self) -> f64 {
    self.to_cmyk().yellow()
  }
}
