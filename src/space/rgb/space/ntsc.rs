use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct Ntsc;

impl RgbSpec for Ntsc {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::C)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "NTSC (1953)";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.67, 0.33),
    Xy::new_const(0.21, 0.71),
    Xy::new_const(0.14, 0.08),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Bt709;
}
