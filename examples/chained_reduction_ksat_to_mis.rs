// # Chained Reduction: 3-SAT → MIS via Reduction Chains
//
// Demonstrates the `find_cheapest_path` + `reduce_along_path` API to chain
// reductions automatically: KSatisfiability<K3> → Satisfiability → MIS.
// The target MIS is then reduced further to ILP and solved there.

// ANCHOR: imports
use problemreductions::models::algebraic::ILP;
use problemreductions::prelude::*;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::solvers::ILPSolver;
use problemreductions::topology::SimpleGraph;
use problemreductions::types::ProblemSize;
use problemreductions::variant::K3;
// ANCHOR_END: imports

pub fn run() {
    // ANCHOR: example
    let graph = ReductionGraph::new();

    // Find variant-exact path from KSat<K3> to MIS<SimpleGraph, i32>
    let src_var = ReductionGraph::variant_to_map(&KSatisfiability::<K3>::variant());
    let dst_var =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let rpath = graph
        .find_cheapest_path(
            "KSatisfiability",
            &src_var,
            "MaximumIndependentSet",
            &dst_var,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();

    // Create: 3-SAT formula (a∨b∨¬c)∧(¬a∨¬b∨¬c)∧(¬a∨b∨c)∧(a∨¬b∨c)
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, -2, -3]),
            CNFClause::new(vec![-1, 2, 3]),
            CNFClause::new(vec![1, -2, 3]),
        ],
    );

    // Reduce: the reduction chain handles all intermediate steps
    let chain = graph
        .reduce_along_path(&rpath, &ksat as &dyn std::any::Any)
        .unwrap();
    let target: &MaximumIndependentSet<SimpleGraph, i32> = chain.target_problem();

    // Reduce the target MIS further to ILP through the registered rule graph.
    let ilp_var = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let ilp_path = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &dst_var,
            "ILP",
            &ilp_var,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();
    let ilp_chain = graph
        .reduce_along_path(&ilp_path, target as &dyn std::any::Any)
        .unwrap();
    let ilp: &ILP<bool> = ilp_chain.target_problem();

    // Solve the target MIS via the derived ILP.
    let solver = ILPSolver::new();
    let ilp_solution = solver.solve(ilp).unwrap();
    let mis_solution = ilp_chain.extract_solution(&ilp_solution);
    let original = chain.extract_solution(&mis_solution);

    // Verify: satisfies the original 3-SAT formula
    assert!(ksat.evaluate(&original));
    // ANCHOR_END: example

    // ANCHOR: overhead
    // Compose overheads symbolically along the path
    // Maps source problem variables → final target problem variables
    let composed = graph.compose_path_overhead(&rpath);
    for (field, poly) in &composed.output_size {
        println!("  {} = {}", field, poly);
    }
    // ANCHOR_END: overhead

    println!("3-SAT solution: {:?}", original);
    println!("Reduction path: {:?}", rpath.type_names());
}

fn main() {
    run()
}
