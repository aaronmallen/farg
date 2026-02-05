use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct SonySGamut3Cine;

impl RgbSpec for SonySGamut3Cine {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Sony S-Gamut3.Cine";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.766, 0.275),
    Xy::new_const(0.225, 0.800),
    Xy::new_const(0.089, -0.087),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
