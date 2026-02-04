use std::fmt::{Display, Formatter, Result as FmtResult};

use super::{ColorSpace, Lms};
use crate::{ColorimetricContext, chromaticity::Xy, component::Component};

#[derive(Clone, Copy, Debug)]
pub struct Xyz {
  context: ColorimetricContext,
  x: Component,
  y: Component,
  z: Component,
}

impl Xyz {
  pub fn new(x: impl Into<Component>, y: impl Into<Component>, z: impl Into<Component>) -> Self {
    Self {
      context: ColorimetricContext::default(),
      x: x.into(),
      y: y.into(),
      z: z.into(),
    }
  }

  pub const fn new_const(x: f64, y: f64, z: f64) -> Self {
    Self {
      context: ColorimetricContext::DEFAULT,
      x: Component::new_const(x),
      y: Component::new_const(y),
      z: Component::new_const(z),
    }
  }

  pub fn amplified_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.amplify(factor);
    xyz
  }

  pub fn amplify(&mut self, factor: impl Into<Component>) {
    let factor = factor.into();
    self.scale_x(factor);
    self.scale_y(factor);
    self.scale_z(factor);
  }

  pub fn attenuate(&mut self, factor: impl Into<Component>) {
    let factor = factor.into();
    self.x /= factor;
    self.y /= factor;
    self.z /= factor;
  }

  pub fn attenuated_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.attenuate(factor);
    xyz
  }

  pub fn chromaticity(&self) -> Xy {
    let [x, y, z] = self.components();
    let sum = x + y + z;

    if sum == 0.0 {
      Xy::new(0.0, 0.0)
    } else {
      Xy::new(x / sum, y / sum)
    }
  }

  pub fn components(&self) -> [f64; 3] {
    [self.x.0, self.y.0, self.z.0]
  }

  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  pub fn decrement_luminance(&mut self, amount: impl Into<Component>) {
    let luminance = self.y - amount.into();

    if self.y.0 != 0.0 {
      let factor = luminance / self.y;
      self.scale_x(factor);
      self.scale_z(factor);
    }

    self.y = luminance;
  }

  pub fn decrement_x(&mut self, amount: impl Into<Component>) {
    self.x -= amount.into();
  }

  pub fn decrement_y(&mut self, amount: impl Into<Component>) {
    self.y -= amount.into();
  }

  pub fn decrement_z(&mut self, amount: impl Into<Component>) {
    self.z -= amount.into();
  }

  pub fn increment_luminance(&mut self, amount: impl Into<Component>) {
    let luminance = self.y + amount.into();

    if self.y.0 != 0.0 {
      let factor = luminance / self.y;
      self.scale_x(factor);
      self.scale_z(factor);
    }

    self.y = luminance;
  }

  pub fn increment_x(&mut self, amount: impl Into<Component>) {
    self.x += amount.into();
  }

  pub fn increment_y(&mut self, amount: impl Into<Component>) {
    self.y += amount.into();
  }

  pub fn increment_z(&mut self, amount: impl Into<Component>) {
    self.z += amount.into();
  }

  pub fn luminance(&self) -> f64 {
    self.y()
  }

  pub fn scale_luminance(&mut self, factor: impl Into<Component>) {
    self.amplify(factor)
  }

  pub fn scale_x(&mut self, factor: impl Into<Component>) {
    self.x *= factor.into();
  }

  pub fn scale_y(&mut self, factor: impl Into<Component>) {
    self.y *= factor.into();
  }

  pub fn scale_z(&mut self, factor: impl Into<Component>) {
    self.z *= factor.into();
  }

  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_x(components[0].clone());
    self.set_y(components[1].clone());
    self.set_z(components[2].clone());
  }

  pub fn set_luminance(&mut self, luminance: impl Into<Component>) {
    self.set_y(luminance);
  }

  pub fn set_x(&mut self, x: impl Into<Component>) {
    self.x = x.into();
  }

  pub fn set_y(&mut self, y: impl Into<Component>) {
    self.y = y.into();
  }

  pub fn set_z(&mut self, z: impl Into<Component>) {
    self.z = z.into();
  }

  pub fn to_lms(&self) -> Lms {
    Lms::from(self.context.cat().matrix() * self.components()).with_context(self.context)
  }

  pub fn with_context(&self, context: ColorimetricContext) -> Self {
    Self {
      context,
      ..*self
    }
  }

  pub fn with_luminance(&self, luminance: impl Into<Component>) -> Self {
    let luminance = luminance.into();

    if self.y.0 == 0.0 {
      return Self {
        y: luminance,
        ..*self
      };
    }

    let factor = luminance / self.y;

    Self {
      x: self.x * factor,
      y: luminance,
      z: self.z * factor,
      ..*self
    }
  }

  pub fn with_luminance_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.decrement_luminance(amount);
    xyz
  }

  pub fn with_luminance_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.increment_luminance(amount);
    xyz
  }

  pub fn with_luminance_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.scale_luminance(factor);
    xyz
  }

  pub fn with_x(&self, x: impl Into<Component>) -> Self {
    Self {
      x: x.into(),
      ..*self
    }
  }

  pub fn with_y(&self, y: impl Into<Component>) -> Self {
    Self {
      y: y.into(),
      ..*self
    }
  }

  pub fn with_z(&self, z: impl Into<Component>) -> Self {
    Self {
      z: z.into(),
      ..*self
    }
  }

  pub fn with_x_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.decrement_x(amount);
    xyz
  }

  pub fn with_x_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.increment_x(amount);
    xyz
  }

  pub fn with_x_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.scale_x(factor);
    xyz
  }

  pub fn with_y_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.decrement_y(amount);
    xyz
  }

  pub fn with_y_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.increment_y(amount);
    xyz
  }

  pub fn with_y_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.scale_y(factor);
    xyz
  }

  pub fn with_z_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.decrement_z(amount);
    xyz
  }

  pub fn with_z_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.increment_z(amount);
    xyz
  }

  pub fn with_z_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut xyz = *self;
    xyz.scale_z(factor);
    xyz
  }

  pub fn x(&self) -> f64 {
    self.x.0
  }

  pub fn y(&self) -> f64 {
    self.y.0
  }

  pub fn z(&self) -> f64 {
    self.z.0
  }
}

