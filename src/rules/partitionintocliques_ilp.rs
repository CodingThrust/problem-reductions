//! Reduction from PartitionIntoCliques to binary ILP.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::PartitionIntoCliques;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

#[derive(Debug, Clone)]
pub struct ReductionPartitionIntoCliquesToILP {
    target: ILP<bool>,
    num_vertices: usize,
    num_cliques: usize,
}

impl ReductionResult for ReductionPartitionIntoCliquesToILP {
    type Source = PartitionIntoCliques<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        (0..self.num_vertices)
            .map(|vertex| {
                (0..self.num_cliques)
                    .find(|&clique| target_solution[vertex * self.num_cliques + clique] == 1)
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(format!(
                            "target solution does not assign vertex {vertex} to a clique"
                        ))
                    })
            })
            .collect()
    }
}

#[reduction(
    transform = upper_bound {
        num_vars = "num_vertices^2",
        num_constraints = "num_vertices + num_vertices^3",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter depends on the source clique bound and non-edges",
    }
)]
impl ReduceTo<ILP<bool>> for PartitionIntoCliques<SimpleGraph> {
    type Result = ReductionPartitionIntoCliquesToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vertices = self.num_vertices();
        let num_cliques = self.num_cliques();
        let num_vars = num_vertices * num_cliques;
        let variable = |vertex: usize, clique: usize| vertex * num_cliques + clique;
        let mut constraints = Vec::new();

        for vertex in 0..num_vertices {
            constraints.push(LinearConstraint::eq(
                (0..num_cliques)
                    .map(|clique| (variable(vertex, clique), 1))
                    .collect(),
                1,
            ));
        }

        for u in 0..num_vertices {
            for v in (u + 1)..num_vertices {
                if !self.graph().has_edge(u, v) {
                    for clique in 0..num_cliques {
                        constraints.push(LinearConstraint::le(
                            vec![(variable(u, clique), 1), (variable(v, clique), 1)],
                            1,
                        ));
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;

        Ok(ReductionPartitionIntoCliquesToILP {
            target,
            num_vertices,
            num_cliques,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partitionintocliques_to_ilp",
        build: || {
            let source = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1)]), 2);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 0, 1]),
                    target_config: serde_json::json!(vec![1, 0, 1, 0, 0, 1]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partitionintocliques_ilp.rs"]
mod tests;
