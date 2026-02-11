use super::Observer;
use crate::{
  space::{Lms, Xyz},
  spectral::Table,
};

/// Modifier for deriving new [`Observer`] instances from existing ones.
///
/// Applies physiological adjustments based on Fairchild's observer metamerism model,
/// including age-related lens yellowing, macular pigment density, rod intrusion, and
/// field-size-dependent S-cone sensitivity to produce an observer tuned to specific
/// viewing conditions.
///
/// Use [`Observer::modifier`] to create a modifier pre-configured with sensible defaults,
/// then chain `with_*` methods to customize parameters before calling [`Modifier::modify`].
pub struct Modifier {
  age: u8,
  age_yellowing_factor: f64,
  blue_range: f64,
  blue_threshold: f64,
  field_size_factor: f64,
  macular_density_decay_rate: f64,
  macular_peak: f64,
  macular_spread: f64,
  rod_contribution_factor: f64,
  rod_peak: f64,
  rod_spread: f64,
  s_cone_field_factor: f64,
  s_cone_peak: f64,
  s_cone_spread: f64,
  source: Observer,
  visual_field: f64,
}

impl Modifier {
  /// Creates a new modifier from the given source observer.
  ///
  /// The visual field defaults to the source observer's field angle; all other parameters
  /// use sensible physiological defaults.
  pub fn new(source: Observer) -> Self {
    Self {
      age: 0,
      age_yellowing_factor: 0.01,
      blue_range: 50.0,
      blue_threshold: 450.0,
      field_size_factor: 0.005,
      macular_density_decay_rate: 0.3,
      macular_peak: 460.0,
      macular_spread: 1000.0,
      rod_contribution_factor: 0.03,
      rod_peak: 500.0,
      rod_spread: 5000.0,
      s_cone_field_factor: 0.05,
      s_cone_peak: 440.0,
      s_cone_spread: 2000.0,
      source,
      visual_field: source.visual_field(),
    }
  }

  /// Produces a modified [`Observer`] by applying physiological adjustments to the source
  /// observer's color matching functions.
  ///
  /// The adjustment converts each CMF tristimulus value to LMS, applies lens yellowing,
  /// macular pigment, rod intrusion, and S-cone corrections, converts back to XYZ, then
  /// normalizes so the luminance sum matches the original observer.
  pub fn modify(&self) -> Observer {
    let original_visual_field = self.source.visual_field();

    let adjusted_cmf_data: Vec<(u32, [f64; 3])> = self
      .source
      .cmf()
      .table()
      .iter()
      .map(|(wavelength, xyz)| {
        let wl = *wavelength as f64;
        let [x, y, z] = xyz.components();
        let lms = Xyz::new(x, y, z).to_lms();
        let [l, m, s] = lms.components();

        let lens_factor = if self.age == 0 {
          1.0 - ((self.visual_field - original_visual_field) * self.field_size_factor)
        } else {
          let delta = self.age as f64 - self.source.age().unwrap_or(0) as f64;
          let weight = (self.blue_threshold - wl).max(0.0) / self.blue_range;
          1.0 - (delta * self.age_yellowing_factor * weight)
        };

        let original_density = (-self.macular_density_decay_rate * original_visual_field).exp();
        let new_density = (-self.macular_density_decay_rate * self.visual_field).exp();
        let wavelength_factor = (-((wl - self.macular_peak).powi(2)) / self.macular_spread).exp();
        let macular_factor = 1.0 - ((original_density - new_density) * wavelength_factor);

        let rod_effect = if original_visual_field > self.visual_field {
          let delta = (self.visual_field - original_visual_field) * self.rod_contribution_factor;
          delta * (-((wl - self.rod_peak).powi(2)) / self.rod_spread).exp()
        } else {
          0.0
        };

        let s_cone_effect =
          if wl < self.blue_threshold && (original_visual_field - self.visual_field).abs() > f64::EPSILON {
            let sign = if self.visual_field > original_visual_field {
              1.0
            } else {
              -1.0
            };
            let s_factor = sign * self.s_cone_field_factor;
            s_factor * (-((wl - self.s_cone_peak).powi(2)) / self.s_cone_spread).exp()
          } else {
            0.0
          };

        let receptor_factor = 1.0 + rod_effect + s_cone_effect;

        let adjusted_l = l * lens_factor * receptor_factor;
        let adjusted_m = m * lens_factor * receptor_factor;
        let adjusted_s = s * macular_factor * lens_factor * receptor_factor;

        let adjusted_xyz = Lms::new(adjusted_l, adjusted_m, adjusted_s).to_xyz();
        (*wavelength, adjusted_xyz.components())
      })
      .collect();

    let y_original: f64 = self.source.cmf().table().iter().map(|(_, xyz)| xyz.y()).sum();
    let y_new: f64 = adjusted_cmf_data.iter().map(|(_, xyz)| xyz[1]).sum();
    let normalization = if y_new.abs() < f64::EPSILON {
      1.0
    } else {
      y_original / y_new
    };

    let normalized_cmf: Vec<(u32, [f64; 3])> = adjusted_cmf_data
      .iter()
      .map(|(wavelength, [x, y, z])| (*wavelength, [x * normalization, y * normalization, z * normalization]))
      .collect();

    let name = if self.source.name.contains("(Modified)") {
      self.source.name.to_owned()
    } else {
      format!("{} (Modified)", self.source.name)
    };

    let age = if self.age > 0 {
      Some(self.age)
    } else {
      self.source.age()
    };

    let mut builder = Observer::builder(&name, self.visual_field).with_cmf(&normalized_cmf);

    if let Some(a) = age {
      builder = builder.with_age(a);
    }

    builder.build().expect("CMF data was provided")
  }

