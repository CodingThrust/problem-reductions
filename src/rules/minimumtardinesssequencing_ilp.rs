//! Reduction from MinimumTardinessSequencing to ILP<bool>.
//!
//! Position-assignment ILP: binary x_{j,p} placing task j in position p,
//! with binary tardy indicator u_j. Precedence constraints and a
//! deadline-based tardy indicator with big-M = n.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MinimumTardinessSequencing;
use crate::reduction;
use crate::rules::ilp_helpers::{one_hot_decode, permutation_to_lehmer};
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumTardinessSequencing to ILP<bool>.
///
/// Variable layout:
/// - x_{j,p} for j in 0..n, p in 0..n: index `j*n + p`
/// - u_j for j in 0..n: index `n*n + j`
///
/// Total: n^2 + n variables.
#[derive(Debug, Clone)]
pub struct ReductionMTSToILP {
    target: ILP<bool>,
    num_tasks: usize,
}

impl ReductionResult for ReductionMTSToILP {
    type Source = MinimumTardinessSequencing;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: decode position assignment x_{j,p} → permutation → Lehmer code.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_tasks;
        // Decode: for each position p, find which job j has x_{j,p}=1
        let schedule = one_hot_decode(target_solution, n, n, 0);
        permutation_to_lehmer(&schedule)
    }
}

#[reduction(overhead = {
    num_vars = "num_tasks * num_tasks + num_tasks",
    num_constraints = "2 * num_tasks + num_precedences + num_tasks",
})]
impl ReduceTo<ILP<bool>> for MinimumTardinessSequencing {
    type Result = ReductionMTSToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let num_x_vars = n * n;
        let num_u_vars = n;
        let num_vars = num_x_vars + num_u_vars;
        let big_m = n as f64;

        let x_var = |j: usize, p: usize| -> usize { j * n + p };
        let u_var = |j: usize| -> usize { num_x_vars + j };

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
        // => Σ_p p*x_{j,p} - Σ_p p*x_{i,p} >= 1
        for &(i, j) in self.precedences() {
            let mut terms: Vec<(usize, f64)> = Vec::new();
            for p in 0..n {
                terms.push((x_var(j, p), p as f64));
                terms.push((x_var(i, p), -(p as f64)));
            }
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // 4. Tardy indicator: Σ_p (p+1)*x_{j,p} - d_j <= M*u_j for all j
        // => Σ_p (p+1)*x_{j,p} - M*u_j <= d_j
        for j in 0..n {
            let mut terms: Vec<(usize, f64)> =
                (0..n).map(|p| (x_var(j, p), (p + 1) as f64)).collect();
            terms.push((u_var(j), -big_m));
            constraints.push(LinearConstraint::le(terms, self.deadlines()[j] as f64));
        }

        // Objective: minimize Σ_j u_j
        let objective: Vec<(usize, f64)> = (0..n).map(|j| (u_var(j), 1.0)).collect();

        ReductionMTSToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_tasks: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumtardinesssequencing_to_ilp",
        build: || {
            let source = MinimumTardinessSequencing::new(3, vec![2, 3, 1], vec![(0, 2)]);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumtardinesssequencing_ilp.rs"]
mod tests;
