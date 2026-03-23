//! Production Planning problem implementation.
//!
//! Given per-period demands, production capacities, setup costs, production
//! costs, inventory costs, and a total cost bound, determine whether there
//! exists a feasible production plan that satisfies all demand without
//! backlogging and stays within budget.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ProductionPlanning",
        display_name: "Production Planning",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a multi-period production plan can satisfy all demand within a cost bound",
        fields: &[
            FieldInfo { name: "num_periods", type_name: "usize", description: "Number of planning periods n" },
            FieldInfo { name: "demands", type_name: "Vec<u64>", description: "Demand r_i for each period" },
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Production capacity c_i for each period" },
            FieldInfo { name: "setup_costs", type_name: "Vec<u64>", description: "Setup cost b_i incurred when x_i > 0" },
            FieldInfo { name: "production_costs", type_name: "Vec<u64>", description: "Per-unit production cost coefficient p_i" },
            FieldInfo { name: "inventory_costs", type_name: "Vec<u64>", description: "Per-unit inventory cost coefficient h_i" },
            FieldInfo { name: "cost_bound", type_name: "u64", description: "Total cost bound B" },
        ],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionPlanning {
    #[serde(deserialize_with = "positive_usize::deserialize")]
    num_periods: usize,
    demands: Vec<u64>,
    capacities: Vec<u64>,
    setup_costs: Vec<u64>,
    production_costs: Vec<u64>,
    inventory_costs: Vec<u64>,
    cost_bound: u64,
}

impl ProductionPlanning {
    pub fn new(
        num_periods: usize,
        demands: Vec<u64>,
        capacities: Vec<u64>,
        setup_costs: Vec<u64>,
        production_costs: Vec<u64>,
        inventory_costs: Vec<u64>,
        cost_bound: u64,
    ) -> Self {
        assert!(num_periods > 0, "num_periods must be positive");
        for len in [
            demands.len(),
            capacities.len(),
            setup_costs.len(),
            production_costs.len(),
            inventory_costs.len(),
        ] {
            assert_eq!(
                len, num_periods,
                "all per-period vectors must have length num_periods"
            );
        }
        assert!(
            capacities.iter().all(|&capacity| {
                usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .is_some()
            }),
            "capacities must fit in usize for dims()"
        );

        Self {
            num_periods,
            demands,
            capacities,
            setup_costs,
            production_costs,
            inventory_costs,
            cost_bound,
        }
    }

    pub fn num_periods(&self) -> usize {
        self.num_periods
    }

    pub fn demands(&self) -> &[u64] {
        &self.demands
    }

    pub fn capacities(&self) -> &[u64] {
        &self.capacities
    }

    pub fn setup_costs(&self) -> &[u64] {
        &self.setup_costs
    }

    pub fn production_costs(&self) -> &[u64] {
        &self.production_costs
    }

    pub fn inventory_costs(&self) -> &[u64] {
        &self.inventory_costs
    }

    pub fn cost_bound(&self) -> u64 {
        self.cost_bound
    }

    pub fn max_capacity(&self) -> u64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }
}

impl Problem for ProductionPlanning {
    const NAME: &'static str = "ProductionPlanning";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .map(|&capacity| {
                usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .expect("capacities validated in constructor")
            })
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_periods {
            return false;
        }

        let mut cumulative_production = 0u128;
        let mut cumulative_demand = 0u128;
        let mut total_cost = 0u128;
        let cost_bound = self.cost_bound as u128;

        for (i, &production) in config.iter().enumerate() {
            let capacity = match usize::try_from(self.capacities[i]) {
                Ok(value) => value,
                Err(_) => return false,
            };
            if production > capacity {
                return false;
            }

            let production = production as u128;
            cumulative_production += production;
            cumulative_demand += self.demands[i] as u128;

            if cumulative_production < cumulative_demand {
                return false;
            }

            let inventory = cumulative_production - cumulative_demand;
            total_cost += self.production_costs[i] as u128 * production;
            total_cost += self.inventory_costs[i] as u128 * inventory;
            if production > 0 {
                total_cost += self.setup_costs[i] as u128;
            }

            if total_cost > cost_bound {
                return false;
            }
        }

        total_cost <= cost_bound
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for ProductionPlanning {}

crate::declare_variants! {
    default sat ProductionPlanning => "(max_capacity + 1)^num_periods",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "production_planning",
        instance: Box::new(ProductionPlanning::new(
            4,
            vec![2, 1, 3, 2],
            vec![4, 4, 4, 4],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            16,
        )),
        optimal_config: vec![3, 0, 4, 1],
        optimal_value: serde_json::json!(true),
    }]
}

mod positive_usize {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = usize::deserialize(deserializer)?;
        if value == 0 {
            return Err(D::Error::custom("expected positive integer, got 0"));
        }
        Ok(value)
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/production_planning.rs"]
mod tests;
