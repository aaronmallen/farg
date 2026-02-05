use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct PanasonicVGamut;

impl RgbSpec for PanasonicVGamut {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Panasonic V-Gamut";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.730, 0.280),
    Xy::new_const(0.165, 0.840),
    Xy::new_const(0.100, -0.030),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Linear;
}
