//! Production Planning problem implementation.
//!
//! Given per-period demands, production capacities, setup costs, production
//! costs, inventory costs, and a total cost bound, determine whether there
//! exists a feasible production plan that satisfies all demand without
//! backlogging and stays within budget.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ProductionPlanning",
        display_name: "Production Planning",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Determine whether a multi-period production plan can satisfy all demand within a cost bound",
        fields: ProductionPlanningCreateSpec::FIELDS,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionPlanning {
    #[serde(deserialize_with = "positive_usize::deserialize")]
    num_periods: usize,
    demands: Vec<i64>,
    capacities: Vec<i64>,
    setup_costs: Vec<i64>,
    production_costs: Vec<i64>,
    inventory_costs: Vec<i64>,
    cost_bound: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct ProductionPlanningCreateSpec {
    /// Number of planning periods.
    num_periods: usize,
    /// Demand per period.
    demands: Vec<i64>,
    /// Production capacity per period.
    capacities: Vec<i64>,
    /// Setup cost per period.
    setup_costs: Vec<i64>,
    /// Per-unit production cost per period.
    production_costs: Vec<i64>,
    /// Per-unit inventory cost per period.
    inventory_costs: Vec<i64>,
    /// Total cost bound.
    cost_bound: i64,
}
impl TryFrom<ProductionPlanningCreateSpec> for ProductionPlanning {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: ProductionPlanningCreateSpec) -> Result<Self, Self::Error> {
        if spec.num_periods == 0 {
            return Err("num_periods must be positive".to_string().into());
        }
        for (name, len) in [
            ("demands", spec.demands.len()),
            ("capacities", spec.capacities.len()),
            ("setup_costs", spec.setup_costs.len()),
            ("production_costs", spec.production_costs.len()),
            ("inventory_costs", spec.inventory_costs.len()),
        ] {
            if len != spec.num_periods {
                return Err(
                    format!("{name} has {len} entries, expected {}", spec.num_periods).into(),
                );
            }
        }
        if spec.capacities.iter().any(|&capacity| {
            usize::try_from(capacity)
                .ok()
                .and_then(|v| v.checked_add(1))
                .is_none()
        }) {
            return Err("capacities must fit in usize for dims()".to_string().into());
        }
        Ok(Self::new(
            spec.num_periods,
            spec.demands,
            spec.capacities,
            spec.setup_costs,
            spec.production_costs,
            spec.inventory_costs,
            spec.cost_bound,
        ))
    }
}

impl ProductionPlanning {
    pub fn new(
        num_periods: usize,
        demands: Vec<i64>,
        capacities: Vec<i64>,
        setup_costs: Vec<i64>,
        production_costs: Vec<i64>,
        inventory_costs: Vec<i64>,
        cost_bound: i64,
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
        assert!(
            demands
                .iter()
                .chain(&capacities)
                .chain(&setup_costs)
                .chain(&production_costs)
                .chain(&inventory_costs)
                .all(|&value| value >= 0),
            "demands, capacities, and costs must be nonnegative"
        );
        assert!(cost_bound >= 0, "cost bound must be nonnegative");

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

    pub fn demands(&self) -> &[i64] {
        &self.demands
    }

    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    pub fn setup_costs(&self) -> &[i64] {
        &self.setup_costs
    }

    pub fn production_costs(&self) -> &[i64] {
        &self.production_costs
    }

    pub fn inventory_costs(&self) -> &[i64] {
        &self.inventory_costs
    }

    pub fn cost_bound(&self) -> i64 {
        self.cost_bound
    }

    pub fn max_capacity(&self) -> i64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }
}

impl Problem for ProductionPlanning {
    const NAME: &'static str = "ProductionPlanning";
    type Solution = Vec<usize>;
    type Value = Or;

    crate::problem_parameters![("max_capacity", max_capacity), ("num_periods", num_periods),];

    fn evaluate(&self, config: &Self::Solution) -> Result<Or, crate::traits::EvaluationError> {
        Ok({
            Or({
                if config.len() != self.num_periods {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "production-plan length does not match the periods".into(),
                    ));
                }

                let mut cumulative_production = 0_i64;
                let mut cumulative_demand = 0_i64;
                let mut total_cost = 0_i64;

                for (i, &production) in config.iter().enumerate() {
                    let capacity = match usize::try_from(self.capacities[i]) {
                        Ok(value) => value,
                        Err(_) => return Ok(Or(false)),
                    };
                    if production > capacity {
                        return Ok(Or(false));
                    }

                    let production = i64::try_from(production).map_err(|_| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "converting production quantity to i64".into(),
                        )
                    })?;
                    cumulative_production = cumulative_production
                        .checked_add(production)
                        .ok_or_else(|| {
                            crate::traits::EvaluationError::IntegerOverflow(
                                "summing cumulative production".to_string(),
                            )
                        })?;
                    cumulative_demand =
                        cumulative_demand
                            .checked_add(self.demands[i])
                            .ok_or_else(|| {
                                crate::traits::EvaluationError::IntegerOverflow(
                                    "summing cumulative demand".to_string(),
                                )
                            })?;

                    if cumulative_production < cumulative_demand {
                        return Ok(Or(false));
                    }

                    let inventory = cumulative_production
                        .checked_sub(cumulative_demand)
                        .ok_or_else(|| {
                            crate::traits::EvaluationError::IntegerOverflow(
                                "computing production inventory".into(),
                            )
                        })?;
                    let production_cost = self.production_costs[i]
                        .checked_mul(production)
                        .ok_or_else(|| {
                            crate::traits::EvaluationError::IntegerOverflow(
                                "multiplying production cost".to_string(),
                            )
                        })?;
                    total_cost = total_cost.checked_add(production_cost).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing production-planning costs".to_string(),
                        )
                    })?;
                    let inventory_cost = self.inventory_costs[i]
                        .checked_mul(inventory)
                        .ok_or_else(|| {
                            crate::traits::EvaluationError::IntegerOverflow(
                                "multiplying inventory cost".to_string(),
                            )
                        })?;
                    total_cost = total_cost.checked_add(inventory_cost).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing production-planning costs".to_string(),
                        )
                    })?;
                    if production > 0 {
                        total_cost =
                            total_cost.checked_add(self.setup_costs[i]).ok_or_else(|| {
                                crate::traits::EvaluationError::IntegerOverflow(
                                    "adding production setup cost".to_string(),
                                )
                            })?;
                    }

                    if total_cost > self.cost_bound {
                        return Ok(Or(false));
                    }
                }

                total_cost <= self.cost_bound
            })
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for ProductionPlanning {
    fn dimensions(&self) -> Vec<usize> {
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
}

crate::declare_variants! {
    default ProductionPlanning => "(max_capacity + 1)^num_periods" create ProductionPlanningCreateSpec,
}

crate::register_brute_force! {
    ProductionPlanning,
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
        optimal_config: serde_json::json!(vec![3, 0, 4, 1]),
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
