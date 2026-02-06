use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  /// The CAT02 chromatic adaptation transform from the CIECAM02 color appearance model.
  pub const CAT02: Self = Self::new(
    "CAT02",
    [
      [0.7328, 0.4296, -0.1624],
      [-0.7036, 1.6975, 0.0061],
      [0.0030, 0.0136, 0.9834],
    ],
  );
  /// The default CAT when this is the highest-priority enabled transform.
  #[cfg(all(not(feature = "cat-bradford"), not(feature = "cat-cat16")))]
  pub const DEFAULT: Self = Self::CAT02;
}
