use super::{Spd, Table, TristimulusResponse};
use crate::space::Xyz;

/// Shorthand alias for [`ColorMatchingFunction`].
pub type Cmf = ColorMatchingFunction;

/// CIE color matching functions mapping wavelengths to XYZ tristimulus responses.
#[derive(Clone, Copy, Debug)]
pub struct ColorMatchingFunction(&'static [(u32, TristimulusResponse)]);

impl ColorMatchingFunction {
  /// Creates a new CMF from static wavelength-response pairs.
  pub const fn new(table: &'static [(u32, TristimulusResponse)]) -> Self {
    Self(table)
  }

  /// Integrates an SPD with this CMF and normalizes to unit luminance (Y=1).
  ///
  /// This produces the reference white XYZ used for chromatic adaptation.
  pub fn calculate_reference_white(&self, spd: &Spd) -> Xyz {
    let [x, y, z] = self.spectral_power_distribution_to_xyz(spd).components();

    if y > 0.0 {
      Xyz::new(x / y, 1.0, z / y)
    } else {
      Xyz::new(0.0, 0.0, 0.0)
    }
  }

  /// Alias for [`Self::spectral_power_distribution_to_xyz`].
  pub fn spd_to_xyz(&self, spd: &Spd) -> Xyz {
    self.spectral_power_distribution_to_xyz(spd)
  }

  /// Integrates a spectral power distribution with this CMF to produce XYZ tristimulus values.
  pub fn spectral_power_distribution_to_xyz(&self, spd: &Spd) -> Xyz {
    let step = self.step() as f64;
    let mut components = [0.0_f64; 3];

    for (wavelength, response) in self.table().iter() {
      let Some(&spd_response) = spd.at(*wavelength) else {
        continue;
      };

      let xyz = response.components();
      components[0] += spd_response * xyz[0] * step;
      components[1] += spd_response * xyz[1] * step;
      components[2] += spd_response * xyz[2] * step;
    }

    Xyz::new(components[0], components[1], components[2])
  }
}

impl Table for ColorMatchingFunction {
  type Value = TristimulusResponse;

  fn table(&self) -> &[(u32, Self::Value)] {
    self.0
  }
}

#[cfg(test)]
mod test {
  use super::*;

  static TEST_CMF: &[(u32, TristimulusResponse)] = &[
    (380, TristimulusResponse::new(0.001, 0.0001, 0.006)),
    (400, TristimulusResponse::new(0.014, 0.0004, 0.068)),
    (420, TristimulusResponse::new(0.134, 0.004, 0.646)),
    (440, TristimulusResponse::new(0.348, 0.023, 1.747)),
  ];

  static TEST_SPD: &[(u32, f64)] = &[(380, 0.5), (400, 0.8), (420, 1.0), (440, 0.9)];

  mod calculate_reference_white {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_normalizes_xyz_to_unit_luminance() {
      let cmf = Cmf::new(TEST_CMF);
      let spd = Spd::new(TEST_SPD);
      let white = cmf.calculate_reference_white(&spd);

      assert_eq!(white.y(), 1.0);
    }

    #[test]
    fn it_scales_x_and_z_proportionally() {
      let cmf = Cmf::new(TEST_CMF);
      let spd = Spd::new(TEST_SPD);
      let xyz = cmf.spectral_power_distribution_to_xyz(&spd);
      let white = cmf.calculate_reference_white(&spd);

      let expected_x = xyz.x() / xyz.y();
      let expected_z = xyz.z() / xyz.y();

      assert_eq!(white.x(), expected_x);
      assert_eq!(white.z(), expected_z);
    }

    #[test]
    fn it_returns_zero_for_zero_luminance() {
      static ZERO_CMF: &[(u32, TristimulusResponse)] = &[
        (380, TristimulusResponse::new(0.0, 0.0, 0.0)),
        (400, TristimulusResponse::new(0.0, 0.0, 0.0)),
      ];
      let cmf = Cmf::new(ZERO_CMF);
      let spd = Spd::new(TEST_SPD);
      let white = cmf.calculate_reference_white(&spd);

      assert_eq!(white.x(), 0.0);
      assert_eq!(white.y(), 0.0);
      assert_eq!(white.z(), 0.0);
    }
  }

  mod spectral_power_distribution_to_xyz {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_integrates_spd_with_cmf() {
      let cmf = Cmf::new(TEST_CMF);
      let spd = Spd::new(TEST_SPD);
      let xyz = cmf.spectral_power_distribution_to_xyz(&spd);

      assert!(xyz.x() > 0.0);
      assert!(xyz.y() > 0.0);
      assert!(xyz.z() > 0.0);
    }

    #[test]
    fn it_weights_by_step_size() {
      let cmf = Cmf::new(TEST_CMF);
      let spd = Spd::new(TEST_SPD);
      let xyz = cmf.spectral_power_distribution_to_xyz(&spd);

      let step = cmf.step() as f64;
      let mut expected = [0.0_f64; 3];
      for (wavelength, response) in cmf.table().iter() {
        if let Some(&power) = spd.at(*wavelength) {
          let [x, y, z] = response.components();
          expected[0] += power * x * step;
          expected[1] += power * y * step;
          expected[2] += power * z * step;
        }
      }

      assert_eq!(xyz.x(), expected[0]);
      assert_eq!(xyz.y(), expected[1]);
      assert_eq!(xyz.z(), expected[2]);
    }

    #[test]
    fn it_returns_zero_for_non_overlapping_wavelengths() {
      static NON_OVERLAPPING_SPD: &[(u32, f64)] = &[(500, 1.0), (520, 1.0)];
      let cmf = Cmf::new(TEST_CMF);
      let spd = Spd::new(NON_OVERLAPPING_SPD);
      let xyz = cmf.spectral_power_distribution_to_xyz(&spd);

      assert_eq!(xyz.x(), 0.0);
      assert_eq!(xyz.y(), 0.0);
      assert_eq!(xyz.z(), 0.0);
    }
  }

  mod spd_to_xyz {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_delegates_to_spectral_power_distribution_to_xyz() {
      let cmf = Cmf::new(TEST_CMF);
      let spd = Spd::new(TEST_SPD);

      assert_eq!(cmf.spd_to_xyz(&spd), cmf.spectral_power_distribution_to_xyz(&spd));
    }
  }
}
