//! Reduction from MaximumLikelihoodRanking to ILP (Integer Linear Programming).
//!
//! Binary variables x_{ij} for each pair (i, j) with i < j:
//! x_{ij} = 1 means item i is ranked before item j.
//!
//! Transitivity constraints: for each triple {a, b, c} with a < b < c:
//!   x_{ab} + x_{bc} - x_{ac} <= 1
//!   -x_{ab} - x_{bc} + x_{ac} <= 0
//!
//! Objective: minimize sum_{i<j} (a_{ji} - a_{ij}) * x_{ij}
//! (the constant sum_{i<j} a_{ij} does not affect the optimal solution)

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MaximumLikelihoodRanking;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MaximumLikelihoodRanking to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMaximumLikelihoodRankingToILP {
    target: ILP<bool>,
    /// Number of items in the original problem.
    n: usize,
}

/// Map a pair (i, j) with i < j to a variable index.
fn pair_index(i: usize, j: usize, n: usize) -> usize {
    debug_assert!(i < j && j < n);
    // Sum of (n-1) + (n-2) + ... + (n-i) for rows before i, plus offset within row i.
    // = i*n - i*(i+1)/2 + (j - i - 1)
    i * n - i * (i + 1) / 2 + (j - i - 1)
}

impl ReductionResult for ReductionMaximumLikelihoodRankingToILP {
    type Source = MaximumLikelihoodRanking;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        if n == 0 {
            return vec![];
        }

        // Count how many items are ranked before each item i.
        // config[i] = number of items ranked before i = rank of item i.
        let mut config = vec![0usize; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let idx = pair_index(i, j, n);
                if target_solution[idx] == 1 {
                    // i is before j -> contributes 1 to config[j]
                    config[j] += 1;
                } else {
                    // j is before i -> contributes 1 to config[i]
                    config[i] += 1;
                }
            }
        }

        config
    }
}

#[reduction(
    overhead = {
        num_vars = "num_items * (num_items - 1) / 2",
        num_constraints = "num_items * (num_items - 1) * (num_items - 2) / 3",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumLikelihoodRanking {
    type Result = ReductionMaximumLikelihoodRankingToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_items();
        let num_vars = n * (n.saturating_sub(1)) / 2;
        let matrix = self.matrix();

        // Build objective: minimize sum_{i<j} (a_{ji} - a_{ij}) * x_{ij}
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let coeff = (matrix[j][i] - matrix[i][j]) as f64;
                if coeff != 0.0 {
                    objective.push((pair_index(i, j, n), coeff));
                }
            }
        }

        // Build transitivity constraints:
        // For each triple (a, b, c) with a < b < c:
        //   x_{ab} + x_{bc} - x_{ac} <= 1
        //   -x_{ab} - x_{bc} + x_{ac} <= 0
        let mut constraints = Vec::new();
        for a in 0..n {
            for b in (a + 1)..n {
                for c in (b + 1)..n {
                    let ab = pair_index(a, b, n);
                    let bc = pair_index(b, c, n);
                    let ac = pair_index(a, c, n);

                    // x_{ab} + x_{bc} - x_{ac} <= 1
                    constraints.push(LinearConstraint::le(
                        vec![(ab, 1.0), (bc, 1.0), (ac, -1.0)],
                        1.0,
                    ));

                    // -x_{ab} - x_{bc} + x_{ac} <= 0
                    constraints.push(LinearConstraint::le(
                        vec![(ab, -1.0), (bc, -1.0), (ac, 1.0)],
                        0.0,
                    ));
                }
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionMaximumLikelihoodRankingToILP { target, n }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximum_likelihood_ranking_to_ilp",
        build: || {
            // Use a 3-item matrix for a small example: C(3,2)=3 variables
            let matrix = vec![vec![0, 3, 2], vec![2, 0, 4], vec![3, 1, 0]];
            crate::example_db::specs::rule_example_via_ilp(MaximumLikelihoodRanking::new(matrix))
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumlikelihoodranking_ilp.rs"]
mod tests;
