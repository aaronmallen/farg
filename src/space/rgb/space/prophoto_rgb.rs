use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct ProPhotoRgb;

impl RgbSpec for ProPhotoRgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "ProPhoto RGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.734699, 0.265301),
    Xy::new_const(0.159597, 0.840403),
    Xy::new_const(0.036598, 0.000105),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::ProPhotoRgb;
}
