//! Minimum Tardiness Sequencing problem implementation.
//!
//! A classical NP-complete single-machine scheduling problem (SS2 from
//! Garey & Johnson, 1979) where tasks with precedence constraints
//! and deadlines must be scheduled to minimize the number of tardy tasks.
//!
//! Variants:
//! - `MinimumTardinessSequencing<One>` — unit-length tasks (`1|prec, pj=1|∑Uj`)
//! - `MinimumTardinessSequencing<i64>` — arbitrary-length tasks (`1|prec|∑Uj`)

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::{Min, One, WeightElement};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumTardinessSequencing",
        display_name: "Minimum Tardiness Sequencing",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "One", &["One", "i64"])],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Schedule tasks with precedence constraints and deadlines to minimize the number of tardy tasks",
        fields: MinimumTardinessSequencingOneCreateSpec::FIELDS,
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
/// * `W` - The weight/length type. `One` for unit-length tasks, `i64` for arbitrary.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumTardinessSequencing;
/// use problemreductions::types::One;
/// use problemreductions::{Problem, BruteForce};
///
/// // Unit-length: 3 tasks, task 0 must precede task 2
/// let problem = MinimumTardinessSequencing::<One>::new(
///     3,
///     vec![2, 3, 1],
///     vec![(0, 2)],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumTardinessSequencing<W> {
    lengths: Vec<W>,
    deadlines: Vec<i64>,
    precedences: Vec<(usize, usize)>,
}

macro_rules! minimum_tardiness_create_spec {
    ($name:ident, $weight:ty, $construct:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            lengths: Vec<$weight>,
            deadlines: Vec<i64>,
            precedences: Option<Vec<(usize, usize)>>,
        }

        impl TryFrom<$name> for MinimumTardinessSequencing<$weight> {
            type Error = crate::registry::ConstructionError;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                if spec.lengths.len() != spec.deadlines.len() {
                    return Err("lengths and deadlines must have the same length"
                        .to_string()
                        .into());
                }
                let precedences = spec.precedences.unwrap_or_default();
                let num_tasks = spec.lengths.len();
                if let Some(&(pred, succ)) = precedences
                    .iter()
                    .find(|&&(pred, succ)| pred >= num_tasks || succ >= num_tasks)
                {
                    return Err(format!(
                        "precedence ({pred}, {succ}) is out of range for {num_tasks} tasks"
                    )
                    .into());
                }
                $construct(spec.lengths, spec.deadlines, precedences)
            }
        }
    };
}

minimum_tardiness_create_spec!(
    MinimumTardinessSequencingOneCreateSpec,
    One,
    |lengths: Vec<One>, deadlines, precedences| {
        Ok(MinimumTardinessSequencing::new(
            lengths.len(),
            deadlines,
            precedences,
        ))
    }
);
minimum_tardiness_create_spec!(
    MinimumTardinessSequencingI64CreateSpec,
    i64,
    |lengths: Vec<i64>, deadlines, precedences| {
        if lengths.iter().any(|&length| length <= 0) {
            return Err("all task lengths must be positive".to_string().into());
        }
        Ok(MinimumTardinessSequencing::with_lengths(
            lengths,
            deadlines,
            precedences,
        ))
    }
);

impl MinimumTardinessSequencing<One> {
    /// Create a new unit-length MinimumTardinessSequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `deadlines.len() != num_tasks` or if any task index in `precedences`
    /// is out of range.
    pub fn new(num_tasks: usize, deadlines: Vec<i64>, precedences: Vec<(usize, usize)>) -> Self {
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

impl MinimumTardinessSequencing<i64> {
    /// Create a new arbitrary-length MinimumTardinessSequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `lengths.len() != deadlines.len()`, if any length is 0,
    /// or if any task index in `precedences` is out of range.
    pub fn with_lengths(
        lengths: Vec<i64>,
        deadlines: Vec<i64>,
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
    pub fn deadlines(&self) -> &[i64] {
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

    /// Validate a schedule and return the inverse permutation (sigma).
    /// Returns None if the config is invalid or violates precedences.
    fn decode_and_validate(&self, config: &[usize]) -> Option<Vec<usize>> {
        let n = self.num_tasks();
        let schedule = super::decode_permutation(config, n)?;

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
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_size![
        ("num_precedences", num_precedences),
        ("num_tasks", num_tasks),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![One]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        let n = self.num_tasks();
        if config.len() != n {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "schedule length does not match the tasks".into(),
            ));
        }
        if config.iter().any(|&task| task >= n) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "schedule contains an out-of-range task".into(),
            ));
        }
        Ok({
            let Some(sigma) = self.decode_and_validate(config) else {
                return Ok(Min(None));
            };

            // Unit length: completion time at position p is p + 1
            let mut tardy_count = 0_i64;
            for (task, &position) in sigma.iter().enumerate() {
                let completion = i64::try_from(position)
                    .ok()
                    .and_then(|position| position.checked_add(1))
                    .ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "computing a unit-length task completion time".to_string(),
                        )
                    })?;
                if completion > self.deadlines[task] {
                    tardy_count = tardy_count.checked_add(1).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "counting tardy tasks".to_string(),
                        )
                    })?;
                }
            }

            Min(Some(tardy_count))
        })
    }
}

