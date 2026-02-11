mod cie_1931_2d;
#[cfg(feature = "observer-cie-1931-judd-2d")]
mod cie_1931_judd_2d;
#[cfg(feature = "observer-cie-1931-judd-vos-2d")]
mod cie_1931_judd_voss_2d;
#[cfg(feature = "observer-cie-1964-10d")]
mod cie_1964_10d;
#[cfg(feature = "observer-cie-2006-10d")]
mod cie_2006_10d;
#[cfg(feature = "observer-cie-2006-2d")]
mod cie_2006_2d;
mod fairchild_modifier;
#[cfg(feature = "observer-stockman-sharpe-10d")]
mod stockman_sharpe_10d;
#[cfg(feature = "observer-stockman-sharpe-2d")]
mod stockman_sharpe_2d;

use std::fmt::{Display, Formatter, Result as FmtResult};

pub use fairchild_modifier::Modifier;

use crate::{
  chromaticity::Xy,
  component::Component,
  error::Error,
  spectral::{ChromaticityCoordinates, Cmf, ConeFundamentals, ConeResponse, TristimulusResponse},
};

/// Builder for constructing custom [`Observer`] instances.
///
/// At minimum, color matching function (CMF) data must be provided. Chromaticity
/// coordinates and cone fundamentals are automatically derived from the CMF if not
/// explicitly set.
pub struct Builder<'a> {
  age: Option<u8>,
  chromaticity_coordinates: Option<&'a [(u32, [f64; 2])]>,
  cmf: Option<&'a [(u32, [f64; 3])]>,
  cone_fundamentals: Option<&'a [(u32, [f64; 3])]>,
  name: &'a str,
  visual_field: f64,
}

impl<'a> Builder<'a> {
  /// Creates a new observer builder with the given name and visual field angle in degrees.
  pub fn new(name: &'a str, visual_field: impl Into<Component>) -> Self {
    Self {
      age: None,
      chromaticity_coordinates: None,
      cmf: None,
      cone_fundamentals: None,
      name,
      visual_field: visual_field.into().0,
    }
  }

  /// Builds the observer, returning an error if CMF data is missing.
  pub fn build(&self) -> Result<Observer, Error> {
    let cmf_data: Box<[(u32, TristimulusResponse)]> = self
      .cmf
      .ok_or(Error::MissingColorMatchingFunction)?
      .iter()
      .map(|(wavelength, [x, y, z])| (*wavelength, TristimulusResponse::new(*x, *y, *z)))
      .collect();
    let cmf = Cmf::new(Box::leak(cmf_data));

    let chromaticity_coordinates = match self.chromaticity_coordinates {
      Some(data) => {
        let chromaticity_coordinates_data: Box<[(u32, Xy)]> = data
          .iter()
          .map(|(wavelength, xy)| (*wavelength, Xy::from(*xy)))
          .collect();
        ChromaticityCoordinates::new(Box::leak(chromaticity_coordinates_data))
      }
      None => ChromaticityCoordinates::from(cmf),
    };

    let cone_fundamentals = match self.cone_fundamentals {
      Some(data) => {
        let cone_fundamentals_data: Box<[(u32, ConeResponse)]> = data
          .iter()
          .map(|(wavelength, [l, m, s])| (*wavelength, ConeResponse::new(*l, *m, *s)))
          .collect();
        ConeFundamentals::new(Box::leak(cone_fundamentals_data))
      }
      None => ConeFundamentals::from(cmf),
    };

    let name: &'static str = Box::leak(self.name.to_owned().into_boxed_str());

    Ok(Observer::new(
      name,
      self.visual_field,
      cmf,
      chromaticity_coordinates,
      cone_fundamentals,
      self.age,
    ))
  }

  /// Sets the observer's age for age-dependent calculations.
  pub fn with_age(mut self, age: u8) -> Self {
    self.age = Some(age);
    self
  }

  /// Sets explicit chromaticity coordinate data, overriding auto-derivation from CMF.
  pub fn with_chromaticity_coordinates(mut self, data: &'a [(u32, [f64; 2])]) -> Self {
    self.chromaticity_coordinates = Some(data);
    self
  }

