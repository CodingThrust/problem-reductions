//! Minimum Tardiness Sequencing problem implementation.
//!
//! A classical NP-complete single-machine scheduling problem (SS2 from
//! Garey & Johnson, 1979) where tasks with precedence constraints
//! and deadlines must be scheduled to minimize the number of tardy tasks.
//!
//! Variants:
//! - `MinimumTardinessSequencing<One>` — unit-length tasks (`1|prec, pj=1|∑Uj`)
//! - `MinimumTardinessSequencing<i32>` — arbitrary-length tasks (`1|prec|∑Uj`)

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::{Min, One, WeightElement};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumTardinessSequencing",
        display_name: "Minimum Tardiness Sequencing",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "One", &["One", "i32"])],
        module_path: module_path!(),
        description: "Schedule tasks with precedence constraints and deadlines to minimize the number of tardy tasks",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<W>", description: "Processing time l(t) for each task" },
            FieldInfo { name: "deadlines", type_name: "Vec<usize>", description: "Deadline d(t) for each task" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (predecessor, successor)" },
        ],
    }
}

/// Minimum Tardiness Sequencing problem.
///
/// Given a set T of tasks, each with a processing time l(t) and a deadline d(t),
/// and a partial order (precedence constraints) on T, find a schedule
/// that is a valid permutation respecting precedence constraints
/// and minimizes the number of tardy tasks.
///
/// # Type Parameters
///
/// * `W` - The weight/length type. `One` for unit-length tasks, `i32` for arbitrary.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumTardinessSequencing;
/// use problemreductions::types::One;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Unit-length: 3 tasks, task 0 must precede task 2
/// let problem = MinimumTardinessSequencing::<One>::new(
///     3,
///     vec![2, 3, 1],
///     vec![(0, 2)],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumTardinessSequencing<W> {
    lengths: Vec<W>,
    deadlines: Vec<usize>,
    precedences: Vec<(usize, usize)>,
}

impl MinimumTardinessSequencing<One> {
    /// Create a new unit-length MinimumTardinessSequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `deadlines.len() != num_tasks` or if any task index in `precedences`
    /// is out of range.
    pub fn new(num_tasks: usize, deadlines: Vec<usize>, precedences: Vec<(usize, usize)>) -> Self {
        assert_eq!(
            deadlines.len(),
            num_tasks,
            "deadlines length must equal num_tasks"
        );
        validate_precedences(num_tasks, &precedences);
        Self {
            lengths: vec![One; num_tasks],
            deadlines,
            precedences,
        }
    }
}

impl MinimumTardinessSequencing<i32> {
    /// Create a new arbitrary-length MinimumTardinessSequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `lengths.len() != deadlines.len()`, if any length is 0,
    /// or if any task index in `precedences` is out of range.
    pub fn with_lengths(
        lengths: Vec<i32>,
        deadlines: Vec<usize>,
        precedences: Vec<(usize, usize)>,
    ) -> Self {
        assert_eq!(
            lengths.len(),
            deadlines.len(),
            "lengths and deadlines must have the same length"
        );
        assert!(
            lengths.iter().all(|&l| l > 0),
            "all task lengths must be positive"
        );
        let num_tasks = lengths.len();
        validate_precedences(num_tasks, &precedences);
        Self {
            lengths,
            deadlines,
            precedences,
        }
    }
}

fn validate_precedences(num_tasks: usize, precedences: &[(usize, usize)]) {
    for &(pred, succ) in precedences {
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
}

impl<W: WeightElement> MinimumTardinessSequencing<W> {
    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.deadlines.len()
    }

    /// Returns the task lengths.
    pub fn lengths(&self) -> &[W] {
        &self.lengths
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[usize] {
        &self.deadlines
    }

    /// Returns the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Returns the number of precedence constraints.
    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }

    /// Decode and validate a schedule, returning the inverse permutation (sigma).
    /// Returns None if the config is invalid or violates precedences.
    fn decode_and_validate(&self, config: &[usize]) -> Option<Vec<usize>> {
        let n = self.num_tasks();
        let schedule = super::decode_lehmer(config, n)?;

        let mut sigma = vec![0usize; n];
        for (pos, &task) in schedule.iter().enumerate() {
            sigma[task] = pos;
        }

        for &(pred, succ) in &self.precedences {
            if sigma[pred] >= sigma[succ] {
                return None;
            }
        }

        Some(sigma)
    }
}

impl Problem for MinimumTardinessSequencing<One> {
    const NAME: &'static str = "MinimumTardinessSequencing";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![One]
    }

    fn dims(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        let n = self.num_tasks();
        let Some(sigma) = self.decode_and_validate(config) else {
            return Min(None);
        };

        // Unit length: completion time at position p is p + 1
        let tardy_count = (0..n).filter(|&t| sigma[t] + 1 > self.deadlines[t]).count();

        Min(Some(tardy_count))
    }
}

impl Problem for MinimumTardinessSequencing<i32> {
    const NAME: &'static str = "MinimumTardinessSequencing";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![i32]
    }

    fn dims(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        let n = self.num_tasks();
        let Some(sigma) = self.decode_and_validate(config) else {
            return Min(None);
        };

        // Build schedule order from sigma (inverse permutation)
        let mut schedule = vec![0usize; n];
        for (task, &pos) in sigma.iter().enumerate() {
            schedule[pos] = task;
        }

        // Compute completion times using actual lengths
        let mut completion = vec![0usize; n];
        let mut cumulative = 0usize;
        for &task in &schedule {
            cumulative += self.lengths[task] as usize;
            completion[task] = cumulative;
        }

        let tardy_count = (0..n)
            .filter(|&t| completion[t] > self.deadlines[t])
            .count();

        Min(Some(tardy_count))
    }
}

crate::declare_variants! {
    default MinimumTardinessSequencing<One> => "2^num_tasks",
    MinimumTardinessSequencing<i32> => "2^num_tasks",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![
        // Unit-length variant
        crate::example_db::specs::ModelExampleSpec {
            id: "minimum_tardiness_sequencing",
            instance: Box::new(MinimumTardinessSequencing::<One>::new(
                4,
                vec![2, 3, 1, 4],
                vec![(0, 2)],
            )),
            optimal_config: vec![0, 0, 0, 0],
            optimal_value: serde_json::json!(1),
        },
        // Arbitrary-length variant
        crate::example_db::specs::ModelExampleSpec {
            id: "minimum_tardiness_sequencing_weighted",
            // 5 tasks, lengths [3,2,2,1,2], deadlines [4,3,8,3,6], prec (0→2, 1→3)
            // Optimal schedule: t0,t4,t2,t1,t3 → 2 tardy
            // Lehmer [0,3,1,0,0]: avail=[0,1,2,3,4] pick 0→0; [1,2,3,4] pick 3→4;
            //   [1,2,3] pick 1→2; [1,3] pick 0→1; [3] pick 0→3
            instance: Box::new(MinimumTardinessSequencing::<i32>::with_lengths(
                vec![3, 2, 2, 1, 2],
                vec![4, 3, 8, 3, 6],
                vec![(0, 2), (1, 3)],
            )),
            optimal_config: vec![0, 3, 1, 0, 0],
            optimal_value: serde_json::json!(2),
        },
    ]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_tardiness_sequencing.rs"]
mod tests;
