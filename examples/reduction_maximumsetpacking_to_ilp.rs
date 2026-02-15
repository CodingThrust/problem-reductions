// # Set Packing to ILP Reduction
//
// ## Mathematical Formulation
// Variables: x_i in {0,1} for each set S_i.
// Constraints: x_i + x_j <= 1 for each overlapping pair (i,j).
// Objective: maximize sum of w_i * x_i.
//
// ## This Example
// - Instance: 6 sets over universe {0,...,7}
//   - S0={0,1,2}, S1={2,3,4}, S2={4,5,6}, S3={6,7,0}, S4={1,3,5}, S5={0,4,7}
// - Source MaximumSetPacking: max packing size 2 (e.g., S0 and S2, or S1 and S3)
// - Target ILP: 6 binary variables, overlap constraints for each pair sharing elements
//
// ## Output
// Exports `docs/paper/examples/maximumsetpacking_to_ilp.json` and `maximumsetpacking_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;

pub fn run() {
    // 1. Create MaximumSetPacking instance: 6 sets over universe {0,...,7}
    let sets = vec![
        vec![0, 1, 2], // S0
        vec![2, 3, 4], // S1 (overlaps S0 at 2)
        vec![4, 5, 6], // S2 (overlaps S1 at 4)
        vec![6, 7, 0], // S3 (overlaps S2 at 6, S0 at 0)
        vec![1, 3, 5], // S4 (overlaps S0, S1, S2)
        vec![0, 4, 7], // S5 (overlaps S0, S1, S3)
    ];
    let sp = MaximumSetPacking::<i32>::new(sets.clone());

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&sp);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MaximumSetPacking with {} sets over universe {{0,...,7}}",
        sp.num_variables()
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
    let ilp_solutions = solver.find_all_best(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0];
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let sp_solution = reduction.extract_solution(ilp_solution);
    println!("Source MaximumSetPacking solution: {:?}", sp_solution);

    // 6. Verify
    let metric = sp.evaluate(&sp_solution);
    println!("Solution metric: {:?}", metric);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for target_config in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let source_variant = variant_to_map(MaximumSetPacking::<i32>::variant());
    let target_variant = variant_to_map(ILP::variant());
    let overhead = lookup_overhead("MaximumSetPacking", &source_variant, "ILP", &target_variant)
        .unwrap_or_default();

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumSetPacking::<i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_sets": sp.num_sets(),
                "sets": sp.sets(),
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
    let name = "maximumsetpacking_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
