use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct ArriWideGamut4;

impl RgbSpec for ArriWideGamut4 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "ARRI Wide Gamut 4";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.7347, 0.2653),
    Xy::new_const(0.1424, 0.8576),
    Xy::new_const(0.0991, -0.0308),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
