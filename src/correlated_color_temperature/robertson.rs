//! Robertson (1968) CCT estimation.
//!
//! Isotherm interpolation in the CIE 1960 UCS (u, v) color space. Uses a 31-entry lookup
//! table of isotherms from Robertson's paper to interpolate between known blackbody
//! temperatures.
//!
//! # Reference
//!
//! Robertson, A. R. (1968). "Computation of Correlated Color Temperature and Distribution
//! Temperature." *Journal of the Optical Society of America*, 58(11), 1528–1535.

use super::{ColorTemperature, MRD_FACTOR};
use crate::space::Xyz;

/// An isotherm entry from Robertson's Table 1: (MRD, u, v, slope).
struct Isotherm {
  mrd: f64,
  u: f64,
  v: f64,
  slope: f64,
}

/// Robertson's 31 isotherms from ~infinity (0 MRD) to 1,667 K (600 MRD).
const ISOTHERMS: [Isotherm; 31] = [
  Isotherm {
    mrd: 0.0,
    u: 0.18006,
    v: 0.26352,
    slope: -0.24341,
  },
  Isotherm {
    mrd: 10.0,
    u: 0.18066,
    v: 0.26589,
    slope: -0.25479,
  },
  Isotherm {
    mrd: 20.0,
    u: 0.18133,
    v: 0.26846,
    slope: -0.26876,
  },
  Isotherm {
    mrd: 30.0,
    u: 0.18208,
    v: 0.27119,
    slope: -0.28539,
  },
  Isotherm {
    mrd: 40.0,
    u: 0.18293,
    v: 0.27407,
    slope: -0.30470,
  },
  Isotherm {
    mrd: 50.0,
    u: 0.18388,
    v: 0.27709,
    slope: -0.32675,
  },
  Isotherm {
    mrd: 60.0,
    u: 0.18494,
    v: 0.28021,
    slope: -0.35156,
  },
  Isotherm {
    mrd: 70.0,
    u: 0.18611,
    v: 0.28342,
    slope: -0.37915,
  },
  Isotherm {
    mrd: 80.0,
    u: 0.18740,
    v: 0.28668,
    slope: -0.40955,
  },
  Isotherm {
    mrd: 90.0,
    u: 0.18880,
    v: 0.28997,
    slope: -0.44278,
  },
  Isotherm {
    mrd: 100.0,
    u: 0.19032,
    v: 0.29326,
    slope: -0.47888,
  },
  Isotherm {
    mrd: 125.0,
    u: 0.19462,
    v: 0.30141,
    slope: -0.58204,
  },
  Isotherm {
    mrd: 150.0,
    u: 0.19962,
    v: 0.30921,
    slope: -0.70471,
  },
  Isotherm {
    mrd: 175.0,
    u: 0.20525,
    v: 0.31647,
    slope: -0.84901,
  },
  Isotherm {
    mrd: 200.0,
    u: 0.21142,
    v: 0.32312,
    slope: -1.0182,
  },
  Isotherm {
    mrd: 225.0,
    u: 0.21807,
    v: 0.32909,
    slope: -1.2168,
  },
  Isotherm {
    mrd: 250.0,
    u: 0.22511,
    v: 0.33439,
    slope: -1.4512,
  },
  Isotherm {
    mrd: 275.0,
    u: 0.23247,
    v: 0.33904,
    slope: -1.7298,
  },
  Isotherm {
    mrd: 300.0,
    u: 0.24010,
    v: 0.34308,
    slope: -2.0637,
  },
  Isotherm {
    mrd: 325.0,
    u: 0.24792,
    v: 0.34655,
    slope: -2.4681,
  },
  Isotherm {
    mrd: 350.0,
    u: 0.25591,
    v: 0.34951,
    slope: -2.9641,
  },
  Isotherm {
    mrd: 375.0,
    u: 0.26400,
    v: 0.35200,
    slope: -3.5814,
  },
  Isotherm {
    mrd: 400.0,
    u: 0.27218,
    v: 0.35407,
    slope: -4.3633,
  },
  Isotherm {
    mrd: 425.0,
    u: 0.28039,
    v: 0.35577,
    slope: -5.3762,
  },
  Isotherm {
    mrd: 450.0,
    u: 0.28863,
    v: 0.35714,
    slope: -6.7262,
  },
  Isotherm {
    mrd: 475.0,
    u: 0.29685,
    v: 0.35823,
    slope: -8.5955,
  },
  Isotherm {
    mrd: 500.0,
    u: 0.30505,
    v: 0.35907,
    slope: -11.324,
  },
  Isotherm {
    mrd: 525.0,
    u: 0.31320,
    v: 0.35968,
    slope: -15.628,
  },
  Isotherm {
    mrd: 550.0,
    u: 0.32129,
    v: 0.36011,
    slope: -23.325,
  },
  Isotherm {
    mrd: 575.0,
    u: 0.32931,
    v: 0.36038,
    slope: -40.770,
  },
  Isotherm {
    mrd: 600.0,
    u: 0.33724,
    v: 0.36051,
    slope: -116.45,
  },
];

