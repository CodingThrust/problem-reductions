//! Reduction from unit-weight MaxCut to Maximum 2-Satisfiability (MAX-2-SAT).
//!
//! Each vertex `v` becomes the Boolean variable `x_v` (CNF literal `v + 1`);
//! its truth value is the side of the cut. Each edge `(u, v)`, in edge order,
//! becomes the two clauses `(x_u OR x_v)` and `(NOT x_u OR NOT x_v)`.
//!
//! When the endpoints are on different sides both clauses hold; when they are
//! on the same side exactly one does. Summing over the `m` edges gives, for
//! every assignment `x`,
//!     `satisfied(x) = m + cut(x)`,
//! so the maximizers coincide and an assignment is extracted unchanged.
//!
//! Reference: Gramm, Hirsch, Niedermeier and Rossmanith, "Worst-case upper
//! bounds for MAX-2-SAT with an application to MAX-CUT", Discrete Applied
//! Mathematics 130(2), 2003, Section 5.

use crate::models::formula::{CNFClause, Maximum2Satisfiability};
use crate::models::graph::MaxCut;
use crate::reduction;
use crate::rules::ReductionError;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

type Source = MaxCut<SimpleGraph, One>;

/// Result of reducing unit-weight MaxCut to Maximum2Satisfiability.
#[derive(Debug, Clone)]
pub struct ReductionMaxCutToMaximum2Satisfiability {
    target: Maximum2Satisfiability,
}

impl ReductionResult for ReductionMaxCutToMaximum2Satisfiability {
    type Source = Source;
    type Target = Maximum2Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: variable `x_v` is the side of vertex `v`.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.clone())
    }
}

/// The positive CNF literal `v + 1` of vertex `v`.
fn vertex_literal(vertex: usize) -> Result<i64, ReductionError> {
    vertex
        .checked_add(1)
        .and_then(|literal| i64::try_from(literal).ok())
        .ok_or_else(|| {
            ReductionError::integer_overflow::<Source, Maximum2Satisfiability>(
                "encoding a vertex as a CNF literal",
            )
        })
}

/// The number of target clauses, two per edge.
fn clause_count(num_edges: usize) -> Result<usize, ReductionError> {
    num_edges.checked_mul(2).ok_or_else(|| {
        ReductionError::integer_overflow::<Source, Maximum2Satisfiability>(
            "counting two clauses per edge",
        )
    })
}

#[reduction(
    transform = exact {
        num_vars = "num_vertices",
        num_clauses = "2 * num_edges",
    }
)]
impl ReduceTo<Maximum2Satisfiability> for MaxCut<SimpleGraph, One> {
    type Result = ReductionMaxCutToMaximum2Satisfiability;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let edges = self.graph().edges();
        let mut clauses = Vec::with_capacity(clause_count(edges.len())?);
        for (u, v) in edges {
            let (lu, lv) = (vertex_literal(u)?, vertex_literal(v)?);
            clauses.push(CNFClause::new(vec![lu, lv]));
            clauses.push(CNFClause::new(vec![-lu, -lv]));
        }
        let target = Maximum2Satisfiability::try_new(self.num_vertices(), clauses)
            .map_err(<Self as ReduceTo<Maximum2Satisfiability>>::target_construction)?;
        Ok(ReductionMaxCutToMaximum2Satisfiability { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maxcut_to_maximum2satisfiability",
        build: || {
            // The 4-cycle 0-1-2-3-0 plus the diagonal {0, 2}.
            let source = Source::unweighted(SimpleGraph::new(
                4,
                vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)],
            ));
            crate::example_db::specs::rule_example_with_witness::<_, Maximum2Satisfiability>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, false, true]),
                    target_config: serde_json::json!(vec![false, true, false, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maxcut_maximum2satisfiability.rs"]
mod tests;
