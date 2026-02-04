use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  pub const CAT02: Self = Self::new(
    "CAT02",
    [
      [0.7328, 0.4296, -0.1624],
      [-0.7036, 1.6975, 0.0061],
      [0.0030, 0.0136, 0.9834],
    ],
  );
  #[cfg(all(not(feature = "cat-bradford"), not(feature = "cat-cat16")))]
  pub const DEFAULT: Self = Self::CAT02;
}
