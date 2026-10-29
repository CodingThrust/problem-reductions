//! Exact cover by multiplicities of eligible value triples.
use std::collections::BTreeMap;

use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::ThreePartition;
use crate::rules::ilp_i64_ilp_bool::binary_weights;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionError, ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
struct TripleMultiplicity {
    values: [usize; 3],
    offset: usize,
    weights: Vec<i64>,
}
#[derive(Debug, Clone)]
pub struct ReductionThreePartitionToQUBO {
    target: Decision<QUBO<i64>>,
    indices: Vec<Vec<usize>>,
    triples: Vec<TripleMultiplicity>,
    num_items: usize,
}
impl ReductionResult for ReductionThreePartitionToQUBO {
    type Source = ThreePartition;
    type Target = Decision<QUBO<i64>>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(
        &self,
        bits: &Vec<bool>,
    ) -> ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        validate_target_witness(
            &self.target,
            bits,
            |v| v.0,
            "QUBO does not certify a source witness",
        )?;
        let mut used = vec![0; self.indices.len()];
        let mut groups = vec![0; self.num_items];
        let mut group = 0;
        for triple in &self.triples {
            let count: i64 = triple
                .weights
                .iter()
                .enumerate()
                .filter(|(bit, _)| bits[triple.offset + bit])
                .map(|(_, &weight)| weight)
                .sum();
            let count = usize::try_from(count)
                .map_err(|_| ExtractionError::invalid("triple multiplicity does not fit usize"))?;
            for _ in 0..count {
                for &value in &triple.values {
                    let &index = self.indices[value].get(used[value]).ok_or_else(|| {
                        ExtractionError::invalid("triple multiplicities exceed an item count")
                    })?;
                    groups[index] = group;
                    used[value] += 1;
                }
                group += 1;
            }
        }
        Ok(groups)
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionThreePartitionToQUBO {}
#[crate::reduction(transform = upper_bound {
    num_vars = "num_elements * (num_elements - 1) / 6",
    num_quadratic_terms = "num_elements * (num_elements - 1) * (num_elements - 2) / 8",
})]
impl ReduceTo<Decision<QUBO<i64>>> for ThreePartition {
    type Result = ReductionThreePartitionToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<ThreePartition, Decision<QUBO<i64>>>;
        let mut by_value: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
        for (index, &size) in self.sizes().iter().enumerate() {
            by_value.entry(size).or_default().push(index);
        }
        let values: Vec<_> = by_value.keys().copied().collect();
        let indices: Vec<_> = by_value.into_values().collect();
        let counts: Vec<_> = indices
            .iter()
            .map(|items| {
                <Self as ReduceTo<Decision<QUBO<i64>>>>::exact_i64(
                    items.len(),
                    "counting items of one size",
                )
            })
            .collect::<Result<_, _>>()?;
        let occurrences =
            |triple: &[usize; 3], value| triple.iter().map(|&v| i64::from(v == value)).sum::<i64>();
        let mut triples = Vec::new();
        let mut num_vars = 0usize;
        for (a, &first) in values.iter().enumerate() {
            for (b, &second) in values.iter().enumerate().skip(a) {
                let third = self
                    .bound()
                    .checked_sub(first)
                    .and_then(|v| v.checked_sub(second))
                    .ok_or_else(Builder::overflow)?;
                if third < second {
                    continue;
                }
                let Ok(c) = values.binary_search(&third) else {
                    continue;
                };
                let triple = [a, b, c];
                let capacities = triple.map(|value| counts[value] / occurrences(&triple, value));
                let capacity = capacities[0].min(capacities[1]).min(capacities[2]);
                if capacity == 0 {
                    continue;
                }
                let weights = binary_weights(capacity);
                let offset = num_vars;
                num_vars = num_vars
                    .checked_add(weights.len())
                    .ok_or_else(Builder::overflow)?;
                triples.push(TripleMultiplicity {
                    values: triple,
                    offset,
                    weights,
                });
            }
        }
        let mut rows = vec![Vec::new(); values.len()];
        for triple in &triples {
            for (position, &value) in triple.values.iter().enumerate() {
                if triple.values[..position].contains(&value) {
                    continue;
                }
                let repetitions = occurrences(&triple.values, value);
                for (bit, &weight) in triple.weights.iter().enumerate() {
                    rows[value].push((
                        triple.offset + bit,
                        weight
                            .checked_mul(repetitions)
                            .ok_or_else(Builder::overflow)?,
                    ));
                }
            }
        }
        let mut q = Builder::new(num_vars);
        for (row, &count) in rows.iter().zip(&counts) {
            q.affine_square(row, count)?;
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(ReductionThreePartitionToQUBO {
            target: Decision::new(target, bound),
            indices,
            triples,
            num_items: self.num_elements(),
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threepartition_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                ThreePartition::new(vec![2, 2, 2], 6),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 0, 0]),
                    target_config: serde_json::json!([true]),
                },
            )
        },
    }]
}
