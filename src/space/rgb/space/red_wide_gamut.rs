use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct RedWideGamutRgb;

impl RgbSpec for RedWideGamutRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "RED Wide Gamut RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.780308, 0.304253),
    Xy::new_const(0.121595, 1.493994),
    Xy::new_const(0.095612, -0.084589),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
