use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct BruceRgb;

impl RgbSpec for BruceRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Bruce RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.6400, 0.3300),
    Xy::new_const(0.2800, 0.6500),
    Xy::new_const(0.1500, 0.0600),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Gamma(2.2);
}
