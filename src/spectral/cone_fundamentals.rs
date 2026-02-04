use super::{Cmf, ConeResponse, Spd, Table};
use crate::space::{Lms, Xyz};

#[derive(Clone, Copy, Debug)]
pub struct ConeFundamentals(&'static [(u32, ConeResponse)]);

impl ConeFundamentals {
  pub const fn new(table: &'static [(u32, ConeResponse)]) -> Self {
    Self(table)
  }

  pub fn spd_to_lms(&self, spd: &Spd) -> Lms {
    self.spectral_power_distribution_to_lms(spd)
  }

  pub fn spectral_power_distribution_to_lms(&self, spd: &Spd) -> Lms {
    let step = self.step() as f64;
    let mut components = [0.0_f64; 3];

    for (wavelength, response) in self.table().iter() {
      let Some(&spd_response) = spd.at(*wavelength) else {
        continue;
      };

      let lms = response.components();
      components[0] += spd_response * lms[0] * step;
      components[1] += spd_response * lms[1] * step;
      components[2] += spd_response * lms[2] * step;
    }

    Lms::new(components[0], components[1], components[2])
  }
}

impl From<Cmf> for ConeFundamentals {
  fn from(cmf: Cmf) -> Self {
    let data: Box<[(u32, ConeResponse)]> = cmf
      .table()
      .iter()
      .map(|(wavelength, xyz)| {
        let [l, m, s] = Xyz::from(xyz.components()).to_lms().components();
        (*wavelength, ConeResponse::new(l, m, s))
      })
      .collect();
    Self::new(Box::leak(data))
  }
}

impl Table for ConeFundamentals {
  type Value = ConeResponse;

  fn table(&self) -> &[(u32, Self::Value)] {
    self.0
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::spectral::TristimulusResponse;

  static TEST_CONE_FUNDAMENTALS: &[(u32, ConeResponse)] = &[
    (380, ConeResponse::new(0.001, 0.0001, 0.006)),
    (400, ConeResponse::new(0.014, 0.0004, 0.068)),
    (420, ConeResponse::new(0.134, 0.004, 0.646)),
    (440, ConeResponse::new(0.348, 0.023, 1.747)),
  ];

  static TEST_SPD: &[(u32, f64)] = &[(380, 0.5), (400, 0.8), (420, 1.0), (440, 0.9)];

  mod from_cmf {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_converts_cmf_to_cone_fundamentals() {
      static CMF: &[(u32, TristimulusResponse)] = &[
        (380, TristimulusResponse::new(0.001, 0.0001, 0.006)),
        (400, TristimulusResponse::new(0.014, 0.0004, 0.068)),
      ];
      let cmf = Cmf::new(CMF);
      let fundamentals = ConeFundamentals::from(cmf);

      assert_eq!(fundamentals.len(), 2);
      assert_eq!(fundamentals.min_wavelength(), Some(380));
      assert_eq!(fundamentals.max_wavelength(), Some(400));
    }

    #[test]
    fn it_preserves_wavelengths() {
      static CMF: &[(u32, TristimulusResponse)] = &[
        (380, TristimulusResponse::new(0.1, 0.2, 0.3)),
        (420, TristimulusResponse::new(0.4, 0.5, 0.6)),
      ];
      let cmf = Cmf::new(CMF);
      let fundamentals = ConeFundamentals::from(cmf);
      let wavelengths: Vec<u32> = fundamentals.wavelengths().collect();

      assert_eq!(wavelengths, vec![380, 420]);
    }
  }

  mod spectral_power_distribution_to_lms {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_integrates_spd_with_cone_fundamentals() {
      let fundamentals = ConeFundamentals::new(TEST_CONE_FUNDAMENTALS);
      let spd = Spd::new(TEST_SPD);
      let lms = fundamentals.spectral_power_distribution_to_lms(&spd);

      assert!(lms.l() > 0.0);
      assert!(lms.m() > 0.0);
      assert!(lms.s() > 0.0);
    }

    #[test]
    fn it_weights_by_step_size() {
      let fundamentals = ConeFundamentals::new(TEST_CONE_FUNDAMENTALS);
      let spd = Spd::new(TEST_SPD);
      let lms = fundamentals.spectral_power_distribution_to_lms(&spd);

      let step = fundamentals.step() as f64;
      let mut expected = [0.0_f64; 3];
      for (wavelength, response) in fundamentals.table().iter() {
        if let Some(&power) = spd.at(*wavelength) {
          let [l, m, s] = response.components();
          expected[0] += power * l * step;
          expected[1] += power * m * step;
          expected[2] += power * s * step;
        }
      }

      assert_eq!(lms.l(), expected[0]);
      assert_eq!(lms.m(), expected[1]);
      assert_eq!(lms.s(), expected[2]);
    }

    #[test]
    fn it_returns_zero_for_non_overlapping_wavelengths() {
      static NON_OVERLAPPING_SPD: &[(u32, f64)] = &[(500, 1.0), (520, 1.0)];
      let fundamentals = ConeFundamentals::new(TEST_CONE_FUNDAMENTALS);
      let spd = Spd::new(NON_OVERLAPPING_SPD);
      let lms = fundamentals.spectral_power_distribution_to_lms(&spd);

      assert_eq!(lms.l(), 0.0);
      assert_eq!(lms.m(), 0.0);
      assert_eq!(lms.s(), 0.0);
    }
  }

  mod spd_to_lms {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_delegates_to_spectral_power_distribution_to_lms() {
      let fundamentals = ConeFundamentals::new(TEST_CONE_FUNDAMENTALS);
      let spd = Spd::new(TEST_SPD);

      assert_eq!(
        fundamentals.spd_to_lms(&spd),
        fundamentals.spectral_power_distribution_to_lms(&spd)
      );
    }
  }
}
