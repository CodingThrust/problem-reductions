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
//! - Source: VertexCovering with minimum size 2
//! - QUBO variables: 4 (one per vertex)
//! - Expected: Optimal vertex covers of size 2 (e.g., {0,2} or {1,3})
//!
//! ## Outputs
//! - `docs/paper/examples/vc_to_qubo.json` - Serialized reduction data
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_vc_to_qubo
//! ```

use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Serializable structure capturing the full reduction for paper export.
#[derive(Serialize)]
struct ExampleData {
    name: String,
    source_problem: String,
    target_problem: String,
    source_instance: SourceInstance,
    qubo: QUBO,
    optimal_solutions: Vec<SolutionEntry>,
}

#[derive(Serialize)]
struct SourceInstance {
    num_vertices: usize,
    edges: Vec<(usize, usize)>,
    description: String,
}

#[derive(Serialize)]
struct SolutionEntry {
    config: Vec<usize>,
    selected_vertices: Vec<usize>,
    size: usize,
}

fn main() {
    println!("=== Vertex Covering -> QUBO Reduction ===\n");

    // Cycle C4: 0-1-2-3-0
    let edges = vec![(0, 1), (1, 2), (2, 3), (0, 3)];
    let vc = VertexCovering::<SimpleGraph, i32>::new(4, edges.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&vc);
    let qubo = reduction.target_problem();

    println!("Source: VertexCovering on cycle C4 (4 vertices, 4 edges)");
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix:");
    for row in qubo.matrix() {
        println!("  {:?}", row);
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let mut optimal_solutions = Vec::new();
    for sol in &solutions {
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

        optimal_solutions.push(SolutionEntry {
            config: extracted,
            selected_vertices: selected,
            size,
        });
    }

    // All optimal solutions should have size 2
    assert!(
        optimal_solutions.iter().all(|s| s.size == 2),
        "All optimal VC solutions on C4 should have size 2"
    );
    println!("\nVerification passed: all solutions are valid with size 2");

    // Export JSON
    let example_data = ExampleData {
        name: "vc_to_qubo".to_string(),
        source_problem: "VertexCovering".to_string(),
        target_problem: "QUBO".to_string(),
        source_instance: SourceInstance {
            num_vertices: 4,
            edges,
            description: "Cycle graph C4: 4 vertices, 4 edges (0-1-2-3-0)".to_string(),
        },
        qubo: qubo.clone(),
        optimal_solutions,
    };

    let output_path = Path::new("docs/paper/examples/vc_to_qubo.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&example_data).expect("Failed to serialize");
    fs::write(output_path, json).expect("Failed to write JSON");
    println!("Exported: {}", output_path.display());
}
