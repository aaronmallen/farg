use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct DaVinciWideGamut;

impl RgbSpec for DaVinciWideGamut {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "DaVinci Wide Gamut";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.8000, 0.3130),
    Xy::new_const(0.1682, 0.9877),
    Xy::new_const(0.0790, -0.1155),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
