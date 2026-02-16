// # Chained Reduction: Factoring -> SpinGlass
//
// Mirrors Julia's examples/Ising.jl — reduces a Factoring problem
// to SpinGlass via the reduction graph (Factoring -> CircuitSAT -> SpinGlass),
// then solves and extracts the factors.

// ANCHOR: imports
use problemreductions::prelude::*;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::solvers::BruteForce;
use problemreductions::topology::SimpleGraph;
use problemreductions::types::ProblemSize;
// ANCHOR_END: imports

pub fn run() {
    // ANCHOR: example
    let graph = ReductionGraph::new();

    // Find path: Factoring -> CircuitSAT -> SpinGlass
    let src_var = ReductionGraph::variant_to_map(&Factoring::variant());
    let dst_var =
        ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());
    let rpath = graph
        .find_cheapest_path(
            "Factoring",
            &src_var,
            "SpinGlass",
            &dst_var,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();
    let path = graph
        .make_executable::<Factoring, SpinGlass<SimpleGraph, f64>>(&rpath)
        .unwrap();

    // Create: factor 3 = p x q with 2-bit first factor and 1-bit second factor
    let factoring = Factoring::new(2, 1, 3);

    // Reduce via the path
    let reduction = path.reduce(&factoring);
    let target = reduction.target_problem();

    // Solve the SpinGlass problem
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(target);

    // Extract and verify each solution
    for sol in &solutions {
        let source_sol = reduction.extract_solution(sol);
        let (p, q) = factoring.read_factors(&source_sol);
        assert_eq!(p * q, 3, "Factors should multiply to 3");
    }
    // ANCHOR_END: example

    println!("Reduction path: {:?}", rpath.type_names());
    println!(
        "Found {} SpinGlass solutions mapping to valid factorizations",
        solutions.len()
    );
}

fn main() {
    run()
}
