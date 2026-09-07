//! Preemptive Scheduling problem implementation.
//!
//! A classical NP-hard scheduling problem (Garey & Johnson A5 SS6) where
//! variable-length tasks may be split across non-contiguous time slots on
//! `m` identical processors, subject to precedence constraints.
//! The goal is to minimize the makespan (latest completion time).

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PreemptiveScheduling",
        display_name: "Preemptive Scheduling",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Minimize makespan for preemptive parallel-processor scheduling with precedence constraints",
        fields: PreemptiveSchedulingCreateSpec::FIELDS,
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
/// `solution[t][u] = true` means task `t` is processed at time slot `u`.
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
/// let problem = PreemptiveScheduling::new(vec![2, 1], 2, vec![]).unwrap();
/// // D_max = 3, so the solution is a 2 × 3 task-by-time matrix.
/// // task 0 active at slots 0,1; task 1 active at slot 0
/// let solution = vec![
///     vec![true, true, false],
///     vec![true, false, false],
/// ];
/// assert_eq!(problem.evaluate(&solution).unwrap(), problemreductions::types::Min(Some(2)));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PreemptiveScheduling {
    /// Processing length for each task.
    lengths: Vec<i64>,
    /// Number of identical processors.
    num_processors: usize,
    /// Precedence constraints: (pred, succ) means pred must finish before succ starts.
    precedences: Vec<(usize, usize)>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct PreemptiveSchedulingCreateSpec {
    lengths: Vec<i64>,
    num_processors: usize,
    precedences: Option<Vec<(usize, usize)>>,
}

impl TryFrom<PreemptiveSchedulingCreateSpec> for PreemptiveScheduling {
    type Error = ConstructionError;

    fn try_from(spec: PreemptiveSchedulingCreateSpec) -> Result<Self, Self::Error> {
        let precedences = spec.precedences.unwrap_or_default();
        Self::new(spec.lengths, spec.num_processors, precedences)
    }
}

#[derive(Deserialize)]
struct PreemptiveSchedulingSerde {
    lengths: Vec<i64>,
    num_processors: usize,
    precedences: Vec<(usize, usize)>,
}

impl PreemptiveScheduling {
    fn validate(
        lengths: &[i64],
        num_processors: usize,
        precedences: &[(usize, usize)],
    ) -> Result<(), ConstructionError> {
        if lengths.iter().any(|&length| length <= 0) {
            return Err(ConstructionError::Conversion(
                "task lengths must be positive".into(),
            ));
        }
        if num_processors == 0 {
            return Err(ConstructionError::Conversion(
                "num_processors must be positive".into(),
            ));
        }
        let n = lengths.len();
        let total_length = lengths
            .iter()
            .try_fold(0_i64, |total, &length| total.checked_add(length))
            .ok_or_else(|| ConstructionError::IntegerOverflow("summing task lengths".into()))?;
        let horizon = usize::try_from(total_length).map_err(|_| {
            ConstructionError::IntegerOverflow("task horizon does not fit usize".into())
        })?;
        n.checked_mul(horizon).ok_or_else(|| {
            ConstructionError::IntegerOverflow("configuration size does not fit usize".into())
        })?;
        for &(pred, succ) in precedences {
            if pred >= n || succ >= n {
                return Err(ConstructionError::Conversion(format!(
                    "precedence index out of range: ({pred}, {succ}) but num_tasks = {n}"
                )));
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
    pub fn new(
        lengths: Vec<i64>,
        num_processors: usize,
        precedences: Vec<(usize, usize)>,
    ) -> Result<Self, ConstructionError> {
        Self::validate(&lengths, num_processors, &precedences)?;
        Ok(Self {
            lengths,
            num_processors,
            precedences,
        })
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
    pub fn lengths(&self) -> &[i64] {
        &self.lengths
    }

    /// Get the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Compute `D_max = sum of all task lengths` (worst-case makespan).
    pub fn d_max(&self) -> usize {
        let total = self
            .lengths
            .iter()
            .try_fold(0_i64, |total, &length| total.checked_add(length))
            .expect("construction validates the task horizon");
        usize::try_from(total).expect("validated task horizon fits usize")
    }
}

impl TryFrom<PreemptiveSchedulingSerde> for PreemptiveScheduling {
    type Error = ConstructionError;

    fn try_from(value: PreemptiveSchedulingSerde) -> Result<Self, Self::Error> {
        Self::new(value.lengths, value.num_processors, value.precedences)
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
    type Solution = Vec<Vec<bool>>;
    type Value = Min<i64>;

    crate::problem_parameters![
        ("d_max", d_max),
        ("num_precedences", num_precedences),
        ("num_processors", num_processors),
        ("num_tasks", num_tasks),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        let n = self.num_tasks();
        let d = self.d_max();
        if solution.len() != n || solution.iter().any(|task| task.len() != d) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "preemptive schedule dimensions do not match the instance".into(),
            ));
        }
        Ok({
            // Check each task t is active in exactly l(t) slots
            for (task, &length) in solution.iter().zip(&self.lengths) {
                let active = task.iter().filter(|&&active| active).count();
                if i64::try_from(active).expect("active slots fit the validated horizon") != length
                {
                    return Ok(Min(None));
                }
            }

            // Check processor capacity at each time slot
            for u in 0..d {
                let active_count = solution.iter().filter(|task| task[u]).count();
                if active_count > self.num_processors {
                    return Ok(Min(None));
                }
            }

            // Check precedence constraints:
            // last active slot of pred < first active slot of succ
            for &(pred, succ) in &self.precedences {
                let last_pred = (0..d).rev().find(|&u| solution[pred][u]);
                let first_succ = (0..d).find(|&u| solution[succ][u]);
                if let (Some(lp), Some(fs)) = (last_pred, first_succ) {
                    if lp >= fs {
                        return Ok(Min(None));
                    }
                }
            }

            // Compute makespan: max over all t of (last active slot + 1)
            let makespan = solution
                .iter()
                .filter_map(|task| (0..d).rev().find(|&u| task[u]))
                .map(|last| last + 1)
                .max()
                .unwrap_or(0);

            Min(Some(
                i64::try_from(makespan).expect("makespan fits the validated horizon"),
            ))
        })
    }
}

impl crate::solvers::BruteForceProblem for PreemptiveScheduling {
    fn dimensions(&self) -> Vec<usize> {
        let d = self.d_max();
        vec![2; self.num_tasks() * d]
    }
}

crate::declare_variants! {
    default PreemptiveScheduling => "2^(num_tasks * num_tasks)" create PreemptiveSchedulingCreateSpec,
}

crate::register_brute_force! {
    PreemptiveScheduling decode |problem: &PreemptiveScheduling, indices: Vec<usize>| if problem.d_max() == 0 { vec![Vec::new(); problem.num_tasks()] } else { indices.chunks(problem.d_max()).map(crate::config::config_to_bits).collect() },
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
    let mut config = vec![vec![false; 9]; 5];
    config[0][0] = true;
    config[0][1] = true;
    config[1][0] = true;
    config[2][2] = true;
    config[2][3] = true;
    config[2][4] = true;
    config[3][2] = true;
    config[3][3] = true;
    config[4][1] = true;
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "preemptive_scheduling",
        instance: Box::new(
            PreemptiveScheduling::new(vec![2, 1, 3, 2, 1], 2, vec![(0, 2), (1, 3)]).unwrap(),
        ),
        optimal_config: serde_json::json!(config),
        optimal_value: serde_json::json!(5),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/preemptive_scheduling.rs"]
mod tests;
