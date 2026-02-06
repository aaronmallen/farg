#[cfg(feature = "cat-bradford")]
mod bradford;
#[cfg(feature = "cat-cat02")]
mod cat02;
#[cfg(feature = "cat-cat16")]
mod cat16;
#[cfg(feature = "cat-cmc-cat2000")]
mod cmc_cat2000;
#[cfg(feature = "cat-cmc-cat97")]
mod cmc_cat97;
#[cfg(feature = "cat-fairchild")]
mod fairchild;
#[cfg(feature = "cat-hunt-pointer-estevez")]
mod hunt_pointer_estevez;
#[cfg(feature = "cat-sharp")]
mod sharp;
#[cfg(feature = "cat-von-kries")]
mod von_kries;
mod xyz_scaling;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
  matrix::Matrix3,
  space::{Lms, Xyz},
};

/// Shorthand alias for [`ChromaticAdaptationTransform`].
pub type Cat = ChromaticAdaptationTransform;

/// A 3x3 matrix transform for adapting colors between different illuminant white points.
///
/// Chromatic adaptation transforms (CATs) model how the human visual system adjusts
/// to changes in illumination. Each transform defines a matrix that converts XYZ tristimulus
/// values into a cone-response-like space where adaptation scaling is applied.
#[derive(Clone, Copy, Debug)]
pub struct ChromaticAdaptationTransform {
  inverse: Matrix3,
  matrix: Matrix3,
  name: &'static str,
}

impl ChromaticAdaptationTransform {
  /// Creates a new chromatic adaptation transform from a name and 3x3 matrix.
  pub const fn new(name: &'static str, matrix: [[f64; 3]; 3]) -> Self {
    let matrix = Matrix3::new(matrix);

    Self {
      inverse: matrix.inverse(),
      matrix,
      name,
    }
  }

  /// Adapts a color from one white point to another.
  ///
  /// Converts the color to a cone-response space using this transform's matrix,
  /// scales each cone channel by the ratio of target to reference white, then
  /// converts back to XYZ.
  pub fn adapt(&self, color: impl Into<Xyz>, reference_white: impl Into<Xyz>, target_white: impl Into<Xyz>) -> Xyz {
    let color = color.into();
    let reference_white = reference_white.into();
    let target_white = target_white.into();

    let lms = color
      .with_context(color.context().with_cat(*self))
      .to_lms()
      .components();
    let target_lms = target_white
      .with_context(target_white.context().with_cat(*self))
      .to_lms()
      .components();
    let reference_lms = reference_white
      .with_context(reference_white.context().with_cat(*self))
      .to_lms()
      .components();

    Lms::from([
      lms[0] * (target_lms[0] / reference_lms[0]),
      lms[1] * (target_lms[1] / reference_lms[1]),
      lms[2] * (target_lms[2] / reference_lms[2]),
    ])
    .to_xyz()
    .with_context(target_white.context().with_cat(*self))
  }

  /// Returns the inverse of the transformation matrix.
  pub fn inverse(&self) -> Matrix3 {
    self.inverse
  }

  /// Returns the transformation matrix.
  pub fn matrix(&self) -> Matrix3 {
    self.matrix
  }

  /// Returns the name of this transform (e.g., "Bradford", "CAT16").
  pub fn name(&self) -> &'static str {
    self.name
  }
}

impl Display for ChromaticAdaptationTransform {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "{} {:.precision$}",
      self.name,
      self.matrix,
      precision = f.precision().unwrap_or(4)
    )
  }
}

impl Default for ChromaticAdaptationTransform {
  fn default() -> Self {
    Self::DEFAULT
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod adapt {
    use super::*;

    #[test]
    fn it_adapts_color_from_one_white_point_to_another() {
      let cat = Cat::default();
      let d65 = Xyz::new(0.95047, 1.0, 1.08883);
      let d50 = Xyz::new(0.96422, 1.0, 0.82521);
      let color = Xyz::new(0.4, 0.2, 0.1);
      let adapted = cat.adapt(color, d65, d50);

      assert!((adapted.x() - color.x()).abs() > 0.001);
      assert!((adapted.z() - color.z()).abs() > 0.001);
      assert!((adapted.y() - color.y()).abs() < 0.1);
    }

    #[test]
    fn it_returns_same_color_when_adapting_to_same_white_point() {
      let cat = Cat::default();
      let white = Xyz::new(0.95047, 1.0, 1.08883);
      let color = Xyz::new(0.4, 0.2, 0.1);
      let adapted = cat.adapt(color, white, white);

      assert!((adapted.x() - color.x()).abs() < 1e-10);
      assert!((adapted.y() - color.y()).abs() < 1e-10);
      assert!((adapted.z() - color.z()).abs() < 1e-10);
    }
  }

  mod default {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_default_cat() {
      let cat = Cat::default();

      assert_eq!(cat.name(), Cat::DEFAULT.name());
    }
  }

  mod display {
    use super::*;

    #[test]
    fn it_formats_with_name_and_matrix() {
      let cat = Cat::XYZ_SCALING;
      let output = format!("{}", cat);

      assert!(output.starts_with("XYZ Scaling"));
      assert!(output.contains("["));
    }

    #[test]
    fn it_respects_precision() {
      let cat = Cat::XYZ_SCALING;
      let output = format!("{:.2}", cat);

      assert!(output.contains("1.00"));
    }
  }

  mod inverse {
    use super::*;

    #[test]
    fn it_returns_inverse_matrix() {
      let cat = Cat::XYZ_SCALING;
      let matrix = cat.matrix();
      let inverse = cat.inverse();
      let result = matrix * inverse;
      let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

      for i in 0..3 {
        for j in 0..3 {
          assert!((result.data()[i][j] - identity[i][j]).abs() < 1e-10);
        }
      }
    }
  }

  mod matrix {
    use super::*;

    #[test]
    fn it_returns_the_transformation_matrix() {
      let cat = Cat::XYZ_SCALING;
      let matrix = cat.matrix();

      assert_eq!(matrix.data()[0][0], 1.0);
      assert_eq!(matrix.data()[1][1], 1.0);
      assert_eq!(matrix.data()[2][2], 1.0);
    }
  }

  mod name {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_cat_name() {
      let cat = Cat::XYZ_SCALING;

      assert_eq!(cat.name(), "XYZ Scaling");
    }
  }
}
