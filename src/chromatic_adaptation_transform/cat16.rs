use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  /// The CAT16 chromatic adaptation transform from the CAM16 color appearance model.
  pub const CAT16: Self = Self::new(
    "CAT16",
    [
      [0.401288, 0.650173, -0.051461],
      [-0.250268, 1.204414, 0.045854],
      [-0.002079, 0.048952, 0.953127],
    ],
  );
  /// The default CAT when this is the highest-priority enabled transform.
  #[cfg(not(feature = "cat-bradford"))]
  pub const DEFAULT: Self = Self::CAT16;
}
