use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{Cat, Illuminant, Observer, space::Xyz};

/// Defines the viewing conditions for colorimetric calculations.
///
/// A context combines an [`Illuminant`], [`Observer`], and [`Cat`] (chromatic adaptation
/// transform) to fully specify the conditions under which colors are interpreted.
/// The default context uses D65, CIE 1931 2°, and the Bradford CAT.
#[derive(Clone, Copy, Debug)]
pub struct ColorimetricContext {
  cat: Cat,
  illuminant: Illuminant,
  observer: Observer,
}

impl ColorimetricContext {
  /// The default colorimetric context (D65, CIE 1931 2°, Bradford CAT).
  pub const DEFAULT: Self = Self {
    cat: Cat::DEFAULT,
    illuminant: Illuminant::DEFAULT,
    observer: Observer::DEFAULT,
  };

  /// Creates a new context with default settings.
  pub const fn new() -> Self {
    Self {
      cat: Cat::DEFAULT,
      illuminant: Illuminant::DEFAULT,
      observer: Observer::DEFAULT,
    }
  }

  /// Returns a reference to the chromatic adaptation transform.
  pub fn cat(&self) -> &Cat {
    &self.cat
  }

  /// Returns a reference to the illuminant.
  pub fn illuminant(&self) -> &Illuminant {
    &self.illuminant
  }

  /// Returns a reference to the observer.
  pub fn observer(&self) -> &Observer {
    &self.observer
  }

  /// Returns a human-readable name combining illuminant and observer names.
  pub fn name(&self) -> String {
    format!("{} {}", self.illuminant.name(), self.observer.name())
  }

  /// Calculates the reference white point XYZ by integrating the illuminant SPD with the observer CMF.
  pub fn reference_white(&self) -> Xyz {
    self.observer.cmf().calculate_reference_white(&self.illuminant.spd())
  }

  /// Returns a new context with the given chromatic adaptation transform.
  pub const fn with_cat(&self, cat: Cat) -> Self {
    Self {
      cat,
      ..*self
    }
  }

  /// Alias for [`Self::with_cat`].
  pub const fn with_chromatic_adaptation_transform(&self, cat: Cat) -> Self {
    self.with_cat(cat)
  }

  /// Returns a new context with the given illuminant.
  pub const fn with_illuminant(&self, illuminant: Illuminant) -> Self {
    Self {
      illuminant,
      ..*self
    }
  }

  /// Returns a new context with the given observer.
  pub const fn with_observer(&self, observer: Observer) -> Self {
    Self {
      observer,
      ..*self
    }
  }
}

impl Display for ColorimetricContext {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.name())
  }
}

impl Default for ColorimetricContext {
  fn default() -> Self {
    Self::DEFAULT
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod default {
    use super::*;

    #[test]
    fn it_returns_the_default_context() {
      let ctx = ColorimetricContext::default();

      assert_eq!(ctx.cat().name(), Cat::DEFAULT.name());
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_displays_name() {
      let ctx = ColorimetricContext::default();

      assert_eq!(format!("{}", ctx), ctx.name());
    }
  }

  mod illuminant {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_the_illuminant() {
      let ctx = ColorimetricContext::default();

      assert_eq!(ctx.illuminant().name(), Illuminant::DEFAULT.name());
    }
  }

  mod name {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_illuminant_and_observer_names() {
      let ctx = ColorimetricContext::default();

      assert_eq!(ctx.name(), "D65 CIE 1931 2°");
    }
  }

  mod reference_white {
    use super::*;

    #[test]
    fn it_calculates_reference_white_from_illuminant_and_observer() {
      let ctx = ColorimetricContext::default();
      let white = ctx.reference_white();

      assert!((white.y() - 1.0).abs() < 0.01);
      assert!(white.x() > 0.9);
      assert!(white.z() > 1.0);
    }
  }

  mod with_cat {
    use super::*;

    #[test]
    fn it_returns_context_with_new_cat() {
      let ctx = ColorimetricContext::new();
      let new_ctx = ctx.with_cat(Cat::XYZ_SCALING);

      assert_eq!(new_ctx.cat().name(), "XYZ Scaling");
    }
  }

  mod with_chromatic_adaptation_transform {
    use super::*;

    #[test]
    fn it_is_alias_for_with_cat() {
      let ctx = ColorimetricContext::new();
      let new_ctx = ctx.with_chromatic_adaptation_transform(Cat::XYZ_SCALING);

      assert_eq!(new_ctx.cat().name(), "XYZ Scaling");
    }
  }

  mod with_illuminant {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{illuminant::IlluminantType, spectral::Spd};

    static TEST_SPD: &[(u32, f64)] = &[(380, 100.0), (400, 100.0), (420, 100.0)];

    #[test]
    fn it_returns_context_with_new_illuminant() {
      let illuminant = Illuminant::new("Custom", IlluminantType::Custom, Spd::new(TEST_SPD));
      let ctx = ColorimetricContext::new();
      let new_ctx = ctx.with_illuminant(illuminant);

      assert_eq!(new_ctx.illuminant().name(), "Custom");
    }

    #[test]
    fn it_preserves_other_fields() {
      let illuminant = Illuminant::new("Custom", IlluminantType::Custom, Spd::new(TEST_SPD));
      let ctx = ColorimetricContext::new().with_cat(Cat::XYZ_SCALING);
      let new_ctx = ctx.with_illuminant(illuminant);

      assert_eq!(new_ctx.cat().name(), "XYZ Scaling");
      assert_eq!(new_ctx.illuminant().name(), "Custom");
      assert_eq!(new_ctx.observer().name(), Observer::DEFAULT.name());
    }
  }

  mod with_observer {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::spectral::Table;

    static TEST_CMF: &[(u32, [f64; 3])] = &[(380, [0.001368, 0.000039, 0.006450])];

    #[test]
    fn it_returns_context_with_new_observer() {
      let observer = Observer::builder("Custom", 10.0).with_cmf(TEST_CMF).build().unwrap();
      let ctx = ColorimetricContext::new();
      let new_ctx = ctx.with_observer(observer);

      assert_eq!(new_ctx.observer().name(), "Custom 10°");
    }

    #[test]
    fn it_preserves_other_fields() {
      let observer = Observer::builder("Custom", 10.0).with_cmf(TEST_CMF).build().unwrap();
      let ctx = ColorimetricContext::new().with_cat(Cat::XYZ_SCALING);
      let new_ctx = ctx.with_observer(observer);

      assert_eq!(new_ctx.cat().name(), "XYZ Scaling");
      assert_eq!(new_ctx.observer().name(), "Custom 10°");
      assert_eq!(new_ctx.observer().cmf().len(), 1);
    }
  }
}
