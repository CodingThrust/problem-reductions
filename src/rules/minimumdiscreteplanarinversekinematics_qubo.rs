//! Reduction from MinimumDiscretePlanarInverseKinematics to QUBO.
//!
//! Each sampled orientation becomes a binary one-hot variable. The planar
//! end-effector coordinates are linear in those selectors, so the squared
//! distance to the target expands directly into a quadratic objective. One-hot
//! penalties enforce exactly one orientation per link, and forbidden-pair
//! penalties enforce the consecutive-joint feasibility relations.
//!
//! Reference: Salloum et al. (2025).

use crate::models::algebraic::QUBO;
use crate::models::misc::MinimumDiscretePlanarInverseKinematics;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

fn block_offsets(block_sizes: &[usize]) -> Vec<usize> {
    let mut offsets = Vec::with_capacity(block_sizes.len());
    let mut offset = 0;
    for &size in block_sizes {
        offsets.push(offset);
        offset += size;
    }
    offsets
}

/// Result of reducing MinimumDiscretePlanarInverseKinematics to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionMinimumDiscretePlanarInverseKinematicsToQUBO {
    target: QUBO<f64>,
    block_offsets: Vec<usize>,
    block_sizes: Vec<usize>,
}

impl ReductionResult for ReductionMinimumDiscretePlanarInverseKinematicsToQUBO {
    type Source = MinimumDiscretePlanarInverseKinematics;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        self.block_offsets
            .iter()
            .zip(&self.block_sizes)
            .enumerate()
            .map(|(link, (&start, &size))| {
                let mut selected = target_solution[start..start + size]
                    .iter()
                    .enumerate()
                    .filter_map(|(orientation, &bit)| bit.then_some(orientation));
                match (selected.next(), selected.next()) {
                    (Some(orientation), None) => Ok(orientation),
                    (None, _) => Err(crate::rules::ExtractionError::invalid(format!(
                        "link {link} has no selected orientation"
                    ))),
                    (Some(_), Some(_)) => Err(crate::rules::ExtractionError::invalid(format!(
                        "link {link} has multiple selected orientations"
                    ))),
                }
            })
            .collect()
    }
}

#[reduction(size = exact {
    num_vars = "num_orientation_samples",
})]
impl ReduceTo<QUBO<f64>> for MinimumDiscretePlanarInverseKinematics {
    type Result = ReductionMinimumDiscretePlanarInverseKinematicsToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let block_sizes: Vec<usize> = self.orientation_samples().iter().map(Vec::len).collect();
        let block_offsets = block_offsets(&block_sizes);
        let total_vars: usize = block_sizes.iter().sum();
        let (gx, gy) = self.target_point();

        let mut x_coeffs = Vec::with_capacity(total_vars);
        let mut y_coeffs = Vec::with_capacity(total_vars);
        for (&length, samples) in self.link_lengths().iter().zip(self.orientation_samples()) {
            for &angle in samples {
                x_coeffs.push(length * angle.cos());
                y_coeffs.push(length * angle.sin());
            }
        }

        // A violation contributes at least one full penalty unit. This bound
        // exceeds the largest possible squared distance of any decoded source
        // configuration, so every QUBO minimizer for a feasible source
        // instance is one-hot and pair-feasible.
        let sum_abs_x: f64 = x_coeffs.iter().map(|coeff| coeff.abs()).sum();
        let sum_abs_y: f64 = y_coeffs.iter().map(|coeff| coeff.abs()).sum();
        let penalty = 1.0 + (sum_abs_x + gx.abs()).powi(2) + (sum_abs_y + gy.abs()).powi(2);

        let mut matrix = vec![vec![0.0; total_vars]; total_vars];
        let mut add_upper = |i: usize, j: usize, value: f64| {
            let (lo, hi) = if i <= j { (i, j) } else { (j, i) };
            matrix[lo][hi] += value;
        };

        // Position objective: (X - g_x)^2 + (Y - g_y)^2, dropping the
        // additive constant g_x^2 + g_y^2.
        for (idx, (&x_coeff, &y_coeff)) in x_coeffs.iter().zip(&y_coeffs).enumerate() {
            add_upper(
                idx,
                idx,
                x_coeff * x_coeff - 2.0 * gx * x_coeff + y_coeff * y_coeff - 2.0 * gy * y_coeff,
            );
        }
        for i in 0..total_vars {
            for j in (i + 1)..total_vars {
                add_upper(
                    i,
                    j,
                    2.0 * (x_coeffs[i] * x_coeffs[j] + y_coeffs[i] * y_coeffs[j]),
                );
            }
        }

        // One-hot penalty per link: P * (sum_a y_{j,a} - 1)^2.
        for (&start, &size) in block_offsets.iter().zip(&block_sizes) {
            for a in 0..size {
                add_upper(start + a, start + a, -penalty);
            }
            for a in 0..size {
                for b in (a + 1)..size {
                    add_upper(start + a, start + b, 2.0 * penalty);
                }
            }
        }

        // Forbidden-pair penalties between consecutive links.
        for (junction, pairs) in self.allowed_pairs().iter().enumerate() {
            let prev_size = block_sizes[junction];
            let curr_size = block_sizes[junction + 1];
            let prev_start = block_offsets[junction];
            let curr_start = block_offsets[junction + 1];

            let mut allowed = vec![vec![false; curr_size]; prev_size];
            for &(a_prev, a_curr) in pairs {
                allowed[a_prev][a_curr] = true;
            }

            for (a_prev, row) in allowed.iter().enumerate() {
                for (a_curr, &is_allowed) in row.iter().enumerate() {
                    if !is_allowed {
                        add_upper(prev_start + a_prev, curr_start + a_curr, penalty);
                    }
                }
            }
        }

        Ok(ReductionMinimumDiscretePlanarInverseKinematicsToQUBO {
            target: QUBO::from_matrix(matrix).map_err(|message| {
                crate::rules::ReductionError::construction::<
                    MinimumDiscretePlanarInverseKinematics,
                    QUBO<f64>,
                >(message)
            })?,
            block_offsets,
            block_sizes,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use std::f64::consts::FRAC_PI_2;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumdiscreteplanarinversekinematics_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<f64>>(
                MinimumDiscretePlanarInverseKinematics::new(
                    vec![2.0, 1.0],
                    (2.0, 1.0),
                    vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
                    vec![vec![(0, 0), (0, 1), (1, 1)]],
                )
                .unwrap(),
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 1]),
                    target_config: serde_json::json!(vec![true, false, false, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumdiscreteplanarinversekinematics_qubo.rs"]
mod tests;
