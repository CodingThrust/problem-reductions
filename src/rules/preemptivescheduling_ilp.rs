//! Time-indexed preemptive scheduling with bounded start and completion variables.
//!
//! Binary x(t,u) records task activity. Start S(t) is no later than any active
//! slot; completion C(t) is later than every active slot. Each precedence
//! (a,b) requires C(a) <= S(b), so interrupted tasks still finish before their
//! successors start. Minimize M with M >= C(t) for every task.
//!
//! Using task endpoints avoids repeating all earlier time slots for every
//! precedence edge: the matrix has 6*A + 2*p + 2*n nonzeros for A admissible
//! activity slots, p precedence edges, and n tasks.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::PreemptiveScheduling;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing PreemptiveScheduling to `ILP<i64, i64, Bounded>`.
#[derive(Debug, Clone)]
pub struct ReductionPSToILP {
    target: ILP<i64, i64, Bounded>,
    num_tasks: usize,
    d_max: usize,
    slots: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionPSToILP {
    type Source = PreemptiveScheduling;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &ILP<i64, i64, Bounded> {
        &self.target
    }

    /// Extract schedule from ILP solution.
    ///
    /// Returns the task-by-time activity matrix, restoring omitted slots to false.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;

        let mut schedule = vec![vec![false; self.d_max]; self.num_tasks];
        for (variable, &(task, time)) in self.slots.iter().enumerate() {
            schedule[task][time] = target_solution[variable] == 1;
        }
        Ok(schedule)
    }
}

#[reduction(transform = {
    exact {
        num_vars = "num_admissible_slots + 2 * num_tasks + 1",
        num_constraints = "2 * num_tasks + schedule_horizon + 2 * num_admissible_slots + num_precedences",
        num_nonzeros = "6 * num_admissible_slots + 2 * num_precedences + 2 * num_tasks",
    },
    upper_bound {
        max_constraint_magnitude_bits = "max_schedule_magnitude_bits",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for PreemptiveScheduling {
    type Result = ReductionPSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_tasks();
        let (d, windows) = self.scheduling_windows();
        let num_task_vars: usize = windows.iter().map(|window| window.len()).sum();
        let m_var = num_task_vars; // index of the makespan variable M
        let num_vars = n
            .checked_mul(2)
            .and_then(|endpoints| num_task_vars.checked_add(endpoints))
            .and_then(|total| total.checked_add(1))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                    "counting scheduling variables",
                )
            })?;
        let horizon = Self::exact_i64(d, "bounding the scheduling horizon")?;
        let lengths = self.lengths();
        let processor_count = Self::exact_i64(
            self.num_processors().min(n),
            "encoding the processor capacity",
        )?;

        // Check endpoint arithmetic before allocating the time-indexed matrix.
        horizon.checked_mul(2).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                "bounding endpoint constraint evaluation",
            )
        })?;
        let slots: Vec<_> = windows
            .into_iter()
            .enumerate()
            .flat_map(|(task, window)| window.map(move |time| (task, time)))
            .collect();
        let mut task_terms = vec![Vec::new(); n];
        let mut time_terms = vec![Vec::new(); d];
        for (variable, &(task, time)) in slots.iter().enumerate() {
            task_terms[task].push((variable, 1));
            time_terms[time].push((variable, 1));
        }
        let start = |t: usize| num_task_vars + 1 + t;
        let completion = |t: usize| num_task_vars + 1 + n + t;

        let mut constraints = Vec::new();

        for (terms, &length) in task_terms.into_iter().zip(lengths) {
            constraints.push(LinearConstraint::eq(terms, length));
        }
        for terms in time_terms {
            constraints.push(LinearConstraint::le(terms, processor_count));
        }
        for (variable, &(t, u)) in slots.iter().enumerate() {
            constraints.push(LinearConstraint::le(
                vec![
                    (start(t), 1),
                    (variable, Self::exact_i64(d - u, "encoding a start bound")?),
                ],
                horizon,
            ));
            constraints.push(LinearConstraint::ge(
                vec![
                    (completion(t), 1),
                    (
                        variable,
                        -Self::exact_i64(u + 1, "encoding a completion bound")?,
                    ),
                ],
                0,
            ));
        }
        for &(pred, succ) in self.precedences() {
            constraints.push(LinearConstraint::le(
                vec![(completion(pred), 1), (start(succ), -1)],
                0,
            ));
        }
        for t in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(completion(t), 1), (m_var, -1)],
                0,
            ));
        }

        // Objective: minimize M
        let objective = vec![(m_var, 1)];

        // Slot domains enforce binary values without redundant bound rows.
        let mut variables = vec![IntegerVariable::binary(); num_vars];
        variables[num_task_vars..]
            .fill(IntegerVariable::new(Some(0), Some(horizon)).map_err(Self::target_construction)?);

        let target =
            ILP::with_variables(variables, constraints, objective, ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(ReductionPSToILP {
            target,
            num_tasks: n,
            d_max: self.d_max(),
            slots,
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
            crate::example_db::specs::rule_example_via_bounded_ilp::<_>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/preemptivescheduling_ilp.rs"]
mod tests;
