//! Machado, Oliveira & Fernandes (2009) anomalous trichromacy simulation.
//!
//! Severity-parameterized simulation of anomalous trichromacy (reduced, not absent, cone
//! sensitivity). Uses precomputed 3x3 simulation matrices in linear sRGB for severity
//! levels 0–10, with linear interpolation for intermediate values.
//!
//! Severity 0.0 means normal vision; 1.0 means complete dichromacy.
//!
//! # Reference
//!
//! Machado, G. M., Oliveira, M. M., & Fernandes, L. A. F. (2009). "A Physiologically-based
//! Model for Simulation of Color Vision Deficiency." *IEEE Transactions on Visualization
//! and Computer Graphics*, 15(6), 1291–1298.

use crate::{
  matrix::Matrix3,
  space::{Srgb, Xyz},
};

/// Identity matrix for severity 0 (normal vision).
const IDENTITY: Matrix3 = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

/// Machado protanomaly matrices for severity levels 1–10.
///
/// From Machado et al. 2009, Table 1. Severity 0 is identity (normal vision).
const PROTAN_MATRICES: [Matrix3; 10] = [
  // Severity 1
  Matrix3::new([
    [0.856167, 0.182038, -0.038205],
    [0.029342, 0.955115, 0.015544],
    [-0.002880, -0.001563, 1.004443],
  ]),
  // Severity 2
  Matrix3::new([
    [0.734766, 0.334872, -0.069637],
    [0.051840, 0.919198, 0.028963],
    [-0.004928, -0.004209, 1.009137],
  ]),
  // Severity 3
  Matrix3::new([
    [0.630323, 0.465641, -0.095964],
    [0.069181, 0.890046, 0.040773],
    [-0.006308, -0.007724, 1.014032],
  ]),
  // Severity 4
  Matrix3::new([
    [0.539009, 0.579343, -0.118352],
    [0.082546, 0.866121, 0.051332],
    [-0.007136, -0.011959, 1.019095],
  ]),
  // Severity 5
  Matrix3::new([
    [0.458064, 0.679578, -0.137642],
    [0.092785, 0.846313, 0.060902],
    [-0.007494, -0.016807, 1.024301],
  ]),
  // Severity 6
  Matrix3::new([
    [0.385450, 0.769005, -0.154455],
    [0.100526, 0.829802, 0.069673],
    [-0.007442, -0.022190, 1.029632],
  ]),
  // Severity 7
  Matrix3::new([
    [0.319627, 0.849633, -0.169261],
    [0.106241, 0.815969, 0.077790],
    [-0.007025, -0.028051, 1.035076],
  ]),
  // Severity 8
  Matrix3::new([
    [0.259411, 0.923008, -0.182420],
    [0.110296, 0.804340, 0.085364],
    [-0.006276, -0.034346, 1.040622],
  ]),
  // Severity 9
  Matrix3::new([
    [0.203876, 0.990338, -0.194214],
    [0.112975, 0.794542, 0.092483],
    [-0.005222, -0.041043, 1.046265],
  ]),
  // Severity 10
  Matrix3::new([
    [0.152286, 1.052583, -0.204868],
    [0.114503, 0.786281, 0.099216],
    [-0.003882, -0.048116, 1.051998],
  ]),
];

