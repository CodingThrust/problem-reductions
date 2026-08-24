//! Reduction from SequencingToMinimizeMaximumCumulativeCost to `ILP<i64>`.
//!
//! Position-assignment ILP: binary x_{j,p} placing task j in position p.
//! Permutation constraints, precedence constraints, and prefix cumulative-cost
//! bounds at every position.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeMaximumCumulativeCost;
use crate::reduction;
use crate::rules::ilp_helpers::{one_hot_decode, permutation_to_lehmer};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::i64_to_exact_f64;

/// Result of reducing SequencingToMinimizeMaximumCumulativeCost to `ILP<i64>`.
///
/// Variable layout:
/// - x_{j,p} for j in 0..n, p in 0..n: index `j*n + p`
///
/// Total: n^2 variables.
#[derive(Debug, Clone)]
pub struct ReductionSTMMCCToILP {
    target: ILP<i64>,
    num_tasks: usize,
}

impl ReductionResult for ReductionSTMMCCToILP {
    type Source = SequencingToMinimizeMaximumCumulativeCost;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    /// Extract: decode position assignment → permutation → Lehmer code.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_tasks;
            let schedule = one_hot_decode(target_solution, n, n, 0)?;
            permutation_to_lehmer(&schedule)
        })
    }
}

#[reduction(size = exact {
    num_vars = "num_tasks^2 + 1",
    num_constraints = "num_tasks^2 + 3 * num_tasks + num_precedences + 1",
})]
impl ReduceTo<ILP<i64>> for SequencingToMinimizeMaximumCumulativeCost {
    type Result = ReductionSTMMCCToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
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

        // Binary bounds for x variables (`ILP<i64>` allows any non-negative integer)
        for j in 0..n {
            for p in 0..n {
                constraints.push(LinearConstraint::le(vec![(x_var(j, p), 1.0)], 1.0));
            }
        }

        // 4. Prefix cumulative cost: Σ_j Σ_{p in 0..=q} c_j * x_{j,p} <= z for all q
        //    (minimax linearization: z >= max_q cumulative_cost(q))
        let costs = self.costs();
        let costs_f64: Vec<f64> = costs
            .iter()
            .map(|&cost| {
                i64_to_exact_f64(cost).map_err(|error| {
                    crate::rules::ReductionError::inexact_float_conversion::<
                        SequencingToMinimizeMaximumCumulativeCost,
                        ILP<i64>,
                    >(error)
                })
            })
            .collect::<Result<_, _>>()?;
        for q in 0..n {
            let mut terms: Vec<(usize, f64)> = Vec::new();
            for (j, &c_j) in costs_f64.iter().enumerate() {
                for p in 0..=q {
                    terms.push((x_var(j, p), c_j));
                }
            }
            terms.push((z_var, -1.0));
            constraints.push(LinearConstraint::le(terms, 0.0));
        }

        // z upper bound: max cumulative cost ≤ sum of absolute costs
        let z_upper_i64 = costs.iter().try_fold(0_i64, |total, &cost| {
            let magnitude = cost.checked_abs().ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    SequencingToMinimizeMaximumCumulativeCost,
                    ILP<i64>,
                >("taking the absolute value of a task cost")
            })?;
            total.checked_add(magnitude).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    SequencingToMinimizeMaximumCumulativeCost,
                    ILP<i64>,
                >("summing absolute task costs")
            })
        })?;
        let z_upper = i64_to_exact_f64(z_upper_i64).map_err(|error| {
            crate::rules::ReductionError::inexact_float_conversion::<
                SequencingToMinimizeMaximumCumulativeCost,
                ILP<i64>,
            >(error)
        })?;
        constraints.push(LinearConstraint::le(vec![(z_var, 1.0)], z_upper));

        // Objective: minimize z (the maximum cumulative cost)
        let objective = vec![(z_var, 1.0)];

        Ok(ReductionSTMMCCToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_tasks: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingtominimizemaximumcumulativecost_to_ilp",
        build: || {
            let source =
                SequencingToMinimizeMaximumCumulativeCost::new(vec![2, -1, 3, -2], vec![(0, 2)]);
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingtominimizemaximumcumulativecost_ilp.rs"]
mod tests;
