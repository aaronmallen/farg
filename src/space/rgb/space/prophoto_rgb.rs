use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct ProPhotoRgb;

impl RgbSpec for ProPhotoRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "ProPhoto RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.734699, 0.265301),
    Xy::new_const(0.159597, 0.840403),
    Xy::new_const(0.036598, 0.000105),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::ProPhotoRgb;
}

impl super::super::Rgb<ProPhotoRgb> {
  /// Returns this color as a CSS Color Level 4 `color(prophoto-rgb ...)` string.
  ///
  /// Components are normalized 0-1 decimal values. Alpha is appended only
  /// when less than 1.0.
  ///
  /// ```
  /// use farg::space::{ColorSpace, ProPhotoRgb, Rgb};
  ///
  /// let color = Rgb::<ProPhotoRgb>::from_normalized(0.5, 0.3, 0.2);
  /// assert_eq!(color.to_css(), "color(prophoto-rgb 0.5 0.3 0.2)");
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
        "color(prophoto-rgb {} {} {} / {})",
        f(self.r()),
        f(self.g()),
        f(self.b()),
        f(a)
      )
    } else {
      format!("color(prophoto-rgb {} {} {})", f(self.r()), f(self.g()), f(self.b()))
    }
  }
}
