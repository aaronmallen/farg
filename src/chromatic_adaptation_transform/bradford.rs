use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  /// The Bradford chromatic adaptation transform.
  ///
  /// Widely considered the best general-purpose CAT. Used as the default.
  pub const BRADFORD: Self = Self::new(
    "Bradford",
    [
      [0.8951, 0.2664, -0.1614],
      [-0.7502, 1.7135, 0.0367],
      [0.0389, -0.0685, 1.0296],
    ],
  );
  /// The default CAT (Bradford when the `cat-bradford` feature is enabled).
  pub const DEFAULT: Self = Self::BRADFORD;
}
