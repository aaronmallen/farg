use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct ColorMatchRgb;

impl RgbSpec for ColorMatchRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "ColorMatch RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.630, 0.340),
    Xy::new_const(0.295, 0.605),
    Xy::new_const(0.150, 0.075),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Gamma(1.8);
}
