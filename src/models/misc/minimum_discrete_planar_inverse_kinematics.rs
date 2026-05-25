//! Minimum Discrete Planar Inverse Kinematics problem implementation.
//!
//! Given positive link lengths, a target point in `R^2`, a finite set of
//! candidate absolute orientations per link, and admissible-pair sets between
//! consecutive links, choose one orientation index per link so that all
//! consecutive-pair constraints are satisfied and the squared distance from
//! the end-effector to the target point is minimized.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDiscretePlanarInverseKinematics",
        display_name: "Minimum Discrete Planar Inverse Kinematics",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Pick one sampled absolute orientation per link, subject to consecutive-pair feasibility constraints, to minimize the squared distance from the end-effector to a target point",
        fields: &[
            FieldInfo {
                name: "link_lengths",
                type_name: "Vec<f64>",
                description: "Positive link lengths l_1, ..., l_n",
            },
            FieldInfo {
                name: "target_point",
                type_name: "(f64, f64)",
                description: "Target point g = (g_x, g_y) in R^2",
            },
            FieldInfo {
                name: "orientation_samples",
                type_name: "Vec<Vec<f64>>",
                description: "Sampled absolute orientations Phi_j for each link j",
            },
            FieldInfo {
                name: "allowed_pairs",
                type_name: "Vec<Vec<(usize, usize)>>",
                description: "Admissible (a_{j-1}, a_j) pair sets A_j for j = 2, ..., n",
            },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "MinimumDiscretePlanarInverseKinematics",
        fields: &["num_links"],
    }
}

/// The Minimum Discrete Planar Inverse Kinematics problem.
///
/// Given positive link lengths `l_1, ..., l_n`, a target point `g` in
/// `R^2`, sampled absolute orientations `Phi_j = {phi_{j,0}, ..., phi_{j,m_j-1}}`
/// for each link, and admissible-pair sets
/// `A_j ⊆ {0, ..., m_{j-1}-1} x {0, ..., m_j-1}` for `j = 2, ..., n`,
/// choose indices `a_j ∈ {0, ..., m_j-1}` such that `(a_{j-1}, a_j) ∈ A_j`
/// for every `j = 2, ..., n`, minimizing
///
/// `|| Σ_{j=1}^n l_j (cos(phi_{j,a_j}), sin(phi_{j,a_j})) - g ||_2^2`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumDiscretePlanarInverseKinematics {
    link_lengths: Vec<f64>,
    target_point: (f64, f64),
    orientation_samples: Vec<Vec<f64>>,
    allowed_pairs: Vec<Vec<(usize, usize)>>,
}

impl MinimumDiscretePlanarInverseKinematics {
    /// Construct a new instance.
    ///
    /// # Panics
    /// Panics if the input fields are not mutually consistent (see the
    /// validation rules in the source).
    pub fn new(
        link_lengths: Vec<f64>,
        target_point: (f64, f64),
        orientation_samples: Vec<Vec<f64>>,
        allowed_pairs: Vec<Vec<(usize, usize)>>,
    ) -> Self {
        let n = link_lengths.len();
        assert!(
            n >= 1,
            "MinimumDiscretePlanarInverseKinematics requires at least one link"
        );
        for &length in &link_lengths {
            assert!(
                length.is_finite() && length > 0.0,
                "link lengths must be positive finite reals"
            );
        }
        assert!(
            target_point.0.is_finite() && target_point.1.is_finite(),
            "target point coordinates must be finite reals"
        );
        assert_eq!(
            orientation_samples.len(),
            n,
            "orientation_samples must have one entry per link"
        );
        for samples in &orientation_samples {
            assert!(
                !samples.is_empty(),
                "each link must have at least one candidate orientation"
            );
            for &angle in samples {
                assert!(
                    angle.is_finite(),
                    "orientation samples must be finite real numbers"
                );
            }
        }
        assert_eq!(
            allowed_pairs.len(),
            n.saturating_sub(1),
            "allowed_pairs must have one entry per junction (n - 1 entries)"
        );
        for (j_minus_1, pairs) in allowed_pairs.iter().enumerate() {
            let m_prev = orientation_samples[j_minus_1].len();
            let m_curr = orientation_samples[j_minus_1 + 1].len();
            for &(a_prev, a_curr) in pairs {
                assert!(
                    a_prev < m_prev,
                    "allowed_pair index out of range for previous link"
                );
                assert!(
                    a_curr < m_curr,
                    "allowed_pair index out of range for current link"
                );
            }
        }
        Self {
            link_lengths,
            target_point,
            orientation_samples,
            allowed_pairs,
        }
    }

