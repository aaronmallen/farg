#[cfg(feature = "rgb-aces-2065-1")]
mod aces_2065_1;
#[cfg(feature = "rgb-aces-cc")]
mod aces_cc;
#[cfg(feature = "rgb-aces-cct")]
mod aces_cct;
#[cfg(feature = "rgb-aces-cg")]
mod aces_cg;
#[cfg(feature = "rgb-adobe-rgb")]
mod adobe_rgb;
#[cfg(feature = "rgb-apple-rgb")]
mod apple_rgb;
#[cfg(feature = "rgb-arri-wide-gamut-3")]
mod arri_wide_gamut_3;
#[cfg(feature = "rgb-arri-wide-gamut-4")]
mod arri_wide_gamut_4;
#[cfg(feature = "rgb-best-rgb")]
mod best_rgb;
#[cfg(feature = "rgb-beta-rgb")]
mod beta_rgb;
#[cfg(feature = "rgb-blackmagic-wide-gamut")]
mod blackmagic_wide_gamut;
#[cfg(feature = "rgb-bruce-rgb")]
mod bruce_rgb;
#[cfg(feature = "rgb-canon-cinema-gamut")]
mod canon_cinema_gamut;
#[cfg(feature = "rgb-cie-rgb")]
mod cie_rgb;
#[cfg(feature = "rgb-colormatch-rgb")]
mod colormatch_rgb;
#[cfg(feature = "rgb-davinci-wide-gamut")]
mod davinci_wide_gamut;
#[cfg(feature = "rgb-dci-p3")]
mod dci_p3;
#[cfg(feature = "rgb-display-p3")]
mod display_p3;
#[cfg(feature = "rgb-don-rgb-4")]
mod don_rgb_4;
#[cfg(feature = "rgb-eci-rgb-v2")]
mod eci_rgb;
#[cfg(feature = "rgb-ektargb-ps5")]
mod ektaspace_ps5;
#[cfg(feature = "rgb-filmlight-e-gamut")]
mod filmlight_egamut;
#[cfg(feature = "rgb-linear-srgb")]
mod linear_srgb;
#[cfg(feature = "rgb-ntsc")]
mod ntsc;
#[cfg(feature = "rgb-pal-secam")]
mod pal_secam;
#[cfg(feature = "rgb-panasonic-v-gamut")]
mod panasonic_vgamut;
#[cfg(feature = "rgb-prophoto-rgb")]
mod prophoto_rgb;
#[cfg(feature = "rgb-rec-2020")]
mod rec2020;
#[cfg(feature = "rgb-rec-2100-hlg")]
mod rec2100_hlg;
#[cfg(feature = "rgb-rec-2100-pq")]
mod rec2100_pq;
#[cfg(feature = "rgb-rec-601")]
mod rec601;
#[cfg(feature = "rgb-rec-709")]
mod rec709;
#[cfg(feature = "rgb-red-wide-gamut-rgb")]
mod red_wide_gamut;
#[cfg(feature = "rgb-scrgb")]
mod scrgb;
#[cfg(feature = "rgb-smpte-c")]
mod smpte_c;
#[cfg(feature = "rgb-sony-s-gamut-3")]
mod sony_sgamut3;
#[cfg(feature = "rgb-sony-s-gamut-3-cine")]
mod sony_sgamut3_cine;
mod standard;
#[cfg(feature = "rgb-wide-gamut-rgb")]
mod wide_gamut_rgb;

use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  marker::PhantomData,
  ops::{Add, Div, Mul, Sub},
};

