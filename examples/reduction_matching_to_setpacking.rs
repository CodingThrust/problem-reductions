//! # Matching to Set Packing Reduction
//!
//! ## Mathematical Equivalence
//! Each edge e = (u,v) becomes a set S_e = {u, v}. Universe U = V.
//! A matching (edges with no shared vertices) maps to a packing (sets with
//! no shared elements) with the same weight.
//!
//! ## This Example
//! - Instance: Path graph P4 (4 vertices, 3 edges) with unit weights
//! - Source matching: max size 2 (e.g., edges {(0,1), (2,3)})
//! - Target SetPacking: max packing 2
//!
//! ## Output
//! Exports `docs/paper/examples/matching_to_setpacking.json` for use in paper code blocks.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// JSON export structure for the Matching to SetPacking reduction example.
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
    println!("\n=== Matching -> Set Packing Reduction ===\n");

    // Path graph P4: 0-1-2-3 with unit weights
    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let source = Matching::<SimpleGraph, i32>::unweighted(4, edges.clone());

    println!("Source: Matching on P4");
    println!("  Vertices: 4");
    println!("  Edges: {:?}", edges);

    // Reduce to SetPacking
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    println!("\nTarget: SetPacking");
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

    assert_eq!(source_size.size, 2, "Matching on P4 has optimal size 2");
    assert_eq!(target_size.size, 2, "SetPacking should also have size 2");

    // Export JSON
    let export = ReductionExport {
        reduction: "Matching -> SetPacking".to_string(),
        source: SourceExport {
            problem: "Matching".to_string(),
            num_vertices: 4,
            num_edges: edges.len(),
            edges: edges.clone(),
        },
        target: TargetExport {
            problem: "SetPacking".to_string(),
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

    let output_path = Path::new("docs/paper/examples/matching_to_setpacking.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&export).expect("Failed to serialize");
    fs::write(output_path, &json).expect("Failed to write JSON");
    println!("\nExported: {}", output_path.display());

    println!("\nDone: Matching(P4) optimal=2 maps to SetPacking optimal=2");
}
