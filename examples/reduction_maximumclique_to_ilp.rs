// # MaximumClique to ILP Reduction
//
// ## Mathematical Formulation
// Variables: x_v in {0,1} for each vertex v.
// Constraints: x_u + x_v <= 1 for each non-edge (u,v) not in E.
// Objective: maximize sum of w_v * x_v.
//
// ## This Example
// - Instance: Octahedron graph (K_{2,2,2}) with 6 vertices and 12 edges.
// - Source MaximumClique: max clique is size 3
// - Target ILP: 6 binary variables, 3 non-edge constraints
//   (non-edges: opposite vertex pairs (0,5), (1,4), (2,3))
//
// ## Output
// Exports `docs/paper/examples/maximumclique_to_ilp.json` and `maximumclique_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::octahedral;
use problemreductions::topology::SimpleGraph;

pub fn run() {
    // 1. Create MaximumClique instance: Octahedron (K_{2,2,2}), 6 vertices, 12 edges, clique number 3
    let (num_vertices, edges) = octahedral();
    let clique = MaximumClique::<SimpleGraph, i32>::new(num_vertices, edges.clone());

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&clique);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MaximumClique with {} variables",
        clique.num_variables()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_all_best(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0];
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let clique_solution = reduction.extract_solution(ilp_solution);
    println!("Source MaximumClique solution: {:?}", clique_solution);

    // 6. Verify
    let size = clique.evaluate(&clique_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid()); // Valid solution
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for target_config in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = clique.evaluate(&source_sol);
        assert!(s.is_valid()); // Valid solution
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let source_variant = variant_to_map(MaximumClique::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(ILP::variant());
    let overhead = lookup_overhead("MaximumClique", &source_variant, "ILP", &target_variant)
        .unwrap_or_default();

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumClique::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": clique.num_vertices(),
                "num_edges": clique.num_edges(),
                "edges": clique.edges(),
            }),
        },
        target: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximumclique_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