  /// Sets the target observer age for age-dependent lens yellowing.
  ///
  /// A value of `0` disables age-based adjustment and uses field-size-based lens correction
  /// instead.
  pub fn with_age(mut self, age: u8) -> Self {
    self.age = age;
    self
  }

  /// Sets the age-related lens yellowing factor (default `0.01`).
  pub fn with_age_yellowing_factor(mut self, factor: f64) -> Self {
    self.age_yellowing_factor = factor;
    self
  }

  /// Sets the wavelength range in nm for the blue absorption band (default `50.0`).
  pub fn with_blue_range(mut self, range: f64) -> Self {
    self.blue_range = range;
    self
  }

  /// Sets the wavelength threshold in nm for blue absorption (default `450.0`).
  pub fn with_blue_threshold(mut self, threshold: f64) -> Self {
    self.blue_threshold = threshold;
    self
  }

  /// Sets the field-size scaling factor (default `0.005`).
  pub fn with_field_size_factor(mut self, factor: f64) -> Self {
    self.field_size_factor = factor;
    self
  }

  /// Sets the macular pigment density decay rate (default `0.3`).
  pub fn with_macular_density_decay_rate(mut self, rate: f64) -> Self {
    self.macular_density_decay_rate = rate;
    self
  }

  /// Sets the peak wavelength in nm for macular pigment absorption (default `460.0`).
  pub fn with_macular_peak(mut self, peak: f64) -> Self {
    self.macular_peak = peak;
    self
  }

  /// Sets the spread of the macular pigment absorption curve (default `1000.0`).
  pub fn with_macular_spread(mut self, spread: f64) -> Self {
    self.macular_spread = spread;
    self
  }

  /// Sets the rod intrusion contribution factor (default `0.03`).
  pub fn with_rod_contribution_factor(mut self, factor: f64) -> Self {
    self.rod_contribution_factor = factor;
    self
  }

  /// Sets the peak wavelength in nm for rod sensitivity (default `500.0`).
  pub fn with_rod_peak(mut self, peak: f64) -> Self {
    self.rod_peak = peak;
    self
  }

  /// Sets the spread of the rod sensitivity curve (default `5000.0`).
  pub fn with_rod_spread(mut self, spread: f64) -> Self {
    self.rod_spread = spread;
    self
  }

  /// Sets the field-size-dependent S-cone sensitivity factor (default `0.05`).
  pub fn with_s_cone_field_factor(mut self, factor: f64) -> Self {
    self.s_cone_field_factor = factor;
    self
  }

  /// Sets the peak wavelength in nm for S-cone sensitivity adjustment (default `440.0`).
  pub fn with_s_cone_peak(mut self, peak: f64) -> Self {
    self.s_cone_peak = peak;
    self
  }

  /// Sets the spread of the S-cone sensitivity adjustment curve (default `2000.0`).
  pub fn with_s_cone_spread(mut self, spread: f64) -> Self {
    self.s_cone_spread = spread;
    self
  }

