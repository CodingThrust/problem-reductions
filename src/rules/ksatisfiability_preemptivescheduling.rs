//! Reduction from KSatisfiability (3-SAT) to PreemptiveScheduling.
//!
//! This follows Ullman's 1975 construction via a unit-task precedence
//! scheduling instance. Since every task has length 1, preemption is inert:
//! the constructed instance is a valid preemptive scheduling problem whose
//! optimal makespan hits the threshold `T = num_vars + 3` iff the 3-SAT
//! instance is satisfiable.
//!
//! Reference: Jeffrey D. Ullman, "NP-complete scheduling problems", JCSS 10,
//! 1975; Garey & Johnson, Appendix A5.2.

use crate::models::formula::KSatisfiability;
use crate::models::misc::PreemptiveScheduling;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;

#[derive(Debug, Clone)]
struct UllmanConstruction {
    time_limit: usize,
    num_processors: usize,
    positive_chains: Vec<Vec<usize>>,
    #[cfg(any(test, feature = "example-db"))]
    negative_chains: Vec<Vec<usize>>,
    #[cfg(any(test, feature = "example-db"))]
    positive_forcing: Vec<usize>,
    #[cfg(any(test, feature = "example-db"))]
    negative_forcing: Vec<usize>,
    #[cfg(any(test, feature = "example-db"))]
    clause_jobs: Vec<Vec<usize>>,
    #[cfg(any(test, feature = "example-db"))]
    filler_jobs_by_slot: Vec<Vec<usize>>,
    precedences: Vec<(usize, usize)>,
    num_jobs: usize,
}

fn time_limit(num_vars: usize) -> usize {
    num_vars + 3
}

fn processor_upper_bound(num_vars: usize, num_clauses: usize) -> usize {
    (2 * num_vars + 2).max(6 * num_clauses)
}

fn slot_capacities(num_vars: usize, num_clauses: usize) -> Vec<usize> {
    let mut capacities = vec![0; time_limit(num_vars)];
    capacities[0] = num_vars;
    capacities[1] = 2 * num_vars + 1;
    for capacity in capacities.iter_mut().take(num_vars + 1).skip(2) {
        *capacity = 2 * num_vars + 2;
    }
    capacities[num_vars + 1] = num_clauses + num_vars + 1;
    capacities[num_vars + 2] = 6 * num_clauses;
    capacities
}

fn literal_endpoint(
    literal: i32,
    pick_literal: bool,
    positive_chains: &[Vec<usize>],
    negative_chains: &[Vec<usize>],
    endpoint_step: usize,
) -> usize {
    let variable = literal.unsigned_abs() as usize - 1;
    match (literal > 0, pick_literal) {
        (true, true) | (false, false) => positive_chains[variable][endpoint_step],
        (true, false) | (false, true) => negative_chains[variable][endpoint_step],
    }
}