impl ColorSpace<3> for Xyz {
  fn components(&self) -> [f64; 3] {
    self.components()
  }

  fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_components(components)
  }

  fn to_xyz(&self) -> Self {
    *self
  }
}

impl Display for Xyz {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "XYZ({:.precision$}, {:.precision$}, {:.precision$})",
      self.x,
      self.y,
      self.z,
      precision = f.precision().unwrap_or(4)
    )
  }
}

impl<T> From<[T; 3]> for Xyz
where
  T: Into<Component>,
{
  fn from([x, y, z]: [T; 3]) -> Self {
    Self::new(x, y, z)
  }
}

impl From<Lms> for Xyz {
  fn from(lms: Lms) -> Self {
    lms.to_xyz()
  }
}

impl<T> PartialEq<T> for Xyz
where
  T: Into<Xyz> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.x == other.x && self.y == other.y && self.z == other.z
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod amplified_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_scales_all_components_by_factor() {
      let xyz = Xyz::new(0.2, 0.4, 0.1);
      let result = xyz.amplified_by(2.0);

      assert_eq!(result.x(), 0.4);
      assert_eq!(result.y(), 0.8);
      assert_eq!(result.z(), 0.2);
    }

    #[test]
    fn it_accepts_integer_factor() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.amplified_by(3);

      assert_eq!(result.x(), 0.30000000000000004);
      assert_eq!(result.y(), 0.6000000000000001);
      assert_eq!(result.z(), 0.8999999999999999);
    }
  }

  mod amplify {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_mutates_in_place() {
      let mut xyz = Xyz::new(0.2, 0.4, 0.1);
      xyz.amplify(2.0);

      assert_eq!(xyz.x(), 0.4);
      assert_eq!(xyz.y(), 0.8);
      assert_eq!(xyz.z(), 0.2);
    }
  }

  mod attenuate {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_divides_all_components_by_factor() {
      let mut xyz = Xyz::new(0.4, 0.8, 0.2);
      xyz.attenuate(2.0);

      assert_eq!(xyz.x(), 0.2);
      assert_eq!(xyz.y(), 0.4);
      assert_eq!(xyz.z(), 0.1);
    }

    #[test]
    fn it_accepts_integer_factor() {
      let mut xyz = Xyz::new(1.0, 0.5, 0.25);
      xyz.attenuate(2);

      assert_eq!(xyz.x(), 0.5);
      assert_eq!(xyz.y(), 0.25);
      assert_eq!(xyz.z(), 0.125);
    }
  }

  mod attenuated_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_new_attenuated_xyz() {
      let xyz = Xyz::new(0.4, 0.8, 0.2);
      let result = xyz.attenuated_by(2.0);

      assert_eq!(result.x(), 0.2);
      assert_eq!(result.y(), 0.4);
      assert_eq!(result.z(), 0.1);
      assert_eq!(xyz.x(), 0.4);
    }
  }

  mod chromaticity {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::chromaticity::Xy;

    #[test]
    fn it_computes_chromaticity_coordinates() {
      let xyz = Xyz::new(0.95047, 1.0, 1.08883);
      let xy = xyz.chromaticity();
      let sum = 0.95047 + 1.0 + 1.08883;

      assert_eq!(xy.x(), 0.95047 / sum);
      assert_eq!(xy.y(), 1.0 / sum);
    }

    #[test]
    fn it_handles_zero_sum() {
      let xyz = Xyz::new(0.0, 0.0, 0.0);
      let xy = xyz.chromaticity();

      assert_eq!(xy, Xy::new(0.0, 0.0));
    }

    #[test]
    fn it_preserves_chromaticity_at_different_luminances() {
      let xyz1 = Xyz::new(0.95047, 1.0, 1.08883);
      let xyz2 = Xyz::new(0.95047 * 0.5, 1.0 * 0.5, 1.08883 * 0.5);

      assert_eq!(xyz1.chromaticity(), xyz2.chromaticity());
    }
  }

  mod decrement_luminance {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_decrements_y_and_scales_x_z_proportionally() {
      let mut xyz = Xyz::new(0.2, 0.4, 0.1);
      xyz.decrement_luminance(0.2);

      assert_eq!(xyz.y(), 0.2);
      assert_eq!(xyz.x(), 0.1);
      assert_eq!(xyz.z(), 0.05);
    }

    #[test]
    fn it_handles_zero_luminance_without_scaling() {
      let mut xyz = Xyz::new(0.0, 0.0, 0.0);
      xyz.decrement_luminance(0.1);

      assert_eq!(xyz.y(), -0.1);
      assert_eq!(xyz.x(), 0.0);
      assert_eq!(xyz.z(), 0.0);
    }
  }

  mod decrement_x {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_x() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.decrement_x(0.2);

      assert_eq!(xyz.x(), 0.3);
    }
  }

  mod decrement_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_y() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.decrement_y(0.1);

      assert_eq!(xyz.y(), 0.19999999999999998);
    }
  }

  mod decrement_z {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_from_z() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.decrement_z(0.1);

      assert_eq!(xyz.z(), 0.1);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_default_precision() {
      let xyz = Xyz::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{}", xyz), "XYZ(0.1235, 0.2346, 0.3457)");
    }

    #[test]
    fn it_formats_with_custom_precision() {
      let xyz = Xyz::new(0.12345678, 0.23456789, 0.34567890);

      assert_eq!(format!("{:.2}", xyz), "XYZ(0.12, 0.23, 0.35)");
      assert_eq!(format!("{:.6}", xyz), "XYZ(0.123457, 0.234568, 0.345679)");
    }
  }

  mod increment_luminance {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_increments_y_and_scales_x_z_proportionally() {
      let mut xyz = Xyz::new(0.1, 0.2, 0.05);
      xyz.increment_luminance(0.2);

      assert_eq!(xyz.y(), 0.4);
      assert_eq!(xyz.x(), 0.2);
      assert_eq!(xyz.z(), 0.1);
    }

    #[test]
    fn it_handles_zero_luminance_without_scaling() {
      let mut xyz = Xyz::new(0.0, 0.0, 0.0);
      xyz.increment_luminance(0.1);

      assert_eq!(xyz.y(), 0.1);
      assert_eq!(xyz.x(), 0.0);
      assert_eq!(xyz.z(), 0.0);
    }
  }

  mod increment_x {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_x() {
      let mut xyz = Xyz::new(0.1, 0.3, 0.2);
      xyz.increment_x(0.2);

      assert_eq!(xyz.x(), 0.30000000000000004);
    }
  }

  mod increment_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_y() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.increment_y(0.1);

      assert_eq!(xyz.y(), 0.4);
    }
  }

  mod increment_z {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_to_z() {
      let mut xyz = Xyz::new(0.5, 0.3, 0.2);
      xyz.increment_z(0.1);

      assert_eq!(xyz.z(), 0.30000000000000004);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_xyz_values() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.1, 0.2, 0.3);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_xyz_values() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.1, 0.2, 0.4);

      assert_ne!(a, b);
    }

    #[test]
    fn it_compares_with_array() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);

      assert_eq!(xyz, [0.1, 0.2, 0.3]);
    }
  }

  mod scale_x {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_x_by_factor() {
      let mut xyz = Xyz::new(0.2, 0.3, 0.4);
      xyz.scale_x(2.0);

      assert_eq!(xyz.x(), 0.4);
    }
  }

  mod scale_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_y_by_factor() {
      let mut xyz = Xyz::new(0.2, 0.3, 0.4);
      xyz.scale_y(2.0);

      assert_eq!(xyz.y(), 0.6);
    }
  }

  mod scale_z {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_multiplies_z_by_factor() {
      let mut xyz = Xyz::new(0.2, 0.3, 0.4);
      xyz.scale_z(2.0);

      assert_eq!(xyz.z(), 0.8);
    }
  }

  mod with_luminance {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_scaled_components() {
      let xyz = Xyz::new(0.1, 0.2, 0.05);
      let result = xyz.with_luminance(0.4);

      assert_eq!(result.y(), 0.4);
      assert_eq!(result.x(), 0.2);
      assert_eq!(result.z(), 0.1);
    }

    #[test]
    fn it_handles_zero_luminance_by_setting_y_only() {
      let xyz = Xyz::new(0.1, 0.0, 0.05);
      let result = xyz.with_luminance(0.4);

      assert_eq!(result.y(), 0.4);
      assert_eq!(result.x(), 0.1);
      assert_eq!(result.z(), 0.05);
    }
  }

  mod with_luminance_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_decremented_xyz() {
      let xyz = Xyz::new(0.2, 0.4, 0.1);
      let result = xyz.with_luminance_decremented_by(0.2);

      assert_eq!(result.y(), 0.2);
      assert_eq!(xyz.y(), 0.4);
    }
  }

  mod with_luminance_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_incremented_xyz() {
      let xyz = Xyz::new(0.1, 0.2, 0.05);
      let result = xyz.with_luminance_incremented_by(0.2);

      assert_eq!(result.y(), 0.4);
      assert_eq!(xyz.y(), 0.2);
    }
  }

  mod with_luminance_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_scaled_xyz() {
      let xyz = Xyz::new(0.1, 0.2, 0.05);
      let result = xyz.with_luminance_scaled_by(2.0);

      assert_eq!(result.x(), 0.2);
      assert_eq!(result.y(), 0.4);
      assert_eq!(result.z(), 0.1);
    }
  }

  mod with_x {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_new_x() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_x(0.5);

      assert_eq!(result.x(), 0.5);
      assert_eq!(result.y(), 0.2);
      assert_eq!(result.z(), 0.3);
    }
  }

  mod with_x_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_decremented_x() {
      let xyz = Xyz::new(0.5, 0.2, 0.3);
      let result = xyz.with_x_decremented_by(0.2);

      assert_eq!(result.x(), 0.3);
      assert_eq!(xyz.x(), 0.5);
    }
  }

  mod with_x_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_incremented_x() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_x_incremented_by(0.2);

      assert_eq!(result.x(), 0.30000000000000004);
    }
  }

  mod with_x_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_scaled_x() {
      let xyz = Xyz::new(0.2, 0.2, 0.3);
      let result = xyz.with_x_scaled_by(2.0);

      assert_eq!(result.x(), 0.4);
    }
  }

  mod with_y {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_new_y() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_y(0.5);

      assert_eq!(result.x(), 0.1);
      assert_eq!(result.y(), 0.5);
      assert_eq!(result.z(), 0.3);
    }
  }

  mod with_y_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_decremented_y() {
      let xyz = Xyz::new(0.1, 0.5, 0.3);
      let result = xyz.with_y_decremented_by(0.2);

      assert_eq!(result.y(), 0.3);
    }
  }

  mod with_y_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_incremented_y() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_y_incremented_by(0.2);

      assert_eq!(result.y(), 0.4);
    }
  }

  mod with_y_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_scaled_y() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_y_scaled_by(2.0);

      assert_eq!(result.y(), 0.4);
    }
  }

  mod with_z {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_new_z() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_z(0.5);

      assert_eq!(result.x(), 0.1);
      assert_eq!(result.y(), 0.2);
      assert_eq!(result.z(), 0.5);
    }
  }

  mod with_z_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_decremented_z() {
      let xyz = Xyz::new(0.1, 0.2, 0.5);
      let result = xyz.with_z_decremented_by(0.2);

      assert_eq!(result.z(), 0.3);
    }
  }

  mod with_z_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_incremented_z() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_z_incremented_by(0.2);

      assert_eq!(result.z(), 0.5);
    }
  }

  mod with_z_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_xyz_with_scaled_z() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let result = xyz.with_z_scaled_by(2.0);

      assert_eq!(result.z(), 0.6);
    }
  }

  mod to_lms {
    use super::*;

    #[test]
    fn it_converts_to_lms() {
      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let lms = xyz.to_lms();

      assert!(lms.l().is_finite());
      assert!(lms.m().is_finite());
      assert!(lms.s().is_finite());
    }

    #[test]
    fn it_preserves_context() {
      use crate::Cat;

      let xyz = Xyz::new(0.5, 0.5, 0.5);
      let ctx = xyz.context().with_cat(Cat::XYZ_SCALING);
      let xyz_with_ctx = xyz.with_context(ctx);
      let lms = xyz_with_ctx.to_lms();

      assert_eq!(lms.context().cat().name(), "XYZ Scaling");
    }
  }

  mod with_context {
    use super::*;
    use crate::Cat;

    #[test]
    fn it_returns_xyz_with_new_context() {
      let xyz = Xyz::new(0.1, 0.2, 0.3);
      let new_ctx = xyz.context().with_cat(Cat::XYZ_SCALING);
      let result = xyz.with_context(new_ctx);

      assert_eq!(result.context().cat().name(), "XYZ Scaling");
      assert_eq!(result.x(), 0.1);
      assert_eq!(result.y(), 0.2);
      assert_eq!(result.z(), 0.3);
    }
  }
}