#[cfg(feature = "rgb-aces-2065-1")]
pub use aces_2065_1::Aces2065_1;
#[cfg(feature = "rgb-aces-cc")]
pub use aces_cc::AcesCc;
#[cfg(feature = "rgb-aces-cct")]
pub use aces_cct::AcesCct;
#[cfg(feature = "rgb-aces-cg")]
pub use aces_cg::AcesCg;
#[cfg(feature = "rgb-adobe-rgb")]
pub use adobe_rgb::AdobeRgb;
#[cfg(feature = "rgb-apple-rgb")]
pub use apple_rgb::AppleRgb;
#[cfg(feature = "rgb-arri-wide-gamut-3")]
pub use arri_wide_gamut_3::ArriWideGamut3;
#[cfg(feature = "rgb-arri-wide-gamut-4")]
pub use arri_wide_gamut_4::ArriWideGamut4;
#[cfg(feature = "rgb-best-rgb")]
pub use best_rgb::BestRgb;
#[cfg(feature = "rgb-beta-rgb")]
pub use beta_rgb::BetaRgb;
#[cfg(feature = "rgb-blackmagic-wide-gamut")]
pub use blackmagic_wide_gamut::BlackmagicWideGamut;
#[cfg(feature = "rgb-bruce-rgb")]
pub use bruce_rgb::BruceRgb;
#[cfg(feature = "rgb-canon-cinema-gamut")]
pub use canon_cinema_gamut::CanonCinemaGamut;
#[cfg(feature = "rgb-cie-rgb")]
pub use cie_rgb::CieRgb;
#[cfg(feature = "rgb-colormatch-rgb")]
pub use colormatch_rgb::ColorMatchRgb;
#[cfg(feature = "rgb-davinci-wide-gamut")]
pub use davinci_wide_gamut::DaVinciWideGamut;
#[cfg(feature = "rgb-dci-p3")]
pub use dci_p3::DciP3;
#[cfg(feature = "rgb-display-p3")]
pub use display_p3::DisplayP3;
#[cfg(feature = "rgb-don-rgb-4")]
pub use don_rgb_4::DonRgb4;
#[cfg(feature = "rgb-eci-rgb-v2")]
pub use eci_rgb::EciRgbV2;
#[cfg(feature = "rgb-ektargb-ps5")]
pub use ektaspace_ps5::EktaSpacePs5;
#[cfg(feature = "rgb-filmlight-e-gamut")]
pub use filmlight_egamut::FilmlightEGamut;
#[cfg(feature = "rgb-linear-srgb")]
pub use linear_srgb::LinearSrgb;
#[cfg(feature = "rgb-ntsc")]
pub use ntsc::Ntsc;
#[cfg(feature = "rgb-pal-secam")]
pub use pal_secam::PalSecam;
#[cfg(feature = "rgb-panasonic-v-gamut")]
pub use panasonic_vgamut::PanasonicVGamut;
#[cfg(feature = "rgb-prophoto-rgb")]
pub use prophoto_rgb::ProPhotoRgb;
#[cfg(feature = "rgb-rec-601")]
pub use rec601::Rec601;
#[cfg(feature = "rgb-rec-709")]
pub use rec709::Rec709;
#[cfg(feature = "rgb-rec-2020")]
pub use rec2020::Rec2020;
#[cfg(feature = "rgb-rec-2100-hlg")]
pub use rec2100_hlg::Rec2100Hlg;
#[cfg(feature = "rgb-rec-2100-pq")]
pub use rec2100_pq::Rec2100Pq;
#[cfg(feature = "rgb-red-wide-gamut-rgb")]
pub use red_wide_gamut::RedWideGamutRgb;
#[cfg(feature = "rgb-scrgb")]
pub use scrgb::ScRgb;
#[cfg(feature = "rgb-smpte-c")]
pub use smpte_c::SmpteC;
#[cfg(feature = "rgb-sony-s-gamut-3")]
pub use sony_sgamut3::SonySGamut3;
#[cfg(feature = "rgb-sony-s-gamut-3-cine")]
pub use sony_sgamut3_cine::SonySGamut3Cine;
pub use standard::Srgb;
#[cfg(feature = "rgb-wide-gamut-rgb")]
pub use wide_gamut_rgb::WideGamutRgb;

use super::{LinearRgb, RgbSpec};
use crate::{
  ColorimetricContext, Error,
  component::Component,
  space::{ColorSpace, Lms, Xyz},
};

#[derive(Clone, Copy, Debug)]
pub struct Rgb<S = Srgb>
where
  S: RgbSpec,
{
  b: Component,
  context: ColorimetricContext,
  g: Component,
  r: Component,
  _spec: PhantomData<S>,
}

