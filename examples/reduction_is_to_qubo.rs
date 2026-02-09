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
//! - Source: IndependentSet with maximum size 2 (e.g., {0,2} or {1,3})
//! - QUBO variables: 4 (one per vertex)
//! - Expected: Two optimal solutions of size 2: vertices {0,2} and {1,3}
//!
//! ## Outputs
//! - `docs/paper/examples/is_to_qubo.json` - Serialized reduction data
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_is_to_qubo
//! ```

use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Serializable structure capturing the full reduction for paper export.
#[derive(Serialize)]
struct ExampleData {
    /// Name of this example
    name: String,
    /// Source problem type
    source_problem: String,
    /// Target problem type
    target_problem: String,
    /// Source instance description
    source_instance: SourceInstance,
    /// The QUBO target problem (Q matrix, num_vars)
    qubo: QUBO,
    /// All optimal solutions (in source problem space)
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
    println!("=== Independent Set -> QUBO Reduction ===\n");

    // Path graph P4: 0-1-2-3
    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let is = IndependentSet::<SimpleGraph, i32>::new(4, edges.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&is);
    let qubo = reduction.target_problem();

    println!("Source: IndependentSet on path P4 (4 vertices, 3 edges)");
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
        println!("  Vertices: {:?} (size {})", selected, size);

        // Closed-loop verification: check solution is valid in original problem
        let sol_size = is.solution_size(&extracted);
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
        "All optimal IS solutions on P4 should have size 2"
    );
    println!("\nVerification passed: all solutions are valid with size 2");

    // Export JSON
    let example_data = ExampleData {
        name: "is_to_qubo".to_string(),
        source_problem: "IndependentSet".to_string(),
        target_problem: "QUBO".to_string(),
        source_instance: SourceInstance {
            num_vertices: 4,
            edges,
            description: "Path graph P4: 4 vertices, 3 edges (0-1-2-3)".to_string(),
        },
        qubo: qubo.clone(),
        optimal_solutions,
    };

    let output_path = Path::new("docs/paper/examples/is_to_qubo.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&example_data).expect("Failed to serialize");
    fs::write(output_path, json).expect("Failed to write JSON");
    println!("Exported: {}", output_path.display());
}
