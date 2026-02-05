use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct PalSecam;

impl RgbSpec for PalSecam {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "PAL/SECAM";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.640, 0.330),
    Xy::new_const(0.290, 0.600),
    Xy::new_const(0.150, 0.060),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Bt709;
}
