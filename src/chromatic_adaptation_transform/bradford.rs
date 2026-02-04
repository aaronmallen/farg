use super::ChromaticAdaptationTransform;

impl ChromaticAdaptationTransform {
  pub const BRADFORD: Self = Self::new(
    "Bradford",
    [
      [0.8951, 0.2664, -0.1614],
      [-0.7502, 1.7135, 0.0367],
      [0.0389, -0.0685, 1.0296],
    ],
  );
  pub const DEFAULT: Self = Self::BRADFORD;
}
