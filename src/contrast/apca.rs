//! APCA (Accessible Perceptual Contrast Algorithm) contrast calculation.
//!
//! Implements the SAPC-4 (S-Luv Accessible Perceptual Contrast) method from the
//! [APCA-W3 specification](https://github.com/Myndex/SAPC-APCA). APCA is a next-generation
//! contrast algorithm designed to replace WCAG 2.x contrast ratios with perceptually uniform
//! lightness contrast (Lc) values.

use crate::space::Xyz;

/// Exponent for the soft clamp applied to near-black luminance values.
pub const BLACK_CLAMP_EXPONENT: f64 = 1.414;

/// Luminance threshold below which the soft clamp is applied to prevent
/// black-level flare artifacts.
pub const BLACK_THRESHOLD: f64 = 0.022;

/// Minimum Lc value recommended for body text in normal polarity
/// (dark text on light background).
pub const DEFAULT_BODY_TEXT_THRESHOLD: f64 = 60.0;

/// Minimum Lc value recommended for large text in normal polarity
/// (dark text on light background).
pub const DEFAULT_LARGE_TEXT_THRESHOLD: f64 = 45.0;

/// Minimum Lc value recommended for very large or non-text elements in normal polarity
/// (dark text on light background).
pub const DEFAULT_VERY_LARGE_TEXT_THRESHOLD: f64 = 30.0;

/// Minimum Lc value recommended for body text in reverse polarity
/// (light text on dark background).
pub const REVERSE_BODY_TEXT_THRESHOLD: f64 = 75.0;

/// Minimum Lc value recommended for large text in reverse polarity
/// (light text on dark background).
pub const REVERSE_LARGE_TEXT_THRESHOLD: f64 = 60.0;

/// Minimum Lc value recommended for very large or non-text elements in reverse polarity
/// (light text on dark background).
pub const REVERSE_VERY_LARGE_TEXT_THRESHOLD: f64 = 45.0;

/// Minimum luminance difference (ΔY) required before computing contrast.
/// Pairs closer than this are treated as zero contrast.
pub const DELTA_Y_MIN: f64 = 0.0005;

/// Absolute Lc threshold below which contrast is clamped to zero.
pub const LOW_CLIP: f64 = 0.1;

/// Offset subtracted (or added) from the raw SAPC value before final scaling,
/// providing a soft toe for low-contrast pairs.
pub const LOW_OFFSET: f64 = 0.027;

/// Overall output scale factor applied to the raw SAPC value.
pub const SCALE: f64 = 1.14;

/// Background exponent for normal polarity (dark text on light background).
/// Referred to as `Nbg` in the APCA specification.
pub const NORMAL_BACKGROUND_EXPONENT: f64 = 0.56;

/// Text exponent for normal polarity (dark text on light background).
/// Referred to as `Ntx` in the APCA specification.
pub const NORMAL_TEXT_EXPONENT: f64 = 0.57;

/// Background exponent for reverse polarity (light text on dark background).
/// Referred to as `Rbg` in the APCA specification.
pub const REVERSE_BACKGROUND_EXPONENT: f64 = 0.57;

/// Text exponent for reverse polarity (light text on dark background).
/// Referred to as `Rtx` in the APCA specification.
pub const REVERSE_TEXT_EXPONENT: f64 = 0.62;

/// APCA lightness contrast (Lc) value.
///
/// Wraps the raw Lc value computed by the APCA algorithm. Positive values indicate
/// dark text on a light background (normal polarity), negative values indicate light
/// text on a dark background (reverse polarity).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LightnessContrast(f64);

impl LightnessContrast {
  /// Returns `true` if the contrast meets the APCA threshold for body text.
  ///
  /// Uses Lc >= 60 for normal polarity (dark-on-light) and Lc >= 75 for reverse
  /// polarity (light-on-dark).
  pub fn meets_body_text_threshold(&self) -> bool {
    self.meets_threshold(DEFAULT_BODY_TEXT_THRESHOLD, REVERSE_BODY_TEXT_THRESHOLD)
  }

  /// Returns `true` if the contrast meets the APCA threshold for large text.
  ///
  /// Uses Lc >= 45 for normal polarity (dark-on-light) and Lc >= 60 for reverse
  /// polarity (light-on-dark).
  pub fn meets_large_text_threshold(&self) -> bool {
    self.meets_threshold(DEFAULT_LARGE_TEXT_THRESHOLD, REVERSE_LARGE_TEXT_THRESHOLD)
  }

