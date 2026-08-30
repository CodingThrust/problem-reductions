//! Forward-only reduction from MinimumVertexCover (unit-weight) to
//! MinimumMaximalMatching.
//!
//! The construction is the identity map on the underlying graph. This edge is
//! registered for topology and documentation purposes only: it intentionally has
//! no witness, aggregate, or Turing execution capability because an optimal
//! maximal matching does not determine an optimal vertex cover in general
//! (for example, on `C5`, `mmm(G) = 2` but `mvc(G) = 3`).

use crate::models::graph::{MinimumMaximalMatching, MinimumVertexCover};
use crate::rules::registry::ReductionParameterDeclarations;
use crate::rules::ReductionEntry;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::One;

inventory::submit! {
    ReductionEntry {
        source_name: MinimumVertexCover::<SimpleGraph, One>::NAME,
        target_name: MinimumMaximalMatching::<SimpleGraph>::NAME,
        source_variant_fn: <MinimumVertexCover<SimpleGraph, One> as Problem>::variant,
        target_variant_fn: <MinimumMaximalMatching<SimpleGraph> as Problem>::variant,
        parameter_declarations_fn: || ReductionParameterDeclarations {
            relation: Some(crate::parameters::ParameterRelation::Exact),
            fields: vec![
                ("num_vertices", crate::expr::Expr::variable("num_vertices")),
                ("num_edges", crate::expr::Expr::variable("num_edges")),
            ],
            unavailable: vec![],
        },
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: None,
        turing: false,
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::assemble_rule_example;
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumvertexcover_to_minimummaximalmatching",
        build: || {
            let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
            let source = MinimumVertexCover::new(SimpleGraph::new(5, edges.clone()), vec![One; 5]);
            let target = MinimumMaximalMatching::new(SimpleGraph::new(5, edges));
            assemble_rule_example(
                &source,
                &target,
                vec![SolutionPair {
                    source_config: serde_json::json!(vec![true, true, false, true, false]),
                    target_config: serde_json::json!(vec![true, false, true, false, false]),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_minimummaximalmatching.rs"]
mod tests;
