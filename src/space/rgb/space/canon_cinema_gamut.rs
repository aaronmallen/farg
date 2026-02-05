use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct CanonCinemaGamut;

impl RgbSpec for CanonCinemaGamut {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Canon Cinema Gamut";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.740, 0.270),
    Xy::new_const(0.170, 1.140),
    Xy::new_const(0.080, -0.100),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
