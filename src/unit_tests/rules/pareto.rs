use super::*;
use crate::expr::Expr;
use crate::rules::registry::{ReductionSizeContract, ReductionSizeDeclarations};
use crate::rules::{ReductionGraph, ReductionMode};
use crate::types::ProblemSize;
use std::collections::BTreeMap;

fn edge(formula: &str) -> ReductionEdgeData {
    ReductionEdgeData {
        size_contract: ReductionSizeContract::new(
            "synthetic edge",
            ReductionSizeDeclarations {
                exact: vec![("x", Expr::try_parse(formula).unwrap())],
                bounds: vec![],
                unavailable: vec![],
            },
        ),
        reduce_fn: Some(|_| panic!("symbolic search must not execute reductions")),
        reduce_aggregate_fn: None,
        turing: false,
    }
}

#[test]
fn exact_search_applies_dominance_only_at_the_terminal_problem() {
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "B", "T"],
        &[
            ("S", "A", edge("1")),
            ("S", "B", edge("2")),
            ("A", "T", edge("x + 10")),
            ("B", "T", edge("x")),
        ],
    );
    let empty = BTreeMap::new();
    let result = graph
        .exact_size_front(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            &ProblemSize::new(vec![("x", 0)]),
        )
        .unwrap();

    assert_eq!(result.front.len(), 1);
    assert_eq!(result.front[0].path.type_names(), ["S", "B", "T"]);
    assert_eq!(result.front[0].terminal_size.get("x"), Some(2));
}
