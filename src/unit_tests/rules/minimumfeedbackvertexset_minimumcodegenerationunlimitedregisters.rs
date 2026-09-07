use super::{issue_example_source, ReductionFVSToCodeGen};
use crate::models::misc::MinimumCodeGenerationUnlimitedRegisters;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;

#[test]
fn test_minimumfeedbackvertexset_to_minimumcodegenerationunlimitedregisters_closed_loop() {
    let source = issue_example_source();
    let reduction: ReductionFVSToCodeGen =
        ReduceTo::<MinimumCodeGenerationUnlimitedRegisters>::reduce_to(&source)
            .expect("reduction should succeed");

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumFeedbackVertexSet -> MinimumCodeGenerationUnlimitedRegisters closed loop",
    );
}

#[test]
fn test_codegen_rule_is_registered_only_for_unit_weight() {
    let entries: Vec<_> = crate::rules::registry::reduction_entries()
        .into_iter()
        .filter(|e| {
            e.source_name == "MinimumFeedbackVertexSet"
                && e.target_name == "MinimumCodeGenerationUnlimitedRegisters"
        })
        .collect();
    assert_eq!(entries.len(), 1);
    assert_eq!((entries[0].source_variant_fn)(), vec![("weight", "One")]);
}

#[test]
fn test_codegen_start_nodes_cover_self_loops_and_parallel_arcs() {
    use crate::models::graph::MinimumFeedbackVertexSet;
    use crate::rules::ReductionResult;
    use crate::topology::DirectedGraph;
    use crate::traits::Problem;
    use crate::types::{Min, One};
    let source = MinimumFeedbackVertexSet::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 0), (2, 2), (2, 2)]),
        vec![One; 3],
    );
    let reduction =
        ReduceTo::<MinimumCodeGenerationUnlimitedRegisters>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    assert_eq!(target.num_vertices(), 11);
    assert_eq!(target.num_internal(), 7);
    let config = (0..7).collect();
    assert_eq!(target.evaluate(&config).unwrap(), Min(Some(9)));
    let removed = reduction.extract_solution(&config).unwrap();
    assert_eq!(removed, vec![true, false, true]);
    assert_eq!(source.evaluate(&removed).unwrap(), Min(Some(2)));
}

#[test]
fn test_codegen_empty_graph_and_invalid_orders() {
    use crate::models::graph::MinimumFeedbackVertexSet;
    use crate::rules::ReductionResult;
    use crate::topology::DirectedGraph;
    use crate::types::One;
    let empty = MinimumFeedbackVertexSet::<One>::new(DirectedGraph::new(0, vec![]), vec![]);
    let reduction = ReduceTo::<MinimumCodeGenerationUnlimitedRegisters>::reduce_to(&empty).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 1);
    assert_eq!(
        reduction.extract_solution(&vec![]).unwrap(),
        Vec::<bool>::new()
    );
    let source = issue_example_source();
    let reduction =
        ReduceTo::<MinimumCodeGenerationUnlimitedRegisters>::reduce_to(&source).unwrap();
    for config in [vec![], vec![9; 6], vec![0; 6], vec![1, 0, 2, 3, 4, 5]] {
        assert!(reduction.extract_solution(&config).is_err());
    }
}

#[test]
fn test_codegen_every_small_evaluation_permutation() {
    use crate::models::graph::MinimumFeedbackVertexSet;
    use crate::rules::ReductionResult;
    use crate::solvers::BruteForce;
    use crate::topology::DirectedGraph;
    use crate::traits::Problem;
    use crate::types::{Min, One};
    fn visit(config: &mut [usize], offset: usize, check: &mut impl FnMut(&[usize])) {
        if offset == config.len() {
            check(config);
        } else {
            for i in offset..config.len() {
                config.swap(offset, i);
                visit(config, offset + 1, check);
                config.swap(offset, i);
            }
        }
    }
    // All directed graphs on <=3 vertices with <=3 arcs, including loops.
    // Every permutation is tested, not just orders grouping whole chains.
    for n in 0..=3 {
        let pairs: Vec<_> = (0..n).flat_map(|u| (0..n).map(move |v| (u, v))).collect();
        for mask in 0u32..(1 << pairs.len()) {
            if mask.count_ones() > 3 {
                continue;
            }
            let arcs: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter_map(|(i, &arc)| (mask & (1 << i) != 0).then_some(arc))
                .collect();
            let source = MinimumFeedbackVertexSet::new(DirectedGraph::new(n, arcs), vec![One; n]);
            let witness = BruteForce::new().solve(&source).unwrap().unwrap();
            let Min(Some(optimum)) = source.evaluate(&witness).unwrap() else {
                unreachable!()
            };
            let reduction =
                ReduceTo::<MinimumCodeGenerationUnlimitedRegisters>::reduce_to(&source).unwrap();
            let target = reduction.target_problem();
            let operations = target.num_internal() as i64;
            let mut best = i64::MAX;
            visit(
                &mut (0..target.num_internal()).collect::<Vec<_>>(),
                0,
                &mut |p| {
                    let config = p.to_vec();
                    if let Min(Some(cost)) = target.evaluate(&config).unwrap() {
                        let removed = reduction.extract_solution(&config).unwrap();
                        let Min(Some(size)) = source.evaluate(&removed).unwrap() else {
                            panic!("every valid target order must extract an FVS");
                        };
                        assert_eq!(cost, operations + size);
                        assert!(size >= optimum);
                        best = best.min(cost);
                    }
                },
            );
            assert_eq!(best, operations + optimum);
        }
    }
}

#[test]
fn test_codegen_vertex_count_representable_boundaries() {
    use super::code_generation_vertex_count;
    // Count arithmetic is independent of graph allocation. These maximum-size
    // cases exercise the actual preallocation calculation without building a graph.
    let half = usize::MAX / 2;
    for (n, m, expected) in [
        (0, 0, 1),
        (3, 3, 10),
        (half - 1, 1, usize::MAX - 1),
        (half, 0, usize::MAX),
        (1, usize::MAX - 3, usize::MAX),
    ] {
        assert_eq!(code_generation_vertex_count(n, m), Ok(expected));
    }
}

#[test]
fn test_codegen_vertex_count_rejects_each_overflow_stage() {
    use super::code_generation_vertex_count;
    use crate::rules::ReductionError;
    let half = usize::MAX / 2;
    for (n, m) in [
        (half + 1, 0),       // doubling the source vertex count
        (1, usize::MAX - 1), // adding the arc count
        (half, 1),           // adding the dummy leaf
    ] {
        assert_eq!(
            code_generation_vertex_count(n, m),
            Err(ReductionError::IntegerOverflow {
                source_problem: "MinimumFeedbackVertexSet",
                target_problem: "MinimumCodeGenerationUnlimitedRegisters",
                operation: "counting code-generation start and arc nodes".to_string(),
            })
        );
    }
}
