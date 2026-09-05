//! Minimum Discrete Planar Inverse Kinematics problem implementation.
//!
//! Given positive link lengths, a target point in `R^2`, a finite set of
//! candidate absolute orientations per link, and admissible-pair sets between
//! consecutive links, choose one orientation index per link so that all
//! consecutive-pair constraints are satisfied and the squared distance from
//! the end-effector to the target point is minimized.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDiscretePlanarInverseKinematics",
        display_name: "Minimum Discrete Planar Inverse Kinematics",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
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
#[derive(Debug, Clone, Serialize)]
pub struct MinimumDiscretePlanarInverseKinematics {
    link_lengths: Vec<f64>,
    target_point: (f64, f64),
    orientation_samples: Vec<Vec<f64>>,
    allowed_pairs: Vec<Vec<(usize, usize)>>,
}

#[derive(Deserialize)]
struct MinimumDiscretePlanarInverseKinematicsData {
    link_lengths: Vec<f64>,
    target_point: (f64, f64),
    orientation_samples: Vec<Vec<f64>>,
    allowed_pairs: Vec<Vec<(usize, usize)>>,
}

impl<'de> Deserialize<'de> for MinimumDiscretePlanarInverseKinematics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = MinimumDiscretePlanarInverseKinematicsData::deserialize(deserializer)?;
        Self::new(
            data.link_lengths,
            data.target_point,
            data.orientation_samples,
            data.allowed_pairs,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl MinimumDiscretePlanarInverseKinematics {
    /// Construct a new instance.
    ///
    pub fn new(
        link_lengths: Vec<f64>,
        target_point: (f64, f64),
        orientation_samples: Vec<Vec<f64>>,
        allowed_pairs: Vec<Vec<(usize, usize)>>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        let n = link_lengths.len();
        if n == 0 {
            return Err("MinimumDiscretePlanarInverseKinematics requires at least one link".into());
        }
        for (index, &length) in link_lengths.iter().enumerate() {
            if !length.is_finite() || length <= 0.0 {
                return Err(
                    format!("link length at index {index} must be positive and finite").into(),
                );
            }
        }
        if !target_point.0.is_finite() || !target_point.1.is_finite() {
            return Err("target point coordinates must be finite".into());
        }
        if orientation_samples.len() != n {
            return Err("orientation_samples must have one entry per link".into());
        }
        let mut total_configurations = 1_usize;
        for (link, samples) in orientation_samples.iter().enumerate() {
            if samples.is_empty() {
                return Err(
                    format!("link {link} must have at least one candidate orientation").into(),
                );
            }
            total_configurations = total_configurations
                .checked_mul(samples.len())
                .ok_or("orientation configuration count exceeds usize")?;
            for (sample, &angle) in samples.iter().enumerate() {
                if !angle.is_finite() {
                    return Err(format!(
                        "orientation sample {sample} for link {link} must be finite"
                    )
                    .into());
                }
            }
        }
        if allowed_pairs.len() != n - 1 {
            return Err("allowed_pairs must have one entry per junction".into());
        }
        for (j_minus_1, pairs) in allowed_pairs.iter().enumerate() {
            let m_prev = orientation_samples[j_minus_1].len();
            let m_curr = orientation_samples[j_minus_1 + 1].len();
            for &(a_prev, a_curr) in pairs {
                if a_prev >= m_prev || a_curr >= m_curr {
                    return Err(format!(
                        "allowed pair ({a_prev}, {a_curr}) at junction {j_minus_1} is out of range"
                    )
                    .into());
                }
            }
        }
        Ok(Self {
            link_lengths,
            target_point,
            orientation_samples,
            allowed_pairs,
        })
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
        self.orientation_samples
            .iter()
            .map(|samples| samples.len())
            .try_fold(1_usize, usize::checked_mul)
            .expect("validated orientation configuration count must fit usize")
    }

    /// Total number of sampled orientations across all links:
    /// `sum_{j=1}^n m_j`. This is the QUBO variable count for the one-hot
    /// encoding used by the QUBO reduction.
    pub fn num_orientation_samples(&self) -> usize {
        self.orientation_samples.iter().map(Vec::len).sum()
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
    pub fn end_effector(
        &self,
        config: &[usize],
    ) -> Result<Option<(f64, f64)>, crate::traits::EvaluationError> {
        if !self.is_feasible(config) {
            return Ok(None);
        }
        let mut x = 0.0_f64;
        let mut y = 0.0_f64;
        for (j, &a) in config.iter().enumerate() {
            let phi = self.orientation_samples[j][a];
            let next_x = x + self.link_lengths[j] * phi.cos();
            let next_y = y + self.link_lengths[j] * phi.sin();
            if !next_x.is_finite() || !next_y.is_finite() {
                return Err(crate::traits::EvaluationError::NonFiniteResult(
                    "computing the inverse-kinematics end-effector position".into(),
                ));
            }
            x = next_x;
            y = next_y;
        }
        Ok(Some((x, y)))
    }

    /// Compute the squared end-effector distance to the target.
    /// Returns `None` if the configuration is infeasible.
    pub fn squared_distance(
        &self,
        config: &[usize],
    ) -> Result<Option<f64>, crate::traits::EvaluationError> {
        let Some((x, y)) = self.end_effector(config)? else {
            return Ok(None);
        };
        let dx = x - self.target_point.0;
        let dy = y - self.target_point.1;
        let squared_distance = dx * dx + dy * dy;
        if !squared_distance.is_finite() {
            return Err(crate::traits::EvaluationError::NonFiniteResult(
                "computing the inverse-kinematics squared distance".into(),
            ));
        }
        Ok(Some(squared_distance))
    }

    /// Whether the configuration represents a valid feasible solution.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_feasible(config)
    }
}

impl Problem for MinimumDiscretePlanarInverseKinematics {
    const NAME: &'static str = "MinimumDiscretePlanarInverseKinematics";
    type Solution = Vec<usize>;
    type Value = Min<f64>;

    crate::problem_parameters![
        ("total_configurations", total_configurations),
        ("num_links", num_links),
        ("num_orientation_samples", num_orientation_samples),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<f64>, crate::traits::EvaluationError> {
        if config.len() != self.num_links() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "orientation assignment length does not match the links".into(),
            ));
        }
        if config
            .iter()
            .enumerate()
            .any(|(link, &orientation)| orientation >= self.orientation_samples[link].len())
        {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "orientation assignment contains an out-of-range sample".into(),
            ));
        }
        Ok({
            match self.squared_distance(config)? {
                Some(value) => Min(Some(value)),
                None => Min(None),
            }
        })
    }
}

impl crate::solvers::BruteForceProblem for MinimumDiscretePlanarInverseKinematics {
    fn dimensions(&self) -> Vec<usize> {
        self.orientation_samples
            .iter()
            .map(|samples| samples.len())
            .collect()
    }
}

crate::declare_variants! {
    default MinimumDiscretePlanarInverseKinematics => "total_configurations",
}

crate::register_brute_force! {
    MinimumDiscretePlanarInverseKinematics,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use std::f64::consts::FRAC_PI_2;
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_discrete_planar_inverse_kinematics",
        instance: Box::new(
            MinimumDiscretePlanarInverseKinematics::new(
                vec![2.0, 1.0],
                (2.0, 1.0),
                vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
                vec![vec![(0, 0), (0, 1), (1, 1)]],
            )
            .unwrap(),
        ),
        optimal_config: serde_json::json!(vec![0, 1]),
        optimal_value: serde_json::json!(0.0),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_discrete_planar_inverse_kinematics.rs"]
mod tests;
