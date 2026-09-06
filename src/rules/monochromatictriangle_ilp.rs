//! Reduction from MonochromaticTriangle to ILP.
//!
//! Use one binary variable per edge color. Every triangle must use both colors,
//! encoded as the pair of inequalities `1 <= sum <= 2` over its three incident
//! edge variables.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MonochromaticTriangle;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use std::collections::HashMap;

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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&value| value == 1).collect())
    }
}

#[reduction(
    transform = upper_bound {
        num_vars = "num_edges",
        num_constraints = "2 * num_triangles + num_vertices^5 / 8",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MonochromaticTriangle<SimpleGraph> {
    type Result = ReductionMonochromaticTriangleToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let mut constraints = Vec::with_capacity(2 * self.num_triangles());
        for triangle in self.triangles() {
            let terms: Vec<(usize, i64)> = triangle.iter().map(|&edge_idx| (edge_idx, 1)).collect();
            constraints.push(LinearConstraint::ge(terms.clone(), 1));
            constraints.push(LinearConstraint::le(terms, 2));
        }

        // Every triangle-free two-colouring of K5 consists of two C5s.
        // Hence each vertex has exactly two incident colour-1 edges in every
        // K5. These valid equalities strengthen the LP relaxation for all
        // inputs, independently of how a graph was constructed.
        let graph = self.graph();
        let edge_list = self.edge_list();
        let edge_indices: HashMap<_, _> = edge_list
            .iter()
            .enumerate()
            .map(|(index, &(u, v))| ((u.min(v), u.max(v)), index))
            .collect();
        for triangle in self.triangles() {
            let mut vertices: Vec<_> = triangle
                .iter()
                .flat_map(|&edge| [edge_list[edge].0, edge_list[edge].1])
                .collect();
            vertices.sort_unstable();
            vertices.dedup();
            let [a, b, c] = [vertices[0], vertices[1], vertices[2]];
            let mut common: Vec<_> = graph
                .neighbors(c)
                .into_iter()
                .filter(|&v| !vertices.contains(&v) && graph.has_edge(a, v) && graph.has_edge(b, v))
                .collect();
            common.sort_unstable();
            common.dedup();
            // In a K5, the edge opposite this triangle has its majority
            // colour. All such opposite edges therefore have equal colours.
            // State these consequences directly so presolve can substitute
            // colour copies instead of rediscovering the implication by MIP.
            let mut first_opposite = None;
            for (index, &d) in common.iter().enumerate() {
                for &e in &common[index + 1..] {
                    if graph.has_edge(d, e) {
                        let opposite = edge_indices[&(d, e)];
                        if let Some(first) = first_opposite {
                            constraints
                                .push(LinearConstraint::eq(vec![(first, 1), (opposite, -1)], 0));
                        } else {
                            first_opposite = Some(opposite);
                        }
                        // Count each K5's degree equalities only once, using
                        // its three smallest vertices as the base triangle.
                        if d <= c {
                            continue;
                        }
                        let clique = [a, b, c, d, e];
                        for &u in &clique {
                            let terms = clique
                                .iter()
                                .filter(|&&v| v != u)
                                .map(|&v| (edge_indices[&(u.min(v), u.max(v))], 1))
                                .collect();
                            constraints.push(LinearConstraint::eq(terms, 2));
                        }
                    }
                }
            }
        }

        Ok(ReductionMonochromaticTriangleToILP {
            target: ILP::new(
                self.num_edges(),
                constraints,
                vec![],
                ObjectiveSense::Minimize,
            )
            .map_err(Self::target_construction)?,
        })
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
