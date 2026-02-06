use super::Table;

/// Shorthand alias for [`SpectralPowerDistribution`].
pub type Spd = SpectralPowerDistribution;

/// Spectral power distribution — the power of a light source at each wavelength.
#[derive(Clone, Copy, Debug)]
pub struct SpectralPowerDistribution(&'static [(u32, f64)]);

impl SpectralPowerDistribution {
  /// Creates a new SPD from static wavelength-power pairs.
  pub const fn new(table: &'static [(u32, f64)]) -> Self {
    Self(table)
  }

  /// Returns the maximum power value across all wavelengths.
  pub fn peak_power(&self) -> f64 {
    self.values().cloned().fold(f64::NEG_INFINITY, f64::max)
  }

  /// Returns the wavelength with the highest power, or `None` if empty.
  pub fn peak_wavelength(&self) -> Option<u32> {
    self
      .table()
      .iter()
      .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
      .map(|(w, _)| *w)
  }

  /// Returns the sum of power values across all wavelengths.
  pub fn total_power(&self) -> f64 {
    self.values().sum()
  }
}

impl Table for SpectralPowerDistribution {
  type Value = f64;

  fn table(&self) -> &[(u32, Self::Value)] {
    self.0
  }
}

#[cfg(test)]
mod test {
  use super::*;

  static TEST_SPD: &[(u32, f64)] = &[(380, 0.1), (400, 0.5), (420, 0.3), (440, 0.2)];
  static EMPTY_SPD: &[(u32, f64)] = &[];

  mod peak_power {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_maximum_power_value() {
      let spd = Spd::new(TEST_SPD);

      assert_eq!(spd.peak_power(), 0.5);
    }

    #[test]
    fn it_returns_negative_infinity_for_empty_table() {
      let spd = Spd::new(EMPTY_SPD);

      assert_eq!(spd.peak_power(), f64::NEG_INFINITY);
    }
  }

  mod peak_wavelength {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_wavelength_with_maximum_power() {
      let spd = Spd::new(TEST_SPD);

      assert_eq!(spd.peak_wavelength(), Some(400));
    }

    #[test]
    fn it_returns_none_for_empty_table() {
      let spd = Spd::new(EMPTY_SPD);

      assert_eq!(spd.peak_wavelength(), None);
    }
  }

  mod total_power {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_sums_all_power_values() {
      let spd = Spd::new(TEST_SPD);
      let expected = 0.1 + 0.5 + 0.3 + 0.2;

      assert_eq!(spd.total_power(), expected);
    }

    #[test]
    fn it_returns_zero_for_empty_table() {
      let spd = Spd::new(EMPTY_SPD);

      assert_eq!(spd.total_power(), 0.0);
    }
  }
}
