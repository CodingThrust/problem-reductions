//! Expected Retrieval Cost problem implementation.
//!
//! Given record access probabilities, find an assignment of records to circular
//! storage sectors that minimizes the expected rotational latency.

use crate::registry::{ConstructionError, FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

const FLOAT_TOLERANCE: f64 = 1e-9;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ExpectedRetrievalCost",
        display_name: "Expected Retrieval Cost",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Assign records to circular storage sectors to minimize expected retrieval latency",
        fields: &[
            FieldInfo { name: "probabilities", type_name: "Vec<f64>", description: "Access probabilities p(r) for each record" },
            FieldInfo { name: "num_sectors", type_name: "usize", description: "Number of sectors on the drum-like device" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "ExpectedRetrievalCost",
        fields: &["num_records", "num_sectors"],
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ExpectedRetrievalCost {
    probabilities: Vec<f64>,
    num_sectors: usize,
}

impl ExpectedRetrievalCost {
    pub fn new(probabilities: Vec<f64>, num_sectors: usize) -> Result<Self, ConstructionError> {
        if probabilities.is_empty() {
            return Err(ConstructionError::Conversion(
                "ExpectedRetrievalCost requires at least one record".into(),
            ));
        }
        if num_sectors < 2 {
            return Err(ConstructionError::Conversion(
                "ExpectedRetrievalCost requires at least two sectors".into(),
            ));
        }
        for (index, &probability) in probabilities.iter().enumerate() {
            if !probability.is_finite() {
                return Err(ConstructionError::NonFiniteFloat(format!(
                    "probability at index {index} must be finite"
                )));
            }
            if !(0.0..=1.0).contains(&probability) {
                return Err(ConstructionError::Conversion(format!(
                    "probability at index {index} must lie in [0, 1]"
                )));
            }
        }
        let total_probability: f64 = probabilities.iter().sum();
        if !total_probability.is_finite() || (total_probability - 1.0).abs() > FLOAT_TOLERANCE {
            if !total_probability.is_finite() {
                return Err(ConstructionError::NonFiniteFloat(
                    "summing probabilities produced a non-finite value".into(),
                ));
            }
            return Err(ConstructionError::Conversion(
                "probabilities must sum to 1.0".into(),
            ));
        }
        Ok(Self {
            probabilities,
            num_sectors,
        })
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

    pub fn sector_masses(
        &self,
        config: &[usize],
    ) -> Result<Option<Vec<f64>>, crate::traits::EvaluationError> {
        if config.len() != self.num_records() {
            return Ok(None);
        }

        let mut masses = vec![0.0; self.num_sectors];
        for (record, &sector) in config.iter().enumerate() {
            if sector >= self.num_sectors {
                return Ok(None);
            }
            let mass = masses[sector] + self.probabilities[record];
            if !mass.is_finite() {
                return Err(crate::traits::EvaluationError::NonFiniteResult(
                    "summing expected-retrieval sector probabilities".to_string(),
                ));
            }
            masses[sector] = mass;
        }
        Ok(Some(masses))
    }

    pub fn expected_cost(
        &self,
        config: &[usize],
    ) -> Result<Option<f64>, crate::traits::EvaluationError> {
        let Some(masses) = self.sector_masses(config)? else {
            return Ok(None);
        };
        let mut total = 0.0;
        for source in 0..self.num_sectors {
            for target in 0..self.num_sectors {
                let latency = i64::try_from(latency_distance(self.num_sectors, source, target))
                    .map_err(|_| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "converting expected-retrieval latency to i64".to_string(),
                        )
                    })?;
                let latency = crate::types::i64_to_exact_f64(latency).map_err(|_| {
                    crate::traits::EvaluationError::InexactFloatConversion(
                        "converting expected-retrieval latency to f64".to_string(),
                    )
                })?;
                let term = masses[source] * masses[target] * latency;
                let next = total + term;
                if !term.is_finite() || !next.is_finite() {
                    return Err(crate::traits::EvaluationError::NonFiniteResult(
                        "computing expected retrieval cost".to_string(),
                    ));
                }
                total = next;
            }
        }
        Ok(Some(total))
    }

    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(self.expected_cost(config)?.is_some())
    }
}

#[derive(Deserialize)]
struct ExpectedRetrievalCostData {
    probabilities: Vec<f64>,
    num_sectors: usize,
}

impl<'de> Deserialize<'de> for ExpectedRetrievalCost {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = ExpectedRetrievalCostData::deserialize(deserializer)?;
        Self::new(data.probabilities, data.num_sectors).map_err(serde::de::Error::custom)
    }
}

impl Problem for ExpectedRetrievalCost {
    const NAME: &'static str = "ExpectedRetrievalCost";
    type Value = Min<f64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_sectors; self.num_records()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<f64>, crate::traits::EvaluationError> {
        Ok({
            match self.expected_cost(config)? {
                Some(cost) => Min(Some(cost)),
                None => Min(None),
            }
        })
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
        instance: Box::new(
            ExpectedRetrievalCost::new(vec![0.2, 0.15, 0.15, 0.2, 0.1, 0.2], 3).unwrap(),
        ),
        optimal_config: vec![0, 1, 2, 1, 0, 2],
        optimal_value: serde_json::json!(1.0025),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/expected_retrieval_cost.rs"]
mod tests;
