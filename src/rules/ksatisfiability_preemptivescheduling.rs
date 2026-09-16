//! Reduction from KSatisfiability (3-SAT) to PreemptiveScheduling.
//!
//! This follows Ullman's 1975 construction via a unit-task precedence
//! scheduling instance. Since every task has length 1, preemption is inert:
//! the constructed instance is a valid preemptive scheduling problem whose
//! optimal makespan hits the threshold `T = num_vars + 3` iff a nontrivial
//! 3-SAT instance is satisfiable. Empty conjunctions and empty clauses use
//! constant instances; shorter nonempty clauses repeat literals.
//!
//! Reference: Jeffrey D. Ullman, "NP-complete scheduling problems", JCSS 10,
//! 1975; Garey & Johnson, Appendix A5.2.

use crate::models::formula::KSatisfiability;
use crate::models::misc::PreemptiveScheduling;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;
#[cfg(any(test, feature = "example-db"))]
use crate::traits::Problem;
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
    literal: i64,
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
    let capacities = slot_capacities(num_vars, num_clauses);
    // Ullman's clock requires a nonempty filler layer in EVERY slot.
    // A chain through all T layers forces layer i into slot i in any
    // T-slot schedule, leaving exactly capacities[i] slots for original jobs.
    let num_processors = capacities.iter().max().unwrap() + 1;

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
            // Repeating literals preserves disjunction for short clauses.
            for (position, &literal) in clause.literals.iter().cycle().take(3).enumerate() {
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

#[cfg(any(test, feature = "example-db"))]
fn set_task_slot(task_slots: &mut [Option<usize>], job: usize, slot: usize) {
    task_slots[job] = Some(slot);
}

#[cfg(any(test, feature = "example-db"))]
fn clause_pattern_for_assignment(
    clause: &crate::models::formula::CNFClause,
    assignment: &[bool],
) -> usize {
    let mut pattern = 0usize;
    for (position, &literal) in clause.literals.iter().cycle().take(3).enumerate() {
        let variable = literal.unsigned_abs() as usize - 1;
        let value = assignment[variable];
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
    assignment: &[bool],
    source: &KSatisfiability<K3>,
) -> Option<Vec<Vec<bool>>> {
    if source.num_clauses() == 0 || source.clauses().iter().any(|c| c.literals.is_empty()) {
        return source
            .evaluate(&assignment.to_vec())
            .unwrap()
            .0
            .then(|| vec![vec![true]]);
    }
    let construction = build_ullman_construction(source);
    if assignment.len() != source.num_vars() || target.num_tasks() != construction.num_jobs {
        return None;
    }

    let mut task_slots = vec![None; construction.num_jobs];

    for variable in 0..source.num_vars() {
        let value_is_true = assignment[variable];
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
    let mut config = vec![vec![false; d_max]; construction.num_jobs];
    for (job, slot) in task_slots.into_iter().enumerate() {
        let slot = slot?;
        config[job][slot] = true;
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

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        match target {
            SolveOutcome::Infeasible => Ok(SolveOutcome::Infeasible),
            SolveOutcome::Optimal {
                solution,
                evaluation,
            } => {
                if evaluation.0.expect("valid schedule has a makespan") > self.threshold as i64 {
                    return Ok(SolveOutcome::Infeasible);
                }
                // A threshold schedule must decode to a satisfying assignment.
                // A mapping failure is an error, never evidence of infeasibility.
                Ok(SolveOutcome::optimal(source, self.map_solution(&solution))?)
            }
            SolveOutcome::Feasible {
                solution,
                evaluation,
            } => {
                if evaluation.0.expect("valid schedule has a makespan") > self.threshold as i64 {
                    return Err(crate::rules::ExtractionError::InsufficientSolutionQuality);
                }
                Ok(SolveOutcome::feasible(
                    source,
                    self.map_solution(&solution),
                )?)
            }
        }
    }
}

impl Reduction3SATToPreemptiveScheduling {
    fn map_solution(&self, target_solution: &[Vec<bool>]) -> Vec<bool> {
        self.positive_start_jobs
            .iter()
            .map(|&job| target_solution[job][0])
            .collect()
    }
}

#[reduction(
    transform = upper_bound {
        num_tasks = "(2 * num_vars + 3 + 6 * num_clauses) * (num_vars + 3)",
        num_processors = "2 * num_vars + 3 + 6 * num_clauses",
        d_max = "(2 * num_vars + 3 + 6 * num_clauses) * (num_vars + 3)",
    },
    unavailable = {
        num_precedences = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<PreemptiveScheduling> for KSatisfiability<K3> {
    type Result = Reduction3SATToPreemptiveScheduling;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let has_empty_clause = self
            .clauses()
            .iter()
            .any(|clause| clause.literals.is_empty());
        if self.num_clauses() == 0 || has_empty_clause {
            // The empty conjunction is true; any empty disjunction is false.
            return Ok(Reduction3SATToPreemptiveScheduling {
                target: PreemptiveScheduling::new(vec![1], 1, vec![])
                    .map_err(<Self as ReduceTo<PreemptiveScheduling>>::target_construction)?,
                positive_start_jobs: vec![0; self.num_vars()],
                threshold: usize::from(!has_empty_clause),
            });
        }
        let construction = build_ullman_construction(self);
        let target = PreemptiveScheduling::new(
            vec![1_i64; construction.num_jobs],
            construction.num_processors,
            construction.precedences.clone(),
        )
        .map_err(
            crate::rules::ReductionError::construction::<KSatisfiability<K3>, PreemptiveScheduling>,
        )?;

        Ok(Reduction3SATToPreemptiveScheduling {
            target,
            positive_start_jobs: construction
                .positive_chains
                .iter()
                .map(|chain| chain[0])
                .collect(),
            threshold: construction.time_limit,
        })
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
            let reduction = ReduceTo::<PreemptiveScheduling>::reduce_to(&source)
                .expect("reduction should succeed");
            let source_config = vec![false, false, true];
            let target_config = construct_schedule_from_assignment(
                reduction.target_problem(),
                &source_config,
                &source,
            )
            .expect("canonical example assignment should yield a schedule");
            crate::example_db::specs::rule_example_with_witness::<_, PreemptiveScheduling>(
                source,
                SolutionPair {
                    source_config: serde_json::to_value(source_config)
                        .expect("solution serialization must succeed"),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_preemptivescheduling.rs"]
mod tests;