  /// Returns `true` if the contrast meets the APCA threshold for very large text
  /// or non-text elements.
  ///
  /// Uses Lc >= 30 for normal polarity (dark-on-light) and Lc >= 45 for reverse
  /// polarity (light-on-dark).
  pub fn meets_very_large_text_threshold(&self) -> bool {
    self.meets_threshold(DEFAULT_VERY_LARGE_TEXT_THRESHOLD, REVERSE_VERY_LARGE_TEXT_THRESHOLD)
  }

  /// Returns the raw Lc value.
  pub fn value(&self) -> f64 {
    self.0
  }

  fn meets_threshold(&self, normal: f64, reverse: f64) -> bool {
    if self.0 >= 0.0 {
      self.0 >= normal
    } else {
      self.0.abs() >= reverse
    }
  }
}

impl From<LightnessContrast> for f64 {
  fn from(lc: LightnessContrast) -> Self {
    lc.0
  }
}

/// Calculates APCA (Accessible Perceptual Contrast Algorithm) contrast between a text color
/// and its background. Returns a [`LightnessContrast`] value in Lc (Lightness Contrast) units
/// where positive values indicate dark text on a light background (normal polarity) and negative
/// values indicate light text on a dark background (reverse polarity).
pub fn calculate(color: impl Into<Xyz>, background: impl Into<Xyz>) -> LightnessContrast {
  let text_y = soft_clamp(color.into().luminance());
  let bg_y = soft_clamp(background.into().luminance());

  if (bg_y - text_y).abs() < DELTA_Y_MIN {
    return LightnessContrast(0.0);
  }

  let sapc = if bg_y > text_y {
    (bg_y.powf(NORMAL_BACKGROUND_EXPONENT) - text_y.powf(NORMAL_TEXT_EXPONENT)) * SCALE
  } else {
    (bg_y.powf(REVERSE_BACKGROUND_EXPONENT) - text_y.powf(REVERSE_TEXT_EXPONENT)) * SCALE
  };

  if sapc > LOW_CLIP {
    LightnessContrast((sapc - LOW_OFFSET) * 100.0)
  } else if sapc < -LOW_CLIP {
    LightnessContrast((sapc + LOW_OFFSET) * 100.0)
  } else {
    LightnessContrast(0.0)
  }
}

