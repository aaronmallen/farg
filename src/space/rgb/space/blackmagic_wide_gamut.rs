use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct BlackmagicWideGamut;

impl RgbSpec for BlackmagicWideGamut {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Blackmagic Wide Gamut";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.7177, 0.3171),
    Xy::new_const(0.2280, 0.8616),
    Xy::new_const(0.1006, -0.0820),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
