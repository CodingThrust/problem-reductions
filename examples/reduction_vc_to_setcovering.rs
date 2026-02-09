//! # Vertex Cover to Set Covering Reduction
//!
//! ## Mathematical Equivalence
//! Universe U = {0, ..., |E|-1} (edge indices). For each vertex v, set
//! S_v = edges incident to v. A vertex cover (every edge has an endpoint
//! in the cover) maps to a set cover (every universe element in some set).
//!
//! ## This Example
//! - Instance: Triangle K3 (3 vertices, 3 edges)
//! - Source VC: min size 2
//! - Target SetCovering: min cover 2
//!
//! ## Output
//! Exports `docs/paper/examples/vc_to_setcovering.json` for use in paper code blocks.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// JSON export structure for the VC to SetCovering reduction example.
#[derive(Serialize)]
struct ReductionExport {
    reduction: String,
    source: SourceExport,
    target: TargetExport,
    solution: SolutionExport,
}

#[derive(Serialize)]
struct SourceExport {
    problem: String,
    num_vertices: usize,
    num_edges: usize,
    edges: Vec<(usize, usize)>,
}

#[derive(Serialize)]
struct TargetExport {
    problem: String,
    universe_size: usize,
    num_sets: usize,
    sets: Vec<Vec<usize>>,
}

#[derive(Serialize)]
struct SolutionExport {
    source_config: Vec<usize>,
    source_size: i32,
    target_config: Vec<usize>,
    target_size: i32,
}

fn main() {
    println!("\n=== Vertex Cover -> Set Covering Reduction ===\n");

    // Triangle K3: 3 vertices, 3 edges
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let source = VertexCovering::<SimpleGraph, i32>::new(3, edges.clone());

    println!("Source: VertexCovering on K3");
    println!("  Vertices: 3");
    println!("  Edges: {:?}", edges);

    // Reduce to SetCovering
    let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    println!("\nTarget: SetCovering");
    println!("  Universe size: {}", target.universe_size());
    println!("  Sets: {} sets", target.num_sets());
    for (i, set) in target.sets().iter().enumerate() {
        println!("    S_{} = {:?}", i, set);
    }

    // Solve the target problem
    let solver = BruteForce::new();
    let solutions = solver.find_best(target);

    println!("\nBest target solutions: {}", solutions.len());

    // Extract and verify each solution
    for (i, target_sol) in solutions.iter().enumerate() {
        let source_sol = reduction.extract_solution(target_sol);
        let source_size = source.solution_size(&source_sol);
        let target_size = target.solution_size(target_sol);

        println!(
            "  Solution {}: target={:?} (size={}), source={:?} (size={}, valid={})",
            i, target_sol, target_size.size, source_sol, source_size.size, source_size.is_valid
        );

        assert!(
            source_size.is_valid,
            "Extracted source solution must be valid"
        );
    }

    // Use the first solution for JSON export
    let target_sol = &solutions[0];
    let source_sol = reduction.extract_solution(target_sol);
    let source_size = source.solution_size(&source_sol);
    let target_size = target.solution_size(target_sol);

    assert_eq!(source_size.size, 2, "VC on K3 has optimal size 2");
    assert_eq!(target_size.size, 2, "SetCovering should also have size 2");

    // Export JSON
    let export = ReductionExport {
        reduction: "VertexCovering -> SetCovering".to_string(),
        source: SourceExport {
            problem: "VertexCovering".to_string(),
            num_vertices: 3,
            num_edges: edges.len(),
            edges: edges.clone(),
        },
        target: TargetExport {
            problem: "SetCovering".to_string(),
            universe_size: target.universe_size(),
            num_sets: target.num_sets(),
            sets: target.sets().to_vec(),
        },
        solution: SolutionExport {
            source_config: source_sol,
            source_size: source_size.size,
            target_config: target_sol.clone(),
            target_size: target_size.size,
        },
    };

    let output_path = Path::new("docs/paper/examples/vc_to_setcovering.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&export).expect("Failed to serialize");
    fs::write(output_path, &json).expect("Failed to write JSON");
    println!("\nExported: {}", output_path.display());

    println!("\nDone: VC(K3) optimal=2 maps to SetCovering optimal=2");
}
