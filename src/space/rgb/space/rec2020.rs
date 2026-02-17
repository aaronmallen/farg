use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct Rec2020;

impl RgbSpec for Rec2020 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Rec. 2020";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.708, 0.292),
    Xy::new_const(0.170, 0.797),
    Xy::new_const(0.131, 0.046),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Bt709;
}

impl super::super::Rgb<Rec2020> {
  /// Returns this color as a CSS Color Level 4 `color(rec2020 ...)` string.
  ///
  /// Components are normalized 0-1 decimal values. Alpha is appended only
  /// when less than 1.0.
  ///
  /// ```
  /// use farg::space::{ColorSpace, Rec2020, Rgb};
  ///
  /// let color = Rgb::<Rec2020>::from_normalized(0.5, 0.3, 0.2);
  /// assert_eq!(color.to_css(), "color(rec2020 0.5 0.3 0.2)");
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
        "color(rec2020 {} {} {} / {})",
        f(self.r()),
        f(self.g()),
        f(self.b()),
        f(a)
      )
    } else {
      format!("color(rec2020 {} {} {})", f(self.r()), f(self.g()), f(self.b()))
    }
  }
}
