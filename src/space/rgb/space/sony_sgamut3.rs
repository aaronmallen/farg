use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct SonySGamut3;

impl RgbSpec for SonySGamut3 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Sony S-Gamut3";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.730, 0.280),
    Xy::new_const(0.140, 0.855),
    Xy::new_const(0.100, -0.050),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
