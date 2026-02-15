mod cie;
mod cylindrical;
mod perceptual;
mod physiological;
mod rgb;
mod subtractive;

pub use cie::*;
#[allow(unused_imports)]
pub use cylindrical::*;
#[allow(unused_imports)]
pub use perceptual::*;
pub use physiological::*;
pub use rgb::*;
#[allow(unused_imports)]
pub use subtractive::*;

#[cfg(feature = "chromaticity-rg")]
use crate::chromaticity::Rg;
#[cfg(feature = "chromaticity-upvp")]
use crate::chromaticity::Upvp;
#[cfg(feature = "chromaticity-uv")]
use crate::chromaticity::Uv;
use crate::{chromaticity::Xy, component::Component};

/// Common interface for all color spaces.
///
/// Provides conversions between spaces, luminance operations, and component access.
/// All color spaces can convert to [`Xyz`], which serves as the universal hub.
pub trait ColorSpace<const N: usize>: Copy + Clone + From<Xyz> {
  /// Returns the alpha (transparency) of the color on a 0.0 to 1.0 scale.
  fn alpha(&self) -> f64;

  /// Scales all components in place by the given factor.
  fn amplify(&mut self, factor: impl Into<Component>) {
    self.set_components(self.amplified_by(factor).components())
  }

  /// Returns a new color with all components scaled by the given factor.
  fn amplified_by(&self, factor: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().amplified_by(factor))
  }

  /// Returns the two analogous colors (±30° hue rotation).
  ///
  /// Analogous colors sit adjacent on the color wheel, creating harmonious palettes
  /// with low contrast. The original color is not included in the result.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn analogous(&self) -> [Self; 2] {
    [self.with_hue_decremented_by(30), self.with_hue_incremented_by(30)]
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

  /// Returns the Oklch chroma channel.
  #[cfg(feature = "space-oklch")]
  fn chroma(&self) -> f64 {
    self.to_oklch().chroma()
  }

  /// Returns the CIE LCh chroma channel.
  #[cfg(all(feature = "space-lch", not(feature = "space-oklch")))]
  fn chroma(&self) -> f64 {
    self.to_lch().chroma()
  }

  /// Alias for [`Self::correlated_color_temperature`].
  #[cfg(any(
    feature = "cct-ohno",
    feature = "cct-robertson",
    feature = "cct-hernandez-andres",
    feature = "cct-mccamy"
  ))]
  fn cct(&self) -> crate::correlated_color_temperature::ColorTemperature {
    self.correlated_color_temperature()
  }

  /// Returns the CIE 1931 xy chromaticity coordinates.
  fn chromaticity(&self) -> Xy {
    self.to_xyz().chromaticity()
  }

  /// Returns the RGB-relative chromaticity coordinates (r, g).
  #[cfg(feature = "chromaticity-rg")]
  fn chromaticity_rg(&self) -> Rg {
    self.chromaticity().to_rg()
  }

  /// Returns the CIE 1976 UCS chromaticity coordinates (u', v').
  #[cfg(feature = "chromaticity-upvp")]
  fn chromaticity_upvp(&self) -> Upvp {
    self.chromaticity().to_upvp()
  }

  /// Returns the CIE 1960 UCS chromaticity coordinates (u, v).
  #[cfg(feature = "chromaticity-uv")]
  fn chromaticity_uv(&self) -> Uv {
    self.chromaticity().to_uv()
  }

  /// Clamps all components into the gamut of the specified RGB space.
  fn clip_to_gamut<S>(&mut self)
  where
    S: RgbSpec,
  {
    let mut rgb = self.to_rgb::<S>();
    rgb.clip_to_gamut();
    self.set_components(Self::from(rgb.to_xyz()).components())
  }

  /// Returns the closest matching color from the given slice, or `None` if empty.
  ///
  /// Uses the CIEDE2000 color difference formula for perceptually accurate matching.
  /// Accepts any color type that can be converted to [`Xyz`].
  #[cfg(feature = "distance-ciede2000")]
  fn closest_match<C>(&self, colors: &[C]) -> Option<C>
  where
    C: Into<Xyz> + Copy,
  {
    let self_xyz = self.to_xyz();
    colors
      .iter()
      .min_by(|a, b| {
        let da = crate::distance::ciede2000::calculate(self_xyz, **a);
        let db = crate::distance::ciede2000::calculate(self_xyz, **b);
        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
      })
      .copied()
  }

  /// Returns the complementary color (180° hue rotation).
  ///
  /// The complementary color sits directly opposite on the color wheel, providing
  /// maximum hue contrast.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn complementary(&self) -> Self {
    self.with_hue_incremented_by(180)
  }

  /// Returns the color's components as an array.
  fn components(&self) -> [f64; N];

  /// Reduces chroma in CIELAB space until the color fits the specified RGB gamut.
  #[cfg(feature = "space-lab")]
  fn compress_to_gamut<S>(&mut self)
  where
    S: RgbSpec,
  {
    let mut rgb = self.to_rgb::<S>();
    rgb.compress_to_gamut();
    self.set_components(Self::from(rgb.to_xyz()).components())
  }

  /// Returns the WCAG 2.x contrast ratio between this color and another.
  ///
  /// The result is always >= 1.0 and is order-independent. Use
  /// [`ContrastRatio::meets_aa`], [`ContrastRatio::meets_aaa`], etc. to check
  /// against WCAG conformance levels.
  ///
  /// [`ContrastRatio::meets_aa`]: crate::contrast::wcag::ContrastRatio::meets_aa
  /// [`ContrastRatio::meets_aaa`]: crate::contrast::wcag::ContrastRatio::meets_aaa
  #[cfg(feature = "contrast-wcag")]
  fn contrast_ratio(&self, other: impl Into<Xyz>) -> crate::contrast::wcag::ContrastRatio {
    crate::contrast::wcag::contrast_ratio(self.to_xyz(), other)
  }

  /// Returns the estimated correlated color temperature (CCT) in Kelvin.
  ///
  /// Uses the highest-precision available algorithm based on enabled features:
  /// Ohno > Robertson > Hernandez-Andres > McCamy.
  #[cfg(feature = "cct-ohno")]
  fn correlated_color_temperature(&self) -> crate::correlated_color_temperature::ColorTemperature {
    crate::correlated_color_temperature::ohno::calculate(self.to_xyz())
  }

  /// Returns the estimated correlated color temperature (CCT) in Kelvin.
  #[cfg(all(feature = "cct-robertson", not(feature = "cct-ohno")))]
  fn correlated_color_temperature(&self) -> crate::correlated_color_temperature::ColorTemperature {
    crate::correlated_color_temperature::robertson::calculate(self.to_xyz())
  }

  /// Returns the estimated correlated color temperature (CCT) in Kelvin.
  #[cfg(all(
    feature = "cct-hernandez-andres",
    not(any(feature = "cct-ohno", feature = "cct-robertson"))
  ))]
  fn correlated_color_temperature(&self) -> crate::correlated_color_temperature::ColorTemperature {
    crate::correlated_color_temperature::hernandez_andres::calculate(self.to_xyz())
  }

  /// Returns the estimated correlated color temperature (CCT) in Kelvin.
  #[cfg(all(
    feature = "cct-mccamy",
    not(any(feature = "cct-ohno", feature = "cct-robertson", feature = "cct-hernandez-andres"))
  ))]
  fn correlated_color_temperature(&self) -> crate::correlated_color_temperature::ColorTemperature {
    crate::correlated_color_temperature::mccamy::calculate(self.to_xyz())
  }

  /// Returns the sRGB cyan component as a percentage (0-100%).
  #[cfg(feature = "space-cmyk")]
  fn cyan(&self) -> f64 {
    self.to_cmyk().cyan()
  }

  /// Decreases alpha in place by the given amount on a 0.0 to 1.0 scale.
  fn decrement_alpha(&mut self, amount: impl Into<Component>) {
    self.set_alpha(self.with_alpha_decremented_by(amount).alpha())
  }

  /// Decreases chroma in place by the given amount.
  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  fn decrement_chroma(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_chroma_decremented_by(amount).components())
  }

  /// Decreases hue in place by the given amount in degrees.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn decrement_hue(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_hue_decremented_by(amount).components())
  }

  /// Decreases luminance in place by the given amount.
  fn decrement_luminance(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_luminance_decremented_by(amount).components())
  }

  /// Decreases opacity in place by the given percentage amount (0-100%).
  fn decrement_opacity(&mut self, amount: impl Into<Component>) {
    self.set_alpha(self.with_opacity_decremented_by(amount).alpha())
  }

  /// Flattens the alpha channel against black, compositing the color.
  fn flatten_alpha(&mut self) {
    let rgb = self.to_rgb::<Srgb>().with_alpha(self.alpha()).with_alpha_flattened();
    self.set_components(Self::from(rgb.to_xyz()).components());
    self.set_alpha(1.0);
  }

  /// Flattens the alpha channel against the given background color.
  fn flatten_alpha_against(&mut self, background: impl Into<Rgb<Srgb>>) {
    let rgb = self
      .to_rgb::<Srgb>()
      .with_alpha(self.alpha())
      .with_alpha_flattened_against(background);
    self.set_components(Self::from(rgb.to_xyz()).components());
    self.set_alpha(1.0);
  }

  /// Alias for [`Self::flatten_alpha`].
  fn flatten_opacity(&mut self) {
    self.flatten_alpha()
  }

  /// Alias for [`Self::flatten_alpha_against`].
  fn flatten_opacity_against(&mut self, background: impl Into<Rgb<Srgb>>) {
    self.flatten_alpha_against(background)
  }

  /// Returns the sRGB green channel as a u8 (0-255).
  fn green(&self) -> u8 {
    self.to_rgb::<Srgb>().green()
  }

  /// Returns the Oklch hue channel.
  #[cfg(feature = "space-oklch")]
  fn hue(&self) -> f64 {
    self.to_oklch().hue()
  }

  /// Returns the CIE LCh hue channel.
  #[cfg(all(feature = "space-lch", not(feature = "space-oklch")))]
  fn hue(&self) -> f64 {
    self.to_lch().hue()
  }

  /// Returns the Okhsl hue channel.
  #[cfg(all(feature = "space-okhsl", not(any(feature = "space-oklch", feature = "space-lch"))))]
  fn hue(&self) -> f64 {
    self.to_okhsl().hue()
  }

  /// Returns the Okhsv hue channel.
  #[cfg(all(
    feature = "space-okhsv",
    not(any(feature = "space-oklch", feature = "space-lch", feature = "space-okhsl"))
  ))]
  fn hue(&self) -> f64 {
    self.to_okhsv().hue()
  }

  /// Returns the Okhwb hue channel.
  #[cfg(all(
    feature = "space-okhwb",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv"
    ))
  ))]
  fn hue(&self) -> f64 {
    self.to_okhwb().hue()
  }

  /// Returns the HSL hue channel.
  #[cfg(all(
    feature = "space-hsl",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv",
      feature = "space-okhwb"
    ))
  ))]
  fn hue(&self) -> f64 {
    self.to_hsl().hue()
  }

  /// Returns the HSV hue channel.
  #[cfg(all(
    feature = "space-hsv",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv",
      feature = "space-okhwb",
      feature = "space-hsl"
    ))
  ))]
  fn hue(&self) -> f64 {
    self.to_hsv().hue()
  }

  /// Returns the HWB hue channel.
  #[cfg(all(
    feature = "space-hwb",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv",
      feature = "space-okhwb",
      feature = "space-hsl",
      feature = "space-hsv"
    ))
  ))]
  fn hue(&self) -> f64 {
    self.to_hwb().hue()
  }

  /// Increases alpha in place by the given amount on a 0.0 to 1.0 scale.
  fn increment_alpha(&mut self, amount: impl Into<Component>) {
    self.set_alpha(self.with_alpha_incremented_by(amount).alpha())
  }

  /// Increases chroma in place by the given amount.
  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  fn increment_chroma(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_chroma_incremented_by(amount).components())
  }

  /// Increases hue in place by the given amount in degrees.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn increment_hue(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_hue_incremented_by(amount).components())
  }

  /// Increases luminance in place by the given amount.
  fn increment_luminance(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_luminance_incremented_by(amount).components())
  }

  /// Increases opacity in place by the given percentage amount (0-100%).
  fn increment_opacity(&mut self, amount: impl Into<Component>) {
    self.set_alpha(self.with_opacity_incremented_by(amount).alpha())
  }

  /// Returns `true` if this color is perceptually distinguishable from another color.
  ///
  /// Uses the CIEDE2000 color difference formula with a Just Noticeable Difference (JND)
  /// threshold of 1.0. Two colors with ΔE\*00 >= 1.0 are generally considered
  /// distinguishable to the human eye.
  #[cfg(feature = "distance-ciede2000")]
  fn is_distinguishable_from(&self, other: impl Into<Xyz>) -> bool {
    !self.is_perceptually_equivalent(other)
  }

  /// Returns `true` if this color is within the gamut of the specified RGB space.
  fn is_in_gamut<S>(&self) -> bool
  where
    S: RgbSpec,
  {
    self.to_rgb::<S>().is_in_gamut()
  }

  /// Returns `true` if this color is perceptually equivalent to another color.
  ///
  /// Uses the CIEDE2000 color difference formula with a Just Noticeable Difference (JND)
  /// threshold of 1.0. Two colors with ΔE\*00 < 1.0 are generally considered
  /// indistinguishable to the human eye.
  #[cfg(feature = "distance-ciede2000")]
  fn is_perceptually_equivalent(&self, other: impl Into<Xyz>) -> bool {
    crate::distance::ciede2000::calculate(self.to_xyz(), other) < crate::distance::ciede2000::JND
  }

  /// Returns `true` if this color is physically realizable under the default observer.
  fn is_realizable(&self) -> bool {
    self.to_xyz().is_realizable()
  }

  /// Returns the APCA lightness contrast (Lc) between this color and the given background.
  ///
  /// Positive values indicate dark-on-light (normal polarity), negative values indicate
  /// light-on-dark (reverse polarity). Use [`LightnessContrast::meets_body_text_threshold`],
  /// [`LightnessContrast::meets_large_text_threshold`], or
  /// [`LightnessContrast::meets_very_large_text_threshold`] to check against APCA
  /// accessibility recommendations.
  ///
  /// [`LightnessContrast::meets_body_text_threshold`]: crate::contrast::apca::LightnessContrast::meets_body_text_threshold
  /// [`LightnessContrast::meets_large_text_threshold`]: crate::contrast::apca::LightnessContrast::meets_large_text_threshold
  /// [`LightnessContrast::meets_very_large_text_threshold`]: crate::contrast::apca::LightnessContrast::meets_very_large_text_threshold
  #[cfg(feature = "contrast-apca")]
  fn lightness_contrast(&self, background: impl Into<Xyz>) -> crate::contrast::apca::LightnessContrast {
    crate::contrast::apca::calculate(self.to_xyz(), background)
  }

  /// Returns the relative luminance (CIE Y).
  fn luminance(&self) -> f64 {
    self.to_xyz().luminance()
  }

  /// Returns the sRGB magenta component as a percentage (0-100%).
  #[cfg(feature = "space-cmyk")]
  fn magenta(&self) -> f64 {
    self.to_cmyk().magenta()
  }

  /// Returns four monochromatic variations (two darker, two lighter).
  ///
  /// Monochromatic palettes vary luminance while preserving chromaticity, producing
  /// shades and tints of the same color. The original color is not included in the
  /// result.
  fn monochromatic(&self) -> [Self; 4] {
    [
      self.with_luminance_scaled_by(0.30),
      self.with_luminance_scaled_by(0.60),
      self.with_luminance_scaled_by(1.40),
      self.with_luminance_scaled_by(1.70),
    ]
  }

  /// Returns the opacity as a percentage (0-100%).
  fn opacity(&self) -> f64 {
    self.alpha() * 100.0
  }

  /// Maps to gamut by scaling LMS components relative to the reference white.
  fn perceptually_map_to_gamut<S>(&mut self)
  where
    S: RgbSpec,
  {
    let mut rgb = self.to_rgb::<S>();
    rgb.perceptually_map_to_gamut();
    self.set_components(Self::from(rgb.to_xyz()).components())
  }

  /// Returns the sRGB red channel as a u8 (0-255).
  fn red(&self) -> u8 {
    self.to_rgb::<Srgb>().red()
  }

  /// Scales alpha in place by the given factor.
  fn scale_alpha(&mut self, factor: impl Into<Component>) {
    self.set_alpha(self.with_alpha_scaled_by(factor).alpha())
  }

  /// Scales chroma in place by the given factor.
  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  fn scale_chroma(&mut self, factor: impl Into<Component>) {
    self.set_components(self.with_chroma_scaled_by(factor).components())
  }

  /// Scales hue in place by the given factor.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn scale_hue(&mut self, factor: impl Into<Component>) {
    self.set_components(self.with_hue_scaled_by(factor).components())
  }

  /// Scales luminance in place by the given factor.
  fn scale_luminance(&mut self, factor: impl Into<Component>) {
    self.set_components(self.with_luminance_scaled_by(factor).components())
  }

  /// Scales opacity in place by the given percentage factor (0-100%).
  fn scale_opacity(&mut self, factor: impl Into<Component>) {
    self.set_alpha(self.with_opacity_scaled_by(factor).alpha())
  }

  /// Scales linear RGB components to fit within the specified RGB gamut.
  fn scale_to_gamut<S>(&mut self)
  where
    S: RgbSpec,
  {
    let mut rgb = self.to_rgb::<S>();
    rgb.scale_to_gamut();
    self.set_components(Self::from(rgb.to_xyz()).components())
  }

  /// Sets the alpha value in place on a 0.0 to 1.0 scale.
  fn set_alpha(&mut self, alpha: impl Into<Component>);

  /// Sets the chroma to the given value in place.
  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  fn set_chroma(&mut self, chroma: impl Into<Component>) {
    self.set_components(self.with_chroma(chroma).components())
  }

  /// Sets the color's components from an array.
  fn set_components(&mut self, components: [impl Into<Component> + Clone; N]);

  /// Sets the hue to the given value in degrees in place.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn set_hue(&mut self, hue: impl Into<Component>) {
    self.set_components(self.with_hue(hue).components())
  }

  /// Sets the luminance to the given value in place.
  fn set_luminance(&mut self, luminance: impl Into<Component>) {
    self.set_components(self.with_luminance(luminance).components())
  }

  /// Sets the opacity to the given percentage value (0-100%) in place.
  fn set_opacity(&mut self, opacity: impl Into<Component>) {
    self.set_alpha(opacity.into() / 100.0)
  }

  /// Returns the two split-complementary colors (+150° and +210° hue rotation).
  ///
  /// Split-complementary uses the two colors adjacent to the complement, offering
  /// strong contrast with more variety than a straight complementary pair. The
  /// original color is not included in the result.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn split_complementary(&self) -> [Self; 2] {
    [self.with_hue_incremented_by(150), self.with_hue_incremented_by(210)]
  }

  /// Returns the three tetradic colors (+90°, +180°, and +270° hue rotation).
  ///
  /// Tetradic (rectangle) harmony places four colors at 90° intervals, forming
  /// a rich palette with two complementary pairs. The original color is not
  /// included in the result.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn tetradic(&self) -> [Self; 3] {
    [
      self.with_hue_incremented_by(90),
      self.with_hue_incremented_by(180),
      self.with_hue_incremented_by(270),
    ]
  }

  /// Converts to the CMY color space with sRGB encoding.
  #[cfg(feature = "space-cmy")]
  fn to_cmy(&self) -> Cmy<Srgb> {
    self.to_rgb::<Srgb>().to_cmy().with_alpha(self.alpha())
  }

  /// Converts to the CMYK color space with sRGB encoding.
  #[cfg(feature = "space-cmyk")]
  fn to_cmyk(&self) -> Cmyk<Srgb> {
    self.to_rgb::<Srgb>().to_cmyk().with_alpha(self.alpha())
  }

  /// Converts to the HSB color space with sRGB encoding.
  #[cfg(feature = "space-hsv")]
  fn to_hsb(&self) -> Hsb<Srgb> {
    self.to_rgb::<Srgb>().to_hsb().with_alpha(self.alpha())
  }

  /// Converts to the HSL color space with sRGB encoding.
  #[cfg(feature = "space-hsl")]
  fn to_hsl(&self) -> Hsl<Srgb> {
    self.to_rgb::<Srgb>().to_hsl().with_alpha(self.alpha())
  }

  /// Converts to the HSV color space with sRGB encoding.
  #[cfg(feature = "space-hsv")]
  fn to_hsv(&self) -> Hsv<Srgb> {
    self.to_rgb::<Srgb>().to_hsv().with_alpha(self.alpha())
  }

  /// Converts to the HWB color space with sRGB encoding.
  #[cfg(feature = "space-hwb")]
  fn to_hwb(&self) -> Hwb<Srgb> {
    self.to_rgb::<Srgb>().to_hwb().with_alpha(self.alpha())
  }

  /// Converts to the CIE L*a*b* color space.
  #[cfg(feature = "space-lab")]
  fn to_lab(&self) -> Lab {
    Lab::from(self.to_xyz()).with_alpha(self.alpha())
  }

  /// Converts to the CIE L*u*v* color space.
  #[cfg(feature = "space-luv")]
  fn to_luv(&self) -> Luv {
    Luv::from(self.to_xyz()).with_alpha(self.alpha())
  }

  /// Converts to the CIE LCh color space (cylindrical form of L*a*b*).
  #[cfg(feature = "space-lch")]
  fn to_lch(&self) -> Lch {
    self.to_lab().to_lch().with_alpha(self.alpha())
  }

  /// Converts to the LMS cone response space.
  fn to_lms(&self) -> Lms {
    self.to_xyz().to_lms().with_alpha(self.alpha())
  }

  /// Converts to the Okhsl perceptual color space.
  #[cfg(feature = "space-okhsl")]
  fn to_okhsl(&self) -> Okhsl {
    self.to_oklab().to_okhsl().with_alpha(self.alpha())
  }

  /// Converts to the Okhsv perceptual color space.
  #[cfg(feature = "space-okhsv")]
  fn to_okhsv(&self) -> Okhsv {
    self.to_oklab().to_okhsv().with_alpha(self.alpha())
  }

  /// Converts to the Okhwb perceptual color space.
  #[cfg(feature = "space-okhwb")]
  fn to_okhwb(&self) -> Okhwb {
    self.to_okhsv().to_okhwb().with_alpha(self.alpha())
  }

  /// Converts to the Oklab perceptual color space.
  #[cfg(feature = "space-oklab")]
  fn to_oklab(&self) -> Oklab {
    self.to_xyz().to_oklab().with_alpha(self.alpha())
  }

  /// Converts to the Oklch perceptual color space.
  #[cfg(feature = "space-oklch")]
  fn to_oklch(&self) -> Oklch {
    self.to_oklab().to_oklch().with_alpha(self.alpha())
  }

  /// Converts to the specified RGB color space.
  fn to_rgb<S>(&self) -> Rgb<S>
  where
    S: RgbSpec,
  {
    self.to_xyz().to_rgb::<S>().with_alpha(self.alpha())
  }

  /// Converts to CIE XYZ.
  fn to_xyz(&self) -> Xyz;

  /// Returns the two triadic colors (+120° and +240° hue rotation).
  ///
  /// Triadic harmony places three colors at equal 120° intervals around the
  /// color wheel, creating vibrant palettes with balanced contrast. The original
  /// color is not included in the result.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn triadic(&self) -> [Self; 2] {
    [self.with_hue_incremented_by(120), self.with_hue_incremented_by(240)]
  }

  /// Returns a new color with the given alpha value on a 0.0 to 1.0 scale.
  fn with_alpha(&self, alpha: impl Into<Component>) -> Self {
    let mut color = *self;
    color.set_alpha(alpha);
    color
  }

  /// Returns a new color with alpha decreased by the given amount.
  fn with_alpha_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_alpha(Component::new((self.alpha() - amount.into().0).clamp(0.0, 1.0)))
  }

  /// Returns a new color with the alpha channel flattened against black.
  fn with_alpha_flattened(&self) -> Self {
    let mut color = *self;
    color.flatten_alpha();
    color
  }

  /// Returns a new color with the alpha channel flattened against the given background.
  fn with_alpha_flattened_against(&self, background: impl Into<Rgb<Srgb>>) -> Self {
    let mut color = *self;
    color.flatten_alpha_against(background);
    color
  }

  /// Returns a new color with alpha increased by the given amount.
  fn with_alpha_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_alpha(Component::new((self.alpha() + amount.into().0).clamp(0.0, 1.0)))
  }

  /// Returns a new color with alpha scaled by the given factor.
  fn with_alpha_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_alpha(Component::new((self.alpha() * factor.into().0).clamp(0.0, 1.0)))
  }

  /// Returns a new color with the given Oklch chroma.
  #[cfg(feature = "space-oklch")]
  fn with_chroma(&self, chroma: impl Into<Component>) -> Self {
    Self::from(self.to_oklch().with_chroma(chroma).to_xyz())
  }

  /// Returns a new color with the given CIE LCh chroma.
  #[cfg(all(feature = "space-lch", not(feature = "space-oklch")))]
  fn with_chroma(&self, chroma: impl Into<Component>) -> Self {
    Self::from(self.to_lch().with_chroma(chroma).to_xyz())
  }

  /// Returns a new color with chroma decreased by the given amount.
  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  fn with_chroma_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_chroma(self.chroma() - amount.into().0)
  }

  /// Returns a new color with chroma increased by the given amount.
  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  fn with_chroma_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_chroma(self.chroma() + amount.into().0)
  }

  /// Returns a new color with chroma scaled by the given factor.
  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  fn with_chroma_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_chroma(self.chroma() * factor.into().0)
  }

  /// Returns a new color with all components clamped into the specified RGB gamut.
  fn with_gamut_clipped<S>(&self) -> Self
  where
    S: RgbSpec,
  {
    let mut color = *self;
    color.clip_to_gamut::<S>();
    color
  }

  /// Returns a new color with chroma reduced in CIELAB space until the color fits the specified RGB gamut.
  #[cfg(feature = "space-lab")]
  fn with_gamut_compressed<S>(&self) -> Self
  where
    S: RgbSpec,
  {
    let mut color = *self;
    color.compress_to_gamut::<S>();
    color
  }

  /// Returns a new color mapped to the specified RGB gamut by scaling LMS components relative to the reference white.
  fn with_gamut_perceptually_mapped<S>(&self) -> Self
  where
    S: RgbSpec,
  {
    let mut color = *self;
    color.perceptually_map_to_gamut::<S>();
    color
  }

  /// Returns a new color with linear RGB components scaled to fit within the specified RGB gamut.
  fn with_gamut_scaled<S>(&self) -> Self
  where
    S: RgbSpec,
  {
    let mut color = *self;
    color.scale_to_gamut::<S>();
    color
  }

  /// Returns a new color with the given Oklch hue in degrees.
  #[cfg(feature = "space-oklch")]
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_oklch().with_hue(hue).to_xyz())
  }

  /// Returns a new color with the given CIE LCh hue in degrees.
  #[cfg(all(feature = "space-lch", not(feature = "space-oklch")))]
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_lch().with_hue(hue).to_xyz())
  }

  /// Returns a new color with the given Okhsl hue in degrees.
  #[cfg(all(feature = "space-okhsl", not(any(feature = "space-oklch", feature = "space-lch"))))]
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_okhsl().with_hue(hue).to_xyz())
  }

  /// Returns a new color with the given Okhsv hue in degrees.
  #[cfg(all(
    feature = "space-okhsv",
    not(any(feature = "space-oklch", feature = "space-lch", feature = "space-okhsl"))
  ))]
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_okhsv().with_hue(hue).to_xyz())
  }

  /// Returns a new color with the given Okhwb hue in degrees.
  #[cfg(all(
    feature = "space-okhwb",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv"
    ))
  ))]
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_okhwb().with_hue(hue).to_xyz())
  }

  /// Returns a new color with the given HSL hue in degrees.
  #[cfg(all(
    feature = "space-hsl",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv",
      feature = "space-okhwb"
    ))
  ))]
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_hsl().with_hue(hue).to_xyz())
  }

  /// Returns a new color with the given HSV hue in degrees.
  #[cfg(all(
    feature = "space-hsv",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv",
      feature = "space-okhwb",
      feature = "space-hsl"
    ))
  ))]
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_hsv().with_hue(hue).to_xyz())
  }

  /// Returns a new color with the given HWB hue in degrees.
  #[cfg(all(
    feature = "space-hwb",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv",
      feature = "space-okhwb",
      feature = "space-hsl",
      feature = "space-hsv"
    ))
  ))]
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_hwb().with_hue(hue).to_xyz())
  }

  /// Returns a new color with hue decreased by the given amount in degrees.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn with_hue_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_hue(self.hue() - amount.into().0)
  }

  /// Returns a new color with hue increased by the given amount in degrees.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_hue(self.hue() + amount.into().0)
  }

  /// Returns a new color with hue scaled by the given factor.
  #[cfg(any(
    feature = "space-oklch",
    feature = "space-lch",
    feature = "space-okhsl",
    feature = "space-okhsv",
    feature = "space-okhwb",
    feature = "space-hsl",
    feature = "space-hsv",
    feature = "space-hwb"
  ))]
  fn with_hue_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_hue(self.hue() * factor.into().0)
  }

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

  /// Returns a new color with the given opacity percentage (0-100%).
  fn with_opacity(&self, opacity: impl Into<Component>) -> Self {
    self.with_alpha(opacity.into() / 100.0)
  }

  /// Returns a new color with opacity decreased by the given percentage amount.
  fn with_opacity_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_alpha_decremented_by(amount.into() / 100.0)
  }

  /// Alias for [`Self::with_alpha_flattened`].
  fn with_opacity_flattened(&self) -> Self {
    self.with_alpha_flattened()
  }

  /// Alias for [`Self::with_alpha_flattened_against`].
  fn with_opacity_flattened_against(&self, background: impl Into<Rgb<Srgb>>) -> Self {
    self.with_alpha_flattened_against(background)
  }

  /// Returns a new color with opacity increased by the given percentage amount.
  fn with_opacity_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_alpha_incremented_by(amount.into() / 100.0)
  }

  /// Returns a new color with opacity scaled by the given percentage factor.
  fn with_opacity_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_alpha_scaled_by(factor.into() / 100.0)
  }

  /// Returns the sRGB yellow component as a percentage (0-100%).
  #[cfg(feature = "space-cmyk")]
  fn yellow(&self) -> f64 {
    self.to_cmyk().yellow()
  }
}
