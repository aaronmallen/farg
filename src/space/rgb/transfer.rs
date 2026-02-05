use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::component::Component;

const BT709_ALPHA: f64 = 0.099;
const BT709_ENCODED_THRESHOLD: f64 = 0.081;
const BT709_GAMMA: f64 = 1.0 / 0.45;
const BT709_LINEAR_SLOPE: f64 = 4.5;
const BT709_LINEAR_THRESHOLD: f64 = 0.018;
const HLG_A: f64 = 0.17883277;
const HLG_B: f64 = 0.28466892;
const HLG_C: f64 = 0.55991073;
const PQ_C1: f64 = 3424.0 / 4096.0;
const PQ_C2: f64 = 2413.0 / 4096.0 * 32.0;
const PQ_C3: f64 = 2392.0 / 4096.0 * 32.0;
const PQ_M1: f64 = 2610.0 / 16384.0;
const PQ_M2: f64 = 2523.0 / 4096.0 * 128.0;
const PROPHOTO_ENCODED_THRESHOLD: f64 = 16.0 / 512.0;
const PROPHOTO_GAMMA: f64 = 1.8;
const PROPHOTO_LINEAR_SLOPE: f64 = 16.0;
const PROPHOTO_LINEAR_THRESHOLD: f64 = 1.0 / 512.0;
const SRGB_ALPHA: f64 = 0.055;
const SRGB_ENCODED_THRESHOLD: f64 = 0.04045;
const SRGB_GAMMA: f64 = 2.4;
const SRGB_LINEAR_SLOPE: f64 = 12.92;
const SRGB_LINEAR_THRESHOLD: f64 = 0.0031308;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TransferFunction {
  Bt601,
  Bt709,
  Gamma(f64),
  Hlg,
  Linear,
  Pq,
  ProPhotoRgb,
  #[default]
  Srgb,
}

impl TransferFunction {
  #[must_use]
  pub fn decode(&self, encoded: impl Into<Component>) -> f64 {
    let encoded = encoded.into().0;

    match self {
      Self::Linear => encoded,
      Self::Gamma(gamma) => encoded.powf(*gamma),
      Self::Srgb => srgb_decode(encoded),
      Self::Bt709 | Self::Bt601 => bt709_decode(encoded),
      Self::Pq => pq_decode(encoded),
      Self::Hlg => hlg_decode(encoded),
      Self::ProPhotoRgb => prophoto_decode(encoded),
    }
  }

  #[must_use]
  pub fn encode(&self, linear: impl Into<Component>) -> f64 {
    let linear = linear.into().0;

    match self {
      Self::Linear => linear,
      Self::Gamma(gamma) => linear.powf(1.0 / gamma),
      Self::Srgb => srgb_encode(linear),
      Self::Bt709 | Self::Bt601 => bt709_encode(linear),
      Self::Pq => pq_encode(linear),
      Self::Hlg => hlg_encode(linear),
      Self::ProPhotoRgb => prophoto_encode(linear),
    }
  }
}

impl Display for TransferFunction {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Linear => write!(f, "Linear"),
      Self::Gamma(gamma) => write!(f, "Gamma {gamma:.2}"),
      Self::Srgb => write!(f, "sRGB"),
      Self::Bt709 => write!(f, "BT.709"),
      Self::Bt601 => write!(f, "BT.601"),
      Self::Pq => write!(f, "PQ (ST 2084)"),
      Self::Hlg => write!(f, "HLG"),
      Self::ProPhotoRgb => write!(f, "ProPhoto RGB"),
    }
  }
}

fn bt709_decode(encoded: f64) -> f64 {
  if encoded < BT709_ENCODED_THRESHOLD {
    encoded / BT709_LINEAR_SLOPE
  } else {
    ((encoded + BT709_ALPHA) / (1.0 + BT709_ALPHA)).powf(BT709_GAMMA)
  }
}

fn bt709_encode(linear: f64) -> f64 {
  if linear < BT709_LINEAR_THRESHOLD {
    linear * BT709_LINEAR_SLOPE
  } else {
    (1.0 + BT709_ALPHA) * linear.powf(0.45) - BT709_ALPHA
  }
}

