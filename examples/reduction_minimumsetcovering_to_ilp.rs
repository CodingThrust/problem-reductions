// # Set Covering to ILP Reduction
//
// ## Mathematical Formulation
// Variables: x_i in {0,1} for each set S_i.
// Constraints: sum_{S_i containing e} x_i >= 1 for each element e in universe.
// Objective: minimize sum of w_i * x_i.
//
// ## This Example
// - Instance: Universe size 8, 6 sets
//   - S0={0,1,2}, S1={2,3,4}, S2={4,5,6}, S3={6,7,0}, S4={1,3,5}, S5={0,4,7}
// - Source MinimumSetCovering: every element in {0,...,7} must be covered
// - Target ILP: 6 binary variables, 8 element-coverage constraints
//
// ## Output
// Exports `docs/paper/examples/minimumsetcovering_to_ilp.json` and `minimumsetcovering_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use std::collections::HashMap;

pub fn run() {
    // 1. Create MinimumSetCovering instance: universe {0,...,7}, 6 sets
    let sets = vec![
        vec![0, 1, 2], // S0
        vec![2, 3, 4], // S1
        vec![4, 5, 6], // S2
        vec![6, 7, 0], // S3
        vec![1, 3, 5], // S4
        vec![0, 4, 7], // S5
    ];
    let sc = MinimumSetCovering::<i32>::new(8, sets.clone());

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&sc);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MinimumSetCovering with {} sets over universe {{0,...,7}}",
        sc.num_variables()
    );
    for (i, s) in sets.iter().enumerate() {
        println!("  S{} = {:?}", i, s);
    }
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0];
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let sc_solution = reduction.extract_solution(ilp_solution);
    println!("Source MinimumSetCovering solution: {:?}", sc_solution);

    // 6. Verify
    let size = sc.evaluate(&sc_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid()); // Valid solution
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for target_config in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = sc.evaluate(&source_sol);
        assert!(s.is_valid()); // Valid solution
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let overhead = lookup_overhead_or_empty("MinimumSetCovering", "ILP");

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumSetCovering::<i32>::NAME.to_string(),
            variant: HashMap::new(),
            instance: serde_json::json!({
                "num_sets": sc.num_sets(),
                "sets": sc.sets(),
                "universe_size": sc.universe_size(),
            }),
        },
        target: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: HashMap::new(),
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "minimumsetcovering_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
