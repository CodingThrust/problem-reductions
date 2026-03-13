// # Bin Packing to ILP Reduction
//
// ## Mathematical Formulation
// Variables: x_{ij} in {0,1} (item i in bin j), y_j in {0,1} (bin j used).
// Constraints:
//   Assignment: sum_j x_{ij} = 1 for each item i.
//   Capacity: sum_i w_i * x_{ij} <= C * y_j for each bin j.
// Objective: minimize sum_j y_j.
//
// ## This Example
// - Instance: 5 items with weights [6, 5, 5, 4, 3], bin capacity 10
// - Optimal: 3 bins (e.g., {6,4}, {5,5}, {3})
// - Target ILP: 30 binary variables (25 assignment + 5 bin-open), 10 constraints
//
// ## Output
// Exports `docs/paper/examples/binpacking_to_ilp.json` and `binpacking_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::models::algebraic::ILP;
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;
use problemreductions::types::SolutionSize;

pub fn run() {
    // 1. Create BinPacking instance: 5 items, capacity 10
    let weights = vec![6, 5, 5, 4, 3];
    let capacity = 10;
    let bp = BinPacking::new(weights.clone(), capacity);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&bp);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: BinPacking with {} items, weights {:?}, capacity {}",
        bp.num_items(),
        bp.sizes(),
        bp.capacity()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP using ILP solver (BruteForce would be too slow: 2^30 configs)
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");

    println!("\n=== Solution ===");

    // 5. Extract source solution
    let bp_solution = reduction.extract_solution(&ilp_solution);
    println!("Source BinPacking solution (bin assignments): {:?}", bp_solution);

    // 6. Verify
    let size = bp.evaluate(&bp_solution);
    println!("Number of bins used: {:?}", size);
    assert!(size.is_valid());
    assert_eq!(size, SolutionSize::Valid(3));
    println!("\nReduction verified successfully");

    // 7. Collect solution and export JSON
    let mut solutions = Vec::new();
    {
        let source_sol = reduction.extract_solution(&ilp_solution);
        let s = bp.evaluate(&source_sol);
        assert!(s.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: ilp_solution.clone(),
        });
    }

    let source_variant = variant_to_map(BinPacking::<i32>::variant());
    let target_variant = variant_to_map(ILP::<bool>::variant());
    let overhead = lookup_overhead(
        "BinPacking",
        &source_variant,
        "ILP",
        &target_variant,
    )
    .unwrap_or_default();

    let data = ReductionData {
        source: ProblemSide {
            problem: BinPacking::<i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_items": bp.num_items(),
                "sizes": bp.sizes(),
                "capacity": bp.capacity(),
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
    let name = "binpacking_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
