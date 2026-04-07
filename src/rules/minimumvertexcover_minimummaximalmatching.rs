//! Forward-only reduction from MinimumVertexCover (unit-weight) to
//! MinimumMaximalMatching.
//!
//! The construction is the identity map on the underlying graph. This edge is
//! registered for topology and documentation purposes only: it intentionally has
//! no witness, aggregate, or Turing execution capability because an optimal
//! maximal matching does not determine an optimal vertex cover in general
//! (for example, on `C5`, `mmm(G) = 2` but `mvc(G) = 3`).

use crate::models::graph::{MinimumMaximalMatching, MinimumVertexCover};
use crate::rules::{EdgeCapabilities, ReductionEntry, ReductionOverhead};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{One, ProblemSize};
use std::any::Any;

fn source_problem_size(any: &dyn Any) -> ProblemSize {
    let source = any
        .downcast_ref::<MinimumVertexCover<SimpleGraph, One>>()
        .expect("MinimumVertexCover -> MinimumMaximalMatching source type mismatch");
    ProblemSize::new(vec![
        ("num_vertices", source.num_vertices()),
        ("num_edges", source.num_edges()),
    ])
}

inventory::submit! {
    ReductionEntry {
        source_name: MinimumVertexCover::<SimpleGraph, One>::NAME,
        target_name: MinimumMaximalMatching::<SimpleGraph>::NAME,
        source_variant_fn: <MinimumVertexCover<SimpleGraph, One> as Problem>::variant,
        target_variant_fn: <MinimumMaximalMatching<SimpleGraph> as Problem>::variant,
        overhead_fn: || ReductionOverhead::identity(&["num_vertices", "num_edges"]),
        module_path: module_path!(),
        reduce_fn: None,
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::none(),
        overhead_eval_fn: source_problem_size,
        source_size_fn: source_problem_size,
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
                    source_config: vec![1, 1, 0, 1, 0],
                    target_config: vec![1, 0, 1, 0, 0],
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_minimummaximalmatching.rs"]
mod tests;
