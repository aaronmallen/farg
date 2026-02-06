use std::sync::OnceLock;

use super::{RgbPrimaries, TransferFunction};
use crate::{ColorimetricContext, matrix::Matrix3};

/// Defines the characteristics of an RGB color space.
///
/// Each RGB space specifies its viewing context, display name, primary chromaticities,
/// and transfer function (gamma curve). The XYZ conversion matrices are computed
/// lazily from the primaries and reference white.
pub trait RgbSpec: Clone + Copy + Send + Sync {
  /// The viewing context (illuminant + observer) for this space.
  const CONTEXT: ColorimetricContext;
  /// The display name of this color space (e.g., "sRGB", "Display P3").
  const NAME: &'static str;
  /// The red, green, and blue primary chromaticity coordinates.
  const PRIMARIES: RgbPrimaries;
  /// The electro-optical transfer function (gamma curve).
  const TRANSFER_FUNCTION: TransferFunction;

  /// Returns the cached XYZ-to-RGB matrix (inverse of the RGB-to-XYZ matrix).
  fn inversed_xyz_matrix() -> &'static Matrix3 {
    static MATRIX: OnceLock<Matrix3> = OnceLock::new();
    MATRIX.get_or_init(|| Self::xyz_matrix().inverse())
  }

  /// Returns the cached RGB-to-XYZ matrix, computed from primaries and reference white.
  fn xyz_matrix() -> &'static Matrix3 {
    static MATRIX: OnceLock<Matrix3> = OnceLock::new();
    MATRIX.get_or_init(|| Self::PRIMARIES.calculate_xyz_matrix(Self::CONTEXT.reference_white()))
  }
}
