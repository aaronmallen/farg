use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct Rec709;

impl RgbSpec for Rec709 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Rec. 709";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.640, 0.330),
    Xy::new_const(0.300, 0.600),
    Xy::new_const(0.150, 0.060),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Bt709;
}