  /// Sets the color matching function data.
  pub fn with_cmf(mut self, data: &'a [(u32, [f64; 3])]) -> Self {
    self.cmf = Some(data);
    self
  }

  /// Alias for [`Self::with_cmf`].
  pub fn with_color_matching_function(self, data: &'a [(u32, [f64; 3])]) -> Self {
    self.with_cmf(data)
  }

  /// Sets explicit cone fundamentals data, overriding auto-derivation from CMF.
  pub fn with_cone_fundamentals(mut self, data: &'a [(u32, [f64; 3])]) -> Self {
    self.cone_fundamentals = Some(data);
    self
  }
}

/// A standard or custom observer defined by color matching functions.
///
/// Observers model the human visual system's response to light at different wavelengths.
/// Each observer includes CMF data, derived chromaticity coordinates, and cone fundamentals.
#[derive(Clone, Copy, Debug)]
pub struct Observer {
  age: Option<u8>,
  chromaticity_coordinates: ChromaticityCoordinates,
  cmf: Cmf,
  cone_fundamentals: ConeFundamentals,
  pub(crate) name: &'static str,
  visual_field: f64,
}

impl Observer {
  /// Creates a new [`Builder`] for constructing a custom observer.
  pub fn builder<'a>(name: &'a str, visual_field: f64) -> Builder<'a> {
    Builder::new(name, visual_field)
  }

  /// Creates a new observer from all required components.
  pub const fn new(
    name: &'static str,
    visual_field: f64,
    cmf: Cmf,
    chromaticity_coordinates: ChromaticityCoordinates,
    cone_fundamentals: ConeFundamentals,
    age: Option<u8>,
  ) -> Self {
    Self {
      age,
      chromaticity_coordinates,
      cmf,
      cone_fundamentals,
      name,
      visual_field,
    }
  }

  /// Returns the observer's age, if set.
  pub fn age(&self) -> Option<u8> {
    self.age
  }

  /// Returns the spectral chromaticity coordinates.
  pub fn chromaticity_coordinates(&self) -> &ChromaticityCoordinates {
    &self.chromaticity_coordinates
  }

  /// Returns the color matching functions.
  pub fn cmf(&self) -> &Cmf {
    &self.cmf
  }

  /// Alias for [`Self::cmf`].
  pub fn color_matching_function(&self) -> &Cmf {
    self.cmf()
  }

  /// Returns the cone fundamentals (LMS sensitivity data).
  pub fn cone_fundamentals(&self) -> &ConeFundamentals {
    &self.cone_fundamentals
  }

  /// Creates a [`Modifier`] for deriving a new observer with adjusted physiological parameters.
  ///
  /// The modifier applies Fairchild's observer metamerism model to this observer's CMF data.
  /// Chain `with_*` methods to set target viewing conditions, then call [`Modifier::modify`]
  /// to produce the adjusted observer.
  pub fn modifier(&self) -> Modifier {
    Modifier::new(*self)
  }

  /// Returns the formatted observer name (e.g., "CIE 1931 2°").
  pub fn name(&self) -> String {
    if self.visual_field.fract() == 0.0 {
      format!("{} {}°", self.name, self.visual_field as i32)
    } else {
      format!("{} {:.2}°", self.name, self.visual_field)
    }
  }

  /// Returns the visual field angle in degrees.
  pub fn visual_field(&self) -> f64 {
    self.visual_field
  }
}

