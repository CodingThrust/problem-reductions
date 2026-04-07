//! Reduction from MonochromaticTriangle to ILP.
//!
//! Use one binary variable per edge color. Every triangle must use both colors,
//! encoded as the pair of inequalities `1 <= sum <= 2` over its three incident
//! edge variables.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MonochromaticTriangle;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing MonochromaticTriangle to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMonochromaticTriangleToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionMonochromaticTriangleToILP {
    type Source = MonochromaticTriangle<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_edges",
        num_constraints = "2 * num_triangles",
    }
)]
impl ReduceTo<ILP<bool>> for MonochromaticTriangle<SimpleGraph> {
    type Result = ReductionMonochromaticTriangleToILP;

    fn reduce_to(&self) -> Self::Result {
        let mut constraints = Vec::with_capacity(2 * self.num_triangles());
        for triangle in self.triangles() {
            let terms: Vec<(usize, f64)> =
                triangle.iter().map(|&edge_idx| (edge_idx, 1.0)).collect();
            constraints.push(LinearConstraint::ge(terms.clone(), 1.0));
            constraints.push(LinearConstraint::le(terms, 2.0));
        }

        ReductionMonochromaticTriangleToILP {
            target: ILP::new(
                self.num_edges(),
                constraints,
                vec![],
                ObjectiveSense::Minimize,
            ),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::SimpleGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "monochromatictriangle_to_ilp",
        build: || {
            let source = MonochromaticTriangle::new(SimpleGraph::new(
                4,
                vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/monochromatictriangle_ilp.rs"]
mod tests;
