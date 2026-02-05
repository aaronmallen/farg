use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct Srgb;

impl RgbSpec for Srgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "sRGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.64, 0.33),
    Xy::new_const(0.30, 0.60),
    Xy::new_const(0.15, 0.06),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Srgb;
}
