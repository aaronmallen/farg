use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  /// The default CAT when this is the highest-priority enabled transform.
  #[cfg(all(
    not(feature = "cat-bradford"),
    not(feature = "cat-cat16"),
    not(feature = "cat-cat02"),
    not(feature = "cat-cmc-cat2000"),
    not(feature = "cat-von-kries"),
    not(feature = "cat-hunt-pointer-estevez"),
  ))]
  pub const DEFAULT: Self = Self::SHARP;
  /// The Sharp chromatic adaptation transform (Süsstrunk et al.).
  pub const SHARP: Self = Self::new(
    "Sharp",
    [
      [1.2694, -0.0988, -0.1706],
      [-0.8364, 1.8006, 0.0357],
      [0.0297, -0.0315, 1.0018],
    ],
  );
}
