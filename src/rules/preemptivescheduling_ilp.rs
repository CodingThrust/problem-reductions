//! Reduction from PreemptiveScheduling to `ILP<i64>`.
//!
//! Time-indexed formulation with an auxiliary integer makespan variable:
//! - Variables: binary x_{t,u} for t in 0..n, u in 0..D_max (task t processed at slot u),
//!   plus integer M (the makespan), indexed at position n*D_max.
//! - Variable index for x_{t,u}: t * D_max + u.
//! - Variable index for M: n * D_max.
//! - Constraints:
//!   1. Work: Σ_u x_{t,u} = l(t) for each task t
//!   2. Capacity: Σ_t x_{t,u} ≤ m for each time slot u
//!   3. Precedence: for each (pred, succ) and each slot u,
//!      `l(pred) * x_{succ,u} ≤ Σ_{v=0}^{u-1} x_{pred,v}`
//!      This ensures succ can only be active at slot u if pred has already
//!      completed all l(pred) units of work in slots 0..u-1.
//!   4. Makespan lower bound: M ≥ (u+1) when x_{t,u}=1:
//!      `M - (u+1)*x_{t,u} ≥ 0` for all t,u
//!   5. Binary bounds: x_{t,u} ≤ 1 for each t,u
//!      (since `ILP<i64>` uses non-negative integer domain)
//! - Objective: Minimize M.
//!
//! Note: `ILP<i64>` treats all variables as non-negative integers. Binary constraints
//! on x_{t,u} are enforced by x_{t,u} ≤ 1.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::PreemptiveScheduling;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing PreemptiveScheduling to `ILP<i64>`.
///
/// Variable layout:
/// - x_{t,u} at index t * D_max + u for t in 0..n, u in 0..D_max  (n*D_max vars)
/// - M at index n * D_max  (1 integer var)
///
/// Total: n * D_max + 1 variables.
#[derive(Debug, Clone)]
pub struct ReductionPSToILP {
    target: ILP<i64>,
    num_tasks: usize,
    d_max: usize,
}

impl ReductionResult for ReductionPSToILP {
    type Source = PreemptiveScheduling;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    /// Extract schedule from ILP solution.
    ///
    /// Returns a binary config of length n * D_max: `config[t * D_max + u] = x_{t,u}`.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok((0..self.num_tasks)
            .map(|task| {
                (0..self.d_max)
                    .map(|time| target_solution[task * self.d_max + time] == 1)
                    .collect()
            })
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_tasks * d_max + 1",
        num_constraints = "num_tasks + d_max + num_precedences * d_max + 2 * num_tasks * d_max",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for PreemptiveScheduling {
    type Result = ReductionPSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_tasks();
        let d = self.d_max();
        let num_task_vars = n * d;
        let m_var = num_task_vars; // index of the makespan variable M
        let num_vars = num_task_vars + 1;
        let lengths = self.lengths();
        let processor_count =
            Self::exact_i64(self.num_processors(), "encoding the processor capacity")?;

        let x = |t: usize, u: usize| t * d + u;

        let mut constraints = Vec::new();

        // 1. Work constraints: Σ_u x_{t,u} = l(t) for each task t
        for (t, &length) in lengths.iter().enumerate() {
            let terms: Vec<(usize, i64)> = (0..d).map(|u| (x(t, u), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, length));
        }

        // 2. Capacity constraints: Σ_t x_{t,u} ≤ m for each time slot u
        for u in 0..d {
            let terms: Vec<(usize, i64)> = (0..n).map(|t| (x(t, u), 1)).collect();
            constraints.push(LinearConstraint::le(terms, processor_count));
        }

        // 3. Precedence constraints: for each (pred, succ) and each slot u:
        //    l(pred) * x_{succ,u} ≤ Σ_{v=0}^{u-1} x_{pred,v}
        //    i.e. l(pred) * x_{succ,u} - Σ_{v=0}^{u-1} x_{pred,v} ≤ 0
        //
        //    Interpretation: succ can only be active at slot u once pred has
        //    accumulated all l(pred) units of work in strictly earlier slots.
        for &(pred, succ) in self.precedences() {
            let l_pred = lengths[pred];
            for u in 0..d {
                // Σ_{v=0}^{u-1} x_{pred,v} - l(pred)*x_{succ,u} ≥ 0
                // i.e. l(pred)*x_{succ,u} - Σ_{v<u} x_{pred,v} ≤ 0
                let mut terms: Vec<(usize, i64)> = Vec::new();
                // Cumulative pred work up to u-1
                for v in 0..u {
                    terms.push((x(pred, v), -1));
                }
                terms.push((x(succ, u), l_pred));
                constraints.push(LinearConstraint::le(terms, 0));
            }
        }

        // 4. Makespan lower bound: M - (u+1)*x_{t,u} ≥ 0 for all t,u
        for t in 0..n {
            for u in 0..d {
                constraints.push(LinearConstraint::ge(
                    vec![
                        (m_var, 1),
                        (x(t, u), -Self::exact_i64(u + 1, "encoding a time slot")?),
                    ],
                    0,
                ));
            }
        }

        // 5. Binary upper bound: x_{t,u} ≤ 1 for all t,u
        for t in 0..n {
            for u in 0..d {
                constraints.push(LinearConstraint::le(vec![(x(t, u), 1)], 1));
            }
        }

        // Objective: minimize M
        let objective = vec![(m_var, 1.0)];

        Ok(ReductionPSToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_tasks: n,
            d_max: d,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "preemptivescheduling_to_ilp",
        build: || {
            // 3 tasks, lengths [2,1,2], 2 processors, precedence (0,2)
            let source = PreemptiveScheduling::new(vec![2, 1, 2], 2, vec![(0, 2)]).unwrap();
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/preemptivescheduling_ilp.rs"]
mod tests;