fn hlg_decode(encoded: f64) -> f64 {
  if encoded <= 0.5 {
    encoded * encoded / 3.0
  } else {
    ((encoded - HLG_C) / HLG_A).exp() / 12.0 + HLG_B / 12.0
  }
}

fn hlg_encode(linear: f64) -> f64 {
  if linear <= 1.0 / 12.0 {
    (3.0 * linear).sqrt()
  } else {
    HLG_A * (12.0 * linear - HLG_B).ln() + HLG_C
  }
}

fn pq_decode(encoded: f64) -> f64 {
  let e = encoded.max(0.0);
  let e_1_m2 = e.powf(1.0 / PQ_M2);
  let numerator = (e_1_m2 - PQ_C1).max(0.0);
  let denominator = PQ_C2 - PQ_C3 * e_1_m2;
  10000.0 * (numerator / denominator).powf(1.0 / PQ_M1)
}

fn pq_encode(linear: f64) -> f64 {
  let y = (linear / 10000.0).max(0.0);
  let y_m1 = y.powf(PQ_M1);
  ((PQ_C1 + PQ_C2 * y_m1) / (1.0 + PQ_C3 * y_m1)).powf(PQ_M2)
}

fn prophoto_decode(encoded: f64) -> f64 {
  if encoded < PROPHOTO_ENCODED_THRESHOLD {
    encoded / PROPHOTO_LINEAR_SLOPE
  } else {
    encoded.powf(PROPHOTO_GAMMA)
  }
}

fn prophoto_encode(linear: f64) -> f64 {
  if linear < PROPHOTO_LINEAR_THRESHOLD {
    linear * PROPHOTO_LINEAR_SLOPE
  } else {
    linear.powf(1.0 / PROPHOTO_GAMMA)
  }
}

fn srgb_decode(encoded: f64) -> f64 {
  if encoded <= SRGB_ENCODED_THRESHOLD {
    encoded / SRGB_LINEAR_SLOPE
  } else {
    ((encoded + SRGB_ALPHA) / (1.0 + SRGB_ALPHA)).powf(SRGB_GAMMA)
  }
}

