use super::{Cmf, Table};
use crate::{chromaticity::Xy, space::Xyz};

/// Spectral locus chromaticity coordinates derived from color matching functions.
#[derive(Clone, Copy, Debug)]
pub struct ChromaticityCoordinates(&'static [(u32, Xy)]);

impl ChromaticityCoordinates {
  /// Creates new chromaticity coordinates from static wavelength-coordinate pairs.
  pub const fn new(table: &'static [(u32, Xy)]) -> Self {
    Self(table)
  }

  /// Tests whether a chromaticity point lies inside the spectral locus.
  ///
  /// Uses the ray-casting (point-in-polygon) algorithm.
  pub fn contains_chromaticity(&self, chromaticity: impl Into<Xy>) -> bool {
    let [x, y] = chromaticity.into().components();

    let locus_points: Vec<[f64; 2]> = self.values().map(|v| v.components()).collect();

    if locus_points.len() < 3 {
      return false;
    }

    let mut inside = false;
    let mut j = locus_points.len() - 1;

    for i in 0..locus_points.len() {
      let [xi, yi] = locus_points[i];
      let [xj, yj] = locus_points[j];

      if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
        inside = !inside;
      }

      j = i;
    }

    inside
  }
}

impl From<Cmf> for ChromaticityCoordinates {
  fn from(cmf: Cmf) -> Self {
    let data: Box<[(u32, Xy)]> = cmf
      .table()
      .iter()
      .map(|(wavelength, xyz)| (*wavelength, Xy::from(Xyz::from(xyz.components()))))
      .collect();
    Self::new(Box::leak(data))
  }
}

impl Table for ChromaticityCoordinates {
  type Value = Xy;

  fn table(&self) -> &[(u32, Self::Value)] {
    self.0
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::spectral::TristimulusResponse;

  static TRIANGLE_LOCUS: &[(u32, Xy)] = &[
    (380, Xy::new_const(0.0, 0.0)),
    (520, Xy::new_const(1.0, 0.0)),
    (600, Xy::new_const(0.5, 1.0)),
  ];

  mod contains_chromaticity {
    use super::*;

    #[test]
    fn it_returns_true_for_point_inside_locus() {
      let coords = ChromaticityCoordinates::new(TRIANGLE_LOCUS);

      assert!(coords.contains_chromaticity(Xy::new(0.5, 0.3)));
    }

    #[test]
    fn it_returns_false_for_point_outside_locus() {
      let coords = ChromaticityCoordinates::new(TRIANGLE_LOCUS);

      assert!(!coords.contains_chromaticity(Xy::new(1.5, 0.5)));
    }

    #[test]
    fn it_returns_false_for_fewer_than_three_points() {
      static TWO_POINTS: &[(u32, Xy)] = &[(380, Xy::new_const(0.0, 0.0)), (520, Xy::new_const(1.0, 0.0))];
      let coords = ChromaticityCoordinates::new(TWO_POINTS);

      assert!(!coords.contains_chromaticity(Xy::new(0.5, 0.0)));
    }

    #[test]
    fn it_accepts_array_as_chromaticity() {
      let coords = ChromaticityCoordinates::new(TRIANGLE_LOCUS);

      assert!(coords.contains_chromaticity([0.5, 0.3]));
    }
  }

  mod from_cmf {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_cmf_to_chromaticity_coordinates() {
      static CMF: &[(u32, TristimulusResponse)] = &[
        (380, TristimulusResponse::new(0.1, 0.2, 0.3)),
        (400, TristimulusResponse::new(0.2, 0.3, 0.4)),
      ];
      let cmf = Cmf::new(CMF);
      let coords = ChromaticityCoordinates::from(cmf);

      assert_eq!(coords.len(), 2);
      assert_eq!(coords.min_wavelength(), Some(380));
      assert_eq!(coords.max_wavelength(), Some(400));
    }

    #[test]
    fn it_computes_chromaticity_from_xyz() {
      static CMF: &[(u32, TristimulusResponse)] = &[
        (550, TristimulusResponse::new(0.5, 0.5, 0.5)),
        (600, TristimulusResponse::new(0.6, 0.3, 0.1)),
      ];
      let cmf = Cmf::new(CMF);
      let coords = ChromaticityCoordinates::from(cmf);

      let first = coords.at(550).unwrap();
      let sum = 0.5 + 0.5 + 0.5;

      assert_eq!(first.x(), 0.5 / sum);
      assert_eq!(first.y(), 0.5 / sum);

      let second = coords.at(600).unwrap();
      let sum2 = 0.6 + 0.3 + 0.1;

      assert_eq!(second.x(), 0.6 / sum2);
      assert_eq!(second.y(), 0.3 / sum2);
    }
  }
}