  /// Sets the target visual field angle in degrees.
  pub fn with_visual_field(mut self, visual_field: f64) -> Self {
    self.visual_field = visual_field;
    self
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::spectral::Table;

  mod modifier {
    use super::*;

    mod modify {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_preserves_cmf_length() {
        let source = Observer::CIE_1931_2D;
        let modified = source.modifier().with_visual_field(10.0).modify();

        assert_eq!(modified.cmf().len(), source.cmf().len());
      }

      #[test]
      fn it_preserves_luminance_sum() {
        let source = Observer::CIE_1931_2D;
        let y_original: f64 = source.cmf().table().iter().map(|(_, xyz)| xyz.y()).sum();

        let modified = source.modifier().with_visual_field(10.0).modify();
        let y_modified: f64 = modified.cmf().table().iter().map(|(_, xyz)| xyz.y()).sum();

        assert!((y_original - y_modified).abs() < 1e-10);
      }

      #[test]
      fn it_returns_identity_when_parameters_unchanged() {
        let source = Observer::CIE_1931_2D;
        let modified = source.modifier().modify();

        for (original, adjusted) in source.cmf().table().iter().zip(modified.cmf().table().iter()) {
          assert_eq!(original.0, adjusted.0);
          assert!((original.1.x() - adjusted.1.x()).abs() < 1e-10);
          assert!((original.1.y() - adjusted.1.y()).abs() < 1e-10);
          assert!((original.1.z() - adjusted.1.z()).abs() < 1e-10);
        }
      }

      #[test]
      fn it_applies_field_size_lens_correction() {
        let source = Observer::CIE_1931_2D;
        let modified = source.modifier().with_visual_field(10.0).modify();

        let original_values: Vec<[f64; 3]> = source.cmf().table().iter().map(|(_, xyz)| xyz.components()).collect();
        let modified_values: Vec<[f64; 3]> = modified.cmf().table().iter().map(|(_, xyz)| xyz.components()).collect();
        let differs = original_values
          .iter()
          .zip(modified_values.iter())
          .any(|(o, m)| (o[0] - m[0]).abs() > 1e-15 || (o[1] - m[1]).abs() > 1e-15 || (o[2] - m[2]).abs() > 1e-15);

        assert!(differs);
      }

      #[test]
      fn it_applies_age_based_yellowing() {
        let source = Observer::CIE_1931_2D;
        let modified = source.modifier().with_age(60).modify();

        let original_values: Vec<[f64; 3]> = source.cmf().table().iter().map(|(_, xyz)| xyz.components()).collect();
        let modified_values: Vec<[f64; 3]> = modified.cmf().table().iter().map(|(_, xyz)| xyz.components()).collect();
        let differs = original_values
          .iter()
          .zip(modified_values.iter())
          .any(|(o, m)| (o[0] - m[0]).abs() > 1e-15 || (o[1] - m[1]).abs() > 1e-15 || (o[2] - m[2]).abs() > 1e-15);

        assert!(differs);
      }

      #[test]
      fn it_applies_rod_intrusion_when_narrowing_field() {
        let source = Observer::CIE_1931_2D;
        let narrowed = source.modifier().with_visual_field(1.0).modify();

        let widened = source.modifier().with_visual_field(10.0).modify();

        let narrowed_values: Vec<[f64; 3]> = narrowed.cmf().table().iter().map(|(_, xyz)| xyz.components()).collect();
        let widened_values: Vec<[f64; 3]> = widened.cmf().table().iter().map(|(_, xyz)| xyz.components()).collect();
        let differs = narrowed_values
          .iter()
          .zip(widened_values.iter())
          .any(|(n, w)| (n[0] - w[0]).abs() > 1e-15 || (n[1] - w[1]).abs() > 1e-15 || (n[2] - w[2]).abs() > 1e-15);

        assert!(differs);
      }

      #[test]
      fn it_appends_modified_to_name() {
        let source = Observer::CIE_1931_2D;
        let modified = source.modifier().with_visual_field(10.0).modify();

        assert!(modified.name().contains("(Modified)"));
      }

      #[test]
      fn it_does_not_double_append_modified_to_name() {
        let source = Observer::CIE_1931_2D;
        let first = source.modifier().with_visual_field(10.0).modify();
        let second = first.modifier().with_visual_field(4.0).modify();

        let count = second.name().matches("(Modified)").count();

        assert_eq!(count, 1);
      }

      #[test]
      fn it_sets_visual_field_on_result() {
        let source = Observer::CIE_1931_2D;
        let modified = source.modifier().with_visual_field(10.0).modify();

        assert!((modified.visual_field() - 10.0).abs() < f64::EPSILON);
      }

      #[test]
      fn it_propagates_source_age_when_no_age_set() {
        let source = Observer::builder("Test", 2.0)
          .with_cmf(&[
            (380, [0.001368, 0.000039, 0.006450]),
            (390, [0.004243, 0.000120, 0.020050]),
          ])
          .with_age(32)
          .build()
          .unwrap();
        let modified = source.modifier().with_visual_field(10.0).modify();

        assert_eq!(modified.age(), Some(32));
      }

      #[test]
      fn it_uses_modifier_age_when_set() {
        let source = Observer::CIE_1931_2D;
        let modified = source.modifier().with_age(45).modify();

        assert_eq!(modified.age(), Some(45));
      }
    }
  }
}
