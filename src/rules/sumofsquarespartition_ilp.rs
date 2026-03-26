//! Reduction from SumOfSquaresPartition to ILP (Integer Linear Programming).
//!
//! The objective Σ_g (Σ_{i ∈ g} s_i)^2 is quadratic, so we linearize using McCormick:
//!
//! Variables:
//! - x_{i,g}: binary, element i in group g (index: i * K + g)
//! - z_{i,j,g}: binary product for x_{i,g} * x_{j,g} (index: n*K + (i*n + j) * K + g)
//!
//! Constraints:
//! - Σ_g x_{i,g} = 1 for each element i (assignment)
//! - McCormick for each (i,j,g):
//!   z_{i,j,g} ≤ x_{i,g}, z_{i,j,g} ≤ x_{j,g}, z_{i,j,g} ≥ x_{i,g} + x_{j,g} - 1
//!
//! Note: Σ_g (Σ_i s_i * x_{i,g})^2 = Σ_g Σ_{i,j} s_i * s_j * x_{i,g} * x_{j,g}
//!       which equals Σ_g Σ_{i,j} s_i * s_j * z_{i,j,g} after linearization.
//!
//! Objective: Minimize Σ_g Σ_{i,j} s_i * s_j * z_{i,j,g}

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SumOfSquaresPartition;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SumOfSquaresPartition to ILP.
///
/// Variable layout:
/// - x_{i,g} at index i * K + g  (i ∈ 0..n, g ∈ 0..K)
/// - z_{i,j,g} at index n*K + (i*n + j) * K + g  (i,j ∈ 0..n, g ∈ 0..K)
///
/// Total: n*K + n^2*K variables.
#[derive(Debug, Clone)]
pub struct ReductionSSPToILP {
    target: ILP<bool>,
    num_elements: usize,
    num_groups: usize,
}

impl ReductionSSPToILP {
    fn x_var(&self, i: usize, g: usize) -> usize {
        i * self.num_groups + g
    }

    fn z_var(&self, i: usize, j: usize, g: usize) -> usize {
        let n = self.num_elements;
        let k = self.num_groups;
        n * k + (i * n + j) * k + g
    }
}

impl ReductionResult for ReductionSSPToILP {
    type Source = SumOfSquaresPartition;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution: for each element i, find the unique group g where x_{i,g} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let num_groups = self.num_groups;
        (0..self.num_elements)
            .map(|i| {
                (0..num_groups)
                    .find(|&g| {
                        let idx = i * num_groups + g;
                        idx < target_solution.len() && target_solution[idx] == 1
                    })
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_elements * num_groups + num_elements^2 * num_groups",
        num_constraints = "num_elements + 3 * num_elements^2 * num_groups",
    }
)]
impl ReduceTo<ILP<bool>> for SumOfSquaresPartition {
    type Result = ReductionSSPToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_elements();
        let k = self.num_groups();
        let num_vars = n * k + n * n * k;

        let result = ReductionSSPToILP {
            target: ILP::empty(),
            num_elements: n,
            num_groups: k,
        };

        let mut constraints = Vec::new();

        // Assignment constraints: for each element i, Σ_g x_{i,g} = 1
        for i in 0..n {
            let terms: Vec<(usize, f64)> = (0..k).map(|g| (result.x_var(i, g), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // McCormick linearization for z_{i,j,g} = x_{i,g} * x_{j,g}
        for i in 0..n {
            for j in 0..n {
                for g in 0..k {
                    let z = result.z_var(i, j, g);
                    let xi = result.x_var(i, g);
                    let xj = result.x_var(j, g);

                    // z ≤ x_{i,g}
                    constraints.push(LinearConstraint::le(vec![(z, 1.0), (xi, -1.0)], 0.0));
                    // z ≤ x_{j,g}
                    constraints.push(LinearConstraint::le(vec![(z, 1.0), (xj, -1.0)], 0.0));
                    // z ≥ x_{i,g} + x_{j,g} - 1  →  -z + x_{i,g} + x_{j,g} ≤ 1
                    constraints.push(LinearConstraint::le(
                        vec![(z, -1.0), (xi, 1.0), (xj, 1.0)],
                        1.0,
                    ));
                }
            }
        }

        // Objective: Minimize Σ_g Σ_{i,j} s_i * s_j * z_{i,j,g}
        let sizes = self.sizes();
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for i in 0..n {
            for j in 0..n {
                for g in 0..k {
                    let coeff = sizes[i] as f64 * sizes[j] as f64;
                    if coeff.abs() > 0.0 {
                        objective.push((result.z_var(i, j, g), coeff));
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionSSPToILP {
            target,
            num_elements: n,
            num_groups: k,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
        vec![crate::example_db::specs::RuleExampleSpec {
        id: "sumofsquarespartition_to_ilp",
        build: || {
            // 4 elements [1, 2, 3, 4], K=2 groups
            // Group {1,4}: sum=5, Group {2,3}: sum=5 → sos = 25+25 = 50
            let source = SumOfSquaresPartition::new(vec![1, 2, 3, 4], 2);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sumofsquarespartition_ilp.rs"]
mod tests;
