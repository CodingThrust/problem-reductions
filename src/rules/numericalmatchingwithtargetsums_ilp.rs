//! Reduction from NumericalMatchingWithTargetSums to ILP (Integer Linear Programming).
//!
//! Binary variables z_{i,j,k} = 1 iff x_i is paired with y_j and assigned
//! to target k, but only created for compatible triples where
//! s(x_i) + s(y_j) = B_k.
//!
//! Constraints:
//! - Each x_i in exactly one pair: Σ_{j,k} z_{i,j,k} = 1
//! - Each y_j in exactly one pair: Σ_{i,k} z_{i,j,k} = 1
//! - Each target used exactly once: Σ_{i,j} z_{i,j,k} = 1
//!
//! Objective: minimize 0 (feasibility).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::NumericalMatchingWithTargetSums;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// A compatible triple (i, j, k) where s(x_i) + s(y_j) = B_k.
#[derive(Debug, Clone)]
struct CompatibleTriple {
    i: usize,
    j: usize,
    #[allow(dead_code)]
    k: usize,
}

/// Result of reducing NumericalMatchingWithTargetSums to ILP.
#[derive(Debug, Clone)]
pub struct ReductionNMTSToILP {
    target: ILP<bool>,
    /// Compatible triples, indexed by variable index.
    triples: Vec<CompatibleTriple>,
    /// Number of pairs (m).
    m: usize,
}

impl ReductionResult for ReductionNMTSToILP {
    type Source = NumericalMatchingWithTargetSums;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution: for each x_i find the y_j it is paired with.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut assignment = vec![0usize; self.m];
        for (var_idx, triple) in self.triples.iter().enumerate() {
            if target_solution[var_idx] == 1 {
                assignment[triple.i] = triple.j;
            }
        }
        assignment
    }
}

#[reduction(
    overhead = {
        num_vars = "num_pairs * num_pairs * num_pairs",
        num_constraints = "3 * num_pairs",
    }
)]
impl ReduceTo<ILP<bool>> for NumericalMatchingWithTargetSums {
    type Result = ReductionNMTSToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_pairs();
        let sx = self.sizes_x();
        let sy = self.sizes_y();
        let targets = self.targets();

        // Enumerate compatible triples: (i, j, k) where s(x_i) + s(y_j) = B_k
        let mut triples = Vec::new();
        for (i, &sxi) in sx.iter().enumerate() {
            for (j, &syj) in sy.iter().enumerate() {
                for (k, &tk) in targets.iter().enumerate() {
                    if sxi + syj == tk {
                        triples.push(CompatibleTriple { i, j, k });
                    }
                }
            }
        }

        let num_vars = triples.len();
        let mut constraints = Vec::with_capacity(3 * m);

        // Each x_i in exactly one pair: Σ_{(i,j,k)} z_{i,j,k} = 1 for each i
        for i in 0..m {
            let terms: Vec<(usize, f64)> = triples
                .iter()
                .enumerate()
                .filter(|(_, t)| t.i == i)
                .map(|(idx, _)| (idx, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Each y_j in exactly one pair: Σ_{(i,j,k)} z_{i,j,k} = 1 for each j
        for j in 0..m {
            let terms: Vec<(usize, f64)> = triples
                .iter()
                .enumerate()
                .filter(|(_, t)| t.j == j)
                .map(|(idx, _)| (idx, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Each target k used exactly once: Σ_{(i,j,k)} z_{i,j,k} = 1 for each k
        for k in 0..m {
            let terms: Vec<(usize, f64)> = triples
                .iter()
                .enumerate()
                .filter(|(_, t)| t.k == k)
                .map(|(idx, _)| (idx, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionNMTSToILP { target, triples, m }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "numericalmatchingwithtargetsums_to_ilp",
        build: || {
            let source =
                NumericalMatchingWithTargetSums::new(vec![1, 4, 7], vec![2, 5, 3], vec![3, 7, 12]);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/numericalmatchingwithtargetsums_ilp.rs"]
mod tests;
