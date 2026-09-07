//! Reduction from MultipleChoiceBranching with integer weights to integer ILP.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MultipleChoiceBranching;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionMultipleChoiceBranchingToILP {
    target: ILP<i64>,
    num_arcs: usize,
}

impl ReductionResult for ReductionMultipleChoiceBranchingToILP {
    type Source = MultipleChoiceBranching<i64>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        Ok(target_solution[..self.num_arcs]
            .iter()
            .map(|&selected| selected == 1)
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_arcs + num_vertices",
        num_constraints = "2 * num_arcs + 2 * num_vertices + num_partition_groups + 1",
    },
    unavailable = {
        num_nonzeros = "zero weights and loop normalization determine the exact nonzero count",
    }
)]
impl ReduceTo<ILP<i64>> for MultipleChoiceBranching<i64> {
    type Result = ReductionMultipleChoiceBranchingToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_arcs = self.num_arcs();
        let num_vertices = self.num_vertices();
        let order = |vertex: usize| num_arcs + vertex;
        let mut constraints = Vec::new();

        for arc in 0..num_arcs {
            constraints.push(LinearConstraint::le(vec![(arc, 1)], 1));
        }
        if num_vertices > 0 {
            let big_m = <Self as ReduceTo<ILP<i64>>>::exact_i64(
                num_vertices,
                "encoding topological-order constraints",
            )?;
            for vertex in 0..num_vertices {
                constraints.push(LinearConstraint::le(vec![(order(vertex), 1)], big_m - 1));
            }
            for (arc, &(source, target)) in self.graph().arcs().iter().enumerate() {
                constraints.push(LinearConstraint::le(
                    vec![(order(source), 1), (order(target), -1), (arc, big_m)],
                    big_m - 1,
                ));
            }
        }
        for group in self.partition() {
            constraints.push(LinearConstraint::le(
                group.iter().map(|&arc| (arc, 1)).collect(),
                1,
            ));
        }
        for vertex in 0..num_vertices {
            constraints.push(LinearConstraint::le(
                self.graph()
                    .arcs()
                    .iter()
                    .enumerate()
                    .filter_map(|(arc, &(_, target))| (target == vertex).then_some((arc, 1)))
                    .collect(),
                1,
            ));
        }
        constraints.push(LinearConstraint::ge(
            self.weights()
                .iter()
                .enumerate()
                .map(|(arc, &weight)| (arc, weight))
                .collect(),
            *self.threshold(),
        ));

        let target = ILP::new(
            num_arcs + num_vertices,
            constraints,
            vec![],
            ObjectiveSense::Minimize,
        )
        .map_err(<Self as ReduceTo<ILP<i64>>>::target_construction)?;
        Ok(ReductionMultipleChoiceBranchingToILP { target, num_arcs })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "multiplechoicebranching_to_ilp",
        build: || {
            let source = MultipleChoiceBranching::new(
                DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![2, 3],
                vec![vec![0], vec![1]],
                5,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/multiplechoicebranching_ilp.rs"]
mod tests;
