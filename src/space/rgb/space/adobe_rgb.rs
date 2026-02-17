use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct AdobeRgb;

impl RgbSpec for AdobeRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Adobe RGB (1998)";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.6400, 0.3300),
    Xy::new_const(0.2100, 0.7100),
    Xy::new_const(0.1500, 0.0600),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Gamma(2.19921875);
}

impl super::super::Rgb<AdobeRgb> {
  /// Returns this color as a CSS Color Level 4 `color(a98-rgb ...)` string.
  ///
  /// Components are normalized 0-1 decimal values. Alpha is appended only
  /// when less than 1.0.
  ///
  /// ```
  /// use farg::space::{AdobeRgb, ColorSpace, Rgb};
  ///
  /// let color = Rgb::<AdobeRgb>::from_normalized(0.5, 0.3, 0.2);
  /// assert_eq!(color.to_css(), "color(a98-rgb 0.5 0.3 0.2)");
  /// ```
  pub fn to_css(&self) -> String {
    fn f(v: f64) -> String {
      format!("{:.6}", v)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
    }

    let a = self.alpha.0;
    if a < 1.0 {
      format!(
        "color(a98-rgb {} {} {} / {})",
        f(self.r()),
        f(self.g()),
        f(self.b()),
        f(a)
      )
    } else {
      format!("color(a98-rgb {} {} {})", f(self.r()), f(self.g()), f(self.b()))
    }
  }
}
