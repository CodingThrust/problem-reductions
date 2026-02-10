//! # Set Packing to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_i in {0,1} for each set S_i.
//! Constraints: x_i + x_j <= 1 for each overlapping pair (i,j).
//! Objective: maximize sum of w_i * x_i.
//!
//! ## This Example
//! - Instance: 3 sets: S0={0,1}, S1={1,2}, S2={2,3,4}
//!   Overlapping pairs: (S0,S1) share element 1, (S1,S2) share element 2
//! - Source MaximumSetPacking: max packing size 2 (S0 and S2 are disjoint)
//! - Target ILP: 3 binary variables, 2 overlap constraints
//!
//! ## Output
//! Exports `docs/paper/examples/maximumsetpacking_to_ilp.json` and `maximumsetpacking_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;

fn main() {
    // 1. Create MaximumSetPacking instance: 3 sets
    let sp = MaximumSetPacking::<i32>::new(vec![
        vec![0, 1],
        vec![1, 2],
        vec![2, 3, 4],
    ]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&sp);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: MaximumSetPacking with {} variables", sp.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let sp_solution = reduction.extract_solution(ilp_solution);
    println!("Source MaximumSetPacking solution: {:?}", sp_solution);

    // 6. Verify
    let size = sp.solution_size(&sp_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for (target_config, _score) in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = sp.solution_size(&source_sol);
        assert!(s.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let overhead = lookup_overhead_or_empty("MaximumSetPacking", "ILP");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumSetPacking::<i32>::NAME.to_string(),
            variant: variant_to_map(MaximumSetPacking::<i32>::variant()),
            instance: serde_json::json!({
                "num_sets": sp.num_sets(),
                "sets": sp.sets(),
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
