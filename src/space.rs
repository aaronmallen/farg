mod lms;
mod rgb;
mod xyz;

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
}
