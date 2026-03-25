//! Reduction from SequencingToMinimizeMaximumCumulativeCost to ILP<i32>.
//!
//! Position-assignment ILP: binary x_{j,p} placing task j in position p.
//! Permutation constraints, precedence constraints, and prefix cumulative-cost
//! bounds at every position.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeMaximumCumulativeCost;
use crate::reduction;
use crate::rules::ilp_helpers::{one_hot_decode, permutation_to_lehmer};
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SequencingToMinimizeMaximumCumulativeCost to ILP<i32>.
///
/// Variable layout:
/// - x_{j,p} for j in 0..n, p in 0..n: index `j*n + p`
///
/// Total: n^2 variables.
#[derive(Debug, Clone)]
pub struct ReductionSTMMCCToILP {
    target: ILP<i32>,
    num_tasks: usize,
}

impl ReductionResult for ReductionSTMMCCToILP {
    type Source = SequencingToMinimizeMaximumCumulativeCost;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract: decode position assignment → permutation → Lehmer code.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_tasks;
        let schedule = one_hot_decode(target_solution, n, n, 0);
        permutation_to_lehmer(&schedule)
    }
}

#[reduction(overhead = {
    num_vars = "num_tasks * num_tasks + 1",
    num_constraints = "2 * num_tasks + num_precedences + num_tasks + num_tasks * num_tasks",
})]
impl ReduceTo<ILP<i32>> for SequencingToMinimizeMaximumCumulativeCost {
    type Result = ReductionSTMMCCToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        // n^2 position variables + 1 minimax variable z
        let z_var = n * n;
        let num_vars = n * n + 1;

        let x_var = |j: usize, p: usize| -> usize { j * n + p };

        let mut constraints = Vec::new();

        // 1. Each task assigned to exactly one position: Σ_p x_{j,p} = 1 for all j
        for j in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|p| (x_var(j, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2. Each position has exactly one task: Σ_j x_{j,p} = 1 for all p
        for p in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|j| (x_var(j, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 3. Precedence: Σ_p p*x_{i,p} + 1 <= Σ_p p*x_{j,p} for each (i,j)
        for &(i, j) in self.precedences() {
            let mut terms: Vec<(usize, f64)> = Vec::new();
            for p in 0..n {
                terms.push((x_var(j, p), p as f64));
                terms.push((x_var(i, p), -(p as f64)));
            }
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // Binary bounds for x variables (ILP<i32> allows any non-negative integer)
        for j in 0..n {
            for p in 0..n {
                constraints.push(LinearConstraint::le(vec![(x_var(j, p), 1.0)], 1.0));
            }
        }

        // 4. Prefix cumulative cost: Σ_j Σ_{p in 0..=q} c_j * x_{j,p} <= z for all q
        //    (minimax linearization: z >= max_q cumulative_cost(q))
        let costs = self.costs();
        for q in 0..n {
            let mut terms: Vec<(usize, f64)> = Vec::new();
            for (j, &c_j) in costs.iter().enumerate() {
                for p in 0..=q {
                    terms.push((x_var(j, p), c_j as f64));
                }
            }
            terms.push((z_var, -1.0));
            constraints.push(LinearConstraint::le(terms, 0.0));
        }

        // Objective: minimize z (the maximum cumulative cost)
        let objective = vec![(z_var, 1.0)];

        ReductionSTMMCCToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_tasks: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingtominimizemaximumcumulativecost_to_ilp",
        build: || {
            // costs=[2,-1,3,-2], precedences=[(0,2)], n=4.
            // Optimal schedule: [1,3,0,2] (task 1 first, then 3, 0, 2).
            // Lehmer code: [1,2,0,0]. Max cumulative cost = 3.
            // Variable layout: x_{j,p} (16) + z (1) = 17 total.
            let source =
                SequencingToMinimizeMaximumCumulativeCost::new(vec![2, -1, 3, -2], vec![(0, 2)]);
            #[rustfmt::skip]
            let target_config = vec![
                // x_{j,p}: task j at position p (one-hot per task)
                0, 0, 1, 0,  // task 0 → pos 2
                1, 0, 0, 0,  // task 1 → pos 0
                0, 0, 0, 1,  // task 2 → pos 3
                0, 1, 0, 0,  // task 3 → pos 1
                // z: max cumulative cost
                3,
            ];
            // Lehmer code for permutation [1,3,0,2]
            let source_config = vec![1, 2, 0, 0];
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingtominimizemaximumcumulativecost_ilp.rs"]
mod tests;
