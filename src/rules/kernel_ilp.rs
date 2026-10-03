//! Binary independence and absorption constraints for a directed graph kernel.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::Kernel;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionKernelToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionKernelToILP {
    type Source = Kernel;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.value.is_some(),
            "target ILP assignment does not select a kernel",
        )?;
        Ok(solution.iter().map(|&value| value == 1).collect())
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionKernelToILP {}

#[reduction(transform = {
    exact {
        num_vars = "num_vertices",
        num_constraints = "num_arcs + num_vertices",
    },
    upper_bound {
        num_nonzeros = "3 * num_arcs + num_vertices",
        max_constraint_magnitude_bits = "2",
    },
})]
impl ReduceTo<ILP<bool>> for Kernel {
    type Result = ReductionKernelToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let mut constraints = Vec::new();
        for (u, v) in self.graph().arcs() {
            constraints.push(LinearConstraint::le(vec![(u, 1), (v, 1)], 1));
        }
        for u in 0..self.num_vertices() {
            let mut successors = self.graph().successors(u);
            successors.sort_unstable();
            successors.dedup();
            let mut terms = vec![(u, 1)];
            terms.extend(successors.into_iter().map(|v| (v, 1)));
            constraints.push(LinearConstraint::ge(terms, 1));
        }
        let target = ILP::new(
            self.num_vertices(),
            constraints,
            vec![],
            ObjectiveSense::Minimize,
        )
        .map_err(Self::target_construction)?;
        Ok(Self::Result { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kernel_to_ilp",
        build: || {
            let source = Kernel::new(DirectedGraph::new(3, vec![(0, 1), (1, 2)]));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kernel_ilp.rs"]
mod tests;