fn soft_clamp(y: f64) -> f64 {
  if y < BLACK_THRESHOLD {
    y + (BLACK_THRESHOLD - y).powf(BLACK_CLAMP_EXPONENT)
  } else {
    y
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod calculate {
    use super::*;

    #[test]
    fn it_returns_positive_lc_for_dark_text_on_light_background() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let lc = calculate(black, white);

      assert!(lc.value() > 0.0);
    }

    #[test]
    fn it_returns_negative_lc_for_light_text_on_dark_background() {
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let black = Xyz::new(0.0, 0.0, 0.0);
      let lc = calculate(white, black);

      assert!(lc.value() < 0.0);
    }

    #[test]
    fn it_returns_zero_for_identical_colors() {
      let color = Xyz::new(0.4, 0.5, 0.3);
      let lc = calculate(color, color);

      assert_eq!(lc.value(), 0.0);
    }

    #[test]
    fn it_returns_zero_for_near_identical_luminance() {
      let a = Xyz::new(0.4, 0.5, 0.3);
      let b = Xyz::new(0.4, 0.5 + DELTA_Y_MIN * 0.5, 0.3);
      let lc = calculate(a, b);

      assert_eq!(lc.value(), 0.0);
    }

    #[test]
    fn it_returns_zero_for_low_contrast_pair() {
      let a = Xyz::new(0.0, 0.5, 0.0);
      let b = Xyz::new(0.0, 0.51, 0.0);
      let lc = calculate(a, b);

      assert_eq!(lc.value(), 0.0);
    }

    #[test]
    fn it_produces_symmetrical_magnitude_for_swapped_polarity() {
      let dark = Xyz::new(0.0, 0.05, 0.0);
      let light = Xyz::new(0.9505, 1.0, 1.089);
      let normal = calculate(dark, light);
      let reverse = calculate(light, dark);

      assert!(normal.value() > 0.0);
      assert!(reverse.value() < 0.0);
    }

    #[test]
    fn it_exceeds_body_text_threshold_for_black_on_white() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let lc = calculate(black, white);

      assert!(lc.meets_body_text_threshold());
    }

    #[test]
    fn it_increases_with_greater_luminance_difference() {
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let mid_gray = Xyz::new(0.0, 0.2, 0.0);
      let dark_gray = Xyz::new(0.0, 0.05, 0.0);
      let lc_mid = calculate(mid_gray, white);
      let lc_dark = calculate(dark_gray, white);

      assert!(lc_dark.value() > lc_mid.value());
    }

    #[test]
    fn it_produces_asymmetric_absolute_magnitudes_across_polarity() {
      let dark = Xyz::new(0.0, 0.1, 0.0);
      let light = Xyz::new(0.0, 0.7, 0.0);
      let normal = calculate(dark, light);
      let reverse = calculate(light, dark);

      assert_ne!(normal.value().abs(), reverse.value().abs());
    }

    #[test]
    fn it_applies_soft_clamp_to_near_black_text() {
      let near_black = Xyz::new(0.0, 0.01, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let lc_near = calculate(near_black, white);

      let above_threshold = Xyz::new(0.0, 0.03, 0.0);
      let lc_above = calculate(above_threshold, white);

      assert!(lc_near.value() > lc_above.value());
    }
  }

  mod lightness_contrast {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_value() {
      let lc = LightnessContrast(42.5);

      assert_eq!(lc.value(), 42.5);
    }

    #[test]
    fn it_converts_to_f64() {
      let lc = LightnessContrast(42.5);
      let value: f64 = lc.into();

      assert_eq!(value, 42.5);
    }

    #[test]
    fn it_meets_body_text_threshold_normal_polarity() {
      assert!(LightnessContrast(60.0).meets_body_text_threshold());
      assert!(LightnessContrast(75.0).meets_body_text_threshold());
      assert!(!LightnessContrast(59.9).meets_body_text_threshold());
    }

    #[test]
    fn it_meets_body_text_threshold_reverse_polarity() {
      assert!(LightnessContrast(-75.0).meets_body_text_threshold());
      assert!(!LightnessContrast(-60.0).meets_body_text_threshold());
      assert!(!LightnessContrast(-74.9).meets_body_text_threshold());
    }

    #[test]
    fn it_meets_large_text_threshold_normal_polarity() {
      assert!(LightnessContrast(45.0).meets_large_text_threshold());
      assert!(!LightnessContrast(44.9).meets_large_text_threshold());
    }

    #[test]
    fn it_meets_large_text_threshold_reverse_polarity() {
      assert!(LightnessContrast(-60.0).meets_large_text_threshold());
      assert!(!LightnessContrast(-59.9).meets_large_text_threshold());
    }

    #[test]
    fn it_meets_very_large_text_threshold_normal_polarity() {
      assert!(LightnessContrast(30.0).meets_very_large_text_threshold());
      assert!(!LightnessContrast(29.9).meets_very_large_text_threshold());
    }

    #[test]
    fn it_meets_very_large_text_threshold_reverse_polarity() {
      assert!(LightnessContrast(-45.0).meets_very_large_text_threshold());
      assert!(!LightnessContrast(-44.9).meets_very_large_text_threshold());
    }

    #[test]
    fn it_fails_all_thresholds_at_zero() {
      let lc = LightnessContrast(0.0);

      assert!(!lc.meets_body_text_threshold());
      assert!(!lc.meets_large_text_threshold());
      assert!(!lc.meets_very_large_text_threshold());
    }
  }

  mod soft_clamp {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_unchanged_value_above_threshold() {
      let result = soft_clamp(0.5);

      assert_eq!(result, 0.5);
    }

    #[test]
    fn it_returns_unchanged_value_at_threshold() {
      let result = soft_clamp(BLACK_THRESHOLD);

      assert_eq!(result, BLACK_THRESHOLD);
    }

    #[test]
    fn it_increases_values_below_threshold() {
      let input = 0.01;
      let result = soft_clamp(input);

      assert!(result > input);
    }

    #[test]
    fn it_clamps_zero() {
      let result = soft_clamp(0.0);

      assert!(result > 0.0);
      assert_eq!(result, BLACK_THRESHOLD.powf(BLACK_CLAMP_EXPONENT));
    }
  }
}
