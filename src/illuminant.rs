#[cfg(feature = "illuminant-a")]
mod a;
#[cfg(feature = "illuminant-b")]
mod b;
#[cfg(feature = "illuminant-c")]
mod c;
#[cfg(feature = "illuminant-d50")]
mod d50;
#[cfg(feature = "illuminant-d55")]
mod d55;
mod d65;
#[cfg(feature = "illuminant-d75")]
mod d75;
#[cfg(feature = "illuminant-e")]
mod e;
#[cfg(feature = "illuminant-fl1")]
mod fl1;
#[cfg(feature = "illuminant-fl10")]
mod fl10;
#[cfg(feature = "illuminant-fl11")]
mod fl11;
#[cfg(feature = "illuminant-fl12")]
mod fl12;
#[cfg(feature = "illuminant-fl2")]
mod fl2;
#[cfg(feature = "illuminant-fl3")]
mod fl3;
#[cfg(feature = "illuminant-fl3-1")]
mod fl3_1;
#[cfg(feature = "illuminant-fl3-10")]
mod fl3_10;
#[cfg(feature = "illuminant-fl3-11")]
mod fl3_11;
#[cfg(feature = "illuminant-fl3-12")]
mod fl3_12;
#[cfg(feature = "illuminant-fl3-13")]
mod fl3_13;
#[cfg(feature = "illuminant-fl3-14")]
mod fl3_14;
#[cfg(feature = "illuminant-fl3-15")]
mod fl3_15;
#[cfg(feature = "illuminant-fl3-2")]
mod fl3_2;
#[cfg(feature = "illuminant-fl3-3")]
mod fl3_3;
#[cfg(feature = "illuminant-fl3-4")]
mod fl3_4;
#[cfg(feature = "illuminant-fl3-5")]
mod fl3_5;
#[cfg(feature = "illuminant-fl3-6")]
mod fl3_6;
#[cfg(feature = "illuminant-fl3-7")]
mod fl3_7;
#[cfg(feature = "illuminant-fl3-8")]
mod fl3_8;
#[cfg(feature = "illuminant-fl3-9")]
mod fl3_9;
#[cfg(feature = "illuminant-fl4")]
mod fl4;
#[cfg(feature = "illuminant-fl5")]
mod fl5;
#[cfg(feature = "illuminant-fl6")]
mod fl6;
#[cfg(feature = "illuminant-fl7")]
mod fl7;
#[cfg(feature = "illuminant-fl8")]
mod fl8;
#[cfg(feature = "illuminant-fl9")]
mod fl9;
#[cfg(feature = "illuminant-hp1")]
mod hp1;
#[cfg(feature = "illuminant-hp2")]
mod hp2;
#[cfg(feature = "illuminant-hp3")]
mod hp3;
#[cfg(feature = "illuminant-hp4")]
mod hp4;
#[cfg(feature = "illuminant-hp5")]
mod hp5;
#[cfg(feature = "illuminant-id50")]
mod id50;
#[cfg(feature = "illuminant-id65")]
mod id65;
#[cfg(feature = "illuminant-led-b1")]
mod led_b1;
#[cfg(feature = "illuminant-led-b2")]
mod led_b2;
#[cfg(feature = "illuminant-led-b3")]
mod led_b3;
#[cfg(feature = "illuminant-led-b4")]
mod led_b4;
#[cfg(feature = "illuminant-led-b5")]
mod led_b5;
#[cfg(feature = "illuminant-led-bh1")]
mod led_bh1;
#[cfg(feature = "illuminant-led-rgb1")]
mod led_rgb1;
#[cfg(feature = "illuminant-led-v1")]
mod led_v1;
#[cfg(feature = "illuminant-led-v2")]
mod led_v2;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{error::Error, spectral::Spd};

/// Builder for constructing custom [`Illuminant`] instances.
pub struct Builder<'a> {
  kind: IlluminantType,
  name: &'a str,
  spd: Option<&'a [(u32, f64)]>,
}

impl<'a> Builder<'a> {
  /// Creates a new illuminant builder with the given name and type.
  pub fn new(name: &'a str, kind: IlluminantType) -> Self {
    Self {
      kind,
      name,
      spd: None,
    }
  }

  /// Builds the illuminant, returning an error if the SPD is missing.
  pub fn build(&self) -> Result<Illuminant, Error> {
    let spd_data: Box<[(u32, f64)]> = self
      .spd
      .ok_or(Error::MissingSpectralPowerDistribution)?
      .iter()
      .copied()
      .collect();

    Ok(Illuminant::new(
      Box::leak(Box::from(self.name)),
      self.kind,
      Spd::new(Box::leak(spd_data)),
    ))
  }

