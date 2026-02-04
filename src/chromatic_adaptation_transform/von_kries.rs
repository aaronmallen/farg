use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  #[cfg(all(
    not(feature = "cat-bradford"),
    not(feature = "cat-cat16"),
    not(feature = "cat-cat02"),
    not(feature = "cat-cmc-cat2000"),
  ))]
  pub const DEFAULT: Self = Self::VON_KRIES;
  pub const VON_KRIES: Self = Self::new(
    "Von Kries",
    [
      [0.4002400, 0.7076000, -0.0808100],
      [-0.2263000, 1.1653200, 0.0457000],
      [0.0000000, 0.0000000, 0.9182200],
    ],
  );
}
