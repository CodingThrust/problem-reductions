// # Independent Set to ILP via Reduction Path
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges, 3-regular)
// - Source IS: max size 4
// - Target: ILP reached through the reduction graph
//
// ## Output
// Exports `docs/paper/examples/maximumindependentset_to_ilp.json` and
// `maximumindependentset_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::models::algebraic::ILP;
use problemreductions::prelude::*;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::{Graph, SimpleGraph};
use problemreductions::types::ProblemSize;

pub fn run() {
    let (num_vertices, edges) = petersen();
    let is = MaximumIndependentSet::new(
        SimpleGraph::new(num_vertices, edges.clone()),
        vec![1i32; num_vertices],
    );

    let graph = ReductionGraph::new();
    let src_variant_bt =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst_variant_bt = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let path = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src_variant_bt,
            "ILP",
            &dst_variant_bt,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .expect("MaximumIndependentSet -> ILP path not found");
    let reduction = graph
        .reduce_along_path(&path, &is as &dyn std::any::Any)
        .expect("MaximumIndependentSet -> ILP path reduction failed");
    let ilp: &ILP<bool> = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MaximumIndependentSet with {} variables",
        is.num_variables()
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

    let is_solution = reduction.extract_solution(ilp_solution);
    println!("Source IS solution: {:?}", is_solution);

    let size = is.evaluate(&is_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid());
    println!("\nReduction verified successfully");

    let mut solutions = Vec::new();
    for target_config in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = is.evaluate(&source_sol);
        assert!(s.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let source_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(ILP::<bool>::variant());
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
    write_example("maximumindependentset_to_ilp", &data, &results);
}

fn main() {
    run()
}