impl crate::solvers::BruteForceProblem for MinimumTardinessSequencing<One> {
    fn dimensions(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }
}

impl Problem for MinimumTardinessSequencing<i64> {
    const NAME: &'static str = "MinimumTardinessSequencing";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_size![
        ("num_precedences", num_precedences),
        ("num_tasks", num_tasks),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![i64]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        let n = self.num_tasks();
        if config.len() != n {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "schedule length does not match the tasks".into(),
            ));
        }
        if config.iter().any(|&task| task >= n) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "schedule contains an out-of-range task".into(),
            ));
        }
        Ok({
            let Some(sigma) = self.decode_and_validate(config) else {
                return Ok(Min(None));
            };

            // Build schedule order from sigma (inverse permutation)
            let mut schedule = vec![0usize; n];
            for (task, &pos) in sigma.iter().enumerate() {
                schedule[pos] = task;
            }

            // Compute completion times using actual lengths
            let mut completion = vec![0_i64; n];
            let mut cumulative = 0_i64;
            for &task in &schedule {
                cumulative = cumulative.checked_add(self.lengths[task]).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "summing task lengths while computing completion times".to_string(),
                    )
                })?;
                completion[task] = cumulative;
            }

            let mut tardy_count = 0_i64;
            for (task, &completion_time) in completion.iter().enumerate() {
                if completion_time > self.deadlines[task] {
                    tardy_count = tardy_count.checked_add(1).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "counting tardy tasks".to_string(),
                        )
                    })?;
                }
            }

            Min(Some(tardy_count))
        })
    }
}

impl crate::solvers::BruteForceProblem for MinimumTardinessSequencing<i64> {
    fn dimensions(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }
}

crate::declare_variants! {
    default MinimumTardinessSequencing<One> => "2^num_tasks" create MinimumTardinessSequencingOneCreateSpec,
    MinimumTardinessSequencing<i64> => "2^num_tasks" create MinimumTardinessSequencingI64CreateSpec,
}

crate::register_brute_force! {
    MinimumTardinessSequencing<One> decode |problem: &MinimumTardinessSequencing<One>, indices: Vec<usize>| super::decode_lehmer(&indices, problem.num_tasks()).expect("enumerated Lehmer digits are valid"),
    MinimumTardinessSequencing<i64> decode |problem: &MinimumTardinessSequencing<i64>, indices: Vec<usize>| super::decode_lehmer(&indices, problem.num_tasks()).expect("enumerated Lehmer digits are valid"),
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
            optimal_config: serde_json::json!(vec![0, 1, 2, 3]),
            optimal_value: serde_json::json!(1),
        },
        // Arbitrary-length variant
        crate::example_db::specs::ModelExampleSpec {
            id: "minimum_tardiness_sequencing_weighted",
            // 5 tasks, lengths [3,2,2,1,2], deadlines [4,3,8,3,6], prec (0→2, 1→3)
            // Optimal schedule: t0,t4,t2,t1,t3 → 2 tardy
            // Lehmer [0,3,1,0,0]: avail=[0,1,2,3,4] pick 0→0; [1,2,3,4] pick 3→4;
            //   [1,2,3] pick 1→2; [1,3] pick 0→1; [3] pick 0→3
            instance: Box::new(MinimumTardinessSequencing::<i64>::with_lengths(
                vec![3, 2, 2, 1, 2],
                vec![4, 3, 8, 3, 6],
                vec![(0, 2), (1, 3)],
            )),
            optimal_config: serde_json::json!(vec![0, 4, 2, 1, 3]),
            optimal_value: serde_json::json!(2),
        },
    ]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_tardiness_sequencing.rs"]
mod tests;
