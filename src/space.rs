mod xyz;

pub use xyz::Xyz;

use crate::component::Component;

pub trait ColorSpace<const N: usize>: Copy + Clone + From<Xyz> {
  fn amplified_by(&self, factor: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().amplified_by(factor))
  }

  fn amplify(&mut self, factor: impl Into<Component>) {
    self.set_components(self.amplified_by(factor).components())
  }

  fn attenuate(&mut self, factor: impl Into<Component>) {
    self.set_components(self.attenuated_by(factor).components())
  }

  fn attenuated_by(&self, factor: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().attenuated_by(factor))
  }

  fn components(&self) -> [f64; N];

  fn decrement_luminance(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_luminance_decremented_by(amount).components())
  }

  fn increment_luminance(&mut self, amount: impl Into<Component>) {
    self.set_components(self.with_luminance_incremented_by(amount).components())
  }

  fn luminance(&self) -> f64 {
    self.to_xyz().luminance()
  }

  fn scale_luminance(&mut self, factor: impl Into<Component>) {
    self.set_components(self.with_luminance_scaled_by(factor).components())
  }

  fn set_components(&mut self, components: [impl Into<Component> + Clone; N]);

  fn set_luminance(&mut self, luminance: impl Into<Component>) {
    self.set_components(self.with_luminance(luminance).components())
  }

  fn to_xyz(&self) -> Xyz;

  fn with_luminance(&self, luminance: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().with_luminance(luminance))
  }

  fn with_luminance_decremented_by(&self, amount: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().with_luminance_decremented_by(amount))
  }

  fn with_luminance_incremented_by(&self, amount: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().with_luminance_incremented_by(amount))
  }

  fn with_luminance_scaled_by(&self, factor: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().with_luminance_scaled_by(factor))
  }
}
