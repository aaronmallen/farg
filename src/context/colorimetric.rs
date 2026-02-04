use crate::{Cat, Observer};

#[derive(Clone, Copy, Debug)]
pub struct ColorimetricContext {
  cat: Cat,
  observer: Observer,
}

impl ColorimetricContext {
  pub const DEFAULT: Self = Self {
    cat: Cat::DEFAULT,
    observer: Observer::DEFAULT,
  };

  pub const fn new() -> Self {
    Self {
      cat: Cat::DEFAULT,
      observer: Observer::DEFAULT,
    }
  }

  pub fn cat(&self) -> &Cat {
    &self.cat
  }

  pub fn observer(&self) -> &Observer {
    &self.observer
  }

  pub const fn with_cat(&self, cat: Cat) -> Self {
    Self {
      cat,
      ..*self
    }
  }

  pub const fn with_chromatic_adaptation_transform(&self, cat: Cat) -> Self {
    self.with_cat(cat)
  }

  pub const fn with_observer(&self, observer: Observer) -> Self {
    Self {
      observer,
      ..*self
    }
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