fn build_ullman_construction(source: &KSatisfiability<K3>) -> UllmanConstruction {
    let num_vars = source.num_vars();
    let num_clauses = source.num_clauses();
    let time_limit = time_limit(num_vars);
    let num_processors = processor_upper_bound(num_vars, num_clauses);
    let capacities = slot_capacities(num_vars, num_clauses);

    let mut next_job = 0usize;

    let mut positive_chains = vec![vec![0; num_vars + 1]; num_vars];
    let mut negative_chains = vec![vec![0; num_vars + 1]; num_vars];
    for variable in 0..num_vars {
        for step in 0..=num_vars {
            positive_chains[variable][step] = next_job;
            next_job += 1;
            negative_chains[variable][step] = next_job;
            next_job += 1;
        }
    }

    let positive_forcing: Vec<usize> = (0..num_vars)
        .map(|_| {
            let job = next_job;
            next_job += 1;
            job
        })
        .collect();
    let negative_forcing: Vec<usize> = (0..num_vars)
        .map(|_| {
            let job = next_job;
            next_job += 1;
            job
        })
        .collect();

    let mut clause_jobs = vec![vec![0; 7]; num_clauses];
    for jobs in &mut clause_jobs {
        for job in jobs {
            *job = next_job;
            next_job += 1;
        }
    }

    let mut filler_jobs_by_slot = vec![Vec::new(); time_limit];
    for slot in 0..time_limit {
        let filler_count = num_processors - capacities[slot];
        filler_jobs_by_slot[slot] = (0..filler_count)
            .map(|_| {
                let job = next_job;
                next_job += 1;
                job
            })
            .collect();
    }

    let mut precedences = Vec::new();

    for variable in 0..num_vars {
        for step in 0..num_vars {
            precedences.push((
                positive_chains[variable][step],
                positive_chains[variable][step + 1],
            ));
            precedences.push((
                negative_chains[variable][step],
                negative_chains[variable][step + 1],
            ));
        }
        precedences.push((
            positive_chains[variable][variable],
            positive_forcing[variable],
        ));
        precedences.push((
            negative_chains[variable][variable],
            negative_forcing[variable],
        ));
    }

    for (clause_index, clause) in source.clauses().iter().enumerate() {
        for (pattern_index, &clause_job) in clause_jobs[clause_index].iter().enumerate() {
            let pattern = pattern_index + 1;
            for position in 0..3 {
                let literal = clause.literals[position];
                let bit_is_one = ((pattern >> (2 - position)) & 1) == 1;
                precedences.push((
                    literal_endpoint(
                        literal,
                        bit_is_one,
                        &positive_chains,
                        &negative_chains,
                        num_vars,
                    ),
                    clause_job,
                ));
            }
        }
    }

    for slot in 0..time_limit.saturating_sub(1) {
        for &pred in &filler_jobs_by_slot[slot] {
            for &succ in &filler_jobs_by_slot[slot + 1] {
                precedences.push((pred, succ));
            }
        }
    }

    UllmanConstruction {
        time_limit,
        num_processors,
        positive_chains,
        #[cfg(any(test, feature = "example-db"))]
        negative_chains,
        #[cfg(any(test, feature = "example-db"))]
        positive_forcing,
        #[cfg(any(test, feature = "example-db"))]
        negative_forcing,
        #[cfg(any(test, feature = "example-db"))]
        clause_jobs,
        #[cfg(any(test, feature = "example-db"))]
        filler_jobs_by_slot,
        precedences,
        num_jobs: next_job,
    }
}

fn task_slot(config: &[usize], task: usize, d_max: usize) -> Option<usize> {
    let start = task.checked_mul(d_max)?;
    let end = start.checked_add(d_max)?;
    let task_slice = config.get(start..end)?;
    task_slice.iter().position(|&value| value == 1)
}

#[cfg(any(test, feature = "example-db"))]
fn set_task_slot(task_slots: &mut [Option<usize>], job: usize, slot: usize) {
    task_slots[job] = Some(slot);
}

#[cfg(any(test, feature = "example-db"))]
fn clause_pattern_for_assignment(
    clause: &crate::models::formula::CNFClause,
    assignment: &[usize],
) -> usize {
    let mut pattern = 0usize;
    for (position, &literal) in clause.literals.iter().enumerate() {
        let variable = literal.unsigned_abs() as usize - 1;
        let value = assignment.get(variable).copied().unwrap_or(0) == 1;
        let literal_true = if literal > 0 { value } else { !value };
        if literal_true {
            pattern |= 1 << (2 - position);
        }
    }
    pattern
}

