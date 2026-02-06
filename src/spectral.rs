mod chromaticity_coordinates;
mod color_matching_function;
mod cone_fundamentals;
mod cone_response;
mod spectral_power_distribution;
mod tristimulus_response;

pub use chromaticity_coordinates::ChromaticityCoordinates;
pub use color_matching_function::{Cmf, ColorMatchingFunction};
pub use cone_fundamentals::ConeFundamentals;
pub use cone_response::ConeResponse;
pub use spectral_power_distribution::{Spd, SpectralPowerDistribution};
pub use tristimulus_response::TristimulusResponse;

/// Common interface for wavelength-indexed spectral data.
///
/// All spectral data types (SPD, CMF, chromaticity coordinates, cone fundamentals)
/// implement this trait, providing uniform access to wavelength-value pairs.
pub trait Table {
  type Value;

  /// Returns the underlying wavelength-value pairs.
  fn table(&self) -> &[(u32, Self::Value)];

  /// Returns the value at the given wavelength, or `None` if not present.
  fn at(&self, wavelength: u32) -> Option<&Self::Value> {
    self
      .table()
      .binary_search_by_key(&wavelength, |(w, _)| *w)
      .ok()
      .map(|i| &self.table()[i].1)
  }

  /// Returns `true` if the table contains no entries.
  fn is_empty(&self) -> bool {
    self.table().is_empty()
  }

  /// Returns the number of wavelength-value pairs.
  fn len(&self) -> usize {
    self.table().len()
  }

  /// Returns the maximum wavelength in the table, or `None` if empty.
  fn max_wavelength(&self) -> Option<u32> {
    self.table().last().map(|(w, _)| *w)
  }

  /// Returns the minimum wavelength in the table, or `None` if empty.
  fn min_wavelength(&self) -> Option<u32> {
    self.table().first().map(|(w, _)| *w)
  }

  /// Returns the minimum step size between consecutive wavelengths.
  fn step(&self) -> u32 {
    if self.table().len() < 2 {
      return 1;
    }

    self.table().windows(2).map(|w| w[1].0 - w[0].0).min().unwrap_or(1)
  }

  /// Returns an iterator over the values (without wavelengths).
  fn values(&self) -> impl Iterator<Item = &Self::Value> + '_ {
    self.table().iter().map(|(_, v)| v)
  }

  /// Returns an iterator over the wavelengths (without values).
  fn wavelengths(&self) -> impl Iterator<Item = u32> + '_ {
    self.table().iter().map(|(w, _)| *w)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod table {
    use super::*;

    static TEST_SPD: &[(u32, f64)] = &[(380, 0.1), (390, 0.2), (400, 0.3), (410, 0.4)];
    static EMPTY_SPD: &[(u32, f64)] = &[];
    static SINGLE_SPD: &[(u32, f64)] = &[(550, 1.0)];

    mod at {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_returns_value_at_wavelength() {
        let spd = Spd::new(TEST_SPD);

        assert_eq!(spd.at(390), Some(&0.2));
      }

      #[test]
      fn it_returns_none_for_missing_wavelength() {
        let spd = Spd::new(TEST_SPD);

        assert_eq!(spd.at(385), None);
      }

      #[test]
      fn it_returns_none_for_empty_table() {
        let spd = Spd::new(EMPTY_SPD);

        assert_eq!(spd.at(380), None);
      }
    }

    mod is_empty {
      use super::*;

      #[test]
      fn it_returns_true_for_empty_table() {
        let spd = Spd::new(EMPTY_SPD);

        assert!(spd.is_empty());
      }

      #[test]
      fn it_returns_false_for_non_empty_table() {
        let spd = Spd::new(TEST_SPD);

        assert!(!spd.is_empty());
      }
    }

    mod len {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_returns_table_length() {
        let spd = Spd::new(TEST_SPD);

        assert_eq!(spd.len(), 4);
      }

      #[test]
      fn it_returns_zero_for_empty_table() {
        let spd = Spd::new(EMPTY_SPD);

        assert_eq!(spd.len(), 0);
      }
    }

    mod max_wavelength {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_returns_last_wavelength() {
        let spd = Spd::new(TEST_SPD);

        assert_eq!(spd.max_wavelength(), Some(410));
      }

      #[test]
      fn it_returns_none_for_empty_table() {
        let spd = Spd::new(EMPTY_SPD);

        assert_eq!(spd.max_wavelength(), None);
      }
    }

    mod min_wavelength {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_returns_first_wavelength() {
        let spd = Spd::new(TEST_SPD);

        assert_eq!(spd.min_wavelength(), Some(380));
      }

      #[test]
      fn it_returns_none_for_empty_table() {
        let spd = Spd::new(EMPTY_SPD);

        assert_eq!(spd.min_wavelength(), None);
      }
    }

    mod step {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_returns_minimum_step_between_wavelengths() {
        let spd = Spd::new(TEST_SPD);

        assert_eq!(spd.step(), 10);
      }

      #[test]
      fn it_returns_one_for_single_entry() {
        let spd = Spd::new(SINGLE_SPD);

        assert_eq!(spd.step(), 1);
      }

      #[test]
      fn it_returns_one_for_empty_table() {
        let spd = Spd::new(EMPTY_SPD);

        assert_eq!(spd.step(), 1);
      }

      #[test]
      fn it_handles_variable_step_sizes() {
        static VARIABLE_SPD: &[(u32, f64)] = &[(380, 0.1), (385, 0.2), (400, 0.3)];
        let spd = Spd::new(VARIABLE_SPD);

        assert_eq!(spd.step(), 5);
      }
    }

    mod values {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_iterates_over_values() {
        let spd = Spd::new(TEST_SPD);
        let values: Vec<&f64> = spd.values().collect();

        assert_eq!(values, vec![&0.1, &0.2, &0.3, &0.4]);
      }

      #[test]
      fn it_returns_empty_iterator_for_empty_table() {
        let spd = Spd::new(EMPTY_SPD);
        let values: Vec<&f64> = spd.values().collect();

        assert_eq!(values.len(), 0);
      }
    }

    mod wavelengths {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_iterates_over_wavelengths() {
        let spd = Spd::new(TEST_SPD);
        let wavelengths: Vec<u32> = spd.wavelengths().collect();

        assert_eq!(wavelengths, vec![380, 390, 400, 410]);
      }

      #[test]
      fn it_returns_empty_iterator_for_empty_table() {
        let spd = Spd::new(EMPTY_SPD);
        let wavelengths: Vec<u32> = spd.wavelengths().collect();

        assert_eq!(wavelengths.len(), 0);
      }
    }
  }
}
