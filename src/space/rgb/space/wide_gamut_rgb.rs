use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct WideGamutRgb;

impl RgbSpec for WideGamutRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Wide Gamut RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.7347, 0.2653),
    Xy::new_const(0.1152, 0.8264),
    Xy::new_const(0.1566, 0.0177),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Gamma(2.2);
}