#[cfg(any(test, feature = "example-db"))]
fn construct_schedule_from_assignment(
    target: &PreemptiveScheduling,
    assignment: &[usize],
    source: &KSatisfiability<K3>,
) -> Option<Vec<usize>> {
    let construction = build_ullman_construction(source);
    if assignment.len() != source.num_vars() || target.num_tasks() != construction.num_jobs {
        return None;
    }

    let mut task_slots = vec![None; construction.num_jobs];

    for variable in 0..source.num_vars() {
        let value_is_true = assignment[variable] == 1;
        for step in 0..=source.num_vars() {
            if value_is_true {
                set_task_slot(
                    &mut task_slots,
                    construction.positive_chains[variable][step],
                    step,
                );
                set_task_slot(
                    &mut task_slots,
                    construction.negative_chains[variable][step],
                    step + 1,
                );
            } else {
                set_task_slot(
                    &mut task_slots,
                    construction.negative_chains[variable][step],
                    step,
                );
                set_task_slot(
                    &mut task_slots,
                    construction.positive_chains[variable][step],
                    step + 1,
                );
            }
        }

        let positive_slot = task_slots[construction.positive_chains[variable][variable]]
            .expect("positive chain slot is assigned");
        let negative_slot = task_slots[construction.negative_chains[variable][variable]]
            .expect("negative chain slot is assigned");
        set_task_slot(
            &mut task_slots,
            construction.positive_forcing[variable],
            positive_slot + 1,
        );
        set_task_slot(
            &mut task_slots,
            construction.negative_forcing[variable],
            negative_slot + 1,
        );
    }

    for (clause_index, clause) in source.clauses().iter().enumerate() {
        let pattern = clause_pattern_for_assignment(clause, assignment);
        if pattern == 0 {
            return None;
        }
        for pattern_index in 0..7 {
            let slot = if pattern_index + 1 == pattern {
                source.num_vars() + 1
            } else {
                source.num_vars() + 2
            };
            set_task_slot(
                &mut task_slots,
                construction.clause_jobs[clause_index][pattern_index],
                slot,
            );
        }
    }

    for (slot, fillers) in construction.filler_jobs_by_slot.iter().enumerate() {
        for &job in fillers {
            set_task_slot(&mut task_slots, job, slot);
        }
    }

    let d_max = target.d_max();
    let mut config = vec![0usize; construction.num_jobs * d_max];
    for (job, slot) in task_slots.into_iter().enumerate() {
        let slot = slot?;
        config[job * d_max + slot] = 1;
    }
    Some(config)
}

/// Result of reducing KSatisfiability<K3> to PreemptiveScheduling.
#[derive(Debug, Clone)]
pub struct Reduction3SATToPreemptiveScheduling {
    target: PreemptiveScheduling,
    positive_start_jobs: Vec<usize>,
    threshold: usize,
}

impl Reduction3SATToPreemptiveScheduling {
    pub fn threshold(&self) -> usize {
        self.threshold
    }
}

impl ReductionResult for Reduction3SATToPreemptiveScheduling {
    type Source = KSatisfiability<K3>;
    type Target = PreemptiveScheduling;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let d_max = self.target.d_max();
        self.positive_start_jobs
            .iter()
            .map(|&job| usize::from(task_slot(target_solution, job, d_max) == Some(0)))
            .collect()
    }
}

#[reduction(
    overhead = {
        num_tasks = "(((2 * num_vars + 2) + 6 * num_clauses + sqrt(((2 * num_vars + 2) - 6 * num_clauses)^2)) / 2) * (num_vars + 3)",
        num_processors = "((2 * num_vars + 2) + 6 * num_clauses + sqrt(((2 * num_vars + 2) - 6 * num_clauses)^2)) / 2",
        d_max = "(((2 * num_vars + 2) + 6 * num_clauses + sqrt(((2 * num_vars + 2) - 6 * num_clauses)^2)) / 2) * (num_vars + 3)",
    }
)]
impl ReduceTo<PreemptiveScheduling> for KSatisfiability<K3> {
    type Result = Reduction3SATToPreemptiveScheduling;

    fn reduce_to(&self) -> Self::Result {
        let construction = build_ullman_construction(self);
        let target = PreemptiveScheduling::new(
            vec![1; construction.num_jobs],
            construction.num_processors,
            construction.precedences.clone(),
        );

        Reduction3SATToPreemptiveScheduling {
            target,
            positive_start_jobs: construction
                .positive_chains
                .iter()
                .map(|chain| chain[0])
                .collect(),
            threshold: construction.time_limit,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_preemptivescheduling",
        build: || {
            let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
            let reduction = ReduceTo::<PreemptiveScheduling>::reduce_to(&source);
            let source_config = vec![0, 0, 1];
            let target_config = construct_schedule_from_assignment(
                reduction.target_problem(),
                &source_config,
                &source,
            )
            .expect("canonical example assignment should yield a schedule");
            crate::example_db::specs::rule_example_with_witness::<_, PreemptiveScheduling>(
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
#[path = "../unit_tests/rules/ksatisfiability_preemptivescheduling.rs"]
mod tests;
