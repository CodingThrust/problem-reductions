//! Reduction from ExpectedRetrievalCost to ILP (Integer Linear Programming).
//!
//! The expected retrieval cost objective is quadratic in the assignment variables,
//! so McCormick linearization is used to produce a binary ILP:
//!
//! Variables:
//! - x_{r,s}: binary, record r placed in sector s (index: r * num_sectors + s)
//! - z_{r,s,r',s'}: binary product linearization for x_{r,s} * x_{r',s'} (index after x vars)
//!
//! Constraints:
//! - Assignment: Σ_s x_{r,s} = 1 for each r
//! - McCormick for each (r,s,r',s') product:
//!   z ≤ x_{r,s}, z ≤ x_{r',s'}, z ≥ x_{r,s} + x_{r',s'} - 1
//!
//! Objective: Minimize Σ_{r,s,r',s'} lat(s,s') * p_r * p_{r'} * z_{r,s,r',s'}

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::ExpectedRetrievalCost;
use crate::reduction;
use crate::rules::ilp_helpers::{mccormick_product, one_hot_decode_rows};
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Compute the latency distance between sectors on a circular device.
///
/// Returns the number of sectors between source and target (not counting source itself),
/// wrapping around. This matches the `latency_distance` function in the model.
fn latency_distance(num_sectors: usize, source: usize, target: usize) -> usize {
    if source < target {
        target - source - 1
    } else {
        num_sectors - source + target - 1
    }
}

/// Result of reducing ExpectedRetrievalCost to ILP.
///
/// Variable layout:
/// - x_{r,s} at index r * num_sectors + s  (0..num_records * num_sectors)
/// - z_{r,s,r',s'} at index num_records*num_sectors + (r * num_sectors + s) * (num_records * num_sectors) + (r' * num_sectors + s')
///
/// Total: num_records * num_sectors + (num_records * num_sectors)^2 variables.
#[derive(Debug, Clone)]
pub struct ReductionERCToILP {
    target: ILP<bool>,
    num_records: usize,
    num_sectors: usize,
}

impl ReductionERCToILP {
    fn x_var(&self, r: usize, s: usize) -> usize {
        r * self.num_sectors + s
    }

    fn z_var(&self, r: usize, s: usize, r2: usize, s2: usize) -> usize {
        let n = self.num_records * self.num_sectors;
        n + (r * self.num_sectors + s) * n + (r2 * self.num_sectors + s2)
    }
}

impl ReductionResult for ReductionERCToILP {
    type Source = ExpectedRetrievalCost;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution: for each record r, find the unique sector s where x_{r,s} = 1.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        one_hot_decode_rows(target_solution, self.num_records, self.num_sectors, 0)
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_records * num_sectors + num_records^2 * num_sectors^2",
        num_constraints = "num_records + 3 * num_records^2 * num_sectors^2",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for ExpectedRetrievalCost {
    type Result = ReductionERCToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_records = self.num_records();
        let num_sectors = self.num_sectors();
        let n = num_records * num_sectors; // total x variables
        let num_vars = n + n * n;

        let result = ReductionERCToILP {
            target: ILP::empty(),
            num_records,
            num_sectors,
        };

        let mut constraints = Vec::new();

        // Assignment constraints: for each record r, Σ_s x_{r,s} = 1
        for r in 0..num_records {
            let terms: Vec<(usize, i64)> =
                (0..num_sectors).map(|s| (result.x_var(r, s), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // McCormick linearization constraints for each product z_{r,s,r',s'}
        // z ≤ x_{r,s}      →  z - x_{r,s} ≤ 0
        // z ≤ x_{r',s'}    →  z - x_{r',s'} ≤ 0
        // z ≥ x_{r,s} + x_{r',s'} - 1  →  -z + x_{r,s} + x_{r',s'} ≤ 1
        for r in 0..num_records {
            for s in 0..num_sectors {
                for r2 in 0..num_records {
                    for s2 in 0..num_sectors {
                        let z = result.z_var(r, s, r2, s2);
                        let x1 = result.x_var(r, s);
                        let x2 = result.x_var(r2, s2);

                        constraints.extend(mccormick_product(z, x1, x2));
                    }
                }
            }
        }

        // Objective: Minimize Σ_{r,s,r',s'} lat(s,s') * p_r * p_{r'} * z_{r,s,r',s'}
        let probabilities = self.probabilities();
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for r in 0..num_records {
            for s in 0..num_sectors {
                for r2 in 0..num_records {
                    for s2 in 0..num_sectors {
                        let lat = latency_distance(num_sectors, s, s2) as f64;
                        if lat > 0.0 {
                            let coeff = lat * probabilities[r] * probabilities[r2];
                            if coeff.abs() > 0.0 {
                                let z = result.z_var(r, s, r2, s2);
                                objective.push((z, coeff));
                            }
                        }
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionERCToILP {
            target,
            num_records,
            num_sectors,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "expectedretrievalcost_to_ilp",
        build: || {
            // 2 records with probabilities [0.5, 0.5], 2 sectors
            // Assignment: record 0 → sector 0, record 1 → sector 1
            let source = ExpectedRetrievalCost::new(vec![0.5, 0.5], 2).unwrap();
            // Compute target_config from solver to ensure consistency
            let reduction: ReductionERCToILP =
                ReduceTo::<ILP<bool>>::reduce_to(&source).expect("reduction should succeed");
            let solver = crate::solvers::ILPSolver::new();
            let target_config = solver
                .solve(reduction.target_problem())
                .expect("canonical example should be feasible");
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 1]),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/expectedretrievalcost_ilp.rs"]
mod tests;
