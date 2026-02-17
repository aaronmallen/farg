use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

/// The standard RGB (sRGB) color space specification (IEC 61966-2-1).
///
/// Uses D65 illuminant, CIE 1931 2 degree observer, and the sRGB transfer function.
#[derive(Clone, Copy, Debug)]
pub struct Srgb;

impl RgbSpec for Srgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "sRGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.64, 0.33),
    Xy::new_const(0.30, 0.60),
    Xy::new_const(0.15, 0.06),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Srgb;
}

impl super::super::Rgb<Srgb> {
  /// Returns this color as a CSS Color Level 4 `rgb(...)` string.
  ///
  /// Uses space-separated modern syntax with integer 0-255 channel values.
  /// Alpha is appended only when less than 1.0.
  ///
  /// ```
  /// use farg::space::{ColorSpace, Rgb, Srgb};
  ///
  /// let color = Rgb::<Srgb>::new(255, 87, 51);
  /// assert_eq!(color.to_css(), "rgb(255 87 51)");
  ///
  /// let translucent = color.with_alpha(0.5);
  /// assert_eq!(translucent.to_css(), "rgb(255 87 51 / 0.5)");
  /// ```
  pub fn to_css(&self) -> String {
    let a = self.alpha.0;
    if a < 1.0 {
      let a = format!("{:.6}", a)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();
      format!("rgb({} {} {} / {})", self.red(), self.green(), self.blue(), a)
    } else {
      format!("rgb({} {} {})", self.red(), self.green(), self.blue())
    }
  }
}