/// Machado deuteranomaly matrices for severity levels 1–10.
const DEUTAN_MATRICES: [Matrix3; 10] = [
  // Severity 1
  Matrix3::new([
    [0.866435, 0.177704, -0.044139],
    [0.049567, 0.939063, 0.011370],
    [-0.003453, 0.007233, 0.996220],
  ]),
  // Severity 2
  Matrix3::new([
    [0.760729, 0.319078, -0.079807],
    [0.090568, 0.889315, 0.020117],
    [-0.006027, 0.013325, 0.992702],
  ]),
  // Severity 3
  Matrix3::new([
    [0.675425, 0.433850, -0.109275],
    [0.125303, 0.847755, 0.026942],
    [-0.007950, 0.018572, 0.989378],
  ]),
  // Severity 4
  Matrix3::new([
    [0.605511, 0.528560, -0.134071],
    [0.155318, 0.812366, 0.032316],
    [-0.009376, 0.023176, 0.986200],
  ]),
  // Severity 5
  Matrix3::new([
    [0.547494, 0.607765, -0.155259],
    [0.181692, 0.781742, 0.036566],
    [-0.010410, 0.027275, 0.983136],
  ]),
  // Severity 6
  Matrix3::new([
    [0.498864, 0.674741, -0.173604],
    [0.205199, 0.754872, 0.039929],
    [-0.011131, 0.030969, 0.980162],
  ]),
  // Severity 7
  Matrix3::new([
    [0.457771, 0.731899, -0.189670],
    [0.226409, 0.731012, 0.042579],
    [-0.011595, 0.034333, 0.977261],
  ]),
  // Severity 8
  Matrix3::new([
    [0.422823, 0.781057, -0.203881],
    [0.245752, 0.709602, 0.044646],
    [-0.011843, 0.037423, 0.974421],
  ]),
  // Severity 9
  Matrix3::new([
    [0.392952, 0.823610, -0.216562],
    [0.263559, 0.690210, 0.046232],
    [-0.011910, 0.040281, 0.971630],
  ]),
  // Severity 10
  Matrix3::new([
    [0.367322, 0.860646, -0.227968],
    [0.280085, 0.672501, 0.047413],
    [-0.011820, 0.042940, 0.968881],
  ]),
];

/// Machado tritanomaly matrices for severity levels 1–10.
const TRITAN_MATRICES: [Matrix3; 10] = [
  // Severity 1
  Matrix3::new([
    [0.926670, 0.092514, -0.019184],
    [0.021191, 0.964503, 0.014306],
    [0.008437, 0.054813, 0.936750],
  ]),
  // Severity 2
  Matrix3::new([
    [0.895720, 0.133330, -0.029050],
    [0.029997, 0.945400, 0.024603],
    [0.013027, 0.104707, 0.882266],
  ]),
  // Severity 3
  Matrix3::new([
    [0.905871, 0.127791, -0.033662],
    [0.026856, 0.941251, 0.031893],
    [0.013410, 0.148296, 0.838294],
  ]),
  // Severity 4
  Matrix3::new([
    [0.948035, 0.089490, -0.037526],
    [0.014364, 0.946792, 0.038844],
    [0.010853, 0.193991, 0.795156],
  ]),
  // Severity 5
  Matrix3::new([
    [1.017277, 0.027029, -0.044306],
    [-0.006113, 0.958479, 0.047634],
    [0.006379, 0.248708, 0.744913],
  ]),
  // Severity 6
  Matrix3::new([
    [1.104996, -0.046633, -0.058363],
    [-0.032137, 0.971635, 0.060503],
    [0.001336, 0.317922, 0.680742],
  ]),
  // Severity 7
  Matrix3::new([
    [1.193214, -0.109812, -0.083402],
    [-0.058496, 0.979410, 0.079086],
    [-0.002346, 0.403492, 0.598854],
  ]),
  // Severity 8
  Matrix3::new([
    [1.257728, -0.139648, -0.118081],
    [-0.078003, 0.975409, 0.102594],
    [-0.003316, 0.501214, 0.502102],
  ]),
  // Severity 9
  Matrix3::new([
    [1.278864, -0.125333, -0.153531],
    [-0.084748, 0.957674, 0.127074],
    [-0.000989, 0.601151, 0.399838],
  ]),
  // Severity 10
  Matrix3::new([
    [1.255528, -0.076749, -0.178779],
    [-0.078411, 0.930809, 0.147602],
    [0.004733, 0.691367, 0.303900],
  ]),
];

/// Interpolates between two matrices at parameter `t` (0.0–1.0).
fn lerp_matrix(a: Matrix3, b: Matrix3, t: f64) -> Matrix3 {
  a * (1.0 - t) + b * t
}

/// Returns the simulation matrix for a given severity (0.0–1.0) from the precomputed table.
fn matrix_for_severity(matrices: &[Matrix3; 10], severity: f64) -> Matrix3 {
  let severity = severity.clamp(0.0, 1.0);
  let scaled = severity * 10.0;
  let index = (scaled.floor() as usize).min(9);

  if index == 0 && scaled < 1.0 {
    lerp_matrix(IDENTITY, matrices[0], scaled)
  } else if index >= 9 {
    matrices[9]
  } else {
    let frac = scaled - index as f64;
    lerp_matrix(matrices[index - 1], matrices[index], frac)
  }
}

