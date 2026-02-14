// # Vertex Covering to QUBO Reduction (Penalty Method)
//
// ## Mathematical Relationship
// The Minimum Vertex Cover (MVC) problem on a graph G = (V, E) is mapped to
// QUBO by constructing a penalty Hamiltonian:
//
//   H(x) = sum_{i in V} x_i + P * sum_{(i,j) in E} (1 - x_i)(1 - x_j)
//
// where P is a penalty weight ensuring every edge has at least one endpoint
// selected. The QUBO minimization finds configurations that minimize the
// number of selected vertices while covering all edges.
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges), VC=6
// - Source: MinimumVertexCover with minimum size 6
// - QUBO variables: 10 (one per vertex)
// - Expected: Optimal vertex covers of size 6
//
// ## Output
// Exports `docs/paper/examples/minimumvertexcover_to_qubo.json` and `minimumvertexcover_to_qubo.result.json`.
//
// ## Usage
// ```bash
// cargo run --example reduction_vc_to_qubo
// ```

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

pub fn run() {
    println!("=== Vertex Covering -> QUBO Reduction ===\n");

    // Petersen graph: 10 vertices, 15 edges, VC=6
    let (num_vertices, edges) = petersen();
    let vc = MinimumVertexCover::<SimpleGraph, i32>::new(num_vertices, edges.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&vc);
    let qubo = reduction.target_problem();

    println!("Source: MinimumVertexCover on Petersen graph (10 vertices, 15 edges)");
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix:");
    for row in qubo.matrix() {
        println!("  {:?}", row);
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    // Extract and verify solutions
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

        // Closed-loop verification: check solution is valid in original problem
        // MinimumVertexCover is a minimization problem, infeasible configs return Invalid
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

    // All optimal solutions should have size 6
    assert!(
        solutions
            .iter()
            .all(|s| s.source_config.iter().filter(|&&x| x == 1).count() == 6),
        "All optimal VC solutions on Petersen graph should have size 6"
    );
    println!("\nVerification passed: all solutions are valid with size 6");

    // Export JSON
    let source_variant = variant_to_map(MinimumVertexCover::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead =
        lookup_overhead("MinimumVertexCover", &source_variant, "QUBO", &target_variant)
            .expect("MinimumVertexCover -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": vc.num_vertices(),
                "num_edges": vc.num_edges(),
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
    let name = "minimumvertexcover_to_qubo";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
