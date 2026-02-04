use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  pub const CAT16: Self = Self::new(
    "CAT16",
    [
      [0.401288, 0.650173, -0.051461],
      [-0.250268, 1.204414, 0.045854],
      [-0.002079, 0.048952, 0.953127],
    ],
  );
  #[cfg(not(feature = "cat-bradford"))]
  pub const DEFAULT: Self = Self::CAT16;
}