/// Applies a Machado simulation matrix in linear sRGB space.
fn simulate(xyz: Xyz, matrices: &[Matrix3; 10], severity: f64) -> Xyz {
  let matrix = matrix_for_severity(matrices, severity);
  let rgb = xyz.to_rgb::<Srgb>();
  let linear = rgb.to_linear().components();
  let simulated = matrix * linear;
  let result = crate::space::LinearRgb::<Srgb>::from_normalized(simulated[0], simulated[1], simulated[2]);
  result.to_encoded().to_xyz()
}

/// Simulates protanomaly (reduced L-cone sensitivity) using the Machado 2009 algorithm.
///
/// `severity` ranges from 0.0 (normal vision) to 1.0 (complete dichromacy).
///
/// ```
/// # #[cfg(feature = "cvd-machado")]
/// # {
/// use farg::color_vision_deficiency::machado;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let mild = machado::protanomaly(color, 0.5);
/// let severe = machado::protanomaly(color, 1.0);
/// # }
/// ```
pub fn protanomaly(color: impl Into<Xyz>, severity: f64) -> Xyz {
  simulate(color.into(), &PROTAN_MATRICES, severity)
}

/// Simulates deuteranomaly (reduced M-cone sensitivity) using the Machado 2009 algorithm.
///
/// `severity` ranges from 0.0 (normal vision) to 1.0 (complete dichromacy).
///
/// ```
/// # #[cfg(feature = "cvd-machado")]
/// # {
/// use farg::color_vision_deficiency::machado;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let mild = machado::deuteranomaly(color, 0.5);
/// let severe = machado::deuteranomaly(color, 1.0);
/// # }
/// ```
pub fn deuteranomaly(color: impl Into<Xyz>, severity: f64) -> Xyz {
  simulate(color.into(), &DEUTAN_MATRICES, severity)
}

/// Simulates tritanomaly (reduced S-cone sensitivity) using the Machado 2009 algorithm.
///
/// `severity` ranges from 0.0 (normal vision) to 1.0 (complete dichromacy).
///
/// ```
/// # #[cfg(feature = "cvd-machado")]
/// # {
/// use farg::color_vision_deficiency::machado;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let mild = machado::tritanomaly(color, 0.5);
/// let severe = machado::tritanomaly(color, 1.0);
/// # }
/// ```
pub fn tritanomaly(color: impl Into<Xyz>, severity: f64) -> Xyz {
  simulate(color.into(), &TRITAN_MATRICES, severity)
}

#[cfg(test)]
mod test {
  use super::*;

  mod protanomaly_fn {
    use super::*;

    #[test]
    fn it_returns_valid_xyz() {
      let result = protanomaly(Xyz::new(0.4, 0.3, 0.2), 0.5);

      assert!(result.x().is_finite());
      assert!(result.y().is_finite());
      assert!(result.z().is_finite());
    }

    #[test]
    fn it_returns_identity_at_zero_severity() {
      let original = Xyz::new(0.4, 0.3, 0.2);
      let result = protanomaly(original, 0.0);

      assert!((result.x() - original.x()).abs() < 1e-4);
      assert!((result.y() - original.y()).abs() < 1e-4);
      assert!((result.z() - original.z()).abs() < 1e-4);
    }

    #[test]
    fn it_increases_distortion_with_severity() {
      let original = Xyz::new(0.4, 0.2, 0.1);
      let mild = protanomaly(original, 0.3);
      let severe = protanomaly(original, 0.9);

      let mild_dist =
        (mild.x() - original.x()).powi(2) + (mild.y() - original.y()).powi(2) + (mild.z() - original.z()).powi(2);
      let severe_dist =
        (severe.x() - original.x()).powi(2) + (severe.y() - original.y()).powi(2) + (severe.z() - original.z()).powi(2);

      assert!(severe_dist > mild_dist);
    }

    #[test]
    fn it_clamps_severity_below_zero() {
      let color = Xyz::new(0.4, 0.3, 0.2);
      let clamped = protanomaly(color, -0.5);
      let zero = protanomaly(color, 0.0);

      assert!((clamped.x() - zero.x()).abs() < 1e-10);
      assert!((clamped.y() - zero.y()).abs() < 1e-10);
      assert!((clamped.z() - zero.z()).abs() < 1e-10);
    }

