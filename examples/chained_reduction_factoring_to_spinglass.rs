// # Chained Reduction: Factoring -> SpinGlass
//
// Mirrors Julia's examples/Ising.jl — reduces a Factoring problem
// to SpinGlass via the reduction graph, then solves and extracts the factors.
// Uses ILPSolver for the solve step (Julia uses GenericTensorNetworks).

// ANCHOR: imports
use problemreductions::models::algebraic::ILP;
use problemreductions::prelude::*;
use problemreductions::rules::{ReductionGraph, ReductionMode};
use problemreductions::solvers::ILPSolver;
use problemreductions::topology::SimpleGraph;
// ANCHOR_END: imports

pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // ANCHOR: example
    // ANCHOR: step1
    let graph = ReductionGraph::new(); // all registered reductions
    let src_var = ReductionGraph::variant_to_map(&Factoring::variant()); // {} (no variant params)
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant()); // {graph: "SimpleGraph", weight: "f64"}
    let paths = graph.find_all_paths_mode(
        "Factoring",
        &src_var,
        "SpinGlass",
        &dst_var,
        ReductionMode::Witness,
    );
    let rpath = paths
        .iter()
        .find(|path| path.type_names() == ["Factoring", "CircuitSAT", "SpinGlass"])
        .expect("explicit Factoring -> CircuitSAT -> SpinGlass route");
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
    // Factoring reduces to ILP<i32>, so we manually reduce, solve, and extract
    let solver = ILPSolver::new();
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&factoring);
    let ilp_solution = solver.solve(reduction.target_problem()).unwrap();
    let solution = reduction.extract_solution(&ilp_solution).unwrap();
    // ANCHOR_END: step3

    // ANCHOR: step4
    let (p, q) = factoring.read_factors(&solution); // decode bit assignments → integers
    println!("{} = {} × {}", factoring.target(), p, q);
    assert_eq!(p * q, 6, "Factors should multiply to 6");
    // ANCHOR_END: step4

    // ANCHOR_END: example
    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    run()
}