impl<S> Rgb<S>
where
  S: RgbSpec,
{
  pub fn from_hexcode(hexcode: impl Into<String>) -> Result<Self, Error> {
    let hexcode = hexcode.into();
    let hex = hexcode.strip_prefix('#').unwrap_or(&hexcode);

    let (r, g, b) = match hex.len() {
      3 => {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| Error::InvalidHexCharacter {
          input: hexcode.clone(),
        })?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| Error::InvalidHexCharacter {
          input: hexcode.clone(),
        })?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| Error::InvalidHexCharacter {
          input: hexcode.clone(),
        })?;
        (r, g, b)
      }
      6 => {
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| Error::InvalidHexCharacter {
          input: hexcode.clone(),
        })?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| Error::InvalidHexCharacter {
          input: hexcode.clone(),
        })?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| Error::InvalidHexCharacter {
          input: hexcode.clone(),
        })?;
        (r, g, b)
      }
      len => {
        return Err(Error::InvalidHexLength {
          input: hexcode,
          length: len,
        });
      }
    };

    Ok(Self::new(r, g, b))
  }

  pub fn from_normalized(r: impl Into<Component>, g: impl Into<Component>, b: impl Into<Component>) -> Self {
    Self {
      b: b.into().clamp(0.0, 1.0),
      context: S::CONTEXT,
      g: g.into().clamp(0.0, 1.0),
      r: r.into().clamp(0.0, 1.0),
      _spec: PhantomData,
    }
  }

  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self {
      b: Component::from(b) / 255.0,
      context: S::CONTEXT,
      g: Component::from(g) / 255.0,
      r: Component::from(r) / 255.0,
      _spec: PhantomData,
    }
  }

  pub const fn new_const(r: u8, g: u8, b: u8) -> Self {
    let r = Component::new_const(r as f64 / 255.0);
    let g = Component::new_const(g as f64 / 255.0);
    let b = Component::new_const(b as f64 / 255.0);

    Self {
      b,
      context: S::CONTEXT,
      g,
      r,
      _spec: PhantomData,
    }
  }

  pub fn b(&self) -> f64 {
    self.b.0
  }

  pub fn blue(&self) -> u8 {
    (self.b.0 * 255.0).round() as u8
  }

  pub fn components(&self) -> [f64; 3] {
    [self.r.0, self.g.0, self.b.0]
  }

  pub fn context(&self) -> &ColorimetricContext {
    &self.context
  }

  pub fn g(&self) -> f64 {
    self.g.0
  }

  pub fn green(&self) -> u8 {
    (self.g.0 * 255.0).round() as u8
  }

  pub fn r(&self) -> f64 {
    self.r.0
  }

  pub fn red(&self) -> u8 {
    (self.r.0 * 255.0).round() as u8
  }

  pub fn decrement_b(&mut self, amount: impl Into<Component>) {
    self.b = (self.b - amount.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn decrement_blue(&mut self, amount: impl Into<Component>) {
    self.decrement_b(amount);
  }

  pub fn decrement_g(&mut self, amount: impl Into<Component>) {
    self.g = (self.g - amount.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn decrement_green(&mut self, amount: impl Into<Component>) {
    self.decrement_g(amount);
  }

  pub fn decrement_r(&mut self, amount: impl Into<Component>) {
    self.r = (self.r - amount.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn decrement_red(&mut self, amount: impl Into<Component>) {
    self.decrement_r(amount);
  }

  pub fn increment_b(&mut self, amount: impl Into<Component>) {
    self.b = (self.b + amount.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn increment_blue(&mut self, amount: impl Into<Component>) {
    self.increment_b(amount);
  }

  pub fn increment_g(&mut self, amount: impl Into<Component>) {
    self.g = (self.g + amount.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn increment_green(&mut self, amount: impl Into<Component>) {
    self.increment_g(amount);
  }

  pub fn increment_r(&mut self, amount: impl Into<Component>) {
    self.r = (self.r + amount.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn increment_red(&mut self, amount: impl Into<Component>) {
    self.increment_r(amount);
  }

  pub fn set_b(&mut self, b: impl Into<Component>) {
    self.b = (b.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn set_blue(&mut self, blue: impl Into<Component>) {
    self.set_b(blue);
  }

  pub fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_r(components[0].clone());
    self.set_g(components[1].clone());
    self.set_b(components[2].clone());
  }

  pub fn set_g(&mut self, g: impl Into<Component>) {
    self.g = (g.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn set_green(&mut self, green: impl Into<Component>) {
    self.set_g(green);
  }

  pub fn set_r(&mut self, r: impl Into<Component>) {
    self.r = (r.into() / 255.0).clamp(0.0, 1.0);
  }

  pub fn set_red(&mut self, red: impl Into<Component>) {
    self.set_r(red);
  }

  pub fn scale_b(&mut self, factor: impl Into<Component>) {
    self.b = (self.b * factor.into()).clamp(0.0, 1.0);
  }

  pub fn scale_blue(&mut self, factor: impl Into<Component>) {
    self.scale_b(factor);
  }

  pub fn scale_g(&mut self, factor: impl Into<Component>) {
    self.g = (self.g * factor.into()).clamp(0.0, 1.0);
  }

  pub fn scale_green(&mut self, factor: impl Into<Component>) {
    self.scale_g(factor);
  }

  pub fn scale_r(&mut self, factor: impl Into<Component>) {
    self.r = (self.r * factor.into()).clamp(0.0, 1.0);
  }

  pub fn scale_red(&mut self, factor: impl Into<Component>) {
    self.scale_r(factor);
  }

  pub fn to_linear(&self) -> LinearRgb<S> {
    let r = S::TRANSFER_FUNCTION.decode(self.r);
    let g = S::TRANSFER_FUNCTION.decode(self.g);
    let b = S::TRANSFER_FUNCTION.decode(self.b);
    LinearRgb::from_normalized(r, g, b)
  }

  pub fn to_rgb<OS>(&self) -> Rgb<OS>
  where
    OS: RgbSpec,
  {
    if S::NAME == OS::NAME {
      Rgb::<OS>::from_normalized(self.r(), self.g(), self.b())
    } else {
      self.to_xyz().to_rgb::<OS>()
    }
  }

  pub fn to_lms(&self) -> Lms {
    self.to_xyz().to_lms()
  }

  pub fn to_xyz(&self) -> Xyz {
    let linear = self.to_linear();
    let [x, y, z] = *S::xyz_matrix() * linear.components();
    Xyz::new(x, y, z).with_context(self.context)
  }

  pub fn with_b(&self, b: impl Into<Component>) -> Self {
    Self {
      b: (b.into() / 255.0).clamp(0.0, 1.0),
      ..*self
    }
  }

  pub fn with_b_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.decrement_b(amount);
    rgb
  }

  pub fn with_b_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.increment_b(amount);
    rgb
  }

  pub fn with_b_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.scale_b(factor);
    rgb
  }

  pub fn with_blue(&self, blue: impl Into<Component>) -> Self {
    self.with_b(blue)
  }

  pub fn with_blue_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_b_decremented_by(amount)
  }

  pub fn with_blue_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_b_incremented_by(amount)
  }

  pub fn with_blue_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_b_scaled_by(factor)
  }

  pub fn with_g(&self, g: impl Into<Component>) -> Self {
    Self {
      g: (g.into() / 255.0).clamp(0.0, 1.0),
      ..*self
    }
  }

  pub fn with_g_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.decrement_g(amount);
    rgb
  }

  pub fn with_g_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.increment_g(amount);
    rgb
  }

  pub fn with_g_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.scale_g(factor);
    rgb
  }

  pub fn with_green(&self, green: impl Into<Component>) -> Self {
    self.with_g(green)
  }

  pub fn with_green_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_g_decremented_by(amount)
  }

  pub fn with_green_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_g_incremented_by(amount)
  }

  pub fn with_green_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_g_scaled_by(factor)
  }

  pub fn with_r(&self, r: impl Into<Component>) -> Self {
    Self {
      r: (r.into() / 255.0).clamp(0.0, 1.0),
      ..*self
    }
  }

  pub fn with_r_decremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.decrement_r(amount);
    rgb
  }

  pub fn with_r_incremented_by(&self, amount: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.increment_r(amount);
    rgb
  }

  pub fn with_r_scaled_by(&self, factor: impl Into<Component>) -> Self {
    let mut rgb = *self;
    rgb.scale_r(factor);
    rgb
  }

  pub fn with_red(&self, red: impl Into<Component>) -> Self {
    self.with_r(red)
  }

  pub fn with_red_decremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_r_decremented_by(amount)
  }

  pub fn with_red_incremented_by(&self, amount: impl Into<Component>) -> Self {
    self.with_r_incremented_by(amount)
  }

  pub fn with_red_scaled_by(&self, factor: impl Into<Component>) -> Self {
    self.with_r_scaled_by(factor)
  }
}

