use crate::{
  ColorimetricContext, Illuminant, Observer,
  chromaticity::Xy,
  space::rgb::{RgbPrimaries, RgbSpec, TransferFunction},
};

#[derive(Clone, Copy, Debug)]
pub struct Rec2100Pq;

impl RgbSpec for Rec2100Pq {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "Rec. 2100 PQ";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.708, 0.292),
    Xy::new_const(0.170, 0.797),
    Xy::new_const(0.131, 0.046),
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Pq;
}