/// Calculates the correlated color temperature using Robertson's method.
///
/// Converts the color to CIE 1960 UCS (u, v) coordinates and interpolates between
/// Robertson's isotherms. Accurate across the full range of standard illuminants
/// (~1,667 K to ~infinity).
///
/// ```
/// # #[cfg(feature = "cct-robertson")]
/// # {
/// use farg::correlated_color_temperature::robertson;
/// use farg::space::Xyz;
///
/// // D65 white point (~6504 K)
/// let d65 = Xyz::new(0.95047, 1.0, 1.08883);
/// let cct = robertson::calculate(d65);
/// assert!((cct.value() - 6504.0).abs() < 50.0);
/// # }
/// ```
pub fn calculate(color: impl Into<Xyz>) -> ColorTemperature {
  let uv = color.into().chromaticity().to_uv();
  let [u, v] = uv.components();

  let mut last_d = 0.0;
  let mut last_i = 0;

  for (i, iso) in ISOTHERMS.iter().enumerate() {
    let du = u - iso.u;
    let dv = v - iso.v;
    let d = (dv - du * iso.slope) / (1.0 + iso.slope * iso.slope).sqrt();

    if i > 0 && d * last_d < 0.0 {
      let t = last_d / (last_d - d);
      let mrd = ISOTHERMS[last_i].mrd + t * (iso.mrd - ISOTHERMS[last_i].mrd);
      return ColorTemperature(MRD_FACTOR / mrd);
    }

    last_d = d;
    last_i = i;
  }

  let mrd = ISOTHERMS[ISOTHERMS.len() - 1].mrd;
  ColorTemperature(MRD_FACTOR / mrd)
}

#[cfg(test)]
mod test {
  use super::*;

  mod calculate {
    use super::*;

    #[test]
    fn it_estimates_d65_white_point() {
      let d65 = Xyz::new(0.95047, 1.0, 1.08883);
      let cct = calculate(d65);

      assert!((cct.value() - 6504.0).abs() < 50.0);
    }

    #[test]
    fn it_estimates_warm_white() {
      let warm = crate::chromaticity::Xy::new(0.4369, 0.4041).to_xyz(1.0);
      let cct = calculate(warm);

      assert!((cct.value() - 3000.0).abs() < 100.0);
    }

    #[test]
    fn it_estimates_cool_white() {
      let cool = crate::chromaticity::Xy::new(0.2807, 0.2884).to_xyz(1.0);
      let cct = calculate(cool);

      assert!((cct.value() - 10000.0).abs() < 200.0);
    }

    #[test]
    fn it_returns_positive_for_typical_illuminants() {
      let d65 = Xyz::new(0.95047, 1.0, 1.08883);

      assert!(calculate(d65).value() > 0.0);
    }

    #[test]
    fn it_estimates_incandescent() {
      let a = crate::chromaticity::Xy::new(0.4476, 0.4074).to_xyz(1.0);
      let cct = calculate(a);

      assert!((cct.value() - 2856.0).abs() < 50.0);
    }

    #[test]
    fn it_estimates_5000k() {
      let d50 = crate::chromaticity::Xy::new(0.3457, 0.3585).to_xyz(1.0);
      let cct = calculate(d50);

      assert!((cct.value() - 5000.0).abs() < 100.0);
    }
  }
}
