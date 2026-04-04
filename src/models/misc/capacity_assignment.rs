//! Capacity Assignment problem implementation.
//!
//! Capacity Assignment asks for the minimum-cost assignment of capacity levels
//! to communication links, subject to a delay budget constraint.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "CapacityAssignment",
        display_name: "Capacity Assignment",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Minimize total cost of capacity assignment subject to a delay budget",
        fields: &[
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Ordered capacity levels M" },
            FieldInfo { name: "cost", type_name: "Vec<Vec<u64>>", description: "Cost matrix g(c, m) for each link and capacity" },
            FieldInfo { name: "delay", type_name: "Vec<Vec<u64>>", description: "Delay matrix d(c, m) for each link and capacity" },
            FieldInfo { name: "delay_budget", type_name: "u64", description: "Budget J on total delay penalty" },
        ],
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
    capacities: Vec<u64>,
    cost: Vec<Vec<u64>>,
    delay: Vec<Vec<u64>>,
    delay_budget: u64,
}

impl CapacityAssignment {
    /// Create a new Capacity Assignment instance.
    pub fn new(
        capacities: Vec<u64>,
        cost: Vec<Vec<u64>>,
        delay: Vec<Vec<u64>>,
        delay_budget: u64,
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
    pub fn capacities(&self) -> &[u64] {
        &self.capacities
    }

    /// Cost matrix indexed by link, then capacity.
    pub fn cost(&self) -> &[Vec<u64>] {
        &self.cost
    }

    /// Delay matrix indexed by link, then capacity.
    pub fn delay(&self) -> &[Vec<u64>] {
        &self.delay
    }

    /// Total delay budget.
    pub fn delay_budget(&self) -> u64 {
        self.delay_budget
    }

    fn total_cost_and_delay(&self, config: &[usize]) -> Option<(u128, u128)> {
        if config.len() != self.num_links() {
            return None;
        }

        let num_capacities = self.num_capacities();
        let mut total_cost = 0u128;
        let mut total_delay = 0u128;

        for (link, &choice) in config.iter().enumerate() {
            if choice >= num_capacities {
                return None;
            }
            total_cost += self.cost[link][choice] as u128;
            total_delay += self.delay[link][choice] as u128;
        }

        Some((total_cost, total_delay))
    }
}

impl Problem for CapacityAssignment {
    const NAME: &'static str = "CapacityAssignment";
    type Value = crate::types::Min<u128>;

    fn dims(&self) -> Vec<usize> {
        vec![self.num_capacities(); self.num_links()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Min<u128> {
        let Some((total_cost, total_delay)) = self.total_cost_and_delay(config) else {
            return crate::types::Min(None);
        };
        if total_delay <= self.delay_budget as u128 {
            crate::types::Min(Some(total_cost))
        } else {
            crate::types::Min(None)
        }
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default CapacityAssignment => "num_capacities ^ num_links",
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
        optimal_config: vec![1, 1, 1],
        optimal_value: serde_json::json!(9),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/capacity_assignment.rs"]
mod tests;
