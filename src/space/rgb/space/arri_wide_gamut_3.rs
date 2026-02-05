use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct ArriWideGamut3;

impl RgbSpec for ArriWideGamut3 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "ARRI Wide Gamut 3";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.6840, 0.3130),
    Xy::new_const(0.2210, 0.8480),
    Xy::new_const(0.0861, -0.1020),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
