// # Vertex Cover to QUBO via Reduction Path
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges), VC = 6
// - Source: MinimumVertexCover
// - Target: QUBO reached through the reduction graph
//
// ## Output
// Exports `docs/paper/examples/minimumvertexcover_to_qubo.json` and
// `minimumvertexcover_to_qubo.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::rules::{Minimize, ReductionGraph};
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::{Graph, SimpleGraph};
use problemreductions::types::ProblemSize;

pub fn run() {
    println!("=== Vertex Cover -> QUBO Reduction ===\n");

    let (num_vertices, edges) = petersen();
    let vc = MinimumVertexCover::new(
        SimpleGraph::new(num_vertices, edges.clone()),
        vec![1i32; num_vertices],
    );

    let graph = ReductionGraph::new();
    let src_variant_bt =
        ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let dst_variant_bt = ReductionGraph::variant_to_map(&QUBO::<f64>::variant());
    let path = graph
        .find_cheapest_path(
            "MinimumVertexCover",
            &src_variant_bt,
            "QUBO",
            &dst_variant_bt,
            &ProblemSize::new(vec![
                ("num_vertices", vc.graph().num_vertices()),
                ("num_edges", vc.graph().num_edges()),
            ]),
            &Minimize("num_vars"),
        )
        .expect("MinimumVertexCover -> QUBO path not found");
    let reduction = graph
        .reduce_along_path(&path, &vc as &dyn std::any::Any)
        .expect("MinimumVertexCover -> QUBO path reduction failed");
    let qubo: &QUBO<f64> = reduction.target_problem();

    println!("Source: MinimumVertexCover on Petersen graph (10 vertices, 15 edges)");
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
        let selected: Vec<usize> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect();
        let size = selected.len();
        println!("  Cover vertices: {:?} ({} vertices)", selected, size);

        let sol_size = vc.evaluate(&extracted);
        assert!(
            sol_size.is_valid(),
            "Solution must be valid in source problem"
        );

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!("\nVerification passed: all solutions are valid");

    let source_variant = variant_to_map(MinimumVertexCover::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead = graph.compose_path_overhead(&path);

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": vc.graph().num_vertices(),
                "num_edges": vc.graph().num_edges(),
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
    write_example("minimumvertexcover_to_qubo", &data, &results);
}

fn main() {
    run()
}
