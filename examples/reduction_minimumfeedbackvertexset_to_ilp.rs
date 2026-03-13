// # Feedback Vertex Set to ILP Reduction
//
// ## Mathematical Formulation
// Uses MTZ-style topological ordering constraints:
// Variables: n binary x_i (vertex removal) + n integer o_i (topological order)
// Constraints: For each arc (u->v): o_v - o_u >= 1 - n*(x_u + x_v)
// Plus binary bounds (x_i <= 1) and order bounds (o_i <= n-1)
// Objective: minimize sum w_i * x_i
//
// ## This Example
// - Instance: "Cycle of triangles" (9 vertices, 15 arcs), FVS = 3
// - Source MinimumFeedbackVertexSet: 9 vertices, unit weights
// - Target ILP: 18 variables (9 binary + 9 integer), 33 constraints
//
// ## Output
// Exports `docs/paper/examples/minimumfeedbackvertexset_to_ilp.json` and
// `minimumfeedbackvertexset_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::models::algebraic::ILP;
use problemreductions::models::graph::MinimumFeedbackVertexSet;
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;
use problemreductions::topology::DirectedGraph;

pub fn run() {
    // 1. Create MinimumFeedbackVertexSet instance: cycle of triangles
    // Three directed triangles (0-1-2, 3-4-5, 6-7-8) connected in a cycle
    let arcs = vec![
        (0, 1),
        (1, 2),
        (2, 0), // triangle 0-1-2
        (3, 4),
        (4, 5),
        (5, 3), // triangle 3-4-5
        (6, 7),
        (7, 8),
        (8, 6), // triangle 6-7-8
        (1, 3),
        (4, 6),
        (7, 0), // inter-triangle forward arcs
        (2, 5),
        (5, 8),
        (8, 2), // inter-triangle cross arcs
    ];
    let graph = DirectedGraph::new(9, arcs.clone());
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 9]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MinimumFeedbackVertexSet with {} vertices, {} arcs",
        problem.num_variables(),
        arcs.len()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP using ILP solver (BruteForce can't handle ILP<i32>)
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    println!("\n=== Solution ===");
    println!("ILP solution: {:?}", &ilp_solution[..9]);

    // 5. Extract source solution
    let fvs_solution = reduction.extract_solution(&ilp_solution);
    println!("Source FVS solution: {:?}", fvs_solution);

    // 6. Verify
    let size = problem.evaluate(&fvs_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid());
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let solutions = vec![SolutionPair {
        source_config: fvs_solution.clone(),
        target_config: ilp_solution.clone(),
    }];

    let source_variant = variant_to_map(MinimumFeedbackVertexSet::<i32>::variant());
    let target_variant = variant_to_map(ILP::<i32>::variant());
    let overhead = lookup_overhead(
        "MinimumFeedbackVertexSet",
        &source_variant,
        "ILP",
        &target_variant,
    )
    .unwrap_or_default();

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumFeedbackVertexSet::<i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": 9,
                "num_arcs": 15,
                "arcs": arcs,
            }),
        },
        target: ProblemSide {
            problem: ILP::<i32>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "minimumfeedbackvertexset_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
