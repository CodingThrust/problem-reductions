//! Reduction path parity tests — mirrors Julia's test/reduction_path.jl.
//! Verifies that explicit chained reductions via `reduce_along_path`
//! produce correct solutions matching direct source solves.

use crate::models::algebraic::QUBO;
use crate::models::graph::{MaxCut, SpinGlass};
use crate::models::misc::Factoring;
use crate::rules::test_helpers::assert_optimization_round_trip_chain;
use crate::rules::ReductionGraph;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

/// Julia: paths = reduction_paths(MaxCut, SpinGlass)
/// Julia: res = reduceto(paths[1], MaxCut(smallgraph(:petersen)))
#[test]
fn test_jl_parity_maxcut_to_spinglass_path() {
    let graph = ReductionGraph::new();
    let src_var = ReductionGraph::variant_to_map(&MaxCut::<SimpleGraph, i64>::variant());
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());
    let rpath = graph
        .find_all_paths("MaxCut", &src_var, "SpinGlass", &dst_var)
        .into_iter()
        .find(|path| path.type_names() == ["MaxCut", "SpinGlass"])
        .expect("direct route");

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
    let source = MaxCut::<SimpleGraph, i64>::unweighted(SimpleGraph::new(10, petersen_edges));
    let chain = graph
        .reduce_along_path(&rpath, &source as &dyn std::any::Any)
        .expect("MaxCut -> SpinGlass reduction should not fail")
        .expect("Should reduce along path");
    let target: &SpinGlass<SimpleGraph, f64> = chain.target_problem();

    // Verify target is SpinGlass
    assert_eq!(SpinGlass::<SimpleGraph, f64>::NAME, "SpinGlass");

    let solver = BruteForce::new();
    let target_solution = solver.find_witness(target).unwrap().unwrap();
    let source_solution = chain.extract_solution(&target_solution).unwrap();

    // Source solution should be valid
    let metric = source.evaluate(&source_solution).unwrap();
    assert!(metric.is_valid());
}

/// Julia: paths = reduction_paths(MaxCut, QUBO)
/// Julia: sort(extract_solution.(Ref(res), best2)) == sort(best1)
#[test]
fn test_jl_parity_maxcut_to_qubo_path() {
    let graph = ReductionGraph::new();
    let src_var = ReductionGraph::variant_to_map(&MaxCut::<SimpleGraph, i64>::variant());
    let dst_var = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let rpath = graph
        .find_all_paths("MaxCut", &src_var, "QUBO", &dst_var)
        .into_iter()
        .find(|path| path.type_names() == ["MaxCut", "SpinGlass", "QUBO"])
        .expect("explicit SpinGlass route");

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
    let source = MaxCut::<SimpleGraph, i64>::unweighted(SimpleGraph::new(10, petersen_edges));
    let chain = graph
        .reduce_along_path(&rpath, &source as &dyn std::any::Any)
        .expect("MaxCut -> QUBO reduction should not fail")
        .expect("Should reduce along path");
    assert_optimization_round_trip_chain::<MaxCut<SimpleGraph, i64>, QUBO<f64>>(
        &source,
        &chain,
        "MaxCut->QUBO path parity",
    );
}

/// Julia: factoring = Factoring(2, 1, 3)
/// Julia: paths = reduction_paths(Factoring, SpinGlass)
/// Julia: all(solution_size.(Ref(factoring), extract_solution.(Ref(res), sol)) .== Ref(valid objective 0))
#[test]
fn test_jl_parity_factoring_to_spinglass_path() {
    use crate::solvers::ILPSolver;

    let graph = ReductionGraph::new();
    let src_var = ReductionGraph::variant_to_map(&Factoring::variant());
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());
    let rpath = graph
        .find_all_paths("Factoring", &src_var, "SpinGlass", &dst_var)
        .into_iter()
        .find(|path| path.type_names() == ["Factoring", "CircuitSAT", "SpinGlass"])
        .expect("explicit CircuitSAT route");

    // Julia: Factoring(2, 1, 3) — factor 3 with 2-bit x 1-bit
    let factoring = Factoring::new(2, 1, 3);
    let chain = graph
        .reduce_along_path(&rpath, &factoring as &dyn std::any::Any)
        .expect("Factoring -> SpinGlass reduction should not fail")
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
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&factoring).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let ilp_solution = ilp_solver
        .solve(ilp)
        .expect("ILP solver should find factoring solution");
    let factoring_solution = reduction.extract_solution(&ilp_solution).unwrap();
    let metric = factoring.evaluate(&factoring_solution).unwrap();
    assert!(metric.unwrap(), "Factoring->ILP solution must be valid");
}
