// # Traveling Salesman to QUBO Reduction (Penalty Method)
//
// ## Mathematical Relationship
// The TSP on a graph G = (V, E) with edge weights is mapped to QUBO using
// position-based encoding. Each vertex v and position k has a binary variable
// x_{v,k}, with penalties enforcing:
//
// 1. Assignment constraint: each vertex appears exactly once in the tour
// 2. Position constraint: each position has exactly one vertex
// 3. Edge constraint: consecutive positions use valid edges
// 4. Objective: total edge weight of the tour
//
// The QUBO has n^2 variables (n vertices x n positions).
//
// ## This Example
// - Instance: K3 complete graph with edge weights [1, 2, 3]
//   (w01=1, w02=2, w12=3)
// - Source: TravelingSalesman on 3 vertices, 3 edges
// - QUBO variables: 9 (3^2 = 9, position encoding)
// - Optimal tour cost = 6 (all edges used: 1 + 2 + 3)
//
// ## Outputs
// - `docs/paper/examples/travelingsalesman_to_qubo.json` — reduction structure
// - `docs/paper/examples/travelingsalesman_to_qubo.result.json` — solutions
//
// ## Usage
// ```bash
// cargo run --example reduction_travelingsalesman_to_qubo
// ```

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    println!("=== TravelingSalesman -> QUBO Reduction ===\n");

    // K3 complete graph with edge weights: w01=1, w02=2, w12=3
    let graph = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let tsp = TravelingSalesman::new(graph, vec![1i32, 2, 3]);

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&tsp);
    let qubo = reduction.target_problem();

    println!(
        "Source: TravelingSalesman on K3 ({} vertices, {} edges)",
        tsp.graph().num_vertices(),
        tsp.graph().num_edges()
    );
    println!(
        "Target: QUBO with {} variables (position encoding: 3 vertices x 3 positions)",
        qubo.num_variables()
    );
    println!("Q matrix:");
    for row in qubo.matrix() {
        let formatted: Vec<String> = row.iter().map(|v| format!("{:8.1}", v)).collect();
        println!("  [{}]", formatted.join(", "));
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal QUBO solutions: {}", qubo_solutions.len());
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let edge_names = ["(0,1)", "(0,2)", "(1,2)"];
        let selected: Vec<&str> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| edge_names[i])
            .collect();
        println!("  Edges: {}", selected.join(", "));

        // Closed-loop verification: check solution is valid in original problem
        let metric = tsp.evaluate(&extracted);
        assert!(metric.is_valid(), "Tour must be valid in source problem");
        println!("  Cost: {:?}", metric);

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    // Cross-check with brute force on original problem
    let bf_solutions = solver.find_all_best(&tsp);
    let bf_metric = tsp.evaluate(&bf_solutions[0]);
    let qubo_metric = tsp.evaluate(&reduction.extract_solution(&qubo_solutions[0]));
    assert_eq!(
        bf_metric, qubo_metric,
        "QUBO reduction must match brute force optimum"
    );

    println!(
        "\nVerification passed: optimal tour cost matches brute force ({:?})",
        bf_metric
    );

    // Export JSON
    let source_variant = variant_to_map(TravelingSalesman::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead = lookup_overhead(
        "TravelingSalesman",
        &source_variant,
        "QUBO",
        &target_variant,
    )
    .expect("TravelingSalesman -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: TravelingSalesman::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": tsp.graph().num_vertices(),
                "num_edges": tsp.graph().num_edges(),
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": qubo.matrix(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "travelingsalesman_to_qubo";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
