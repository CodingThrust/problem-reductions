//! Reduction from Partition to Production Planning.

use crate::models::misc::{Partition, ProductionPlanning};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionPartitionToProductionPlanning {
    target: ProductionPlanning,
}

impl ReductionResult for ReductionPartitionToProductionPlanning {
    type Source = Partition;
    type Target = ProductionPlanning;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution
            .iter()
            .take(self.target.num_periods().saturating_sub(1))
            .map(|&production| usize::from(production > 0))
            .collect()
    }
}

#[reduction(overhead = {
    num_periods = "num_elements + 1",
})]
impl ReduceTo<ProductionPlanning> for Partition {
    type Result = ReductionPartitionToProductionPlanning;

    fn reduce_to(&self) -> Self::Result {
        let half_floor = self.total_sum() / 2;
        let half_ceil = half_floor + (self.total_sum() % 2);
        let mut demands = vec![0; self.num_elements()];
        demands.push(half_ceil);

        let mut capacities = self.sizes().to_vec();
        capacities.push(0);

        let mut setup_costs = self.sizes().to_vec();
        setup_costs.push(0);

        let production_costs = vec![0; self.num_elements() + 1];
        let inventory_costs = vec![0; self.num_elements() + 1];

        ReductionPartitionToProductionPlanning {
            target: ProductionPlanning::new(
                demands,
                capacities,
                setup_costs,
                production_costs,
                inventory_costs,
                half_floor,
            ),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_production_planning",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ProductionPlanning>(
                Partition::new(vec![3, 5, 2, 4, 6]),
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1],
                    target_config: vec![0, 0, 0, 4, 6, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_productionplanning.rs"]
mod tests;
