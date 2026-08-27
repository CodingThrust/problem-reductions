//! Resource Constrained Scheduling problem implementation.
//!
//! A classical NP-complete scheduling problem (Garey & Johnson A5 SS10) where
//! unit-length tasks must be assigned to identical processors under both a
//! processor capacity limit and resource usage constraints per time slot.

use crate::registry::{ConstructionError, FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ResourceConstrainedScheduling",
        display_name: "Resource Constrained Scheduling",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Schedule unit-length tasks on m processors with resource constraints and a deadline",
        fields: &[
            FieldInfo { name: "num_processors", type_name: "usize", description: "Number of identical processors m" },
            FieldInfo { name: "resource_bounds", type_name: "Vec<i64>", description: "Resource bound B_i for each resource i" },
            FieldInfo { name: "resource_requirements", type_name: "Vec<Vec<i64>>", description: "R_i(t) for each task t and resource i (n x r matrix)" },
            FieldInfo { name: "deadline", type_name: "i64", description: "Overall deadline D" },
        ],
    }
}

/// The Resource Constrained Scheduling problem.
///
/// Given `n` unit-length tasks, `m` identical processors, `r` resources with
/// bounds `B_i`, resource requirements `R_i(t)` for each task `t` and resource `i`,
/// and an overall deadline `D`, determine whether there exists a schedule
/// `σ: T → {0, ..., D-1}` such that:
/// - At each time slot `u`, at most `m` tasks are scheduled (processor capacity)
/// - At each time slot `u` and for each resource `i`, the sum of `R_i(t)` over
///   all tasks `t` scheduled at `u` does not exceed `B_i`
///
/// # Representation
///
/// Each task has a variable in `{0, ..., D-1}` representing its assigned time slot.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::ResourceConstrainedScheduling;
/// use problemreductions::{Problem, BruteForce};
///
/// // 6 tasks, 3 processors, 1 resource with bound 20, deadline 2
/// let problem = ResourceConstrainedScheduling::new(
///     3,
///     vec![20],
///     vec![vec![6], vec![7], vec![7], vec![6], vec![8], vec![6]],
///     2,
/// ).unwrap();
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ResourceConstrainedScheduling {
    /// Number of identical processors.
    num_processors: usize,
    /// Resource bounds B_i for each resource.
    resource_bounds: Vec<i64>,
    /// Resource requirements R_i(t) for each task t and resource i (n x r matrix).
    resource_requirements: Vec<Vec<i64>>,
    /// Overall deadline D.
    deadline: i64,
}

impl ResourceConstrainedScheduling {
    /// Create a new Resource Constrained Scheduling instance.
    ///
    /// # Arguments
    /// * `num_processors` - Number of identical processors `m`
    /// * `resource_bounds` - Resource bound `B_i` for each resource `i` (length = r)
    /// * `resource_requirements` - `R_i(t)` for each task `t` and resource `i` (n x r matrix)
    /// * `deadline` - Overall deadline `D`
    pub fn new(
        num_processors: usize,
        resource_bounds: Vec<i64>,
        resource_requirements: Vec<Vec<i64>>,
        deadline: i64,
    ) -> Result<Self, ConstructionError> {
        if deadline <= 0 {
            return Err(ConstructionError::Conversion(
                "deadline must be positive".into(),
            ));
        }
        usize::try_from(deadline).map_err(|_| {
            ConstructionError::IntegerOverflow("deadline does not fit usize".into())
        })?;
        if resource_bounds.iter().any(|&bound| bound < 0) {
            return Err(ConstructionError::Conversion(
                "resource bounds must be nonnegative".into(),
            ));
        }
        let r = resource_bounds.len();
        for (t, row) in resource_requirements.iter().enumerate() {
            if row.len() != r {
                return Err(ConstructionError::Conversion(format!(
                    "task {t} has {} resource requirements, expected {r}",
                    row.len()
                )));
            }
            if row.iter().any(|&requirement| requirement < 0) {
                return Err(ConstructionError::Conversion(format!(
                    "task {t} resource requirements must be nonnegative"
                )));
            }
        }
        Ok(Self {
            num_processors,
            resource_bounds,
            resource_requirements,
            deadline,
        })
    }

    /// Get the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.resource_requirements.len()
    }

    /// Get the number of processors.
    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    /// Get the resource bounds.
    pub fn resource_bounds(&self) -> &[i64] {
        &self.resource_bounds
    }

    /// Get the resource requirements matrix.
    pub fn resource_requirements(&self) -> &[Vec<i64>] {
        &self.resource_requirements
    }

    /// Get the deadline.
    pub fn deadline(&self) -> i64 {
        self.deadline
    }

    /// Get the number of resources.
    pub fn num_resources(&self) -> usize {
        self.resource_bounds.len()
    }
}

impl<'de> Deserialize<'de> for ResourceConstrainedScheduling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            num_processors: usize,
            resource_bounds: Vec<i64>,
            resource_requirements: Vec<Vec<i64>>,
            deadline: i64,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(
            raw.num_processors,
            raw.resource_bounds,
            raw.resource_requirements,
            raw.deadline,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl Problem for ResourceConstrainedScheduling {
    const NAME: &'static str = "ResourceConstrainedScheduling";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_size![
        ("deadline", deadline),
        ("num_resources", num_resources),
        ("num_tasks", num_tasks),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                let n = self.num_tasks();
                let d = self.deadline as usize;
                let r = self.num_resources();

                // Check config length
                if config.len() != n {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "schedule length does not match the tasks".into(),
                    ));
                }

                if config.iter().any(|&start| start >= d) {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "schedule contains an out-of-range start time".into(),
                    ));
                }

                // Check processor capacity and resource constraints at each time slot
                for u in 0..d {
                    // Collect tasks scheduled at time slot u
                    let mut task_count = 0usize;
                    let mut resource_usage = vec![0i64; r];

                    for (t, &slot) in config.iter().enumerate() {
                        if slot == u {
                            task_count += 1;
                            // Accumulate resource usage
                            for (usage, &req) in resource_usage
                                .iter_mut()
                                .zip(self.resource_requirements[t].iter())
                            {
                                *usage = usage.checked_add(req).ok_or_else(|| {
                                    crate::traits::EvaluationError::IntegerOverflow(
                                        "summing scheduled resource usage".to_string(),
                                    )
                                })?;
                            }
                        }
                    }

                    // Check processor capacity
                    if task_count > self.num_processors {
                        return Ok(crate::types::Or(false));
                    }

                    // Check resource bounds
                    for (usage, bound) in resource_usage.iter().zip(self.resource_bounds.iter()) {
                        if usage > bound {
                            return Ok(crate::types::Or(false));
                        }
                    }
                }

                true
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for ResourceConstrainedScheduling {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.deadline as usize; self.num_tasks()]
    }
}

crate::declare_variants! {
    default ResourceConstrainedScheduling => "deadline ^ num_tasks",
}

crate::register_brute_force! {
    ResourceConstrainedScheduling,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "resource_constrained_scheduling",
        // 6 tasks, 3 processors, 1 resource B_1=20, deadline 2
        instance: Box::new(
            ResourceConstrainedScheduling::new(
                3,
                vec![20],
                vec![vec![6], vec![7], vec![7], vec![6], vec![8], vec![6]],
                2,
            )
            .expect("canonical resource-constrained-scheduling instance must be valid"),
        ),
        optimal_config: serde_json::json!(vec![0, 0, 0, 1, 1, 1]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/resource_constrained_scheduling.rs"]
mod tests;
