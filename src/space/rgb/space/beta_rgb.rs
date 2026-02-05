use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct BetaRgb;

impl RgbSpec for BetaRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Beta RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.6888, 0.3112),
    Xy::new_const(0.1986, 0.7551),
    Xy::new_const(0.1265, 0.0352),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Gamma(2.2);
}
