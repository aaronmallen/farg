use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  #[cfg(all(
    not(feature = "cat-bradford"),
    not(feature = "cat-cat16"),
    not(feature = "cat-cat02"),
    not(feature = "cat-cmc-cat2000"),
    not(feature = "cat-von-kries"),
  ))]
  pub const DEFAULT: Self = Self::HUNT_POINTER_ESTEVEZ;
  pub const HPE: Self = Self::HUNT_POINTER_ESTEVEZ;
  pub const HUNT_POINTER_ESTEVEZ: Self = Self::new(
    "Hunt-Pointer-Estevez",
    [
      [0.38971, 0.68898, -0.07868],
      [-0.22981, 1.18340, 0.04641],
      [0.00000, 0.00000, 1.00000],
    ],
  );
}
