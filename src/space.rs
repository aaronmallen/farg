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

  #[cfg(feature = "space-oklch")]
  /// Returns the Oklch chroma channel.
  fn chroma(&self) -> f64 {
    self.to_oklch().chroma()
  }

  #[cfg(all(feature = "space-lch", not(feature = "space-oklch")))]
  /// Returns the CIE LCh chroma channel.
  fn chroma(&self) -> f64 {
    self.to_lch().chroma()
  }

  /// Returns the CIE 1931 xy chromaticity coordinates.
  fn chromaticity(&self) -> Xy {
    self.to_xyz().chromaticity()
  }

  #[cfg(feature = "chromaticity-rg")]
  /// Returns the RGB-relative chromaticity coordinates (r, g).
  fn chromaticity_rg(&self) -> Rg {
    self.chromaticity().to_rg()
  }

  #[cfg(feature = "chromaticity-upvp")]
  /// Returns the CIE 1976 UCS chromaticity coordinates (u', v').
  fn chromaticity_upvp(&self) -> Upvp {
    self.chromaticity().to_upvp()
  }

  #[cfg(feature = "chromaticity-uv")]
  /// Returns the CIE 1960 UCS chromaticity coordinates (u, v).
  fn chromaticity_uv(&self) -> Uv {
    self.chromaticity().to_uv()
  }

  /// Returns the color's components as an array.
  fn components(&self) -> [f64; N];

  #[cfg(feature = "contrast-wcag")]
  /// Returns the WCAG 2.x contrast ratio between this color and another.
  ///
  /// The result is always >= 1.0 and is order-independent. Use
  /// [`ContrastRatio::meets_aa`], [`ContrastRatio::meets_aaa`], etc. to check
  /// against WCAG conformance levels.
  ///
  /// [`ContrastRatio::meets_aa`]: crate::contrast::wcag::ContrastRatio::meets_aa
  /// [`ContrastRatio::meets_aaa`]: crate::contrast::wcag::ContrastRatio::meets_aaa
  fn contrast_ratio(&self, other: impl Into<Xyz>) -> crate::contrast::wcag::ContrastRatio {
    crate::contrast::wcag::contrast_ratio(self.to_xyz(), other)
  }

  #[cfg(feature = "space-cmyk")]
  /// Returns the sRGB cyan component as a percentage (0-100%).
  fn cyan(&self) -> f64 {
    self.to_cmyk().cyan()
  }

  /// Decreases alpha in place by the given amount on a 0.0 to 1.0 scale.
  fn decrement_alpha(&mut self, amount: impl Into<Component>) {
    self.set_alpha(self.with_alpha_decremented_by(amount).alpha())
  }

  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  /// Decreases chroma in place by the given amount.
  fn decrement_chroma(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_chroma_decremented_by(amount).components())
  }

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
  /// Decreases hue in place by the given amount in degrees.
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

  #[cfg(feature = "space-oklch")]
  /// Returns the Oklch hue channel.
  fn hue(&self) -> f64 {
    self.to_oklch().hue()
  }

  #[cfg(all(feature = "space-lch", not(feature = "space-oklch")))]
  /// Returns the CIE LCh hue channel.
  fn hue(&self) -> f64 {
    self.to_lch().hue()
  }

  #[cfg(all(feature = "space-okhsl", not(any(feature = "space-oklch", feature = "space-lch"))))]
  /// Returns the Okhsl hue channel.
  fn hue(&self) -> f64 {
    self.to_okhsl().hue()
  }

  #[cfg(all(
    feature = "space-okhsv",
    not(any(feature = "space-oklch", feature = "space-lch", feature = "space-okhsl"))
  ))]
  /// Returns the Okhsv hue channel.
  fn hue(&self) -> f64 {
    self.to_okhsv().hue()
  }

  #[cfg(all(
    feature = "space-okhwb",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv"
    ))
  ))]
  /// Returns the Okhwb hue channel.
  fn hue(&self) -> f64 {
    self.to_okhwb().hue()
  }

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
  /// Returns the HSL hue channel.
  fn hue(&self) -> f64 {
    self.to_hsl().hue()
  }

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
  /// Returns the HSV hue channel.
  fn hue(&self) -> f64 {
    self.to_hsv().hue()
  }

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
  /// Returns the HWB hue channel.
  fn hue(&self) -> f64 {
    self.to_hwb().hue()
  }

  /// Increases alpha in place by the given amount on a 0.0 to 1.0 scale.
  fn increment_alpha(&mut self, amount: impl Into<Component>) {
    self.set_alpha(self.with_alpha_incremented_by(amount).alpha())
  }

  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  /// Increases chroma in place by the given amount.
  fn increment_chroma(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_chroma_incremented_by(amount).components())
  }

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
  /// Increases hue in place by the given amount in degrees.
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

  #[cfg(feature = "contrast-apca")]
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
  fn lightness_contrast(&self, background: impl Into<Xyz>) -> crate::contrast::apca::LightnessContrast {
    crate::contrast::apca::calculate(self.to_xyz(), background)
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

  /// Returns the opacity as a percentage (0-100%).
  fn opacity(&self) -> f64 {
    self.alpha() * 100.0
  }

  /// Returns the sRGB red channel as a u8 (0-255).
  fn red(&self) -> u8 {
    self.to_rgb::<Srgb>().red()
  }

  /// Scales alpha in place by the given factor.
  fn scale_alpha(&mut self, factor: impl Into<Component>) {
    self.set_alpha(self.with_alpha_scaled_by(factor).alpha())
  }

  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  /// Scales chroma in place by the given factor.
  fn scale_chroma(&mut self, factor: impl Into<Component>) {
    self.set_components(self.with_chroma_scaled_by(factor).components())
  }

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
  /// Scales hue in place by the given factor.
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

  /// Sets the alpha value in place on a 0.0 to 1.0 scale.
  fn set_alpha(&mut self, alpha: impl Into<Component>);

  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  /// Sets the chroma to the given value in place.
  fn set_chroma(&mut self, chroma: impl Into<Component>) {
    self.set_components(self.with_chroma(chroma).components())
  }

  /// Sets the color's components from an array.
  fn set_components(&mut self, components: [impl Into<Component> + Clone; N]);

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
  /// Sets the hue to the given value in degrees in place.
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

  #[cfg(feature = "space-cmy")]
  /// Converts to the CMY color space with sRGB encoding.
  fn to_cmy(&self) -> Cmy<Srgb> {
    self.to_rgb::<Srgb>().to_cmy().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-cmyk")]
  /// Converts to the CMYK color space with sRGB encoding.
  fn to_cmyk(&self) -> Cmyk<Srgb> {
    self.to_rgb::<Srgb>().to_cmyk().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-hsv")]
  /// Converts to the HSB color space with sRGB encoding.
  fn to_hsb(&self) -> Hsb<Srgb> {
    self.to_rgb::<Srgb>().to_hsb().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-hsl")]
  /// Converts to the HSL color space with sRGB encoding.
  fn to_hsl(&self) -> Hsl<Srgb> {
    self.to_rgb::<Srgb>().to_hsl().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-hsv")]
  /// Converts to the HSV color space with sRGB encoding.
  fn to_hsv(&self) -> Hsv<Srgb> {
    self.to_rgb::<Srgb>().to_hsv().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-hwb")]
  /// Converts to the HWB color space with sRGB encoding.
  fn to_hwb(&self) -> Hwb<Srgb> {
    self.to_rgb::<Srgb>().to_hwb().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-lab")]
  /// Converts to the CIE L*a*b* color space.
  fn to_lab(&self) -> Lab {
    Lab::from(self.to_xyz()).with_alpha(self.alpha())
  }

  #[cfg(feature = "space-luv")]
  /// Converts to the CIE L*u*v* color space.
  fn to_luv(&self) -> Luv {
    Luv::from(self.to_xyz()).with_alpha(self.alpha())
  }

  #[cfg(feature = "space-lch")]
  /// Converts to the CIE LCh color space (cylindrical form of L*a*b*).
  fn to_lch(&self) -> Lch {
    self.to_lab().to_lch().with_alpha(self.alpha())
  }

  /// Converts to the LMS cone response space.
  fn to_lms(&self) -> Lms {
    self.to_xyz().to_lms().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-okhsl")]
  /// Converts to the Okhsl perceptual color space.
  fn to_okhsl(&self) -> Okhsl {
    self.to_oklab().to_okhsl().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-okhsv")]
  /// Converts to the Okhsv perceptual color space.
  fn to_okhsv(&self) -> Okhsv {
    self.to_oklab().to_okhsv().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-okhwb")]
  /// Converts to the Okhwb perceptual color space.
  fn to_okhwb(&self) -> Okhwb {
    self.to_okhsv().to_okhwb().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-oklab")]
  /// Converts to the Oklab perceptual color space.
  fn to_oklab(&self) -> Oklab {
    self.to_xyz().to_oklab().with_alpha(self.alpha())
  }

  #[cfg(feature = "space-oklch")]
  /// Converts to the Oklch perceptual color space.
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

  #[cfg(feature = "space-oklch")]
  /// Returns a new color with the given Oklch chroma.
  fn with_chroma(&self, chroma: impl Into<Component>) -> Self {
    Self::from(self.to_oklch().with_chroma(chroma).to_xyz())
  }

  #[cfg(all(feature = "space-lch", not(feature = "space-oklch")))]
  /// Returns a new color with the given CIE LCh chroma.
  fn with_chroma(&self, chroma: impl Into<Component>) -> Self {
    Self::from(self.to_lch().with_chroma(chroma).to_xyz())
  }

  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  /// Returns a new color with chroma decreased by the given amount.
  fn with_chroma_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_chroma(self.chroma() - amount.into().0)
  }

  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  /// Returns a new color with chroma increased by the given amount.
  fn with_chroma_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_chroma(self.chroma() + amount.into().0)
  }

  #[cfg(any(feature = "space-oklch", feature = "space-lch"))]
  /// Returns a new color with chroma scaled by the given factor.
  fn with_chroma_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_chroma(self.chroma() * factor.into().0)
  }

  #[cfg(feature = "space-oklch")]
  /// Returns a new color with the given Oklch hue in degrees.
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_oklch().with_hue(hue).to_xyz())
  }

  #[cfg(all(feature = "space-lch", not(feature = "space-oklch")))]
  /// Returns a new color with the given CIE LCh hue in degrees.
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_lch().with_hue(hue).to_xyz())
  }

  #[cfg(all(feature = "space-okhsl", not(any(feature = "space-oklch", feature = "space-lch"))))]
  /// Returns a new color with the given Okhsl hue in degrees.
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_okhsl().with_hue(hue).to_xyz())
  }

  #[cfg(all(
    feature = "space-okhsv",
    not(any(feature = "space-oklch", feature = "space-lch", feature = "space-okhsl"))
  ))]
  /// Returns a new color with the given Okhsv hue in degrees.
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_okhsv().with_hue(hue).to_xyz())
  }

  #[cfg(all(
    feature = "space-okhwb",
    not(any(
      feature = "space-oklch",
      feature = "space-lch",
      feature = "space-okhsl",
      feature = "space-okhsv"
    ))
  ))]
  /// Returns a new color with the given Okhwb hue in degrees.
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_okhwb().with_hue(hue).to_xyz())
  }

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
  /// Returns a new color with the given HSL hue in degrees.
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_hsl().with_hue(hue).to_xyz())
  }

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
  /// Returns a new color with the given HSV hue in degrees.
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_hsv().with_hue(hue).to_xyz())
  }

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
  /// Returns a new color with the given HWB hue in degrees.
  fn with_hue(&self, hue: impl Into<Component>) -> Self {
    Self::from(self.to_hwb().with_hue(hue).to_xyz())
  }

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
  /// Returns a new color with hue decreased by the given amount in degrees.
  fn with_hue_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_hue(self.hue() - amount.into().0)
  }

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
  /// Returns a new color with hue increased by the given amount in degrees.
  fn with_hue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_hue(self.hue() + amount.into().0)
  }

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
  /// Returns a new color with hue scaled by the given factor.
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

  #[cfg(feature = "space-cmyk")]
  /// Returns the sRGB yellow component as a percentage (0-100%).
  fn yellow(&self) -> f64 {
    self.to_cmyk().yellow()
  }
}
