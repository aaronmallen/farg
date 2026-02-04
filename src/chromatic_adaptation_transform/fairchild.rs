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
  ))]
  pub const DEFAULT: Self = Self::FAIRCHILD;
  pub const FAIRCHILD: Self = Self::new(
    "Fairchild",
    [
      [0.8562, 0.3372, -0.1934],
      [-0.8360, 1.8327, 0.0033],
      [0.0357, -0.0469, 1.0112],
    ],
  );
}