impl<S> Add for Rgb<S>
where
  S: RgbSpec,
{
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    let r = (self.r + rhs.r).clamp(0.0, 1.0);
    let g = (self.g + rhs.g).clamp(0.0, 1.0);
    let b = (self.b + rhs.b).clamp(0.0, 1.0);
    Self::from_normalized(r, g, b)
  }
}

impl<S> ColorSpace<3> for Rgb<S>
where
  S: RgbSpec,
{
  fn components(&self) -> [f64; 3] {
    self.components()
  }

  fn set_components(&mut self, components: [impl Into<Component> + Clone; 3]) {
    self.set_components(components)
  }

  fn to_rgb<OS>(&self) -> Rgb<OS>
  where
    OS: RgbSpec,
  {
    self.to_rgb::<OS>()
  }

  fn to_xyz(&self) -> Xyz {
    self.to_xyz()
  }
}

impl<S> Display for Rgb<S>
where
  S: RgbSpec,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}({}, {}, {})", S::NAME, self.red(), self.green(), self.blue())
  }
}

impl<S> Div for Rgb<S>
where
  S: RgbSpec,
{
  type Output = Self;

  fn div(self, rhs: Self) -> Self {
    let r = (self.r / rhs.r).clamp(0.0, 1.0);
    let g = (self.g / rhs.g).clamp(0.0, 1.0);
    let b = (self.b / rhs.b).clamp(0.0, 1.0);
    Self::from_normalized(r, g, b)
  }
}

