//! Capacity Assignment problem implementation.
//!
//! Capacity Assignment asks for the minimum-cost assignment of capacity levels
//! to communication links, subject to a delay budget constraint.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "CapacityAssignment",
        display_name: "Capacity Assignment",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Minimize total cost of capacity assignment subject to a delay budget",
        fields: CapacityAssignmentCreateSpec::FIELDS,
    }
}

/// Capacity Assignment optimization problem.
///
/// Each variable chooses one capacity index for one communication link.
/// Costs are monotone non-decreasing and delays are monotone non-increasing
/// with respect to the ordered capacity list. The objective is to minimize
/// total cost subject to a delay budget constraint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityAssignment {
    capacities: Vec<i64>,
    cost: Vec<Vec<i64>>,
    delay: Vec<Vec<i64>>,
    delay_budget: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct CapacityAssignmentCreateSpec {
    #[create(codec = "comma-separated")]
    capacities: Vec<i64>,
    #[create(codec = "semicolon-separated")]
    cost: Vec<Vec<i64>>,
    #[create(codec = "semicolon-separated")]
    delay: Vec<Vec<i64>>,
    delay_budget: i64,
}

impl TryFrom<CapacityAssignmentCreateSpec> for CapacityAssignment {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: CapacityAssignmentCreateSpec) -> Result<Self, Self::Error> {
        if spec.capacities.is_empty() {
            return Err("capacities must be non-empty".into());
        }
        if spec.capacities.contains(&0) {
            return Err("capacities must be positive".into());
        }
        if !spec.capacities.windows(2).all(|w| w[0] < w[1]) {
            return Err("capacities must be strictly increasing".into());
        }
        if spec.cost.len() != spec.delay.len() {
            return Err("cost and delay must have the same number of links".into());
        }
        for (i, row) in spec.cost.iter().enumerate() {
            if row.len() != spec.capacities.len() {
                return Err(format!("cost row {i} length must match capacities length").into());
            }
            if !row.windows(2).all(|w| w[0] <= w[1]) {
                return Err(format!("cost row {i} must be non-decreasing").into());
            }
        }
        for (i, row) in spec.delay.iter().enumerate() {
            if row.len() != spec.capacities.len() {
                return Err(format!("delay row {i} length must match capacities length").into());
            }
            if !row.windows(2).all(|w| w[0] >= w[1]) {
                return Err(format!("delay row {i} must be non-increasing").into());
            }
        }
        Ok(Self {
            capacities: spec.capacities,
            cost: spec.cost,
            delay: spec.delay,
            delay_budget: spec.delay_budget,
        })
    }
}

impl CapacityAssignment {
    /// Create a new Capacity Assignment instance.
    pub fn new(
        capacities: Vec<i64>,
        cost: Vec<Vec<i64>>,
        delay: Vec<Vec<i64>>,
        delay_budget: i64,
    ) -> Self {
        assert!(!capacities.is_empty(), "capacities must be non-empty");
        assert!(
            capacities.iter().all(|&capacity| capacity > 0),
            "capacities must be positive"
        );
        assert!(
            capacities.windows(2).all(|w| w[0] < w[1]),
            "capacities must be strictly increasing"
        );
        assert_eq!(
            cost.len(),
            delay.len(),
            "cost and delay must have the same number of links"
        );

        let num_capacities = capacities.len();
        for (link, row) in cost.iter().enumerate() {
            assert_eq!(
                row.len(),
                num_capacities,
                "cost row {link} length must match capacities length"
            );
            assert!(
                row.windows(2).all(|w| w[0] <= w[1]),
                "cost row {link} must be non-decreasing"
            );
        }
        for (link, row) in delay.iter().enumerate() {
            assert_eq!(
                row.len(),
                num_capacities,
                "delay row {link} length must match capacities length"
            );
            assert!(
                row.windows(2).all(|w| w[0] >= w[1]),
                "delay row {link} must be non-increasing"
            );
        }

        Self {
            capacities,
            cost,
            delay,
            delay_budget,
        }
    }

    /// Number of communication links.
    pub fn num_links(&self) -> usize {
        self.cost.len()
    }

    /// Number of discrete capacity choices per link.
    pub fn num_capacities(&self) -> usize {
        self.capacities.len()
    }

    /// Ordered capacity levels.
    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    /// Cost matrix indexed by link, then capacity.
    pub fn cost(&self) -> &[Vec<i64>] {
        &self.cost
    }

    /// Delay matrix indexed by link, then capacity.
    pub fn delay(&self) -> &[Vec<i64>] {
        &self.delay
    }

    /// Total delay budget.
    pub fn delay_budget(&self) -> i64 {
        self.delay_budget
    }

    fn total_cost_and_delay(
        &self,
        config: &[usize],
    ) -> Result<Option<(i64, i64)>, crate::traits::EvaluationError> {
        if config.len() != self.num_links() {
            return Ok(None);
        }

        let num_capacities = self.num_capacities();
        let mut total_cost = 0i64;
        let mut total_delay = 0i64;

        for (link, &choice) in config.iter().enumerate() {
            if choice >= num_capacities {
                return Ok(None);
            }
            total_cost = total_cost
                .checked_add(self.cost[link][choice])
                .ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "summing capacity-assignment costs".to_string(),
                    )
                })?;
            total_delay = total_delay
                .checked_add(self.delay[link][choice])
                .ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "summing capacity-assignment delays".to_string(),
                    )
                })?;
        }

        Ok(Some((total_cost, total_delay)))
    }
}

impl Problem for CapacityAssignment {
    const NAME: &'static str = "CapacityAssignment";
    type Solution = Vec<usize>;
    type Value = crate::types::Min<i64>;

    crate::problem_size![("num_capacities", num_capacities), ("num_links", num_links),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Min<i64>, crate::traits::EvaluationError> {
        if config.len() != self.num_links() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "capacity-choice length does not match the links".into(),
            ));
        }
        if config.iter().any(|&choice| choice >= self.num_capacities()) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "capacity assignment contains an out-of-range choice".into(),
            ));
        }
        Ok({
            let Some((total_cost, total_delay)) = self.total_cost_and_delay(config)? else {
                return Ok(crate::types::Min(None));
            };
            if total_delay <= self.delay_budget {
                crate::types::Min(Some(total_cost))
            } else {
                crate::types::Min(None)
            }
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for CapacityAssignment {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.num_capacities(); self.num_links()]
    }
}

crate::declare_variants! {
    default CapacityAssignment => "num_capacities ^ num_links" create CapacityAssignmentCreateSpec,
}

crate::register_brute_force! {
    CapacityAssignment,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "capacity_assignment",
        instance: Box::new(CapacityAssignment::new(
            vec![1, 2, 3],
            vec![vec![1, 3, 6], vec![2, 4, 7], vec![1, 2, 5]],
            vec![vec![8, 4, 1], vec![7, 3, 1], vec![6, 3, 1]],
            12,
        )),
        optimal_config: serde_json::json!(vec![1, 1, 1]),
        optimal_value: serde_json::json!(9),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/capacity_assignment.rs"]
mod tests;
