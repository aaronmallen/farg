use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct Aces2065_1;

impl RgbSpec for Aces2065_1 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "ACES 2065-1";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.7347, 0.2653),
    Xy::new_const(0.0000, 1.0000),
    Xy::new_const(0.0001, -0.0770),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
