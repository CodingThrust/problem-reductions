use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MonochromaticTriangle;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn k4_instance() -> MonochromaticTriangle<SimpleGraph> {
    MonochromaticTriangle::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ))
}

#[test]
fn test_monochromatic_triangle_to_ilp_structure() {
    let problem = k4_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 6);
    assert_eq!(ilp.constraints().len(), 8);
    assert_eq!(ilp.objective(), vec![]);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_monochromatic_triangle_to_ilp_constraint_pairs_on_single_triangle() {
    let problem = MonochromaticTriangle::new(SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 3);
    assert_eq!(ilp.constraints().len(), 2);
    assert_eq!(ilp.constraints()[0].rhs(), 1);
    assert_eq!(ilp.constraints()[1].rhs(), 2);
    assert_eq!(ilp.constraints()[0].terms().len(), 3);
    assert_eq!(ilp.constraints()[1].terms().len(), 3);
}

#[test]
fn test_monochromatic_triangle_to_ilp_closed_loop() {
    let problem = k4_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("K4 should admit a monochromatic-triangle-free 2-edge-coloring");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(
        extracted,
        ilp_solution
            .iter()
            .map(|&value| value != 0)
            .collect::<Vec<_>>()
    );
    assert!(problem.evaluate(&extracted).unwrap());
}

#[test]
fn test_monochromatic_triangle_to_ilp_infeasible_k6() {
    let mut edges = Vec::new();
    for u in 0..6 {
        for v in (u + 1)..6 {
            edges.push((u, v));
        }
    }
    let problem = MonochromaticTriangle::new(SimpleGraph::new(6, edges));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    assert_eq!(
        ILPSolver::new().solve(reduction.target_problem()),
        Err(crate::solvers::ILPSolveError::Infeasible),
        "K6 should be infeasible by R(3,3)=6"
    );
}

#[test]
fn test_monochromatic_triangle_to_ilp_extract_solution_identity() {
    let problem = k4_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let coloring = vec![0, 0, 1, 1, 0, 1];

    let extracted = reduction.extract_solution(&coloring).unwrap();

    assert_eq!(extracted, vec![false, false, true, true, false, true]);
    assert!(problem.evaluate(&extracted).unwrap());
}

#[test]
fn test_monochromatictriangle_to_ilp_preserves_every_small_coloring() {
    use crate::traits::Problem;
    // Every labelled subgraph of K5 and every edge colouring: 3^10 cases.
    let pairs: Vec<_> = (0..5)
        .flat_map(|u| (u + 1..5).map(move |v| (u, v)))
        .collect();
    for mask in 0..(1 << 10) {
        let edges: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter_map(|(i, &edge)| (mask & (1 << i) != 0).then_some(edge))
            .collect();
        let source = MonochromaticTriangle::new(SimpleGraph::new(5, edges));
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        for bits in 0..(1 << source.num_edges()) {
            let coloring: Vec<_> = (0..source.num_edges())
                .map(|i| bits & (1 << i) != 0)
                .collect();
            let assignment = coloring.iter().map(|&value| i64::from(value)).collect();
            assert_eq!(
                source.evaluate(&coloring).unwrap().0,
                reduction
                    .target_problem()
                    .evaluate(&assignment)
                    .unwrap()
                    .is_valid()
            );
        }
    }
}

#[test]
fn test_monochromatictriangle_to_ilp_shared_k5_all_colorings() {
    let mut edges = vec![(0, 1), (2, 3), (4, 5), (4, 6), (5, 6)];
    edges.extend((0..4).flat_map(|u| (4..7).map(move |v| (u, v))));
    let source = MonochromaticTriangle::new(SimpleGraph::new(7, edges));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_constraints(), 49);
    for bits in 0..(1 << source.num_edges()) {
        let coloring: Vec<_> = (0..source.num_edges())
            .map(|i| bits & (1 << i) != 0)
            .collect();
        let assignment = coloring.iter().map(|&value| i64::from(value)).collect();
        assert_eq!(
            source.evaluate(&coloring).unwrap().0,
            reduction
                .target_problem()
                .evaluate(&assignment)
                .unwrap()
                .is_valid()
        );
    }
}