fn srgb_encode(linear: f64) -> f64 {
  if linear <= SRGB_LINEAR_THRESHOLD {
    linear * SRGB_LINEAR_SLOPE
  } else {
    (1.0 + SRGB_ALPHA) * linear.powf(1.0 / SRGB_GAMMA) - SRGB_ALPHA
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod decode {
    use super::*;

    #[test]
    fn it_decodes_linear_as_identity() {
      let tf = TransferFunction::Linear;

      assert_eq!(tf.decode(0.5), 0.5);
      assert_eq!(tf.decode(0.0), 0.0);
      assert_eq!(tf.decode(1.0), 1.0);
    }

    #[test]
    fn it_decodes_gamma_with_power_function() {
      let tf = TransferFunction::Gamma(2.2);
      let encoded: f64 = 0.5;
      let expected = encoded.powf(2.2);

      assert!((tf.decode(encoded) - expected).abs() < 1e-10);
    }

    #[test]
    fn it_decodes_srgb_in_linear_region() {
      let tf = TransferFunction::Srgb;
      let encoded = 0.01;
      let expected = encoded / SRGB_LINEAR_SLOPE;

      assert!((tf.decode(encoded) - expected).abs() < 1e-10);
    }

    #[test]
    fn it_decodes_srgb_in_gamma_region() {
      let tf = TransferFunction::Srgb;
      let encoded = 0.5;
      let expected = ((encoded + SRGB_ALPHA) / (1.0 + SRGB_ALPHA)).powf(SRGB_GAMMA);

      assert!((tf.decode(encoded) - expected).abs() < 1e-10);
    }

    #[test]
    fn it_decodes_bt709_in_linear_region() {
      let tf = TransferFunction::Bt709;
      let encoded = 0.05;
      let expected = encoded / BT709_LINEAR_SLOPE;

      assert!((tf.decode(encoded) - expected).abs() < 1e-10);
    }

    #[test]
    fn it_decodes_bt709_in_gamma_region() {
      let tf = TransferFunction::Bt709;
      let encoded = 0.5;
      let expected = ((encoded + BT709_ALPHA) / (1.0 + BT709_ALPHA)).powf(BT709_GAMMA);

      assert!((tf.decode(encoded) - expected).abs() < 1e-10);
    }

    #[test]
    fn it_decodes_bt601_same_as_bt709() {
      let encoded = 0.5;

      assert_eq!(
        TransferFunction::Bt601.decode(encoded),
        TransferFunction::Bt709.decode(encoded)
      );
    }
  }

  mod display {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_formats_linear() {
      assert_eq!(format!("{}", TransferFunction::Linear), "Linear");
    }

    #[test]
    fn it_formats_gamma_with_value() {
      assert_eq!(format!("{}", TransferFunction::Gamma(2.2)), "Gamma 2.20");
    }

    #[test]
    fn it_formats_srgb() {
      assert_eq!(format!("{}", TransferFunction::Srgb), "sRGB");
    }

    #[test]
    fn it_formats_bt709() {
      assert_eq!(format!("{}", TransferFunction::Bt709), "BT.709");
    }

    #[test]
    fn it_formats_bt601() {
      assert_eq!(format!("{}", TransferFunction::Bt601), "BT.601");
    }

    #[test]
    fn it_formats_pq() {
      assert_eq!(format!("{}", TransferFunction::Pq), "PQ (ST 2084)");
    }

    #[test]
    fn it_formats_hlg() {
      assert_eq!(format!("{}", TransferFunction::Hlg), "HLG");
    }

    #[test]
    fn it_formats_prophoto_rgb() {
      assert_eq!(format!("{}", TransferFunction::ProPhotoRgb), "ProPhoto RGB");
    }
  }

  mod encode {
    use super::*;

    #[test]
    fn it_encodes_linear_as_identity() {
      let tf = TransferFunction::Linear;

      assert_eq!(tf.encode(0.5), 0.5);
      assert_eq!(tf.encode(0.0), 0.0);
      assert_eq!(tf.encode(1.0), 1.0);
    }

    #[test]
    fn it_encodes_gamma_with_inverse_power() {
      let tf = TransferFunction::Gamma(2.2);
      let linear: f64 = 0.5;
      let expected = linear.powf(1.0 / 2.2);

      assert!((tf.encode(linear) - expected).abs() < 1e-10);
    }

    #[test]
    fn it_encodes_srgb_in_linear_region() {
      let tf = TransferFunction::Srgb;
      let linear = 0.001;
      let expected = linear * SRGB_LINEAR_SLOPE;

      assert!((tf.encode(linear) - expected).abs() < 1e-10);
    }

    #[test]
    fn it_encodes_srgb_in_gamma_region() {
      let tf = TransferFunction::Srgb;
      let linear: f64 = 0.5;
      let expected = (1.0 + SRGB_ALPHA) * linear.powf(1.0 / SRGB_GAMMA) - SRGB_ALPHA;

      assert!((tf.encode(linear) - expected).abs() < 1e-10);
    }
  }

  mod roundtrip {
    use super::*;

    #[test]
    fn it_roundtrips_linear() {
      let tf = TransferFunction::Linear;
      let original = 0.5;

      assert!((tf.decode(tf.encode(original)) - original).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_gamma() {
      let tf = TransferFunction::Gamma(2.2);
      let original = 0.5;

      assert!((tf.decode(tf.encode(original)) - original).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_srgb() {
      let tf = TransferFunction::Srgb;
      let original = 0.5;

      assert!((tf.decode(tf.encode(original)) - original).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_bt709() {
      let tf = TransferFunction::Bt709;
      let original = 0.5;

      assert!((tf.decode(tf.encode(original)) - original).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_prophoto_rgb() {
      let tf = TransferFunction::ProPhotoRgb;
      let original = 0.5;

      assert!((tf.decode(tf.encode(original)) - original).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_hlg() {
      let tf = TransferFunction::Hlg;
      let original = 0.5;

      assert!((tf.decode(tf.encode(original)) - original).abs() < 1e-10);
    }

    #[test]
    fn it_roundtrips_pq() {
      let tf = TransferFunction::Pq;
      let original = 5000.0;

      assert!((tf.decode(tf.encode(original)) - original).abs() < 1e-6);
    }
  }
}
