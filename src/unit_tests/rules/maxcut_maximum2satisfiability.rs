use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Max;

fn four_cycle_with_diagonal() -> MaxCut<SimpleGraph, One> {
    MaxCut::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)]),
        vec![One; 5],
    )
}

fn assert_affine_identity(source: &MaxCut<SimpleGraph, One>) {
    let reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), source.num_vertices());
    assert_eq!(target.num_clauses(), 2 * source.num_edges());

    for mask in 0..(1usize << source.num_vertices()) {
        let config: Vec<usize> = (0..source.num_vertices())
            .map(|bit| (mask >> bit) & 1)
            .collect();
        let source_value = source.evaluate(&config).unwrap();
        let target_value = target.evaluate(&config).unwrap();
        assert_eq!(target_value, source.num_edges() + source_value as usize);
        assert_eq!(reduction.extract_solution(&config), config);
    }
}

#[test]
fn test_maxcut_to_maximum2satisfiability_closed_loop() {
    let source = four_cycle_with_diagonal();
    let reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&source);
    let solver = BruteForce::new();

    assert_eq!(solver.solve(&source), Max(Some(4)));
    assert_eq!(solver.solve(reduction.target_problem()), Max(Some(9)));
    for target_solution in solver.find_all_witnesses(reduction.target_problem()) {
        let source_solution = reduction.extract_solution(&target_solution);
        assert_eq!(source.evaluate(&source_solution), Max(Some(4)));
    }
}

#[test]
fn test_maxcut_to_maximum2satisfiability_structure_and_pointwise_identity() {
    let source = four_cycle_with_diagonal();
    let reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_clauses(), 10);
    assert_eq!(
        target.clauses(),
        &[
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, -2]),
            CNFClause::new(vec![2, 3]),
            CNFClause::new(vec![-2, -3]),
            CNFClause::new(vec![3, 4]),
            CNFClause::new(vec![-3, -4]),
            CNFClause::new(vec![4, 1]),
            CNFClause::new(vec![-4, -1]),
            CNFClause::new(vec![1, 3]),
            CNFClause::new(vec![-1, -3]),
        ]
    );
    assert_affine_identity(&source);
}

#[test]
fn test_maxcut_to_maximum2satisfiability_boundaries() {
    let empty = MaxCut::new(SimpleGraph::empty(0), Vec::<One>::new());
    assert_affine_identity(&empty);

    let isolated = MaxCut::new(SimpleGraph::empty(3), Vec::<One>::new());
    assert_affine_identity(&isolated);

    let disconnected = MaxCut::new(SimpleGraph::new(4, vec![(0, 1)]), vec![One]);
    assert_affine_identity(&disconnected);

    let loop_graph = MaxCut::new(SimpleGraph::new(2, vec![(1, 1)]), vec![One]);
    let loop_reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&loop_graph);
    assert_eq!(
        loop_reduction.target_problem().clauses(),
        &[CNFClause::new(vec![2, 2]), CNFClause::new(vec![-2, -2]),]
    );
    assert_affine_identity(&loop_graph);

    let parallel = MaxCut::new(SimpleGraph::new(2, vec![(0, 1), (0, 1)]), vec![One, One]);
    let parallel_reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&parallel);
    assert_eq!(
        parallel_reduction.target_problem().clauses(),
        &[
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, -2]),
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, -2]),
        ]
    );
    assert_affine_identity(&parallel);
}

#[test]
#[should_panic(expected = "MaxCut vertex index exceeds Maximum2Satisfiability's i32 literal range")]
fn test_maxcut_to_maximum2satisfiability_rejects_unrepresentable_vertex_literal() {
    vertex_literal(i32::MAX as usize);
}

#[test]
fn test_maxcut_to_maximum2satisfiability_exhaustive_small_graphs() {
    for num_vertices in 0..=4 {
        let possible_edges: Vec<_> = (0..num_vertices)
            .flat_map(|u| ((u + 1)..num_vertices).map(move |v| (u, v)))
            .collect();
        for edge_mask in 0..(1usize << possible_edges.len()) {
            let edges: Vec<_> = possible_edges
                .iter()
                .enumerate()
                .filter_map(|(i, edge)| ((edge_mask >> i) & 1 == 1).then_some(*edge))
                .collect();
            let source = MaxCut::new(
                SimpleGraph::new(num_vertices, edges.clone()),
                vec![One; edges.len()],
            );
            assert_affine_identity(&source);
        }
    }
}

#[cfg(feature = "example-db")]
#[test]
fn test_maxcut_to_maximum2satisfiability_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "maxcut_to_maximum2satisfiability")
        .expect("missing canonical MaxCut -> Maximum2Satisfiability example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "MaxCut");
    assert_eq!(example.target.problem, "Maximum2Satisfiability");
    assert_eq!(example.source.instance["graph"]["num_vertices"], 4);
    assert_eq!(
        example.source.instance["graph"]["edges"]
            .as_array()
            .unwrap()
            .len(),
        5
    );
    assert_eq!(example.target.instance["num_vars"], 4);
    assert_eq!(
        example.target.instance["clauses"].as_array().unwrap().len(),
        10
    );
    assert_eq!(
        example.solutions,
        vec![crate::export::SolutionPair {
            source_config: vec![0, 1, 0, 1],
            target_config: vec![0, 1, 0, 1],
        }]
    );
}