impl<S, T> From<[T; 3]> for Rgb<S>
where
  S: RgbSpec,
  T: Into<Component>,
{
  fn from([r, g, b]: [T; 3]) -> Self {
    Self::from_normalized(r, g, b)
  }
}

impl<S> From<Lms> for Rgb<S>
where
  S: RgbSpec,
{
  fn from(lms: Lms) -> Self {
    lms.to_rgb::<S>()
  }
}

impl<S> From<Xyz> for Rgb<S>
where
  S: RgbSpec,
{
  fn from(xyz: Xyz) -> Self {
    xyz.to_rgb::<S>()
  }
}

impl<S> Mul for Rgb<S>
where
  S: RgbSpec,
{
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    let r = (self.r * rhs.r).clamp(0.0, 1.0);
    let g = (self.g * rhs.g).clamp(0.0, 1.0);
    let b = (self.b * rhs.b).clamp(0.0, 1.0);
    Self::from_normalized(r, g, b)
  }
}

impl<S, T> PartialEq<T> for Rgb<S>
where
  S: RgbSpec,
  T: Into<Rgb<S>> + Copy,
{
  fn eq(&self, other: &T) -> bool {
    let other = (*other).into();
    self.red() == other.red() && self.green() == other.green() && self.blue() == other.blue()
  }
}

impl<S> Sub for Rgb<S>
where
  S: RgbSpec,
{
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    let r = (self.r - rhs.r).clamp(0.0, 1.0);
    let g = (self.g - rhs.g).clamp(0.0, 1.0);
    let b = (self.b - rhs.b).clamp(0.0, 1.0);
    Self::from_normalized(r, g, b)
  }
}

impl<S> TryFrom<&str> for Rgb<S>
where
  S: RgbSpec,
{
  type Error = crate::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Self::from_hexcode(value)
  }
}

