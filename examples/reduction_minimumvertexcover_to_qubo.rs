//! # Vertex Covering to QUBO Reduction (Penalty Method)
//!
//! ## Mathematical Relationship
//! The Minimum Vertex Cover (MVC) problem on a graph G = (V, E) is mapped to
//! QUBO by constructing a penalty Hamiltonian:
//!
//!   H(x) = sum_{i in V} x_i + P * sum_{(i,j) in E} (1 - x_i)(1 - x_j)
//!
//! where P is a penalty weight ensuring every edge has at least one endpoint
//! selected. The QUBO minimization finds configurations that minimize the
//! number of selected vertices while covering all edges.
//!
//! ## This Example
//! - Instance: Cycle graph C4 with 4 vertices and 4 edges (0-1-2-3-0)
//! - Source: MinimumVertexCover with minimum size 2
//! - QUBO variables: 4 (one per vertex)
//! - Expected: Optimal vertex covers of size 2 (e.g., {0,2} or {1,3})
//!
//! ## Output
//! Exports `docs/paper/examples/minimumvertexcover_to_qubo.json` and `minimumvertexcover_to_qubo.result.json`.
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_vc_to_qubo
//! ```

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    println!("=== Vertex Covering -> QUBO Reduction ===\n");

    // Cycle C4: 0-1-2-3-0
    let edges = vec![(0, 1), (1, 2), (2, 3), (0, 3)];
    let vc = MinimumVertexCover::<SimpleGraph, i32>::new(4, edges.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&vc);
    let qubo = reduction.target_problem();

    println!("Source: MinimumVertexCover on cycle C4 (4 vertices, 4 edges)");
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
        let selected: Vec<usize> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect();
        let size = selected.len();
        println!(
            "  Cover vertices: {:?} ({} vertices)",
            selected, size
        );

        // Closed-loop verification: check solution is valid in original problem
        let sol_size = vc.solution_size(&extracted);
        assert!(sol_size.is_valid, "Solution must be valid in source problem");

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    // All optimal solutions should have size 2
    assert!(
        solutions.iter().all(|s| s.source_config.iter().filter(|&&x| x == 1).count() == 2),
        "All optimal VC solutions on C4 should have size 2"
    );
    println!("\nVerification passed: all solutions are valid with size 2");

    // Export JSON
    let overhead = lookup_overhead("MinimumVertexCover", "QUBO")
        .expect("MinimumVertexCover -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MinimumVertexCover::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": vc.num_vertices(),
                "num_edges": vc.num_edges(),
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
