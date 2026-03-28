//! Reduction from QuadraticAssignment to ILP (Integer Linear Programming).
//!
//! Linearized assignment formulation:
//! - Binary x_{i,p}: facility i at location p
//! - Binary z_{(i,p),(j,q)}: product x_{i,p} * x_{j,q} for i != j
//! - Assignment: each facility to exactly one location, each location at most one facility
//! - McCormick linearization for z variables
//! - Objective: minimize sum_{i!=j} C[i][j] * D[p][q] * z_{(i,p),(j,q)}

use crate::models::algebraic::QuadraticAssignment;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::reduction;
use crate::rules::ilp_helpers::{mccormick_product, one_hot_assignment_constraints};
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing QuadraticAssignment to ILP.
///
/// Variable layout (all binary):
/// - `x_{i,p}` at index `i * m + p` for facility i, location p
/// - `z` variables for McCormick products, indexed sequentially after x
#[derive(Debug, Clone)]
pub struct ReductionQAPToILP {
    target: ILP<bool>,
    num_facilities: usize,
    num_locations: usize,
}

impl ReductionResult for ReductionQAPToILP {
    type Source = QuadraticAssignment;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: for each facility i, output the unique location p with x_{i,p} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let loc = self.num_locations;
        (0..self.num_facilities)
            .map(|i| {
                (0..loc)
                    .find(|&p| target_solution[i * loc + p] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_facilities * num_locations + num_facilities^2 * num_locations^2",
        num_constraints = "num_facilities + num_locations + 3 * num_facilities^2 * num_locations^2",
    }
)]
impl ReduceTo<ILP<bool>> for QuadraticAssignment {
    type Result = ReductionQAPToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_facilities();
        let loc = self.num_locations();
        let cost = self.cost_matrix();
        let dist = self.distance_matrix();

        let num_x = n * loc;

        let x_idx = |i: usize, p: usize| -> usize { i * loc + p };

        // Enumerate z-variable pairs: (i, p, j, q) for i != j
        let mut z_pairs = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                for p in 0..loc {
                    for q in 0..loc {
                        z_pairs.push((i, p, j, q));
                    }
                }
            }
        }

        let num_z = z_pairs.len();
        let num_vars = num_x + num_z;

        let z_idx = |z_seq: usize| -> usize { num_x + z_seq };

        let mut constraints = Vec::new();

        // Assignment constraints
        constraints.extend(one_hot_assignment_constraints(n, loc, 0));

        // McCormick linearization for z variables
        for (z_seq, &(i, p, j, q)) in z_pairs.iter().enumerate() {
            constraints.extend(mccormick_product(z_idx(z_seq), x_idx(i, p), x_idx(j, q)));
        }

        // Objective: minimize sum_{i!=j,p,q} C[i][j] * D[p][q] * z_{(i,p),(j,q)}
        let mut objective = Vec::new();
        for (z_seq, &(i, p, j, q)) in z_pairs.iter().enumerate() {
            let coeff = cost[i][j] as f64 * dist[p][q] as f64;
            if coeff != 0.0 {
                objective.push((z_idx(z_seq), coeff));
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionQAPToILP {
            target,
            num_facilities: n,
            num_locations: loc,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "quadraticassignment_to_ilp",
        build: || {
            // 2x2 QAP: 2 facilities, 2 locations
            let source = QuadraticAssignment::new(
                vec![vec![0, 1], vec![1, 0]],
                vec![vec![0, 2], vec![2, 0]],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/quadraticassignment_ilp.rs"]
mod tests;