impl<S> TryFrom<String> for Rgb<S>
where
  S: RgbSpec,
{
  type Error = crate::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::from_hexcode(value)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod default_type_parameter {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_defaults_to_srgb_with_type_annotation() {
      let rgb: Rgb = Rgb::new(255, 0, 0);

      assert_eq!(format!("{}", rgb), "sRGB(255, 0, 0)");
    }

    #[test]
    fn it_defaults_to_srgb_for_from_hexcode() {
      let rgb: Rgb = Rgb::from_hexcode("#FF0000").unwrap();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }

    #[test]
    fn it_defaults_to_srgb_for_from_normalized() {
      let rgb: Rgb = Rgb::from_normalized(1.0, 0.0, 0.0);

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 0);
      assert_eq!(rgb.blue(), 0);
    }
  }

  mod add {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_two_rgb_values() {
      let a = Rgb::<Srgb>::from_normalized(0.2, 0.3, 0.4);
      let b = Rgb::<Srgb>::from_normalized(0.1, 0.2, 0.3);
      let result = a + b;

      assert!((result.r() - 0.3).abs() < 1e-10);
      assert!((result.g() - 0.5).abs() < 1e-10);
      assert!((result.b() - 0.7).abs() < 1e-10);
    }

    #[test]
    fn it_clamps_result_to_1() {
      let a = Rgb::<Srgb>::from_normalized(0.7, 0.8, 0.9);
      let b = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let result = a + b;

      assert_eq!(result.r(), 1.0);
      assert_eq!(result.g(), 1.0);
      assert_eq!(result.b(), 1.0);
    }
  }

  mod decrement_b {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_decrements_blue_by_amount() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 128);
      rgb.decrement_b(64);

      assert_eq!(rgb.blue(), 64);
    }

    #[test]
    fn it_clamps_to_zero() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 32);
      rgb.decrement_b(64);

      assert_eq!(rgb.blue(), 0);
    }
  }

  mod decrement_g {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_decrements_green_by_amount() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 128);
      rgb.decrement_g(64);

      assert_eq!(rgb.green(), 64);
    }

    #[test]
    fn it_clamps_to_zero() {
      let mut rgb = Rgb::<Srgb>::new(128, 32, 128);
      rgb.decrement_g(64);

      assert_eq!(rgb.green(), 0);
    }
  }

  mod decrement_r {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_decrements_red_by_amount() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 128);
      rgb.decrement_r(64);

      assert_eq!(rgb.red(), 64);
    }

    #[test]
    fn it_clamps_to_zero() {
      let mut rgb = Rgb::<Srgb>::new(32, 128, 128);
      rgb.decrement_r(64);

      assert_eq!(rgb.red(), 0);
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_with_space_name_and_8bit_values() {
      let rgb = Rgb::<Srgb>::new(255, 128, 64);

      assert_eq!(format!("{}", rgb), "sRGB(255, 128, 64)");
    }
  }

  mod div {
    use super::*;

    #[test]
    fn it_divides_two_rgb_values() {
      let a = Rgb::<Srgb>::from_normalized(0.8, 0.6, 0.4);
      let b = Rgb::<Srgb>::from_normalized(0.4, 0.3, 0.2);
      let result = a / b;

      assert!((result.r() - 1.0).abs() < 1e-10);
      assert!((result.g() - 1.0).abs() < 1e-10);
      assert!((result.b() - 1.0).abs() < 1e-10);
    }
  }

  mod from_hexcode {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses_6_digit_hex_with_hash() {
      let rgb = Rgb::<Srgb>::from_hexcode("#FF8040").unwrap();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 128);
      assert_eq!(rgb.blue(), 64);
    }

    #[test]
    fn it_parses_6_digit_hex_without_hash() {
      let rgb = Rgb::<Srgb>::from_hexcode("FF8040").unwrap();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 128);
      assert_eq!(rgb.blue(), 64);
    }

    #[test]
    fn it_parses_3_digit_shorthand_with_hash() {
      let rgb = Rgb::<Srgb>::from_hexcode("#F84").unwrap();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 136);
      assert_eq!(rgb.blue(), 68);
    }

    #[test]
    fn it_parses_3_digit_shorthand_without_hash() {
      let rgb = Rgb::<Srgb>::from_hexcode("F84").unwrap();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 136);
      assert_eq!(rgb.blue(), 68);
    }

    #[test]
    fn it_parses_lowercase_hex() {
      let rgb = Rgb::<Srgb>::from_hexcode("#ff8040").unwrap();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 128);
      assert_eq!(rgb.blue(), 64);
    }

    #[test]
    fn it_returns_error_for_invalid_length() {
      let result = Rgb::<Srgb>::from_hexcode("#FF80");

      assert_eq!(
        result.unwrap_err(),
        crate::Error::InvalidHexLength {
          input: "#FF80".to_string(),
          length: 4
        }
      );
    }

    #[test]
    fn it_returns_error_for_invalid_characters() {
      let result = Rgb::<Srgb>::from_hexcode("#GGHHII");

      assert_eq!(
        result.unwrap_err(),
        crate::Error::InvalidHexCharacter {
          input: "#GGHHII".to_string()
        }
      );
    }

    #[test]
    fn it_parses_black_and_white() {
      let black = Rgb::<Srgb>::from_hexcode("#000000").unwrap();
      let white = Rgb::<Srgb>::from_hexcode("#FFFFFF").unwrap();

      assert_eq!(black.red(), 0);
      assert_eq!(black.green(), 0);
      assert_eq!(black.blue(), 0);
      assert_eq!(white.red(), 255);
      assert_eq!(white.green(), 255);
      assert_eq!(white.blue(), 255);
    }
  }

  mod from_normalized {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_clamps_values_to_0_1_range() {
      let rgb = Rgb::<Srgb>::from_normalized(1.5, -0.5, 0.5);

      assert_eq!(rgb.r(), 1.0);
      assert_eq!(rgb.g(), 0.0);
      assert_eq!(rgb.b(), 0.5);
    }
  }

  mod from_array {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_creates_rgb_from_normalized_array() {
      let rgb: Rgb<Srgb> = [0.5, 0.25, 0.75].into();

      assert_eq!(rgb.r(), 0.5);
      assert_eq!(rgb.g(), 0.25);
      assert_eq!(rgb.b(), 0.75);
    }
  }

  mod increment_b {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_increments_blue_by_amount() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 64);
      rgb.increment_b(64);

      assert_eq!(rgb.blue(), 128);
    }

    #[test]
    fn it_clamps_to_255() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 200);
      rgb.increment_b(100);

      assert_eq!(rgb.blue(), 255);
    }
  }

  mod increment_g {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_increments_green_by_amount() {
      let mut rgb = Rgb::<Srgb>::new(128, 64, 128);
      rgb.increment_g(64);

      assert_eq!(rgb.green(), 128);
    }

    #[test]
    fn it_clamps_to_255() {
      let mut rgb = Rgb::<Srgb>::new(128, 200, 128);
      rgb.increment_g(100);

      assert_eq!(rgb.green(), 255);
    }
  }

  mod increment_r {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_increments_red_by_amount() {
      let mut rgb = Rgb::<Srgb>::new(64, 128, 128);
      rgb.increment_r(64);

      assert_eq!(rgb.red(), 128);
    }

    #[test]
    fn it_clamps_to_255() {
      let mut rgb = Rgb::<Srgb>::new(200, 128, 128);
      rgb.increment_r(100);

      assert_eq!(rgb.red(), 255);
    }
  }

  mod mul {
    use super::*;

    #[test]
    fn it_multiplies_two_rgb_values() {
      let a = Rgb::<Srgb>::from_normalized(0.5, 0.4, 0.8);
      let b = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let result = a * b;

      assert!((result.r() - 0.25).abs() < 1e-10);
      assert!((result.g() - 0.2).abs() < 1e-10);
      assert!((result.b() - 0.4).abs() < 1e-10);
    }
  }

  mod partial_eq {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn it_compares_equal_rgb_values() {
      let a = Rgb::<Srgb>::new(128, 64, 32);
      let b = Rgb::<Srgb>::new(128, 64, 32);

      assert_eq!(a, b);
    }

    #[test]
    fn it_compares_unequal_rgb_values() {
      let a = Rgb::<Srgb>::new(128, 64, 32);
      let b = Rgb::<Srgb>::new(128, 64, 33);

      assert_ne!(a, b);
    }
  }

  mod scale_b {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_scales_blue_by_factor() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 128);
      rgb.scale_b(0.5);

      assert_eq!(rgb.blue(), 64);
    }

    #[test]
    fn it_clamps_to_1() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 200);
      rgb.scale_b(2.0);

      assert_eq!(rgb.blue(), 255);
    }
  }

  mod scale_g {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_scales_green_by_factor() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 128);
      rgb.scale_g(0.5);

      assert_eq!(rgb.green(), 64);
    }
  }

  mod scale_r {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_scales_red_by_factor() {
      let mut rgb = Rgb::<Srgb>::new(128, 128, 128);
      rgb.scale_r(0.5);

      assert_eq!(rgb.red(), 64);
    }
  }

  mod sub {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_subtracts_two_rgb_values() {
      let a = Rgb::<Srgb>::from_normalized(0.8, 0.6, 0.4);
      let b = Rgb::<Srgb>::from_normalized(0.3, 0.2, 0.1);
      let result = a - b;

      assert!((result.r() - 0.5).abs() < 1e-10);
      assert!((result.g() - 0.4).abs() < 1e-10);
      assert!((result.b() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn it_clamps_result_to_0() {
      let a = Rgb::<Srgb>::from_normalized(0.2, 0.2, 0.2);
      let b = Rgb::<Srgb>::from_normalized(0.5, 0.5, 0.5);
      let result = a - b;

      assert_eq!(result.r(), 0.0);
      assert_eq!(result.g(), 0.0);
      assert_eq!(result.b(), 0.0);
    }
  }

  mod to_linear {
    use super::*;

    #[test]
    fn it_converts_to_linear_rgb() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let linear = rgb.to_linear();

      assert!(linear.r() < rgb.r());
      assert!(linear.g() < rgb.g());
      assert!(linear.b() < rgb.b());
    }

    #[test]
    fn it_roundtrips_with_to_encoded() {
      let original = Rgb::<Srgb>::new(128, 64, 200);
      let linear = original.to_linear();
      let back = linear.to_encoded();

      assert_eq!(back.red(), original.red());
      assert_eq!(back.green(), original.green());
      assert_eq!(back.blue(), original.blue());
    }
  }

  mod to_lms {
    use super::*;

    #[test]
    fn it_converts_to_lms_via_xyz() {
      let rgb = Rgb::<Srgb>::new(200, 100, 50);
      let lms = rgb.to_lms();

      assert!(lms.l().is_finite());
      assert!(lms.m().is_finite());
      assert!(lms.s().is_finite());
    }

    #[test]
    fn it_roundtrips_with_from_lms() {
      let original = Rgb::<Srgb>::new(200, 100, 50);
      let lms = original.to_lms();
      let back: Rgb<Srgb> = Rgb::from(lms);

      assert_eq!(back.red(), original.red());
      assert_eq!(back.green(), original.green());
      assert_eq!(back.blue(), original.blue());
    }
  }

  mod to_rgb {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_same_values_for_same_space() {
      let rgb = Rgb::<Srgb>::new(200, 100, 50);
      let result: Rgb<Srgb> = rgb.to_rgb();

      assert_eq!(result.red(), rgb.red());
      assert_eq!(result.green(), rgb.green());
      assert_eq!(result.blue(), rgb.blue());
    }
  }

  mod to_xyz {
    use super::*;

    #[test]
    fn it_converts_white_to_approximately_d65() {
      let white = Rgb::<Srgb>::new(255, 255, 255);
      let xyz = white.to_xyz();

      assert!((xyz.x() - 0.95047).abs() < 0.01);
      assert!((xyz.y() - 1.0).abs() < 0.01);
      assert!((xyz.z() - 1.08883).abs() < 0.01);
    }

    #[test]
    fn it_converts_black_to_origin() {
      let black = Rgb::<Srgb>::new(0, 0, 0);
      let xyz = black.to_xyz();

      assert!(xyz.x().abs() < 1e-10);
      assert!(xyz.y().abs() < 1e-10);
      assert!(xyz.z().abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_with_from_xyz() {
      let original = Rgb::<Srgb>::new(200, 100, 50);
      let xyz = original.to_xyz();
      let back: Rgb<Srgb> = Rgb::from(xyz);

      assert_eq!(back.red(), original.red());
      assert_eq!(back.green(), original.green());
      assert_eq!(back.blue(), original.blue());
    }
  }

  mod try_from_str {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses_valid_hexcode() {
      let rgb: Rgb<Srgb> = Rgb::try_from("#FF8040").unwrap();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 128);
      assert_eq!(rgb.blue(), 64);
    }

    #[test]
    fn it_returns_error_for_invalid_hexcode() {
      let result: Result<Rgb<Srgb>, _> = Rgb::try_from("#GGHHII");

      assert!(result.is_err());
    }
  }

  mod try_from_string {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_parses_valid_hexcode() {
      let rgb: Rgb<Srgb> = Rgb::try_from("#FF8040".to_string()).unwrap();

      assert_eq!(rgb.red(), 255);
      assert_eq!(rgb.green(), 128);
      assert_eq!(rgb.blue(), 64);
    }
  }

  mod with_b {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_new_blue() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_b(64);

      assert_eq!(result.blue(), 64);
      assert_eq!(result.red(), 128);
      assert_eq!(result.green(), 128);
    }
  }

  mod with_b_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_decremented_blue() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_b_decremented_by(64);

      assert_eq!(result.blue(), 64);
      assert_eq!(rgb.blue(), 128);
    }
  }

  mod with_b_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_incremented_blue() {
      let rgb = Rgb::<Srgb>::new(128, 128, 64);
      let result = rgb.with_b_incremented_by(64);

      assert_eq!(result.blue(), 128);
    }
  }

  mod with_b_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_scaled_blue() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_b_scaled_by(0.5);

      assert_eq!(result.blue(), 64);
    }
  }

  mod with_g {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_new_green() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_g(64);

      assert_eq!(result.green(), 64);
      assert_eq!(result.red(), 128);
      assert_eq!(result.blue(), 128);
    }
  }

  mod with_g_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_decremented_green() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_g_decremented_by(64);

      assert_eq!(result.green(), 64);
    }
  }

  mod with_g_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_incremented_green() {
      let rgb = Rgb::<Srgb>::new(128, 64, 128);
      let result = rgb.with_g_incremented_by(64);

      assert_eq!(result.green(), 128);
    }
  }

  mod with_g_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_scaled_green() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_g_scaled_by(0.5);

      assert_eq!(result.green(), 64);
    }
  }

  mod with_r {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_new_red() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_r(64);

      assert_eq!(result.red(), 64);
      assert_eq!(result.green(), 128);
      assert_eq!(result.blue(), 128);
    }
  }

  mod with_r_decremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_decremented_red() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_r_decremented_by(64);

      assert_eq!(result.red(), 64);
    }
  }

  mod with_r_incremented_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_incremented_red() {
      let rgb = Rgb::<Srgb>::new(64, 128, 128);
      let result = rgb.with_r_incremented_by(64);

      assert_eq!(result.red(), 128);
    }
  }

  mod with_r_scaled_by {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_rgb_with_scaled_red() {
      let rgb = Rgb::<Srgb>::new(128, 128, 128);
      let result = rgb.with_r_scaled_by(0.5);

      assert_eq!(result.red(), 64);
    }
  }
}
