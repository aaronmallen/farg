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
