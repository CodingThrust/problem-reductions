// # Vertex Cover to ILP via Reduction Path
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges), VC = 6
// - Source VC: min size 6
// - Target: ILP reached through the reduction graph
//
// ## Output
// Exports `docs/paper/examples/minimumvertexcover_to_ilp.json` and
// `minimumvertexcover_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::models::algebraic::ILP;
use problemreductions::prelude::*;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::{Graph, SimpleGraph};
use problemreductions::types::ProblemSize;

pub fn run() {
    let (num_vertices, edges) = petersen();
    let vc = MinimumVertexCover::new(
        SimpleGraph::new(num_vertices, edges.clone()),
        vec![1i32; num_vertices],
    );

    let graph = ReductionGraph::new();
    let src_variant_bt =
        ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let dst_variant_bt = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let path = graph
        .find_cheapest_path(
            "MinimumVertexCover",
            &src_variant_bt,
            "ILP",
            &dst_variant_bt,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .expect("MinimumVertexCover -> ILP path not found");
    let reduction = graph
        .reduce_along_path(&path, &vc as &dyn std::any::Any)
        .expect("MinimumVertexCover -> ILP path reduction failed");
    let ilp: &ILP<bool> = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MinimumVertexCover with {} variables",
        vc.num_variables()
    );
    println!("Path: {}", path);
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    let solver = BruteForce::new();
    let ilp_solutions = solver.find_all_best(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0];
    println!("ILP solution: {:?}", ilp_solution);

    let vc_solution = reduction.extract_solution(ilp_solution);
    println!("Source VC solution: {:?}", vc_solution);

    let size = vc.evaluate(&vc_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid());
    println!("\nReduction verified successfully");

    let mut solutions = Vec::new();
    for target_config in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = vc.evaluate(&source_sol);
        assert!(s.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let source_variant = variant_to_map(MinimumVertexCover::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(ILP::<bool>::variant());
    let overhead = graph.compose_path_overhead(&path);

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": vc.graph().num_vertices(),
                "num_edges": vc.graph().num_edges(),
                "edges": vc.graph().edges(),
            }),
        },
        target: ProblemSide {
            problem: ILP::<bool>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("minimumvertexcover_to_ilp", &data, &results);
}

fn main() {
    run()
}
