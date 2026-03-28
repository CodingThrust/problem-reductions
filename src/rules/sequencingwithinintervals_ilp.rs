//! Reduction from SequencingWithinIntervals to ILP<bool>.
//!
//! Uses a time-indexed binary formulation:
//! - Variables: Binary x_{j,k} where x_{j,k} = 1 iff task j starts at offset k
//!   from its release time (actual start = r_j + k), k in 0..(d_j - r_j - l_j).
//! - Variable index: task j at offset k has global index: Σ_{i<j} slot_count_i + k,
//!   where slot_count_i = d_i - r_i - l_i + 1 is the number of valid start offsets.
//!   For simplicity we use a flat layout: each task j occupies slot_count[j] variables.
//! - Constraints:
//!   1. One-hot: Σ_k x_{j,k} = 1 for each task j
//!   2. Non-overlap: for each pair (i, j), they cannot be active at the same time.
//!      Active time of task j starting at r_j+k: [r_j+k, r_j+k+l_j).
//!      Non-overlap: no shared time. Modeled with: for each pair (i,j) with i<j,
//!      Σ_{(k1,k2): windows overlap} (x_{i,k1} + x_{j,k2}) ≤ 1.
//! - Objective: Minimize 0 (feasibility)
//! - Extraction: For task j, find offset k where x_{j,k}=1; config[j] = k.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingWithinIntervals;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SequencingWithinIntervals to ILP<bool>.
///
/// Variable layout: task j occupies variables at offsets [base_j, base_j + slot_count_j).
/// where base_j = Σ_{i<j} slot_count_i and slot_count_j = d_j - r_j - l_j + 1.
#[derive(Debug, Clone)]
pub struct ReductionSWIToILP {
    target: ILP<bool>,
    /// For each task: (base variable index, number of start offsets).
    task_layout: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionSWIToILP {
    type Source = SequencingWithinIntervals;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract schedule from ILP solution.
    ///
    /// For each task j, find the offset k where x_{j,k} = 1.
    /// Returns config[j] = k (start time offset from release time).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.task_layout
            .iter()
            .map(|&(base, count)| {
                (0..count)
                    .find(|&k| target_solution.get(base + k).copied().unwrap_or(0) == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_tasks^2",
        num_constraints = "num_tasks^2 + num_tasks",
    }
)]
impl ReduceTo<ILP<bool>> for SequencingWithinIntervals {
    type Result = ReductionSWIToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let release = self.release_times();
        let deadlines = self.deadlines();
        let lengths = self.lengths();

        // Compute per-task variable layout: how many start slots each task has
        let slot_counts: Vec<usize> = (0..n)
            .map(|j| (deadlines[j] - release[j] - lengths[j] + 1) as usize)
            .collect();

        let mut bases = vec![0usize; n];
        for j in 1..n {
            bases[j] = bases[j - 1] + slot_counts[j - 1];
        }
        let num_vars =
            bases.last().copied().unwrap_or(0) + slot_counts.last().copied().unwrap_or(0);

        let task_layout: Vec<(usize, usize)> = (0..n).map(|j| (bases[j], slot_counts[j])).collect();

        let mut constraints = Vec::new();

        // 1. One-hot per task
        for j in 0..n {
            let terms: Vec<(usize, f64)> =
                (0..slot_counts[j]).map(|k| (bases[j] + k, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2. Non-overlap for each pair (i, j) with i < j
        // For each (k1, k2) where task i at offset k1 overlaps task j at offset k2:
        // x_{i,k1} + x_{j,k2} <= 1
        for i in 0..n {
            for j in (i + 1)..n {
                for k1 in 0..slot_counts[i] {
                    let start_i = release[i] + k1 as u64;
                    let end_i = start_i + lengths[i];
                    for k2 in 0..slot_counts[j] {
                        let start_j = release[j] + k2 as u64;
                        let end_j = start_j + lengths[j];
                        // Overlap if neither ends before the other starts
                        if !(end_i <= start_j || end_j <= start_i) {
                            constraints.push(LinearConstraint::le(
                                vec![(bases[i] + k1, 1.0), (bases[j] + k2, 1.0)],
                                1.0,
                            ));
                        }
                    }
                }
            }
        }

        ReductionSWIToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            task_layout,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingwithinintervals_to_ilp",
        build: || {
            // 2 tasks: task 0 [r=0, d=3, l=2], task 1 [r=2, d=5, l=2]
            // Task 0 can start at offset 0 or 1, task 1 can start at offset 0 or 1
            // No overlap when both at offset 0: [0,2) and [2,4)
            let source = SequencingWithinIntervals::new(vec![0, 2], vec![3, 5], vec![2, 2]);
            let reduction: ReductionSWIToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let solver = crate::solvers::ILPSolver::new();
            let target_config = solver
                .solve(reduction.target_problem())
                .expect("canonical example should be feasible");
            let source_config = reduction.extract_solution(&target_config);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
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
#[path = "../unit_tests/rules/sequencingwithinintervals_ilp.rs"]
mod tests;
