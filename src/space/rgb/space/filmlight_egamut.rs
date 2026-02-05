use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct FilmlightEGamut;

impl RgbSpec for FilmlightEGamut {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Filmlight E-Gamut";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.8000, 0.3177),
    Xy::new_const(0.1800, 0.9000),
    Xy::new_const(0.0650, -0.0805),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
