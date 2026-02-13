// # Traveling Salesman to ILP Reduction
//
// ## Mathematical Formulation
// Variables: x_{v,k} in {0,1} for vertex v and position k;
// auxiliary y variables for McCormick linearization of products.
// Constraints: assignment, non-edge consecutive prohibition, McCormick.
// Objective: minimize total edge weight of the tour.
//
// ## This Example
// - Instance: K4 complete graph with weights
// - Source: TravelingSalesman with 4 vertices, 6 edges
// - Target: ILP with position-based binary variables
//
// ## Output
// Exports `docs/paper/examples/travelingsalesman_to_ilp.json` and `travelingsalesman_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;
use problemreductions::topology::SimpleGraph;

pub fn run() {
    // 1. Create TSP instance: K4 with weights
    let problem = TravelingSalesman::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10),
            (0, 2, 15),
            (0, 3, 20),
            (1, 2, 35),
            (1, 3, 25),
            (2, 3, 30),
        ],
    );

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: TravelingSalesman with {} variables ({} edges)",
        problem.num_variables(),
        problem.num_edges()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");

    // 5. Extract source solution
    let tsp_solution = reduction.extract_solution(&ilp_solution);
    println!("\n=== Solution ===");
    println!("Edge selection: {:?}", tsp_solution);

    // 6. Verify
    let metric = problem.evaluate(&tsp_solution);
    println!("Tour cost: {:?}", metric);
    assert!(metric.is_valid());

    // Cross-check with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_best(&problem);
    let bf_metric = problem.evaluate(&bf_solutions[0]);
    assert_eq!(metric, bf_metric, "ILP must match brute force optimum");
    println!("Brute force confirms optimality");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    solutions.push(SolutionPair {
        source_config: tsp_solution.clone(),
        target_config: ilp_solution,
    });

    let overhead = lookup_overhead_or_empty("TravelingSalesman", "ILP");
    let edges: Vec<(usize, usize)> = problem.edges().iter().map(|&(u, v, _)| (u, v)).collect();

    let data = ReductionData {
        source: ProblemSide {
            problem: TravelingSalesman::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(TravelingSalesman::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": problem.num_vertices(),
                "num_edges": problem.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: variant_to_map(ILP::variant()),
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "travelingsalesman_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
