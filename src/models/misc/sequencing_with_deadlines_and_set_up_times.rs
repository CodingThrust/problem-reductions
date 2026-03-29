//! Sequencing with Deadlines and Set-Up Times problem implementation.
//!
//! A classical NP-hard single-machine scheduling feasibility problem (SS14
//! from Garey & Johnson, 1979) where tasks use one of several compilers and
//! a setup time is charged whenever consecutive tasks switch compilers.
//! The question is whether all deadlines can be met.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingWithDeadlinesAndSetUpTimes",
        display_name: "Sequencing with Deadlines and Set-Up Times",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether all tasks can be scheduled on a single machine by their deadlines given compiler-switch setup penalties",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing time for each task" },
            FieldInfo { name: "deadlines", type_name: "Vec<u64>", description: "Deadline d(t) for each task" },
            FieldInfo { name: "compilers", type_name: "Vec<usize>", description: "Compiler index k(t) for each task" },
            FieldInfo { name: "setup_times", type_name: "Vec<u64>", description: "Setup time s(c) charged when switching away from compiler c" },
        ],
    }
}

/// Sequencing with Deadlines and Set-Up Times problem.
///
/// Given tasks with processing times `l(t)`, deadlines `d(t)`, compiler
/// assignments `k(t)`, and per-compiler setup times `s(c)`, find a
/// single-machine schedule in which all tasks meet their deadlines, where a
/// setup penalty `s(k(t'))` is added before any task `t` that uses a
/// different compiler than the immediately preceding task `t'`.
///
/// This is problem SS14 in Garey & Johnson (1979), written
/// $1 | s_{ij} | \text{feasibility}$.
///
/// Configurations are direct permutation encodings with `dims() = [n; n]`:
/// each position holds the index of the task scheduled at that position.
/// A configuration is valid iff it is a permutation of `0..n`.
#[derive(Debug, Clone, Serialize)]
pub struct SequencingWithDeadlinesAndSetUpTimes {
    lengths: Vec<u64>,
    deadlines: Vec<u64>,
    compilers: Vec<usize>,
    setup_times: Vec<u64>,
}

#[derive(Deserialize)]
struct SequencingWithDeadlinesAndSetUpTimesSerde {
    lengths: Vec<u64>,
    deadlines: Vec<u64>,
    compilers: Vec<usize>,
    setup_times: Vec<u64>,
}

impl SequencingWithDeadlinesAndSetUpTimes {
    fn validate(
        lengths: &[u64],
        deadlines: &[u64],
        compilers: &[usize],
        setup_times: &[u64],
    ) -> Result<(), String> {
        if lengths.len() != deadlines.len() {
            return Err("lengths length must equal deadlines length".to_string());
        }
        if lengths.len() != compilers.len() {
            return Err("lengths length must equal compilers length".to_string());
        }
        if lengths.contains(&0) {
            return Err("task lengths must be positive".to_string());
        }
        let num_compilers = setup_times.len();
        for &c in compilers {
            if c >= num_compilers {
                return Err(format!(
                    "compiler index {c} is out of range for setup_times of length {num_compilers}"
                ));
            }
        }
        Ok(())
    }