    #[test]
    fn it_clamps_severity_above_one() {
      let color = Xyz::new(0.4, 0.3, 0.2);
      let clamped = protanomaly(color, 1.5);
      let one = protanomaly(color, 1.0);

      assert!((clamped.x() - one.x()).abs() < 1e-10);
      assert!((clamped.y() - one.y()).abs() < 1e-10);
      assert!((clamped.z() - one.z()).abs() < 1e-10);
    }
  }

  mod deuteranomaly_fn {
    use super::*;

    #[test]
    fn it_returns_valid_xyz() {
      let result = deuteranomaly(Xyz::new(0.4, 0.3, 0.2), 0.5);

      assert!(result.x().is_finite());
      assert!(result.y().is_finite());
      assert!(result.z().is_finite());
    }

    #[test]
    fn it_returns_identity_at_zero_severity() {
      let original = Xyz::new(0.4, 0.3, 0.2);
      let result = deuteranomaly(original, 0.0);

      assert!((result.x() - original.x()).abs() < 1e-4);
      assert!((result.y() - original.y()).abs() < 1e-4);
      assert!((result.z() - original.z()).abs() < 1e-4);
    }

    #[test]
    fn it_increases_distortion_with_severity() {
      let original = Xyz::new(0.2, 0.4, 0.1);
      let mild = deuteranomaly(original, 0.3);
      let severe = deuteranomaly(original, 0.9);

      let mild_dist =
        (mild.x() - original.x()).powi(2) + (mild.y() - original.y()).powi(2) + (mild.z() - original.z()).powi(2);
      let severe_dist =
        (severe.x() - original.x()).powi(2) + (severe.y() - original.y()).powi(2) + (severe.z() - original.z()).powi(2);

      assert!(severe_dist > mild_dist);
    }
  }

  mod tritanomaly_fn {
    use super::*;

    #[test]
    fn it_returns_valid_xyz() {
      let result = tritanomaly(Xyz::new(0.4, 0.3, 0.2), 0.5);

      assert!(result.x().is_finite());
      assert!(result.y().is_finite());
      assert!(result.z().is_finite());
    }

    #[test]
    fn it_returns_identity_at_zero_severity() {
      let original = Xyz::new(0.4, 0.3, 0.2);
      let result = tritanomaly(original, 0.0);

      assert!((result.x() - original.x()).abs() < 1e-4);
      assert!((result.y() - original.y()).abs() < 1e-4);
      assert!((result.z() - original.z()).abs() < 1e-4);
    }

    #[test]
    fn it_increases_distortion_with_severity() {
      let original = Xyz::new(0.2, 0.2, 0.4);
      let mild = tritanomaly(original, 0.3);
      let severe = tritanomaly(original, 0.9);

      let mild_dist =
        (mild.x() - original.x()).powi(2) + (mild.y() - original.y()).powi(2) + (mild.z() - original.z()).powi(2);
      let severe_dist =
        (severe.x() - original.x()).powi(2) + (severe.y() - original.y()).powi(2) + (severe.z() - original.z()).powi(2);

      assert!(severe_dist > mild_dist);
    }
  }

  mod matrix_for_severity_fn {
    use super::*;

    #[test]
    fn it_returns_identity_at_zero() {
      let m = matrix_for_severity(&PROTAN_MATRICES, 0.0);

      assert_eq!(m, IDENTITY);
    }

    #[test]
    fn it_returns_max_severity_matrix_at_one() {
      let m = matrix_for_severity(&PROTAN_MATRICES, 1.0);

      assert_eq!(m, PROTAN_MATRICES[9]);
    }

    #[test]
    fn it_interpolates_between_levels() {
      let m = matrix_for_severity(&PROTAN_MATRICES, 0.15);
      let a = PROTAN_MATRICES[0];
      let b = PROTAN_MATRICES[1];

      // At 0.15, we're between severity 1 (index 0) and severity 2 (index 1), at t=0.5
      let expected_00 = a.data()[0][0] * 0.5 + b.data()[0][0] * 0.5;
      assert!((m.data()[0][0] - expected_00).abs() < 1e-10);
    }
  }
}
