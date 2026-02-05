use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct AcesCct;

impl RgbSpec for AcesCct {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "ACEScct";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.713, 0.293),
    Xy::new_const(0.165, 0.830),
    Xy::new_const(0.128, 0.044),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
