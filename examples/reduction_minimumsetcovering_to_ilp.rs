//! # Set Covering to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_i in {0,1} for each set S_i.
//! Constraints: sum_{S_i containing e} x_i >= 1 for each element e in universe.
//! Objective: minimize sum of w_i * x_i.
//!
//! ## This Example
//! - Instance: Universe size 3, sets: S0={0,1}, S1={1,2}, S2={0,2}
//! - Source MinimumSetCovering: min cover size 2 (any two sets cover all elements)
//! - Target ILP: 3 binary variables, 3 element-coverage constraints
//!
//! ## Output
//! Exports `docs/paper/examples/msc_to_ilp.json` for use in paper code blocks.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;

fn main() {
    // 1. Create MinimumSetCovering instance: universe {0,1,2}, 3 sets
    let sc = MinimumSetCovering::<i32>::new(
        3,
        vec![
            vec![0, 1],
            vec![1, 2],
            vec![0, 2],
        ],
    );

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&sc);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: MinimumSetCovering with {} variables", sc.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let sc_solution = reduction.extract_solution(ilp_solution);
    println!("Source MinimumSetCovering solution: {:?}", sc_solution);

    // 6. Verify
    let size = sc.solution_size(&sc_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for (target_config, _score) in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = sc.solution_size(&source_sol);
        assert!(s.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let overhead = lookup_overhead_or_empty("MinimumSetCovering", "ILP");

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumSetCovering::<i32>::NAME.to_string(),
            variant: variant_to_map(MinimumSetCovering::<i32>::variant()),
            instance: serde_json::json!({
                "num_sets": sc.num_sets(),
                "sets": sc.sets(),
                "universe_size": sc.universe_size(),
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
    write_example("msc_to_ilp", &data, &results);
}
