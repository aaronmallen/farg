use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct AppleRgb;

impl RgbSpec for AppleRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Apple RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.625, 0.340),
    Xy::new_const(0.280, 0.595),
    Xy::new_const(0.155, 0.070),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Gamma(1.8);
}
