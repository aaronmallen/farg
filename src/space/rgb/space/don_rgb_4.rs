use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct DonRgb4;

impl RgbSpec for DonRgb4 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Don RGB 4";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.6960, 0.3000),
    Xy::new_const(0.2150, 0.7650),
    Xy::new_const(0.1300, 0.0350),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Gamma(2.2);
}
