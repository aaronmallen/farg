use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  pub const CMC_CAT2000: Self = Self::new(
    "CMC CAT2000",
    [
      [0.7982, 0.3389, -0.1371],
      [-0.5918, 1.5512, 0.0406],
      [0.0008, 0.0239, 0.9753],
    ],
  );
  #[cfg(all(
    not(feature = "cat-bradford"),
    not(feature = "cat-cat16"),
    not(feature = "cat-cat02"),
  ))]
  pub const DEFAULT: Self = Self::CMC_CAT2000;
}
