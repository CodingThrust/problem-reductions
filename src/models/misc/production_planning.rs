//! Production Planning problem implementation.
//!
//! Given per-period demands, capacities, and cost coefficients, determine
//! whether there exists a feasible production plan within a total cost bound.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ProductionPlanning",
        display_name: "Production Planning",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a multi-period production plan meets all demands within a total cost bound",
        fields: &[
            FieldInfo { name: "demands", type_name: "Vec<u64>", description: "Demand r_i for each period" },
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Production capacity c_i for each period" },
            FieldInfo { name: "setup_costs", type_name: "Vec<u64>", description: "Set-up cost b_i charged when x_i > 0" },
            FieldInfo { name: "production_costs", type_name: "Vec<u64>", description: "Incremental production cost p_i per unit" },
            FieldInfo { name: "inventory_costs", type_name: "Vec<u64>", description: "Inventory holding cost h_i per unit of ending inventory" },
            FieldInfo { name: "bound", type_name: "u64", description: "Total cost bound B" },
        ],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionPlanning {
    demands: Vec<u64>,
    capacities: Vec<u64>,
    setup_costs: Vec<u64>,
    production_costs: Vec<u64>,
    inventory_costs: Vec<u64>,
    bound: u64,
}

impl ProductionPlanning {
    pub fn new(
        demands: Vec<u64>,
        capacities: Vec<u64>,
        setup_costs: Vec<u64>,
        production_costs: Vec<u64>,
        inventory_costs: Vec<u64>,
        bound: u64,
    ) -> Self {
        let num_periods = demands.len();
        assert!(
            num_periods > 0,
            "ProductionPlanning requires at least one period"
        );
        assert_eq!(
            capacities.len(),
            num_periods,
            "capacities length must match demands length"
        );
        assert_eq!(
            setup_costs.len(),
            num_periods,
            "setup_costs length must match demands length"
        );
        assert_eq!(
            production_costs.len(),
            num_periods,
            "production_costs length must match demands length"
        );
        assert_eq!(
            inventory_costs.len(),
            num_periods,
            "inventory_costs length must match demands length"
        );
        Self {
            demands,
            capacities,
            setup_costs,
            production_costs,
            inventory_costs,
            bound,
        }
    }

    pub fn num_periods(&self) -> usize {
        self.demands.len()
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

    pub fn bound(&self) -> u64 {
        self.bound
    }
}

impl Problem for ProductionPlanning {
    const NAME: &'static str = "ProductionPlanning";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .map(|&capacity| {
                usize::try_from(capacity)
                    .expect("capacity exceeds usize")
                    .checked_add(1)
                    .expect("capacity + 1 overflowed usize")
            })
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        if config.len() != self.num_periods() {
            return crate::types::Or(false);
        }

        let mut inventory = 0i128;
        let mut total_cost = 0u128;
        let bound = u128::from(self.bound);

        for (period, &production) in config.iter().enumerate() {
            let Ok(production) = u64::try_from(production) else {
                return crate::types::Or(false);
            };
            if production > self.capacities[period] {
                return crate::types::Or(false);
            }

            inventory += i128::from(production);
            inventory -= i128::from(self.demands[period]);
            if inventory < 0 {
                return crate::types::Or(false);
            }

            let Some(production_term) =
                u128::from(self.production_costs[period]).checked_mul(u128::from(production))
            else {
                return crate::types::Or(false);
            };
            let Some(inventory_term) = u128::from(self.inventory_costs[period])
                .checked_mul(u128::try_from(inventory).expect("inventory is non-negative"))
            else {
                return crate::types::Or(false);
            };

            let mut period_cost = match production_term.checked_add(inventory_term) {
                Some(cost) => cost,
                None => return crate::types::Or(false),
            };
            if production > 0 {
                period_cost = match period_cost.checked_add(u128::from(self.setup_costs[period])) {
                    Some(cost) => cost,
                    None => return crate::types::Or(false),
                };
            }

            total_cost = match total_cost.checked_add(period_cost) {
                Some(cost) => cost,
                None => return crate::types::Or(false),
            };
            if total_cost > bound {
                return crate::types::Or(false);
            }
        }

        crate::types::Or(total_cost <= bound)
    }
}

crate::declare_variants! {
    default ProductionPlanning => "2^num_periods",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "production_planning",
        instance: Box::new(ProductionPlanning::new(
            vec![0, 1, 2],
            vec![2, 1, 0],
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![1, 0, 0],
            6,
        )),
        optimal_config: vec![2, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/production_planning.rs"]
mod tests;