    /// Create a new sequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if the input vectors are inconsistent or contain invalid values.
    pub fn new(
        lengths: Vec<u64>,
        deadlines: Vec<u64>,
        compilers: Vec<usize>,
        setup_times: Vec<u64>,
    ) -> Self {
        Self::validate(&lengths, &deadlines, &compilers, &setup_times)
            .unwrap_or_else(|err| panic!("{err}"));
        Self {
            lengths,
            deadlines,
            compilers,
            setup_times,
        }
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Returns the number of distinct compilers (= `setup_times.len()`).
    pub fn num_compilers(&self) -> usize {
        self.setup_times.len()
    }

    /// Returns the processing times.
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the task deadlines.
    pub fn deadlines(&self) -> &[u64] {
        &self.deadlines
    }

    /// Returns the compiler index for each task.
    pub fn compilers(&self) -> &[usize] {
        &self.compilers
    }

    /// Returns the per-compiler setup times.
    pub fn setup_times(&self) -> &[u64] {
        &self.setup_times
    }

    /// Decode a direct permutation configuration.
    ///
    /// Returns `Some(schedule)` if the config is a valid permutation of `0..n`,
    /// or `None` otherwise.
    fn decode_permutation(config: &[usize], n: usize) -> Option<Vec<usize>> {
        if config.len() != n {
            return None;
        }
        let mut seen = vec![false; n];
        for &task in config {
            if task >= n || seen[task] {
                return None;
            }
            seen[task] = true;
        }
        Some(config.to_vec())
    }

    /// Check whether a schedule meets all deadlines.
    ///
    /// Returns `true` iff every task in the schedule completes by its deadline.
    fn all_deadlines_met(&self, schedule: &[usize]) -> bool {
        let mut elapsed: u64 = 0;
        let mut prev_compiler: Option<usize> = None;
        for &task in schedule {
            // Add setup time if the compiler switches.
            if let Some(prev) = prev_compiler {
                if prev != self.compilers[task] {
                    elapsed = elapsed
                        .checked_add(self.setup_times[self.compilers[task]])
                        .expect("elapsed time overflowed u64");
                }
            }
            elapsed = elapsed
                .checked_add(self.lengths[task])
                .expect("elapsed time overflowed u64");
            if elapsed > self.deadlines[task] {
                return false;
            }
            prev_compiler = Some(self.compilers[task]);
        }
        true
    }
}

impl TryFrom<SequencingWithDeadlinesAndSetUpTimesSerde> for SequencingWithDeadlinesAndSetUpTimes {
    type Error = String;

    fn try_from(value: SequencingWithDeadlinesAndSetUpTimesSerde) -> Result<Self, Self::Error> {
        Self::validate(
            &value.lengths,
            &value.deadlines,
            &value.compilers,
            &value.setup_times,
        )?;
        Ok(Self {
            lengths: value.lengths,
            deadlines: value.deadlines,
            compilers: value.compilers,
            setup_times: value.setup_times,
        })
    }
}

impl<'de> Deserialize<'de> for SequencingWithDeadlinesAndSetUpTimes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = SequencingWithDeadlinesAndSetUpTimesSerde::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl Problem for SequencingWithDeadlinesAndSetUpTimes {
    const NAME: &'static str = "SequencingWithDeadlinesAndSetUpTimes";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_tasks();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        let n = self.num_tasks();
        let Some(schedule) = Self::decode_permutation(config, n) else {
            return Or(false);
        };
        Or(self.all_deadlines_met(&schedule))
    }
}

crate::declare_variants! {
    default SequencingWithDeadlinesAndSetUpTimes => "factorial(num_tasks)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_with_deadlines_and_set_up_times",
        // 5 tasks, lengths [2,3,1,2,2], deadlines [4,11,3,16,7], compilers [0,1,0,1,0],
        // setup_times [1,2].
        // Optimal config: [2,0,4,1,3] (tasks t3,t1,t5,t2,t4 in 1-indexed)
        // Position 0: task 2 (compiler 0), no prev  → elapsed = 0+1 = 1  ≤ d[2]=3 ✓
        // Position 1: task 0 (compiler 0), same     → elapsed = 1+2 = 3  ≤ d[0]=4 ✓
        // Position 2: task 4 (compiler 0), same     → elapsed = 3+2 = 5  ≤ d[4]=7 ✓
        // Position 3: task 1 (compiler 1), switch+s[1]=2 → elapsed = 5+2+3 = 10 ≤ d[1]=11 ✓
        // Position 4: task 3 (compiler 1), same     → elapsed = 10+2 = 12 ≤ d[3]=16 ✓
        instance: Box::new(SequencingWithDeadlinesAndSetUpTimes::new(
            vec![2, 3, 1, 2, 2],
            vec![4, 11, 3, 16, 7],
            vec![0, 1, 0, 1, 0],
            vec![1, 2],
        )),
        optimal_config: vec![2, 0, 4, 1, 3],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_with_deadlines_and_set_up_times.rs"]
mod tests;
