use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct DisplayP3;

impl RgbSpec for DisplayP3 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Display P3";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.680, 0.320),
    Xy::new_const(0.265, 0.690),
    Xy::new_const(0.150, 0.060),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Srgb;
}

impl super::super::Rgb<DisplayP3> {
  /// Returns this color as a CSS Color Level 4 `color(display-p3 ...)` string.
  ///
  /// Components are normalized 0-1 decimal values. Alpha is appended only
  /// when less than 1.0.
  ///
  /// ```
  /// use farg::space::{ColorSpace, DisplayP3, Rgb};
  ///
  /// let color = Rgb::<DisplayP3>::from_normalized(0.5, 0.3, 0.2);
  /// assert_eq!(color.to_css(), "color(display-p3 0.5 0.3 0.2)");
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
        "color(display-p3 {} {} {} / {})",
        f(self.r()),
        f(self.g()),
        f(self.b()),
        f(a)
      )
    } else {
      format!("color(display-p3 {} {} {})", f(self.r()), f(self.g()), f(self.b()))
    }
  }
}
