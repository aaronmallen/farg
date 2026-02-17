use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct LinearSrgb;

impl RgbSpec for LinearSrgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Linear sRGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.64, 0.33),
    Xy::new_const(0.30, 0.60),
    Xy::new_const(0.15, 0.06),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}

impl super::super::Rgb<LinearSrgb> {
  /// Returns this color as a CSS Color Level 4 `color(srgb-linear ...)` string.
  ///
  /// Components are normalized 0-1 decimal values. Alpha is appended only
  /// when less than 1.0.
  ///
  /// ```
  /// use farg::space::{ColorSpace, LinearSrgb, Rgb};
  ///
  /// let color = Rgb::<LinearSrgb>::from_normalized(0.5, 0.3, 0.2);
  /// assert_eq!(color.to_css(), "color(srgb-linear 0.5 0.3 0.2)");
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
        "color(srgb-linear {} {} {} / {})",
        f(self.r()),
        f(self.g()),
        f(self.b()),
        f(a)
      )
    } else {
      format!("color(srgb-linear {} {} {})", f(self.r()), f(self.g()), f(self.b()))
    }
  }
}
