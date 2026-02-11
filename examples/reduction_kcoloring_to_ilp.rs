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
//! - Instance: Petersen graph (10 vertices, 15 edges) with 3 colors, χ=3
//! - Source KColoring: feasible, each vertex gets a color such that no adjacent vertices share a color
//! - Target ILP: 30 binary variables (10 vertices * 3 colors), many constraints
//!
//! ## Output
//! Exports `docs/paper/examples/kcoloring_to_ilp.json` and `kcoloring_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create KColoring instance: Petersen graph (10 vertices, 15 edges) with 3 colors, χ=3
    let (num_vertices, edges) = petersen();
    let coloring = KColoring::<3, SimpleGraph, i32>::new(num_vertices, edges.clone());

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&coloring);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: KColoring<3> with {} variables", coloring.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP using HiGHS solver (BruteForce on 30 vars is too slow)
    let solver = ILPSolver::new();
    let ilp_solution = solver.solve(ilp).expect("ILP should be feasible");
    println!("\n=== Solution ===");
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let coloring_solution = reduction.extract_solution(&ilp_solution);
    println!("Source Coloring solution: {:?}", coloring_solution);

    // 6. Verify
    let size = coloring.solution_size(&coloring_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    let source_sol = reduction.extract_solution(&ilp_solution);
    let s = coloring.solution_size(&source_sol);
    assert!(s.is_valid);
    solutions.push(SolutionPair {
        source_config: source_sol,
        target_config: ilp_solution,
    });

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
