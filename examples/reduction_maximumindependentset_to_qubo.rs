//! # Independent Set to QUBO Reduction (Penalty Method)
//!
//! ## Mathematical Relationship
//! The Maximum Independent Set (MIS) problem on a graph G = (V, E) is mapped to
//! QUBO by constructing a penalty Hamiltonian:
//!
//!   H(x) = -sum_{i in V} x_i + P * sum_{(i,j) in E} x_i * x_j
//!
//! where P > 1 is a penalty weight ensuring no two adjacent vertices are both
//! selected. The QUBO minimization finds configurations that maximize the
//! independent set size while respecting adjacency constraints.
//!
//! ## This Example
//! - Instance: Path graph P4 with 4 vertices and 3 edges (0-1-2-3)
//! - Source: MaximumIndependentSet with maximum size 2 (e.g., {0,2} or {1,3})
//! - QUBO variables: 4 (one per vertex)
//! - Expected: Two optimal solutions of size 2: vertices {0,2} and {1,3}
//!
//! ## Output
//! Exports `docs/paper/examples/maximumindependentset_to_qubo.json` and `maximumindependentset_to_qubo.result.json`.
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_is_to_qubo
//! ```

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    println!("=== Independent Set -> QUBO Reduction ===\n");

    // Path graph P4: 0-1-2-3
    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let is = MaximumIndependentSet::<SimpleGraph, i32>::new(4, edges.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&is);
    let qubo = reduction.target_problem();

    println!("Source: MaximumIndependentSet on path P4 (4 vertices, 3 edges)");
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix:");
    for row in qubo.matrix() {
        println!("  {:?}", row);
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let sol_size = is.solution_size(&extracted);
        assert!(sol_size.is_valid, "Solution must be valid in source problem");

        let selected: Vec<usize> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect();
        println!("  Vertices: {:?} (size {})", selected, selected.len());

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!("\nVerification passed: all solutions are valid");

    // Export JSON
    let overhead = lookup_overhead("MaximumIndependentSet", "QUBO")
        .expect("MaximumIndependentSet -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": is.num_vertices(),
                "num_edges": is.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: variant_to_map(QUBO::<f64>::variant()),
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": qubo.matrix(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
