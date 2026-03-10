// # Independent Set to QUBO via Reduction Path
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges, 3-regular)
// - Source: MaximumIndependentSet with maximum size 4
// - Target: QUBO reached through the reduction graph
//
// ## Output
// Exports `docs/paper/examples/maximumindependentset_to_qubo.json` and
// `maximumindependentset_to_qubo.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::rules::{Minimize, ReductionGraph};
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::{Graph, SimpleGraph};
use problemreductions::types::ProblemSize;

pub fn run() {
    println!("=== Independent Set -> QUBO Reduction ===\n");

    let (num_vertices, edges) = petersen();
    let is = MaximumIndependentSet::new(
        SimpleGraph::new(num_vertices, edges.clone()),
        vec![1i32; num_vertices],
    );

    let graph = ReductionGraph::new();
    let src_variant_bt =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst_variant_bt = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let path = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src_variant_bt,
            "QUBO",
            &dst_variant_bt,
            &ProblemSize::new(vec![
                ("num_vertices", is.graph().num_vertices()),
                ("num_edges", is.graph().num_edges()),
            ]),
            &Minimize("num_vars"),
        )
        .expect("MaximumIndependentSet -> QUBO path not found");
    let reduction = graph
        .reduce_along_path(&path, &is as &dyn std::any::Any)
        .expect("MaximumIndependentSet -> QUBO path reduction failed");
    let qubo: &QUBO<f64> = reduction.target_problem();

    println!("Source: MaximumIndependentSet on Petersen graph (10 vertices, 15 edges)");
    println!("Path: {}", path);
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix:");
    for row in qubo.matrix() {
        println!("  {:?}", row);
    }

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    println!("\nOptimal solutions:");
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let sol_size = is.evaluate(&extracted);
        assert!(
            sol_size.is_valid(),
            "Solution must be valid in source problem"
        );

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

    let source_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead = graph.compose_path_overhead(&path);

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": is.graph().num_vertices(),
                "num_edges": is.graph().num_edges(),
                "edges": edges,
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
    write_example("maximumindependentset_to_qubo", &data, &results);
}

fn main() {
    run()
}
