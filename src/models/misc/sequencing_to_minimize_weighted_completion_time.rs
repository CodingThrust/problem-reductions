//! Sequencing to Minimize Weighted Completion Time problem implementation.
//!
//! A classical NP-hard single-machine scheduling problem (SS4 from
//! Garey & Johnson, 1979) where tasks with processing times, weights,
//! and precedence constraints must be scheduled to minimize the total
//! weighted completion time.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingToMinimizeWeightedCompletionTime",
        display_name: "Sequencing to Minimize Weighted Completion Time",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule tasks with lengths, weights, and precedence constraints to minimize total weighted completion time",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing time l(t) for each task" },
            FieldInfo { name: "weights", type_name: "Vec<u64>", description: "Weight w(t) for each task" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (predecessor, successor)" },
        ],
    }
}

/// Sequencing to Minimize Weighted Completion Time problem.
///
/// Given tasks with processing times `l(t)`, weights `w(t)`, and precedence
/// constraints, find a single-machine schedule that respects the precedences
/// and minimizes `sum_t w(t) * C(t)`, where `C(t)` is the completion time of
/// task `t`.
///
/// Configurations use Lehmer code with `dims() = [n, n-1, ..., 1]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencingToMinimizeWeightedCompletionTime {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    precedences: Vec<(usize, usize)>,
}

impl SequencingToMinimizeWeightedCompletionTime {
    /// Create a new sequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `lengths.len() != weights.len()` or if any precedence endpoint
    /// is out of range.
    pub fn new(lengths: Vec<u64>, weights: Vec<u64>, precedences: Vec<(usize, usize)>) -> Self {
        assert_eq!(
            lengths.len(),
            weights.len(),
            "lengths length must equal weights length"
        );

        let num_tasks = lengths.len();
        for &(pred, succ) in &precedences {
            assert!(
                pred < num_tasks,
                "predecessor index {} out of range (num_tasks = {})",
                pred,
                num_tasks
            );
            assert!(
                succ < num_tasks,
                "successor index {} out of range (num_tasks = {})",
                succ,
                num_tasks
            );
        }

        Self {
            lengths,
            weights,
            precedences,
        }
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Returns the processing times.
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the task weights.
    pub fn weights(&self) -> &[u64] {
        &self.weights
    }

    /// Returns the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Returns the number of precedence constraints.
    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }

    /// Returns the sum of all processing times.
    pub fn total_processing_time(&self) -> u64 {
        self.lengths.iter().sum()
    }

    fn decode_schedule(&self, config: &[usize]) -> Option<Vec<usize>> {
        let n = self.num_tasks();
        if config.len() != n {
            return None;
        }

        let mut available: Vec<usize> = (0..n).collect();
        let mut schedule = Vec::with_capacity(n);
        for &digit in config {
            if digit >= available.len() {
                return None;
            }
            schedule.push(available.remove(digit));
        }
        Some(schedule)
    }

    fn weighted_completion_time(&self, schedule: &[usize]) -> SolutionSize<u64> {
        let n = self.num_tasks();
        let mut positions = vec![0usize; n];
        let mut completion_times = vec![0u64; n];
        let mut elapsed = 0u64;

        for (position, &task) in schedule.iter().enumerate() {
            positions[task] = position;
            elapsed += self.lengths[task];
            completion_times[task] = elapsed;
        }

        for &(pred, succ) in &self.precedences {
            if positions[pred] >= positions[succ] {
                return SolutionSize::Invalid;
            }
        }

        let total = completion_times
            .iter()
            .enumerate()
            .map(|(task, &completion)| completion * self.weights[task])
            .sum();
        SolutionSize::Valid(total)
    }
}

impl Problem for SequencingToMinimizeWeightedCompletionTime {
    const NAME: &'static str = "SequencingToMinimizeWeightedCompletionTime";
    type Metric = SolutionSize<u64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_tasks();
        (0..n).rev().map(|i| i + 1).collect()
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<u64> {
        let Some(schedule) = self.decode_schedule(config) else {
            return SolutionSize::Invalid;
        };
        self.weighted_completion_time(&schedule)
    }
}

impl OptimizationProblem for SequencingToMinimizeWeightedCompletionTime {
    type Value = u64;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default opt SequencingToMinimizeWeightedCompletionTime => "factorial(num_tasks)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_to_minimize_weighted_completion_time",
        build: || {
            let problem = SequencingToMinimizeWeightedCompletionTime::new(
                vec![2, 1, 3, 1, 2],
                vec![3, 5, 1, 4, 2],
                vec![(0, 2), (1, 4)],
            );
            crate::example_db::specs::optimization_example(problem, vec![vec![1, 2, 0, 1, 0]])
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_to_minimize_weighted_completion_time.rs"]
mod tests;