    /// Get the link lengths.
    pub fn link_lengths(&self) -> &[f64] {
        &self.link_lengths
    }

    /// Get the target point.
    pub fn target_point(&self) -> (f64, f64) {
        self.target_point
    }

    /// Get the per-link orientation samples.
    pub fn orientation_samples(&self) -> &[Vec<f64>] {
        &self.orientation_samples
    }

    /// Get the admissible-pair sets for consecutive junctions.
    pub fn allowed_pairs(&self) -> &[Vec<(usize, usize)>] {
        &self.allowed_pairs
    }

    /// Number of links `n`.
    pub fn num_links(&self) -> usize {
        self.link_lengths.len()
    }

    /// Total number of configurations (product of per-link sample counts):
    /// `prod_{j=1}^n m_j`. This is the size of the brute-force search space.
    pub fn total_configurations(&self) -> usize {
        self.orientation_samples.iter().map(|s| s.len()).product()
    }

    /// Check if a configuration is feasible (one index per link, in range,
    /// and every consecutive pair lies in the corresponding admissible set).
    pub fn is_feasible(&self, config: &[usize]) -> bool {
        let n = self.num_links();
        if config.len() != n {
            return false;
        }
        for (j, &a) in config.iter().enumerate() {
            if a >= self.orientation_samples[j].len() {
                return false;
            }
        }
        for j in 1..n {
            let pair = (config[j - 1], config[j]);
            if !self.allowed_pairs[j - 1].contains(&pair) {
                return false;
            }
        }
        true
    }

    /// Compute the end-effector position for a configuration.
    /// Returns `None` if the configuration is infeasible.
    pub fn end_effector(&self, config: &[usize]) -> Option<(f64, f64)> {
        if !self.is_feasible(config) {
            return None;
        }
        let mut x = 0.0_f64;
        let mut y = 0.0_f64;
        for (j, &a) in config.iter().enumerate() {
            let phi = self.orientation_samples[j][a];
            x += self.link_lengths[j] * phi.cos();
            y += self.link_lengths[j] * phi.sin();
        }
        Some((x, y))
    }

    /// Compute the squared end-effector distance to the target.
    /// Returns `None` if the configuration is infeasible.
    pub fn squared_distance(&self, config: &[usize]) -> Option<f64> {
        let (x, y) = self.end_effector(config)?;
        let dx = x - self.target_point.0;
        let dy = y - self.target_point.1;
        Some(dx * dx + dy * dy)
    }

    /// Whether the configuration represents a valid feasible solution.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_feasible(config)
    }
}

impl Problem for MinimumDiscretePlanarInverseKinematics {
    const NAME: &'static str = "MinimumDiscretePlanarInverseKinematics";
    type Value = Min<f64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.orientation_samples
            .iter()
            .map(|samples| samples.len())
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> Min<f64> {
        match self.squared_distance(config) {
            Some(value) => Min(Some(value)),
            None => Min(None),
        }
    }
}

crate::declare_variants! {
    default MinimumDiscretePlanarInverseKinematics => "num_links * total_configurations",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use std::f64::consts::FRAC_PI_2;
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_discrete_planar_inverse_kinematics",
        instance: Box::new(MinimumDiscretePlanarInverseKinematics::new(
            vec![2.0, 1.0],
            (2.0, 1.0),
            vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
            vec![vec![(0, 0), (0, 1), (1, 1)]],
        )),
        optimal_config: vec![0, 1],
        optimal_value: serde_json::json!(0.0),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_discrete_planar_inverse_kinematics.rs"]
mod tests;
