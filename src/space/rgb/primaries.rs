use crate::{chromaticity::Xy, matrix::Matrix3, space::Xyz};

/// The red, green, and blue primary chromaticity coordinates defining an RGB gamut.
pub struct RgbPrimaries {
  blue: Xy,
  green: Xy,
  red: Xy,
}

impl RgbPrimaries {
  /// Creates new RGB primaries from red, green, and blue chromaticity coordinates.
  pub fn new(red: impl Into<Xy>, green: impl Into<Xy>, blue: impl Into<Xy>) -> Self {
    Self {
      red: red.into(),
      green: green.into(),
      blue: blue.into(),
    }
  }

  /// Creates new RGB primaries in a const context.
  pub const fn new_const(red: Xy, green: Xy, blue: Xy) -> Self {
    Self {
      red,
      green,
      blue,
    }
  }

  /// Returns the blue primary chromaticity.
  pub fn blue(&self) -> &Xy {
    &self.blue
  }

  /// Computes the 3x3 RGB-to-XYZ matrix for the given reference white point.
  pub fn calculate_xyz_matrix(&self, reference_white: impl Into<Xyz>) -> Matrix3 {
    let reference_white = reference_white.into();

    let r_xyz = self.red.to_xyz(1.0);
    let g_xyz = self.green.to_xyz(1.0);
    let b_xyz = self.blue.to_xyz(1.0);

    let primary = Matrix3::new([
      [r_xyz.x(), g_xyz.x(), b_xyz.x()],
      [r_xyz.y(), g_xyz.y(), b_xyz.y()],
      [r_xyz.z(), g_xyz.z(), b_xyz.z()],
    ]);

    let scaling_vector = primary.inverse() * reference_white.components();
    let scaling = Matrix3::new([
      [scaling_vector[0], 0.0, 0.0],
      [0.0, scaling_vector[1], 0.0],
      [0.0, 0.0, scaling_vector[2]],
    ]);

    primary * scaling
  }

  /// Returns the green primary chromaticity.
  pub fn green(&self) -> &Xy {
    &self.green
  }

  /// Returns the red primary chromaticity.
  pub fn red(&self) -> &Xy {
    &self.red
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{ColorimetricContext, Illuminant, Observer, chromaticity::Xy};

  mod calculate_xyz_matrix {
    use super::*;

    #[test]
    fn it_produces_matrix_that_maps_white_to_white_point() {
      let primaries = RgbPrimaries::new_const(
        Xy::new_const(0.64, 0.33),
        Xy::new_const(0.30, 0.60),
        Xy::new_const(0.15, 0.06),
      );
      let context = ColorimetricContext::new()
        .with_illuminant(Illuminant::D65)
        .with_observer(Observer::CIE_1931_2D);
      let white_point = context.reference_white();
      let matrix = primaries.calculate_xyz_matrix(white_point);

      let result = matrix * [1.0, 1.0, 1.0];

      assert!((result[0] - white_point.x()).abs() < 1e-6);
      assert!((result[1] - white_point.y()).abs() < 1e-6);
      assert!((result[2] - white_point.z()).abs() < 1e-6);
    }

    #[test]
    fn it_maps_red_primary_to_xyz_with_correct_chromaticity() {
      let primaries = RgbPrimaries::new_const(
        Xy::new_const(0.64, 0.33),
        Xy::new_const(0.30, 0.60),
        Xy::new_const(0.15, 0.06),
      );
      let context = ColorimetricContext::new()
        .with_illuminant(Illuminant::D65)
        .with_observer(Observer::CIE_1931_2D);
      let white_point = context.reference_white();
      let matrix = primaries.calculate_xyz_matrix(white_point);

      let [x, y, z] = matrix * [1.0, 0.0, 0.0];
      let sum = x + y + z;
      let chromaticity_x = x / sum;
      let chromaticity_y = y / sum;

      assert!((chromaticity_x - 0.64).abs() < 1e-6);
      assert!((chromaticity_y - 0.33).abs() < 1e-6);
    }

    #[test]
    fn it_maps_green_primary_to_xyz_with_correct_chromaticity() {
      let primaries = RgbPrimaries::new_const(
        Xy::new_const(0.64, 0.33),
        Xy::new_const(0.30, 0.60),
        Xy::new_const(0.15, 0.06),
      );
      let context = ColorimetricContext::new()
        .with_illuminant(Illuminant::D65)
        .with_observer(Observer::CIE_1931_2D);
      let white_point = context.reference_white();
      let matrix = primaries.calculate_xyz_matrix(white_point);

      let [x, y, z] = matrix * [0.0, 1.0, 0.0];
      let sum = x + y + z;
      let chromaticity_x = x / sum;
      let chromaticity_y = y / sum;

      assert!((chromaticity_x - 0.30).abs() < 1e-6);
      assert!((chromaticity_y - 0.60).abs() < 1e-6);
    }

    #[test]
    fn it_maps_blue_primary_to_xyz_with_correct_chromaticity() {
      let primaries = RgbPrimaries::new_const(
        Xy::new_const(0.64, 0.33),
        Xy::new_const(0.30, 0.60),
        Xy::new_const(0.15, 0.06),
      );
      let context = ColorimetricContext::new()
        .with_illuminant(Illuminant::D65)
        .with_observer(Observer::CIE_1931_2D);
      let white_point = context.reference_white();
      let matrix = primaries.calculate_xyz_matrix(white_point);

      let [x, y, z] = matrix * [0.0, 0.0, 1.0];
      let sum = x + y + z;
      let chromaticity_x = x / sum;
      let chromaticity_y = y / sum;

      assert!((chromaticity_x - 0.15).abs() < 1e-6);
      assert!((chromaticity_y - 0.06).abs() < 1e-6);
    }
  }
}
