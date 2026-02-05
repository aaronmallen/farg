use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct EktaSpacePs5;

impl RgbSpec for EktaSpacePs5 {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "EktaSpace PS5";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.6950, 0.3050),
    Xy::new_const(0.2600, 0.7000),
    Xy::new_const(0.1100, 0.0050),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Gamma(2.2);
}
