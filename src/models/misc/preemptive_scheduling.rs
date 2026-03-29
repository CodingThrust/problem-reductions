//! Preemptive Scheduling problem implementation.
//!
//! A classical NP-hard scheduling problem (Garey & Johnson A5 SS6) where
//! variable-length tasks may be split across non-contiguous time slots on
//! `m` identical processors, subject to precedence constraints.
//! The goal is to minimize the makespan (latest completion time).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PreemptiveScheduling",
        display_name: "Preemptive Scheduling",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Minimize makespan for preemptive parallel-processor scheduling with precedence constraints",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<usize>", description: "Processing length l(t) for each task" },
            FieldInfo { name: "num_processors", type_name: "usize", description: "Number of identical processors m" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (pred, succ) — pred must finish before succ starts" },
        ],
    }
}

/// The Preemptive Scheduling problem.
///
/// Given `n` tasks with processing lengths `l(0), ..., l(n-1)`, `m` identical
/// processors, and a set of precedence constraints, find a preemptive schedule
/// that minimizes the makespan.
///
/// Tasks may be interrupted and resumed at later time slots (preemption).
/// A configuration is a binary vector of length `n × D_max` where
/// `D_max = sum of all lengths` is the worst-case makespan.
///
/// `config[t * D_max + u] = 1` means task `t` is processed at time slot `u`.
///
/// A valid schedule satisfies:
/// - Each task `t` is active in exactly `l(t)` time slots.
/// - At most `m` tasks are active at any time slot.
/// - For each precedence `(pred, succ)`, the last active slot of `pred` is
///   strictly less than the first active slot of `succ`.
///
/// The makespan is `max_t (last active slot of t + 1)`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::PreemptiveScheduling;
/// use problemreductions::Problem;
///
/// let problem = PreemptiveScheduling::new(vec![2, 1], 2, vec![]);
/// // D_max = 3, config length = 2 * 3 = 6
/// // task 0 active at slots 0,1; task 1 active at slot 0
/// let config = vec![1, 1, 0, 1, 0, 0];
/// assert_eq!(problem.evaluate(&config), problemreductions::types::Min(Some(2)));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PreemptiveScheduling {
    /// Processing length for each task.
    lengths: Vec<usize>,
    /// Number of identical processors.
    num_processors: usize,
    /// Precedence constraints: (pred, succ) means pred must finish before succ starts.
    precedences: Vec<(usize, usize)>,
}

#[derive(Deserialize)]
struct PreemptiveSchedulingSerde {
    lengths: Vec<usize>,
    num_processors: usize,
    precedences: Vec<(usize, usize)>,
}

impl PreemptiveScheduling {
    fn validate(
        lengths: &[usize],
        num_processors: usize,
        precedences: &[(usize, usize)],
    ) -> Result<(), String> {
        if lengths.contains(&0) {
            return Err("task lengths must be positive".to_string());
        }
        if num_processors == 0 {
            return Err("num_processors must be positive".to_string());
        }
        let n = lengths.len();
        for &(pred, succ) in precedences {
            if pred >= n || succ >= n {
                return Err(format!(
                    "precedence index out of range: ({pred}, {succ}) but num_tasks = {n}"
                ));
            }
        }
        Ok(())
    }

    /// Create a new Preemptive Scheduling instance.
    ///
    /// # Arguments
    /// * `lengths` - Processing length `l(t)` for each task (must be positive)
    /// * `num_processors` - Number of identical processors `m` (must be positive)
    /// * `precedences` - Pairs `(pred, succ)`: task `pred` must finish before task `succ` starts
    ///
    /// # Panics
    ///
    /// Panics if any length is zero, `num_processors` is zero, or any precedence
    /// index is out of range.
    pub fn new(
        lengths: Vec<usize>,
        num_processors: usize,
        precedences: Vec<(usize, usize)>,
    ) -> Self {
        Self::validate(&lengths, num_processors, &precedences)
            .unwrap_or_else(|err| panic!("{err}"));
        Self {
            lengths,
            num_processors,
            precedences,
        }
    }

    /// Get the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Get the number of processors.
    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    /// Get the number of precedence constraints.
    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }

    /// Get the processing lengths.
    pub fn lengths(&self) -> &[usize] {
        &self.lengths
    }

    /// Get the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Compute `D_max = sum of all task lengths` (worst-case makespan).
    pub fn d_max(&self) -> usize {
        self.lengths.iter().sum()
    }
}

