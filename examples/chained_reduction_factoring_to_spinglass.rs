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
    // ANCHOR: step1
    let graph = ReductionGraph::new(); // all registered reductions
    let src_var = ReductionGraph::variant_to_map(&Factoring::variant()); // {} (no variant params)
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant()); // {graph: "SimpleGraph", weight: "f64"}
    let rpath = graph
        .find_cheapest_path(
            "Factoring",               // source problem name
            &src_var,                  // source variant map
            "SpinGlass",               // target problem name
            &dst_var,                  // target variant map
            &ProblemSize::new(vec![]), // input size (empty = unknown)
            &MinimizeSteps,            // cost function: fewest hops
        )
        .unwrap();
    println!("  {}", rpath);
    // ANCHOR_END: step1

    // ANCHOR: step2
    let factoring = Factoring::new(
        2, // num_bits_first:  p is a 2-bit factor
        2, // num_bits_second: q is a 2-bit factor
        6, // target_product:  find p × q = 6
    );
    // ANCHOR_END: step2

    // ANCHOR: step3
    // solve_reduced: reduce → ILP, solve with HiGHS, extract back
    let solver = ILPSolver::new();
    let solution = solver.solve_reduced(&factoring).unwrap();
    // ANCHOR_END: step3

    // ANCHOR: step4
    let (p, q) = factoring.read_factors(&solution); // decode bit assignments → integers
    println!("{} = {} × {}", factoring.target(), p, q);
    assert_eq!(p * q, 6, "Factors should multiply to 6");
    // ANCHOR_END: step4

    // ANCHOR: overhead
    // Print per-edge overhead polynomials
    let edge_overheads = graph.path_overheads(&rpath);
    for (i, overhead) in edge_overheads.iter().enumerate() {
        println!("{} → {}:", rpath.steps[i], rpath.steps[i + 1]);
        for (field, poly) in &overhead.output_size {
            println!("  {} = {}", field, poly);
        }
    }

    // Compose overheads symbolically along the full path
    let composed = graph.compose_path_overhead(&rpath);
    println!("Composed (source → target):");
    for (field, poly) in &composed.output_size {
        println!("  {} = {}", field, poly);
    }
    // ANCHOR_END: overhead
    // ANCHOR_END: example
}

fn main() {
    run()
}
