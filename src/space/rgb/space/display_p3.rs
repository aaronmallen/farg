use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct DisplayP3;

impl RgbSpec for DisplayP3 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Display P3";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.680, 0.320),
    Xy::new_const(0.265, 0.690),
    Xy::new_const(0.150, 0.060),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Srgb;
}
