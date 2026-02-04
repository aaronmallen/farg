use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  #[cfg(all(
    not(feature = "cat-bradford"),
    not(feature = "cat-cat16"),
    not(feature = "cat-cat02"),
    not(feature = "cat-cmc-cat2000"),
    not(feature = "cat-von-kries"),
    not(feature = "cat-hunt-pointer-estevez"),
    not(feature = "cat-sharp"),
    not(feature = "cat-fairchild"),
    not(feature = "cat-cmc-cat97"),
  ))]
  pub const DEFAULT: Self = Self::XYZ_SCALING;
  pub const XYZ_SCALING: Self = Self::new("XYZ Scaling", [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
}
