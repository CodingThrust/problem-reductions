// # Chained Reduction: 3-SAT → MIS via Executable Paths
//
// Demonstrates the `find_cheapest_path` + `make_executable` API to chain
// reductions automatically: KSatisfiability<K3> → Satisfiability → MIS.
// The target MIS is then solved via `ILPSolver::solve_reduced`.

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
    let path = graph
        .make_executable::<KSatisfiability<K3>, MaximumIndependentSet<SimpleGraph, i32>>(&rpath)
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

    // Reduce: the executable path handles all intermediate steps
    let reduction = path.reduce(&ksat);
    let target = reduction.target_problem();

    // Solve the target MIS via ILP
    let solver = ILPSolver::new();
    let solution = solver.solve_reduced(target).unwrap();
    let original = reduction.extract_solution(&solution);

    // Verify: satisfies the original 3-SAT formula
    assert!(ksat.evaluate(&original));
    // ANCHOR_END: example
    println!("3-SAT solution: {:?}", original);
    println!("Reduction path: {:?}", rpath.type_names());
}

fn main() {
    run()
}
