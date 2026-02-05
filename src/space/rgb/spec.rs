use std::sync::OnceLock;

use super::{RgbPrimaries, TransferFunction};
use crate::{ColorimetricContext, matrix::Matrix3};

pub trait RgbSpec: Clone + Copy + Send + Sync {
  const CONTEXT: ColorimetricContext;
  const NAME: &'static str;
  const PRIMARIES: RgbPrimaries;
  const TRANSFER_FUNCTION: TransferFunction;

  fn inversed_xyz_matrix() -> &'static Matrix3 {
    static MATRIX: OnceLock<Matrix3> = OnceLock::new();
    MATRIX.get_or_init(|| Self::xyz_matrix().inverse())
  }

  fn xyz_matrix() -> &'static Matrix3 {
    static MATRIX: OnceLock<Matrix3> = OnceLock::new();
    MATRIX.get_or_init(|| Self::PRIMARIES.calculate_xyz_matrix(Self::CONTEXT.reference_white()))
  }
}
