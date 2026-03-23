//! Expected Retrieval Cost problem implementation.
//!
//! Given record access probabilities, decide whether records can be assigned to
//! circular storage sectors so the expected rotational latency stays below a
//! prescribed bound.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

const FLOAT_TOLERANCE: f64 = 1e-9;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ExpectedRetrievalCost",
        display_name: "Expected Retrieval Cost",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Assign records to circular storage sectors so the expected retrieval latency stays within a bound",
        fields: &[
            FieldInfo { name: "probabilities", type_name: "Vec<f64>", description: "Access probabilities p(r) for each record" },
            FieldInfo { name: "num_sectors", type_name: "usize", description: "Number of sectors on the drum-like device" },
            FieldInfo { name: "bound", type_name: "f64", description: "Upper bound K on the expected retrieval cost" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "ExpectedRetrievalCost",
        fields: &["num_records", "num_sectors"],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedRetrievalCost {
    probabilities: Vec<f64>,
    num_sectors: usize,
    bound: f64,
}

impl ExpectedRetrievalCost {
    pub fn new(probabilities: Vec<f64>, num_sectors: usize, bound: f64) -> Self {
        assert!(
            !probabilities.is_empty(),
            "ExpectedRetrievalCost requires at least one record"
        );
        assert!(
            num_sectors >= 2,
            "ExpectedRetrievalCost requires at least two sectors"
        );
        assert!(bound.is_finite(), "bound must be finite");
        assert!(bound >= 0.0, "bound must be non-negative");
        for &probability in &probabilities {
            assert!(
                probability.is_finite(),
                "probabilities must be finite real numbers"
            );
            assert!(
                (0.0..=1.0).contains(&probability),
                "probabilities must lie in [0, 1]"
            );
        }
        let total_probability: f64 = probabilities.iter().sum();
        assert!(
            (total_probability - 1.0).abs() <= FLOAT_TOLERANCE,
            "probabilities must sum to 1.0"
        );
        Self {
            probabilities,
            num_sectors,
            bound,
        }
    }

    pub fn probabilities(&self) -> &[f64] {
        &self.probabilities
    }

    pub fn num_records(&self) -> usize {
        self.probabilities.len()
    }

    pub fn num_sectors(&self) -> usize {
        self.num_sectors
    }

    pub fn bound(&self) -> f64 {
        self.bound
    }

    pub fn sector_masses(&self, config: &[usize]) -> Option<Vec<f64>> {
        if config.len() != self.num_records() {
            return None;
        }

        let mut masses = vec![0.0; self.num_sectors];
        for (record, &sector) in config.iter().enumerate() {
            if sector >= self.num_sectors {
                return None;
            }
            masses[sector] += self.probabilities[record];
        }
        Some(masses)
    }

    pub fn expected_cost(&self, config: &[usize]) -> Option<f64> {
        let masses = self.sector_masses(config)?;
        let mut total = 0.0;
        for source in 0..self.num_sectors {
            for target in 0..self.num_sectors {
                total += masses[source]
                    * masses[target]
                    * latency_distance(self.num_sectors, source, target) as f64;
            }
        }
        Some(total)
    }

    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.expected_cost(config)
            .is_some_and(|cost| cost <= self.bound + FLOAT_TOLERANCE)
    }
}

impl Problem for ExpectedRetrievalCost {
    const NAME: &'static str = "ExpectedRetrievalCost";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_sectors; self.num_records()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(self.is_valid_solution(config))
    }
}

fn latency_distance(num_sectors: usize, source: usize, target: usize) -> usize {
    if source < target {
        target - source - 1
    } else {
        num_sectors - source + target - 1
    }
}

crate::declare_variants! {
    default ExpectedRetrievalCost => "num_sectors ^ num_records",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "expected_retrieval_cost",
        instance: Box::new(ExpectedRetrievalCost::new(
            vec![0.2, 0.15, 0.15, 0.2, 0.1, 0.2],
            3,
            1.01,
        )),
        optimal_config: vec![0, 1, 2, 1, 0, 2],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/expected_retrieval_cost.rs"]
mod tests;