impl Display for Observer {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.name())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::spectral::Table;

  mod builder {
    use super::*;

    mod build {
      use pretty_assertions::assert_eq;

      use super::*;

      static TEST_CMF: &[(u32, [f64; 3])] = &[
        (380, [0.001368, 0.000039, 0.006450]),
        (390, [0.004243, 0.000120, 0.020050]),
        (400, [0.014310, 0.000396, 0.067850]),
      ];

      #[test]
      fn it_builds_observer_with_cmf() {
        let observer = Builder::new("CIE 1931", 2.0).with_cmf(TEST_CMF).build().unwrap();

        assert_eq!(observer.name(), "CIE 1931 2°");
        assert_eq!(observer.visual_field(), 2.0);
        assert_eq!(observer.cmf().len(), 3);
      }

      #[test]
      fn it_derives_chromaticity_coordinates_from_cmf() {
        let observer = Builder::new("CIE 1931", 2.0).with_cmf(TEST_CMF).build().unwrap();

        assert_eq!(observer.chromaticity_coordinates().len(), 3);
      }

      #[test]
      fn it_derives_cone_fundamentals_from_cmf() {
        let observer = Builder::new("CIE 1931", 2.0).with_cmf(TEST_CMF).build().unwrap();

        assert_eq!(observer.cone_fundamentals().len(), 3);
      }

      #[test]
      fn it_uses_explicit_chromaticity_coordinates() {
        static COORDS: &[(u32, [f64; 2])] = &[(380, [0.1741, 0.0050]), (390, [0.1740, 0.0049])];

        let observer = Builder::new("CIE 1931", 2.0)
          .with_cmf(TEST_CMF)
          .with_chromaticity_coordinates(COORDS)
          .build()
          .unwrap();

        assert_eq!(observer.chromaticity_coordinates().len(), 2);
      }

      #[test]
      fn it_uses_explicit_cone_fundamentals() {
        static CONES: &[(u32, [f64; 3])] = &[(380, [0.001, 0.002, 0.003]), (390, [0.004, 0.005, 0.006])];

        let observer = Builder::new("CIE 1931", 2.0)
          .with_cmf(TEST_CMF)
          .with_cone_fundamentals(CONES)
          .build()
          .unwrap();

        assert_eq!(observer.cone_fundamentals().len(), 2);
      }

      #[test]
      fn it_sets_age() {
        let observer = Builder::new("CIE 2006", 2.0)
          .with_cmf(TEST_CMF)
          .with_age(32)
          .build()
          .unwrap();

        assert_eq!(observer.age(), Some(32));
      }

      #[test]
      fn it_returns_error_without_cmf() {
        use crate::error::Error;

        let result = Builder::new("CIE 1931", 2.0).build();

        assert_eq!(result.unwrap_err(), Error::MissingColorMatchingFunction);
      }
    }
  }

  mod observer {
    use super::*;

    mod display {
      use pretty_assertions::assert_eq;

      use super::*;

      static TEST_CMF: &[(u32, [f64; 3])] = &[(380, [0.001368, 0.000039, 0.006450])];

      #[test]
      fn it_displays_name() {
        let observer = Builder::new("CIE 1931", 2.0).with_cmf(TEST_CMF).build().unwrap();

        assert_eq!(format!("{}", observer), "CIE 1931 2°");
      }
    }

    mod modifier {
      use super::*;

      #[test]
      fn it_returns_modifier_from_observer() {
        let modified = Observer::CIE_1931_2D.modifier().with_visual_field(10.0).modify();

        assert!(modified.name().contains("(Modified)"));
      }
    }

    mod name {
      use pretty_assertions::assert_eq;

      use super::*;

      static TEST_CMF: &[(u32, [f64; 3])] = &[(380, [0.001368, 0.000039, 0.006450])];

      #[test]
      fn it_formats_integer_visual_field() {
        let observer = Builder::new("CIE 1931", 2.0).with_cmf(TEST_CMF).build().unwrap();

        assert_eq!(observer.name(), "CIE 1931 2°");
      }

      #[test]
      fn it_formats_fractional_visual_field() {
        let observer = Builder::new("Custom", 4.5).with_cmf(TEST_CMF).build().unwrap();

        assert_eq!(observer.name(), "Custom 4.50°");
      }

      #[test]
      fn it_formats_ten_degree_field() {
        let observer = Builder::new("CIE 1964", 10.0).with_cmf(TEST_CMF).build().unwrap();

        assert_eq!(observer.name(), "CIE 1964 10°");
      }
    }
  }
}
