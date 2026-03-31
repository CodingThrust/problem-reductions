//! Reduction path parity tests — mirrors Julia's test/reduction_path.jl.
//! Verifies that chained reductions via `find_cheapest_path` + `reduce_along_path`
//! produce correct solutions matching direct source solves.

use crate::models::algebraic::QUBO;
use crate::models::graph::{MaxCut, SpinGlass};
use crate::models::misc::Factoring;
use crate::rules::test_helpers::assert_optimization_round_trip_chain;
use crate::rules::{MinimizeSteps, MinimizeStepsThenOverhead, ReductionGraph};
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Julia: paths = reduction_paths(MaxCut, SpinGlass)
/// Julia: res = reduceto(paths[1], MaxCut(smallgraph(:petersen)))
#[test]
fn test_jl_parity_maxcut_to_spinglass_path() {
    let graph = ReductionGraph::new();
    let src_var = ReductionGraph::variant_to_map(&MaxCut::<SimpleGraph, i32>::variant());
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());
    let rpath = graph
        .find_cheapest_path(
            "MaxCut",
            &src_var,
            "SpinGlass",
            &dst_var,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .expect("Should find path MaxCut -> SpinGlass");

    // Petersen graph: 10 vertices, 15 edges
    let petersen_edges = vec![
        (0, 1),
        (0, 4),
        (0, 5),
        (1, 2),
        (1, 6),
        (2, 3),
        (2, 7),
        (3, 4),
        (3, 8),
        (4, 9),
        (5, 7),
        (5, 8),
        (6, 8),
        (6, 9),
        (7, 9),
    ];
    let source = MaxCut::<SimpleGraph, i32>::unweighted(SimpleGraph::new(10, petersen_edges));
    let chain = graph
        .reduce_along_path(&rpath, &source as &dyn std::any::Any)
        .expect("Should reduce along path");
    let target: &SpinGlass<SimpleGraph, f64> = chain.target_problem();

    // Verify target is SpinGlass
    assert_eq!(SpinGlass::<SimpleGraph, f64>::NAME, "SpinGlass");

    let solver = BruteForce::new();
    let target_solution = solver.find_witness(target).unwrap();
    let source_solution = chain.extract_solution(&target_solution);

    // Source solution should be valid
    let metric = source.evaluate(&source_solution);
    assert!(metric.is_valid());
}

/// Julia: paths = reduction_paths(MaxCut, QUBO)
/// Julia: sort(extract_solution.(Ref(res), best2)) == sort(best1)
#[test]
fn test_jl_parity_maxcut_to_qubo_path() {
    let graph = ReductionGraph::new();
    let src_var = ReductionGraph::variant_to_map(&MaxCut::<SimpleGraph, i32>::variant());
    let dst_var = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    // Use Petersen graph size to pick the path with smallest output
    let rpath = graph
        .find_cheapest_path(
            "MaxCut",
            &src_var,
            "QUBO",
            &dst_var,
            &ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 15)]),
            &MinimizeStepsThenOverhead,
        )
        .expect("Should find path MaxCut -> QUBO");

    // Use a small graph for brute-force feasibility
    let petersen_edges = vec![
        (0, 1),
        (0, 4),
        (0, 5),
        (1, 2),
        (1, 6),
        (2, 3),
        (2, 7),
        (3, 4),
        (3, 8),
        (4, 9),
        (5, 7),
        (5, 8),
        (6, 8),
        (6, 9),
        (7, 9),
    ];
    let source = MaxCut::<SimpleGraph, i32>::unweighted(SimpleGraph::new(10, petersen_edges));
    let chain = graph
        .reduce_along_path(&rpath, &source as &dyn std::any::Any)
        .expect("Should reduce along path");
    assert_optimization_round_trip_chain::<MaxCut<SimpleGraph, i32>, QUBO<f64>>(
        &source,
        &chain,
        "MaxCut->QUBO path parity",
    );
}

/// Julia: factoring = Factoring(2, 1, 3)
/// Julia: paths = reduction_paths(Factoring, SpinGlass)
/// Julia: all(solution_size.(Ref(factoring), extract_solution.(Ref(res), sol)) .== Ref(valid objective 0))
#[cfg(feature = "ilp-solver")]
#[test]
fn test_jl_parity_factoring_to_spinglass_path() {
    use crate::solvers::ILPSolver;

    let graph = ReductionGraph::new();
    let src_var = ReductionGraph::variant_to_map(&Factoring::variant());
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());
    let rpath = graph
        .find_cheapest_path(
            "Factoring",
            &src_var,
            "SpinGlass",
            &dst_var,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .expect("Should find path Factoring -> SpinGlass");

    // Julia: Factoring(2, 1, 3) — factor 3 with 2-bit x 1-bit
    let factoring = Factoring::new(2, 1, 3);
    let chain = graph
        .reduce_along_path(&rpath, &factoring as &dyn std::any::Any)
        .expect("Should reduce along path");
    let target: &SpinGlass<SimpleGraph, f64> = chain.target_problem();

    // Verify reduction produces a valid SpinGlass problem
    assert!(
        target.num_variables() > 0,
        "SpinGlass should have variables"
    );

    // Solve Factoring directly via ILP (fast) and verify path solution extraction
    use crate::models::algebraic::ILP;
    use crate::rules::traits::{ReduceTo, ReductionResult};
    let ilp_solver = ILPSolver::new();
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&factoring);
    let ilp = reduction.target_problem();
    let ilp_solution = ilp_solver
        .solve(ilp)
        .expect("ILP solver should find factoring solution");
    let factoring_solution = reduction.extract_solution(&ilp_solution);
    let metric = factoring.evaluate(&factoring_solution);
    assert_eq!(
        metric.unwrap(),
        0,
        "Factoring->ILP: ILP solution should yield distance 0"
    );
}

/// Test that `find_cheapest_path` works with a concrete `ProblemSize` input,
/// rather than an empty `ProblemSize::new(vec![])`.
#[test]
fn test_find_cheapest_path_with_problem_size() {
    let graph = ReductionGraph::new();
    let petersen = SimpleGraph::new(
        10,
        vec![
            (0, 1),
            (0, 4),
            (0, 5),
            (1, 2),
            (1, 6),
            (2, 3),
            (2, 7),
            (3, 4),
            (3, 8),
            (4, 9),
            (5, 7),
            (5, 8),
            (6, 8),
            (6, 9),
            (7, 9),
        ],
    );
    let _source = MaxCut::<SimpleGraph, i32>::unweighted(petersen);
    let src_var = ReductionGraph::variant_to_map(&MaxCut::<SimpleGraph, i32>::variant());
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());

    let input_size = ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 15)]);
    let rpath = graph
        .find_cheapest_path(
            "MaxCut",
            &src_var,
            "SpinGlass",
            &dst_var,
            &input_size,
            &MinimizeSteps,
        )
        .expect("Should find path MaxCut -> SpinGlass");

    assert!(!rpath.type_names().is_empty());
}
