//! Reduction from MinimumTardinessSequencing to ILP<bool>.
//!
//! Position-assignment ILP: binary x_{j,p} placing task j in position p,
//! with binary tardy indicator u_j. Precedence constraints and a
//! length-aware tardy indicator with big-M linearization.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MinimumTardinessSequencing;
use crate::reduction;
use crate::rules::ilp_helpers::{one_hot_decode, permutation_to_lehmer};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::One;

/// Result of reducing MinimumTardinessSequencing<One> to ILP<bool>.
#[derive(Debug, Clone)]
pub struct ReductionMTSToILP {
    target: ILP<bool>,
    num_tasks: usize,
}

impl ReductionResult for ReductionMTSToILP {
    type Source = MinimumTardinessSequencing<One>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_tasks;
        let schedule = one_hot_decode(target_solution, n, n, 0);
        permutation_to_lehmer(&schedule)
    }
}

/// Result of reducing MinimumTardinessSequencing<i32> to ILP<bool>.
#[derive(Debug, Clone)]
pub struct ReductionMTSWeightedToILP {
    target: ILP<bool>,
    num_tasks: usize,
}

impl ReductionResult for ReductionMTSWeightedToILP {
    type Source = MinimumTardinessSequencing<i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_tasks;
        let schedule = one_hot_decode(target_solution, n, n, 0);
        permutation_to_lehmer(&schedule)
    }
}

/// Build task assignment + position filling + precedence constraints (shared).
fn build_common_constraints(
    n: usize,
    precedences: &[(usize, usize)],
    x_var: impl Fn(usize, usize) -> usize,
) -> Vec<LinearConstraint> {
    let mut constraints = Vec::new();

    // 1. Each task assigned to exactly one position
    for j in 0..n {
        let terms: Vec<(usize, f64)> = (0..n).map(|p| (x_var(j, p), 1.0)).collect();
        constraints.push(LinearConstraint::eq(terms, 1.0));
    }

    // 2. Each position has exactly one task
    for p in 0..n {
        let terms: Vec<(usize, f64)> = (0..n).map(|j| (x_var(j, p), 1.0)).collect();
        constraints.push(LinearConstraint::eq(terms, 1.0));
    }

    // 3. Precedence constraints
    for &(i, j) in precedences {
        let mut terms: Vec<(usize, f64)> = Vec::new();
        for p in 0..n {
            terms.push((x_var(j, p), p as f64));
            terms.push((x_var(i, p), -(p as f64)));
        }
        constraints.push(LinearConstraint::ge(terms, 1.0));
    }

    constraints
}

// Unit-length variant
#[reduction(overhead = {
    num_vars = "num_tasks * num_tasks + num_tasks",
    num_constraints = "2 * num_tasks + num_precedences + num_tasks",
})]
impl ReduceTo<ILP<bool>> for MinimumTardinessSequencing<One> {
    type Result = ReductionMTSToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let num_x_vars = n * n;
        let num_vars = num_x_vars + n;
        let big_m = n as f64;

        let x_var = |j: usize, p: usize| -> usize { j * n + p };
        let u_var = |j: usize| -> usize { num_x_vars + j };

        let mut constraints = build_common_constraints(n, self.precedences(), x_var);

        // Tardy indicator (unit length: completion = p+1)
        for j in 0..n {
            let mut terms: Vec<(usize, f64)> =
                (0..n).map(|p| (x_var(j, p), (p + 1) as f64)).collect();
            terms.push((u_var(j), -big_m));
            constraints.push(LinearConstraint::le(terms, self.deadlines()[j] as f64));
        }

        let objective: Vec<(usize, f64)> = (0..n).map(|j| (u_var(j), 1.0)).collect();

        ReductionMTSToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_tasks: n,
        }
    }
}

// Arbitrary-length variant
#[reduction(overhead = {
    num_vars = "num_tasks * num_tasks + num_tasks",
    num_constraints = "2 * num_tasks + num_precedences + num_tasks * num_tasks",
})]
impl ReduceTo<ILP<bool>> for MinimumTardinessSequencing<i32> {
    type Result = ReductionMTSWeightedToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let num_x_vars = n * n;
        let num_vars = num_x_vars + n;
        let total_length: i32 = self.lengths().iter().copied().sum();
        let big_m = total_length as f64;

        let x_var = |j: usize, p: usize| -> usize { j * n + p };
        let u_var = |j: usize| -> usize { num_x_vars + j };

        let mut constraints = build_common_constraints(n, self.precedences(), x_var);

        // Tardy indicator for arbitrary lengths.
        let lengths = self.lengths();
        for j in 0..n {
            for p in 0..n {
                let mut terms: Vec<(usize, f64)> = Vec::new();
                terms.push((x_var(j, p), big_m));
                for pp in 0..p {
                    for (jj, &len) in lengths.iter().enumerate() {
                        terms.push((x_var(jj, pp), len as f64));
                    }
                }
                terms.push((u_var(j), -big_m));
                let rhs = self.deadlines()[j] as f64 - lengths[j] as f64 + big_m;
                constraints.push(LinearConstraint::le(terms, rhs));
            }
        }

        let objective: Vec<(usize, f64)> = (0..n).map(|j| (u_var(j), 1.0)).collect();

        ReductionMTSWeightedToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_tasks: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "minimumtardinesssequencing_to_ilp",
            build: || {
                let source = MinimumTardinessSequencing::<One>::new(3, vec![2, 3, 1], vec![(0, 2)]);
                crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "minimumtardinesssequencing_weighted_to_ilp",
            build: || {
                let source = MinimumTardinessSequencing::<i32>::with_lengths(
                    vec![2, 1, 3],
                    vec![3, 4, 5],
                    vec![(0, 2)],
                );
                crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumtardinesssequencing_ilp.rs"]
mod tests;
