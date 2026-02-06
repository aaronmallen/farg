use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  /// The CMC CAT97 chromatic adaptation transform.
  pub const CMC_CAT97: Self = Self::new(
    "CMC CAT97",
    [
      [0.8951, 0.2664, -0.1614],
      [-0.7502, 1.7135, 0.0367],
      [0.0389, -0.0685, 1.0296],
    ],
  );
  /// The default CAT when this is the highest-priority enabled transform.
  #[cfg(all(
    not(feature = "cat-bradford"),
    not(feature = "cat-cat16"),
    not(feature = "cat-cat02"),
    not(feature = "cat-cmc-cat2000"),
    not(feature = "cat-von-kries"),
    not(feature = "cat-hunt-pointer-estevez"),
    not(feature = "cat-sharp"),
    not(feature = "cat-fairchild"),
  ))]
  pub const DEFAULT: Self = Self::CMC_CAT97;
}
