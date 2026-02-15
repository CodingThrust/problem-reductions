// # Factoring to ILP Reduction
//
// ## Mathematical Formulation
// Uses McCormick linearization for binary products with carry propagation.
// Variables: p_i, q_j (factor bits), z_ij (product bits), c_k (carries).
// Constraints:
//   (1) McCormick: z_ij <= p_i, z_ij <= q_j, z_ij >= p_i + q_j - 1
//   (2) Bit equations: sum_{i+j=k} z_ij + c_{k-1} = N_k + 2*c_k
//   (3) No overflow: c_{m+n-1} = 0
// Objective: feasibility (minimize 0).
//
// ## This Example
// - Instance: Factor 35 = 5 × 7 (m=3 bits, n=3 bits)
// - NOTE: Uses ILPSolver (not BruteForce) since the ILP has many variables
// - Target ILP: ~21 variables (factor bits + product bits + carries)
//
// ## Output
// Exports `docs/paper/examples/factoring_to_ilp.json` for use in paper code blocks.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;

pub fn run() {
    // 1. Create Factoring instance: find p (3-bit) x q (3-bit) = 35
    let problem = Factoring::new(3, 3, 35);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: Factoring with {} variables ({}+{} bits)",
        problem.num_variables(),
        problem.m(),
        problem.n()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve ILP using ILPSolver (too many variables for BruteForce)
    let solver = ILPSolver::new();
    let ilp_solution = solver
        .solve(ilp)
        .expect("ILP should be feasible for 35 = 5 * 7");
    println!("\n=== Solution ===");
    println!(
        "ILP solution found (first 6 vars): {:?}",
        &ilp_solution[..6]
    );

    // 5. Extract factoring solution
    let extracted = reduction.extract_solution(&ilp_solution);
    println!("Source Factoring solution: {:?}", extracted);

    // 6. Verify: read factors and confirm p * q = 35
    let (p, q) = problem.read_factors(&extracted);
    println!("Factors: {} x {} = {}", p, q, p * q);
    assert_eq!(p * q, 35);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let solutions = vec![SolutionPair {
        source_config: extracted,
        target_config: ilp_solution,
    }];

    let source_variant = variant_to_map(Factoring::variant());
    let target_variant = variant_to_map(ILP::variant());
    let overhead = lookup_overhead("Factoring", &source_variant, "ILP", &target_variant)
        .expect("Factoring -> ILP overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: Factoring::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "number": problem.target(),
                "num_bits_first": problem.m(),
                "num_bits_second": problem.n(),
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
    let name = "factoring_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
