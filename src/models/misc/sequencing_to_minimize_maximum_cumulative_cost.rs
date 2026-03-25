//! Sequencing to Minimize Maximum Cumulative Cost problem implementation.
//!
//! Given a set of tasks with integer costs and precedence constraints, find
//! a valid one-machine schedule that minimizes the maximum cumulative cost
//! over all prefixes.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingToMinimizeMaximumCumulativeCost",
        display_name: "Sequencing to Minimize Maximum Cumulative Cost",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule tasks with precedence constraints to minimize the maximum cumulative cost prefix",
        fields: &[
            FieldInfo { name: "costs", type_name: "Vec<i64>", description: "Task costs in schedule order-independent indexing" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (predecessor, successor)" },
        ],
    }
}

/// Sequencing to Minimize Maximum Cumulative Cost.
///
/// Given a set of tasks `T`, a cost `c(t) in Z` for each task, and a partial
/// order on the tasks, find a schedule that respects the precedences and
/// minimizes the maximum cumulative cost over all prefixes.
///
/// # Representation
///
/// Configurations use Lehmer-code dimensions `[n, n-1, ..., 1]` to encode a
/// permutation of the task indices.
#[derive(Debug, Clone, Serialize)]
pub struct SequencingToMinimizeMaximumCumulativeCost {
    costs: Vec<i64>,
    precedences: Vec<(usize, usize)>,
}

#[derive(Debug, Deserialize)]
struct SequencingToMinimizeMaximumCumulativeCostUnchecked {
    costs: Vec<i64>,
    precedences: Vec<(usize, usize)>,
}

impl SequencingToMinimizeMaximumCumulativeCost {
    /// Create a new instance.
    ///
    /// # Panics
    ///
    /// Panics if any precedence endpoint is out of range.
    pub fn new(costs: Vec<i64>, precedences: Vec<(usize, usize)>) -> Self {
        validate_precedences(&precedences, costs.len());
        Self { costs, precedences }
    }

    /// Return the task costs.
    pub fn costs(&self) -> &[i64] {
        &self.costs
    }

    /// Return the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Return the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.costs.len()
    }

    /// Return the number of precedence constraints.
    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
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
}

impl<'de> Deserialize<'de> for SequencingToMinimizeMaximumCumulativeCost {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let unchecked =
            SequencingToMinimizeMaximumCumulativeCostUnchecked::deserialize(deserializer)?;
        if let Some(message) =
            precedence_validation_error(&unchecked.precedences, unchecked.costs.len())
        {
            return Err(D::Error::custom(message));
        }
        Ok(Self {
            costs: unchecked.costs,
            precedences: unchecked.precedences,
        })
    }
}

fn validate_precedences(precedences: &[(usize, usize)], num_tasks: usize) {
    if let Some(message) = precedence_validation_error(precedences, num_tasks) {
        panic!("{message}");
    }
}

fn precedence_validation_error(precedences: &[(usize, usize)], num_tasks: usize) -> Option<String> {
    for &(pred, succ) in precedences {
        if pred >= num_tasks {
            return Some(format!(
                "predecessor index {} out of range (num_tasks = {})",
                pred, num_tasks
            ));
        }
        if succ >= num_tasks {
            return Some(format!(
                "successor index {} out of range (num_tasks = {})",
                succ, num_tasks
            ));
        }
    }
    None
}

impl Problem for SequencingToMinimizeMaximumCumulativeCost {
    const NAME: &'static str = "SequencingToMinimizeMaximumCumulativeCost";
    type Value = crate::types::Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_tasks();
        (0..n).rev().map(|i| i + 1).collect()
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Min<i64> {
        let Some(schedule) = self.decode_schedule(config) else {
            return crate::types::Min(None);
        };

        let mut positions = vec![0usize; self.num_tasks()];
        for (position, &task) in schedule.iter().enumerate() {
            positions[task] = position;
        }
        for &(pred, succ) in &self.precedences {
            if positions[pred] >= positions[succ] {
                return crate::types::Min(None);
            }
        }

        let mut cumulative = 0i64;
        let mut max_cumulative = 0i64;
        for &task in &schedule {
            cumulative += self.costs[task];
            if cumulative > max_cumulative {
                max_cumulative = cumulative;
            }
        }
        crate::types::Min(Some(max_cumulative))
    }
}

crate::declare_variants! {
    default SequencingToMinimizeMaximumCumulativeCost => "factorial(num_tasks)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_to_minimize_maximum_cumulative_cost",
        instance: Box::new(SequencingToMinimizeMaximumCumulativeCost::new(
            vec![2, -1, 3, -2, 1, -3],
            vec![(0, 2), (1, 2), (1, 3), (2, 4), (3, 5), (4, 5)],
        )),
        optimal_config: vec![1, 0, 1, 0, 0, 0],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs"]
mod tests;