impl TryFrom<PreemptiveSchedulingSerde> for PreemptiveScheduling {
    type Error = String;

    fn try_from(value: PreemptiveSchedulingSerde) -> Result<Self, Self::Error> {
        Self::validate(&value.lengths, value.num_processors, &value.precedences)?;
        Ok(Self {
            lengths: value.lengths,
            num_processors: value.num_processors,
            precedences: value.precedences,
        })
    }
}

impl<'de> Deserialize<'de> for PreemptiveScheduling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = PreemptiveSchedulingSerde::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl Problem for PreemptiveScheduling {
    const NAME: &'static str = "PreemptiveScheduling";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let d = self.d_max();
        vec![2; self.num_tasks() * d]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        let n = self.num_tasks();
        let d = self.d_max();

        // Check config length
        if config.len() != n * d {
            return Min(None);
        }

        // Check each slot is binary
        if config.iter().any(|&v| v > 1) {
            return Min(None);
        }

        // Check each task t is active in exactly l(t) slots
        for t in 0..n {
            let active: usize = config[t * d..(t + 1) * d].iter().sum();
            if active != self.lengths[t] {
                return Min(None);
            }
        }

        // Check processor capacity at each time slot
        for u in 0..d {
            let active_count: usize = (0..n).filter(|&t| config[t * d + u] == 1).count();
            if active_count > self.num_processors {
                return Min(None);
            }
        }

        // Check precedence constraints:
        // last active slot of pred < first active slot of succ
        for &(pred, succ) in &self.precedences {
            let last_pred = (0..d).rev().find(|&u| config[pred * d + u] == 1);
            let first_succ = (0..d).find(|&u| config[succ * d + u] == 1);
            if let (Some(lp), Some(fs)) = (last_pred, first_succ) {
                if lp >= fs {
                    return Min(None);
                }
            }
        }

        // Compute makespan: max over all t of (last active slot + 1)
        let makespan = (0..n)
            .filter_map(|t| (0..d).rev().find(|&u| config[t * d + u] == 1))
            .map(|last| last + 1)
            .max()
            .unwrap_or(0);

        Min(Some(makespan))
    }
}

crate::declare_variants! {
    default PreemptiveScheduling => "2^(num_tasks * num_tasks)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 5 tasks, lengths [2,1,3,2,1], 2 processors, precedences [(0,2),(1,3)]
    // D_max = 2+1+3+2+1 = 9
    // Optimal schedule (makespan 5):
    //   t0: slots 0,1         → t0*9+0=1, t0*9+1=1
    //   t1: slot 0            → t1*9+0=1
    //   t2: slots 2,3,4       → t2*9+2=1, t2*9+3=1, t2*9+4=1
    //   t3: slots 2,3         → t3*9+2=1, t3*9+3=1
    //   t4: slot 1            → t4*9+1=1
    // config indices (length 45):
    //   t0 (0..9):  [1,1,0,0,0,0,0,0,0]
    //   t1 (9..18): [1,0,0,0,0,0,0,0,0]
    //   t2 (18..27):[0,0,1,1,1,0,0,0,0]
    //   t3 (27..36):[0,0,1,1,0,0,0,0,0]
    //   t4 (36..45):[0,1,0,0,0,0,0,0,0]
    let mut config = vec![0usize; 5 * 9];
    // t0 (config[0..9]) at slots 0,1
    config[0] = 1;
    config[1] = 1;
    // t1 (config[9..18]) at slot 0
    config[9] = 1;
    // t2 (config[18..27]) at slots 2,3,4
    config[18 + 2] = 1;
    config[18 + 3] = 1;
    config[18 + 4] = 1;
    // t3 (config[27..36]) at slots 2,3
    config[27 + 2] = 1;
    config[27 + 3] = 1;
    // t4 (config[36..45]) at slot 1
    config[36 + 1] = 1;
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "preemptive_scheduling",
        instance: Box::new(PreemptiveScheduling::new(
            vec![2, 1, 3, 2, 1],
            2,
            vec![(0, 2), (1, 3)],
        )),
        optimal_config: config,
        optimal_value: serde_json::json!(5),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/preemptive_scheduling.rs"]
mod tests;
