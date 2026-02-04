use crate::Cat;

#[derive(Clone, Copy, Debug)]
pub struct ColorimetricContext {
  cat: Cat,
}

impl ColorimetricContext {
  pub const DEFAULT: Self = Self {
    cat: Cat::DEFAULT,
  };

  pub const fn new() -> Self {
    Self {
      cat: Cat::DEFAULT,
    }
  }

  pub fn cat(&self) -> &Cat {
    &self.cat
  }

  pub const fn with_cat(&self, cat: Cat) -> Self {
    Self {
      cat,
    }
  }

  pub const fn with_chromatic_adaptation_transform(&self, cat: Cat) -> Self {
    self.with_cat(cat)
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
}
