//! Reduction from MinimumTardinessSequencing to `ILP<bool>`.
//!
//! Position-assignment ILP: binary x_{j,p} placing task j in position p,
//! with binary tardy indicator u_j. Precedence constraints and a
//! length-aware tardy indicator with big-M linearization.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MinimumTardinessSequencing;
use crate::reduction;
use crate::rules::ilp_helpers::one_hot_decode;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::One;

/// Result of reducing MinimumTardinessSequencing<One> to `ILP<bool>`.
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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_tasks;

            one_hot_decode(target_solution, n, n, 0)?
        })
    }
}

/// Result of reducing MinimumTardinessSequencing<i64> to `ILP<bool>`.
#[derive(Debug, Clone)]
pub struct ReductionMTSWeightedToILP {
    target: ILP<bool>,
    num_tasks: usize,
}

impl ReductionResult for ReductionMTSWeightedToILP {
    type Source = MinimumTardinessSequencing<i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_tasks;

            one_hot_decode(target_solution, n, n, 0)?
        })
    }
}

/// Build task assignment + position filling + precedence constraints (shared).
fn build_common_constraints(
    n: usize,
    positions: &[i64],
    precedences: &[(usize, usize)],
    x_var: impl Fn(usize, usize) -> usize,
) -> Vec<LinearConstraint> {
    let mut constraints = Vec::new();

    // 1. Each task assigned to exactly one position
    for j in 0..n {
        let terms: Vec<(usize, i64)> = (0..n).map(|p| (x_var(j, p), 1)).collect();
        constraints.push(LinearConstraint::eq(terms, 1));
    }

    // 2. Each position has exactly one task
    for p in 0..n {
        let terms: Vec<(usize, i64)> = (0..n).map(|j| (x_var(j, p), 1)).collect();
        constraints.push(LinearConstraint::eq(terms, 1));
    }

    // 3. Precedence constraints
    for &(i, j) in precedences {
        let mut terms: Vec<(usize, i64)> = Vec::new();
        for (p, &position) in positions.iter().enumerate() {
            terms.push((x_var(j, p), position));
            terms.push((x_var(i, p), -position));
        }
        constraints.push(LinearConstraint::ge(terms, 1));
    }

    constraints
}

// Unit-length variant
#[reduction(
    transform = exact {
        num_vars = "num_tasks * num_tasks + num_tasks",
        num_constraints = "2 * num_tasks + num_precedences + num_tasks",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumTardinessSequencing<One> {
    type Result = ReductionMTSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_tasks();
        let num_x_vars = n * n;
        let num_vars = num_x_vars + n;
        let positions = (0..n)
            .map(|position| Self::exact_i64(position, "representing a task position in ILP rows"))
            .collect::<Result<Vec<_>, _>>()?;
        let big_m = Self::exact_i64(n, "representing the number of tasks in ILP rows")?;

        let x_var = |j: usize, p: usize| -> usize { j * n + p };
        let u_var = |j: usize| -> usize { num_x_vars + j };

        let mut constraints = build_common_constraints(n, &positions, self.precedences(), x_var);

        // Tardy indicator (unit length: completion = p+1)
        for j in 0..n {
            let mut terms: Vec<(usize, i64)> =
                (0..n).map(|p| (x_var(j, p), positions[p] + 1)).collect();
            terms.push((u_var(j), -big_m));
            let deadline = self.deadlines()[j];
            constraints.push(LinearConstraint::le(terms, deadline));
        }

        let objective: Vec<(usize, f64)> = (0..n).map(|j| (u_var(j), 1.0)).collect();

        Ok(ReductionMTSToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_tasks: n,
        })
    }
}

// Arbitrary-length variant
#[reduction(
    transform = exact {
        num_vars = "num_tasks * num_tasks + num_tasks",
        num_constraints = "2 * num_tasks + num_precedences + num_tasks * num_tasks",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumTardinessSequencing<i64> {
    type Result = ReductionMTSWeightedToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_tasks();
        let num_x_vars = n * n;
        let num_vars = num_x_vars + n;
        let total_length = self.lengths().iter().try_fold(0_i64, |total, &length| {
            total.checked_add(length).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    MinimumTardinessSequencing<i64>,
                    ILP<bool>,
                >("summing task lengths")
            })
        })?;
        let big_m = total_length;
        let positions = (0..n)
            .map(|position| Self::exact_i64(position, "representing a task position in ILP rows"))
            .collect::<Result<Vec<_>, _>>()?;

        let x_var = |j: usize, p: usize| -> usize { j * n + p };
        let u_var = |j: usize| -> usize { num_x_vars + j };

        let mut constraints = build_common_constraints(n, &positions, self.precedences(), x_var);

        // Tardy indicator for arbitrary lengths.
        let lengths = self.lengths();
        for j in 0..n {
            for p in 0..n {
                let mut terms: Vec<(usize, i64)> = Vec::new();
                terms.push((x_var(j, p), big_m));
                for pp in 0..p {
                    for (jj, &len) in lengths.iter().enumerate() {
                        terms.push((x_var(jj, pp), len));
                    }
                }
                terms.push((u_var(j), -big_m));
                let rhs = self.deadlines()[j]
                    .checked_sub(lengths[j])
                    .and_then(|value| value.checked_add(total_length))
                    .ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<
                            MinimumTardinessSequencing<i64>,
                            ILP<bool>,
                        >("computing a tardiness constraint bound")
                    })?;
                constraints.push(LinearConstraint::le(terms, rhs));
            }
        }

        let objective: Vec<(usize, f64)> = (0..n).map(|j| (u_var(j), 1.0)).collect();

        Ok(ReductionMTSWeightedToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_tasks: n,
        })
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
                let source = MinimumTardinessSequencing::<i64>::with_lengths(
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
