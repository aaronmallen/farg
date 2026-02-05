use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct CieRgb;

impl RgbSpec for CieRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::E)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "CIE RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.7347, 0.2653),
    Xy::new_const(0.2738, 0.7174),
    Xy::new_const(0.1666, 0.0089),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
