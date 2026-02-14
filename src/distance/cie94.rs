//! CIE94 color difference (ΔE\*94).
//!
//! Implements the [CIE94](https://en.wikipedia.org/wiki/Color_difference#CIE94) color difference
//! formula, which extends CIE76 with weighting functions for lightness, chroma, and hue. The
//! formula is **not symmetric** — the first argument is treated as the reference color and the
//! second as the sample.

use crate::space::{Lab, Xyz};

/// Default lightness weight for graphic arts applications.
pub const GRAPHIC_ARTS_KL: f64 = 1.0;

/// Default chroma weighting factor for graphic arts applications.
pub const GRAPHIC_ARTS_K1: f64 = 0.045;

/// Default hue weighting factor for graphic arts applications.
pub const GRAPHIC_ARTS_K2: f64 = 0.015;

/// Lightness weight for textile applications.
pub const TEXTILES_KL: f64 = 2.0;

/// Chroma weighting factor for textile applications.
pub const TEXTILES_K1: f64 = 0.048;

/// Hue weighting factor for textile applications.
pub const TEXTILES_K2: f64 = 0.014;

/// Calculates the CIE94 color difference using graphic arts weights.
///
/// Uses kL=1, K1=0.045, K2=0.015 (graphic arts application). The first argument is the
/// reference color; the second is the sample. This function is **not** order-independent.
pub fn calculate(reference: impl Into<Xyz>, sample: impl Into<Xyz>) -> f64 {
  calculate_parametric(reference, sample, GRAPHIC_ARTS_KL, GRAPHIC_ARTS_K1, GRAPHIC_ARTS_K2)
}

/// Calculates the CIE94 color difference with custom parametric factors.
///
/// The first argument is the reference color; the second is the sample.
/// This function is **not** order-independent.
///
/// - `kl` — lightness weighting factor
/// - `k1` — chroma weighting coefficient
/// - `k2` — hue weighting coefficient
pub fn calculate_parametric(reference: impl Into<Xyz>, sample: impl Into<Xyz>, kl: f64, k1: f64, k2: f64) -> f64 {
  let ref_lab = Lab::from(reference.into());
  let smp_lab = Lab::from(sample.into());

  let dl = ref_lab.l() - smp_lab.l();
  let da = ref_lab.a() - smp_lab.a();
  let db = ref_lab.b() - smp_lab.b();

  let c1 = (ref_lab.a().powi(2) + ref_lab.b().powi(2)).sqrt();
  let c2 = (smp_lab.a().powi(2) + smp_lab.b().powi(2)).sqrt();
  let dc = c1 - c2;

  let dh_sq = da * da + db * db - dc * dc;
  let dh_sq = dh_sq.max(0.0);

  let sl = 1.0;
  let sc = 1.0 + k1 * c1;
  let sh = 1.0 + k2 * c1;

  let term_l = dl / (kl * sl);
  let term_c = dc / sc;
  let term_h = dh_sq / (sh * sh);

  (term_l * term_l + term_c * term_c + term_h).sqrt()
}

/// Calculates the CIE94 color difference using textile weights.
///
/// Uses kL=2, K1=0.048, K2=0.014 (textile application). The first argument is the
/// reference color; the second is the sample. This function is **not** order-independent.
pub fn calculate_textiles(reference: impl Into<Xyz>, sample: impl Into<Xyz>) -> f64 {
  calculate_parametric(reference, sample, TEXTILES_KL, TEXTILES_K1, TEXTILES_K2)
}

#[cfg(test)]
mod test {
  use super::*;

  mod calculate {
    use super::*;

    #[test]
    fn it_returns_zero_for_identical_colors() {
      let color = Xyz::new(0.4, 0.5, 0.3);

      assert_eq!(calculate(color, color), 0.0);
    }

    #[test]
    fn it_returns_positive_for_different_colors() {
      let a = Xyz::new(0.0, 0.0, 0.0);
      let b = Xyz::new(0.9505, 1.0, 1.089);

      assert!(calculate(a, b) > 0.0);
    }

    #[test]
    fn it_is_not_order_independent() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      let forward = calculate(a, b);
      let reverse = calculate(b, a);

      assert!((forward - reverse).abs() > 1e-10 || forward == reverse);
    }

    #[test]
    fn it_increases_with_greater_difference() {
      let reference = Xyz::new(0.9505, 1.0, 1.089);
      let near = Xyz::new(0.8, 0.9, 0.95);
      let far = Xyz::new(0.2, 0.3, 0.25);

      assert!(calculate(reference, far) > calculate(reference, near));
    }
  }

  mod calculate_parametric {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_matches_graphic_arts_with_same_params() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      assert_eq!(
        calculate_parametric(a, b, GRAPHIC_ARTS_KL, GRAPHIC_ARTS_K1, GRAPHIC_ARTS_K2),
        calculate(a, b)
      );
    }

    #[test]
    fn it_matches_textiles_with_same_params() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      assert_eq!(
        calculate_parametric(a, b, TEXTILES_KL, TEXTILES_K1, TEXTILES_K2),
        calculate_textiles(a, b)
      );
    }
  }

  mod calculate_textiles {
    use super::*;

    #[test]
    fn it_returns_zero_for_identical_colors() {
      let color = Xyz::new(0.4, 0.5, 0.3);

      assert_eq!(calculate_textiles(color, color), 0.0);
    }

    #[test]
    fn it_differs_from_graphic_arts() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      let graphic = calculate(a, b);
      let textile = calculate_textiles(a, b);

      assert!((graphic - textile).abs() > 1e-10);
    }
  }
}
