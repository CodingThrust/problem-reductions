//! Exact cover of indexed items by triples whose sizes sum to the bound.
use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::ThreePartition;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionThreePartitionToILP {
    target: ILP<bool>,
    triples: Vec<[usize; 3]>,
    num_elements: usize,
}

impl ReductionResult for ReductionThreePartitionToILP {
    type Source = ThreePartition;
    type Target = ILP<bool>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        let mut groups = vec![0; self.num_elements];
        for (group, triple) in self
            .triples
            .iter()
            .zip(solution)
            .filter(|(_, selected)| **selected == 1)
            .map(|(triple, _)| triple)
            .enumerate()
        {
            for &item in triple {
                groups[item] = group;
            }
        }
        Ok(groups)
    }
}
#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionThreePartitionToILP {}

#[crate::reduction(transform = {
    exact {
        num_constraints = "num_elements",
        max_constraint_magnitude_bits = "1",
    },
    upper_bound {
        num_vars = "num_elements * (num_elements - 1) * (num_elements - 2) / 6",
        num_nonzeros = "num_elements * (num_elements - 1) * (num_elements - 2) / 2",
    },
})]
impl ReduceTo<ILP<bool>> for ThreePartition {
    type Result = ReductionThreePartitionToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let mut indices = std::collections::BTreeMap::<i64, Vec<usize>>::new();
        for (item, &size) in self.sizes().iter().enumerate() {
            indices.entry(size).or_default().push(item);
        }
        let mut triples = Vec::new();
        let mut rows = vec![Vec::new(); self.num_elements()];
        for (i, &a) in self.sizes().iter().enumerate() {
            for (j, &b) in self.sizes().iter().enumerate().skip(i + 1) {
                // Classical input bounds imply a+b < bound, without overflow.
                if let Some(candidates) = indices.get(&(self.bound() - a - b)) {
                    for &k in &candidates[candidates.partition_point(|&k| k <= j)..] {
                        for item in [i, j, k] {
                            rows[item].push((triples.len(), 1));
                        }
                        triples.push([i, j, k]);
                    }
                }
            }
        }
        let target: ILP<bool> = ILP::new(
            triples.len(),
            rows.into_iter()
                .map(|terms| LinearConstraint::eq(terms, 1))
                .collect(),
            vec![],
            ObjectiveSense::Minimize,
        )
        .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;
        Ok(Self::Result {
            target,
            triples,
            num_elements: self.num_elements(),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threepartition_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(ThreePartition::new(
                vec![4, 5, 6, 4, 6, 5],
                15,
            ))
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threepartition_ilp.rs"]
mod tests;