  /// Sets the spectral power distribution data.
  pub fn with_spd(mut self, spd: &'a [(u32, f64)]) -> Self {
    self.spd = Some(spd);
    self
  }

  /// Alias for [`Self::with_spd`].
  pub fn with_spectral_power_distribution(self, spd: &'a [(u32, f64)]) -> Self {
    self.with_spd(spd)
  }
}

/// A standard or custom illuminant (light source) defined by its spectral power distribution.
#[derive(Clone, Copy, Debug)]
pub struct Illuminant {
  kind: IlluminantType,
  name: &'static str,
  spd: Spd,
}

impl Illuminant {
  /// Creates a new [`Builder`] for constructing a custom illuminant.
  pub fn builder<'a>(name: &'a str, kind: IlluminantType) -> Builder<'a> {
    Builder::new(name, kind)
  }

  /// Creates a new illuminant from a name, type, and spectral power distribution.
  pub const fn new(name: &'static str, kind: IlluminantType, spd: Spd) -> Self {
    Self {
      kind,
      name,
      spd,
    }
  }

  /// Returns the illuminant type.
  pub fn kind(&self) -> IlluminantType {
    self.kind
  }

  /// Returns the illuminant name (e.g., "D65", "A").
  pub fn name(&self) -> &'static str {
    self.name
  }

  /// Returns the spectral power distribution.
  pub fn spd(&self) -> Spd {
    self.spd
  }

  /// Alias for [`Self::spd`].
  pub fn spectral_power_distribution(&self) -> Spd {
    self.spd()
  }
}

impl Display for Illuminant {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.name)
  }
}

/// The category of an illuminant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IlluminantType {
  /// A Planckian (blackbody) radiator.
  Blackbody,
  /// A user-defined illuminant.
  Custom,
  /// A CIE daylight illuminant (e.g., D50, D65).
  Daylight,
  /// The equal-energy illuminant (E).
  EqualEnergy,
  /// A fluorescent lamp illuminant.
  Fluorescent,
  /// A gas discharge lamp illuminant.
  GasDischarge,
  /// An incandescent lamp illuminant (e.g., Illuminant A).
  Incandescent,
  /// An LED illuminant.
  Led,
  /// A narrow-band illuminant.
  NarrowBand,
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::spectral::Table;

  static TEST_SPD: &[(u32, f64)] = &[(380, 0.1), (400, 0.5), (420, 0.3)];

  mod builder {
    use super::*;

    mod build {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_builds_illuminant_with_kind_and_spd() {
        let illuminant = Builder::new("Test", IlluminantType::Daylight)
          .with_spd(TEST_SPD)
          .build()
          .unwrap();

        assert_eq!(illuminant.name(), "Test");
        assert_eq!(illuminant.kind(), IlluminantType::Daylight);
        assert_eq!(illuminant.spd().len(), 3);
      }

      #[test]
      fn it_accepts_spectral_power_distribution_alias() {
        let illuminant = Builder::new("Test", IlluminantType::Fluorescent)
          .with_spectral_power_distribution(TEST_SPD)
          .build()
          .unwrap();

        assert_eq!(illuminant.spd().len(), 3);
      }

      #[test]
      fn it_returns_error_without_spd() {
        use crate::error::Error;

        let result = Builder::new("Test", IlluminantType::Daylight).build();

        assert_eq!(result.unwrap_err(), Error::MissingSpectralPowerDistribution);
      }
    }
  }

  mod illuminant {
    use super::*;

    mod display {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_displays_name() {
        let illuminant = Illuminant::D65;

        assert_eq!(format!("{}", illuminant), "D65");
      }
    }

    mod kind {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_returns_illuminant_type() {
        let illuminant = Illuminant::D65;

        assert_eq!(illuminant.kind(), IlluminantType::Daylight);
      }
    }

    mod name {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_returns_illuminant_name() {
        let illuminant = Illuminant::D65;

        assert_eq!(illuminant.name(), "D65");
      }
    }

    mod spd {
      use super::*;

      #[test]
      fn it_returns_spectral_power_distribution() {
        let illuminant = Illuminant::D65;

        assert!(illuminant.spd().len() > 0);
      }
    }

    mod spectral_power_distribution {
      use super::*;

      #[test]
      fn it_is_alias_for_spd() {
        let illuminant = Illuminant::D65;

        assert_eq!(illuminant.spectral_power_distribution().len(), illuminant.spd().len());
      }
    }
  }
}
