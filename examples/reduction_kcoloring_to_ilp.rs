//! # K-Coloring to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_{v,c} in {0,1} for each vertex v and color c.
//! Constraints:
//!   (1) sum_c x_{v,c} = 1 for each vertex v (exactly one color).
//!   (2) x_{u,c} + x_{v,c} <= 1 for each edge (u,v) and color c (different colors on adjacent).
//! Objective: feasibility (minimize 0).
//!
//! ## This Example
//! - Instance: Triangle K3 (3 vertices, 3 edges) with 3 colors
//! - Source KColoring: feasible, each vertex gets a distinct color
//! - Target ILP: 9 binary variables (3 vertices * 3 colors), 12 constraints
//!
//! ## Output
//! Exports `docs/paper/examples/kcoloring_to_ilp.json` and `kcoloring_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create KColoring instance: triangle K3 with 3 colors
    let coloring = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&coloring);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: KColoring<3> with {} variables", coloring.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let coloring_solution = reduction.extract_solution(ilp_solution);
    println!("Source Coloring solution: {:?}", coloring_solution);

    // 6. Verify
    let size = coloring.solution_size(&coloring_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for (target_config, _score) in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = coloring.solution_size(&source_sol);
        assert!(s.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let overhead = lookup_overhead("KColoring", "ILP")
        .expect("KColoring -> ILP overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: KColoring::<3, SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(KColoring::<3, SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": coloring.num_vertices(),
                "num_edges": coloring.num_edges(),
                "num_colors": 3,
            }),
        },
        target: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: variant_to_map(ILP::variant()),
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
