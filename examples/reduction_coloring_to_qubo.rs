//! # K-Coloring to QUBO Reduction (Penalty Method)
//!
//! ## Mathematical Relationship
//! The K-Coloring problem on a graph G = (V, E) with K colors is mapped to QUBO
//! using a one-hot encoding. Each vertex i has K binary variables x_{i,c} for
//! c = 0..K-1, with penalties enforcing:
//!
//! 1. One-hot constraint: each vertex gets exactly one color
//!    P1 * sum_i (1 - sum_c x_{i,c})^2
//!
//! 2. Edge constraint: adjacent vertices get different colors
//!    P2 * sum_{(i,j) in E} sum_c x_{i,c} * x_{j,c}
//!
//! The QUBO has n*K variables (n vertices, K colors).
//!
//! ## This Example
//! - Instance: Complete graph K3 (triangle) with 3 colors
//! - Source: KColoring<3> on 3 vertices, 3 edges
//! - QUBO variables: 9 (3 vertices x 3 colors, one-hot encoding)
//! - Expected: 6 valid 3-colorings (3! = 6 permutations of 3 colors on 3 vertices)
//!
//! ## Outputs
//! - `docs/paper/examples/coloring_to_qubo.json` - Serialized reduction data
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_coloring_to_qubo
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
    num_valid_colorings: usize,
    optimal_solutions: Vec<SolutionEntry>,
}

#[derive(Serialize)]
struct SourceInstance {
    num_vertices: usize,
    num_colors: usize,
    edges: Vec<(usize, usize)>,
    description: String,
}

#[derive(Serialize)]
struct SolutionEntry {
    config: Vec<usize>,
    coloring: Vec<String>,
}

fn main() {
    println!("=== K-Coloring -> QUBO Reduction ===\n");

    // Triangle K3: all 3 vertices are adjacent
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let kc = KColoring::<3, SimpleGraph, i32>::new(3, edges.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&kc);
    let qubo = reduction.target_problem();

    let colors = ["Red", "Green", "Blue"];
    println!("Source: KColoring<3> on triangle K3 (3 vertices, 3 edges)");
    println!(
        "Target: QUBO with {} variables (one-hot: 3 vertices x 3 colors)",
        qubo.num_variables()
    );
    println!("Q matrix:");
    for row in qubo.matrix() {
        let formatted: Vec<String> = row.iter().map(|v| format!("{:6.1}", v)).collect();
        println!("  [{}]", formatted.join(", "));
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nValid 3-colorings: {}", solutions.len());
    let mut optimal_solutions = Vec::new();
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let coloring: Vec<String> = extracted
            .iter()
            .enumerate()
            .map(|(i, &c)| format!("V{}={}", i, colors[c]))
            .collect();
        println!("  {}", coloring.join(", "));

        // Closed-loop verification: check solution is valid in original problem
        let sol_size = kc.solution_size(&extracted);
        assert!(sol_size.is_valid, "Coloring must be valid in source problem");

        optimal_solutions.push(SolutionEntry {
            config: extracted,
            coloring,
        });
    }

    // K3 with 3 colors has exactly 3! = 6 valid colorings
    assert_eq!(
        solutions.len(),
        6,
        "Triangle K3 with 3 colors should have exactly 6 valid colorings"
    );
    println!("\nVerification passed: 6 valid colorings found");

    // Export JSON
    let example_data = ExampleData {
        name: "coloring_to_qubo".to_string(),
        source_problem: "KColoring<3>".to_string(),
        target_problem: "QUBO".to_string(),
        source_instance: SourceInstance {
            num_vertices: 3,
            num_colors: 3,
            edges,
            description: "Complete graph K3 (triangle): 3 vertices, 3 edges, 3 colors".to_string(),
        },
        qubo: qubo.clone(),
        num_valid_colorings: solutions.len(),
        optimal_solutions,
    };

    let output_path = Path::new("docs/paper/examples/coloring_to_qubo.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&example_data).expect("Failed to serialize");
    fs::write(output_path, json).expect("Failed to write JSON");
    println!("Exported: {}", output_path.display());
}
