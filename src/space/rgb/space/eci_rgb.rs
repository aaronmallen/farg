use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct EciRgbV2;

impl RgbSpec for EciRgbV2 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "ECI RGB v2";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.6700, 0.3300),
    Xy::new_const(0.2100, 0.7100),
    Xy::new_const(0.1400, 0.0800),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
