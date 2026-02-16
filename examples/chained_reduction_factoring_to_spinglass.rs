// # Chained Reduction: Factoring -> SpinGlass
//
// Mirrors Julia's examples/Ising.jl — reduces a Factoring problem
// to SpinGlass via the reduction graph, then solves and extracts the factors.
// Uses ILPSolver for the solve step (Julia uses GenericTensorNetworks).

// ANCHOR: imports
use problemreductions::prelude::*;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::solvers::ILPSolver;
use problemreductions::topology::SimpleGraph;
use problemreductions::types::ProblemSize;
// ANCHOR_END: imports

pub fn run() {
    // ANCHOR: example
    let graph = ReductionGraph::new();

    // Find reduction path: Factoring -> ... -> SpinGlass
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
    println!("Reduction path: {:?}", rpath.type_names());

    // Create: factor 6 = p × q with 2-bit factors (mirrors Julia's Factoring(2, 2, 6))
    let factoring = Factoring::new(2, 2, 6);

    // Solve Factoring via ILP
    let solver = ILPSolver::new();
    let solution = solver.solve_reduced(&factoring).unwrap();

    // Extract and display the factors
    let (p, q) = factoring.read_factors(&solution);
    println!("{} = {} × {}", factoring.target(), p, q);
    assert_eq!(p * q, 6, "Factors should multiply to 6");
    // ANCHOR_END: example
}

fn main() {
    run()
}
