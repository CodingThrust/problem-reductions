//! # Factoring to ILP Reduction
//!
//! ## Mathematical Formulation
//! Uses McCormick linearization for binary products with carry propagation.
//! Variables: p_i, q_j (factor bits), z_ij (product bits), c_k (carries).
//! Constraints:
//!   (1) McCormick: z_ij <= p_i, z_ij <= q_j, z_ij >= p_i + q_j - 1
//!   (2) Bit equations: sum_{i+j=k} z_ij + c_{k-1} = N_k + 2*c_k
//!   (3) No overflow: c_{m+n-1} = 0
//! Objective: feasibility (minimize 0).
//!
//! ## This Example
//! - Instance: Factor 15 = 3 * 5 (m=4 bits, n=4 bits)
//! - NOTE: Uses ILPSolver (not BruteForce) since the ILP has many variables
//! - Target ILP: 4+4+16+8 = 32 variables
//!
//! ## Output
//! Exports `docs/paper/examples/factoring_to_ilp.json` for use in paper code blocks.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;

fn main() {
    // 1. Create Factoring instance: find p (4-bit) x q (4-bit) = 15
    let problem = Factoring::new(4, 4, 15);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: Factoring with {} variables ({}+{} bits)", problem.num_variables(), problem.m(), problem.n());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve ILP using ILPSolver (too many variables for BruteForce)
    let solver = ILPSolver::new();
    let ilp_solution = solver.solve(ilp).expect("ILP should be feasible for 15 = 3 * 5");
    println!("\n=== Solution ===");
    println!("ILP solution found (first 8 vars): {:?}", &ilp_solution[..8]);

    // 5. Extract factoring solution
    let extracted = reduction.extract_solution(&ilp_solution);
    println!("Source Factoring solution: {:?}", extracted);

    // 6. Verify: read factors and confirm p * q = 15
    let (p, q) = problem.read_factors(&extracted);
    println!("Factors: {} x {} = {}", p, q, p * q);
    assert_eq!(p * q, 15);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let solutions = vec![SolutionPair {
        source_config: extracted,
        target_config: ilp_solution,
    }];

    let overhead = lookup_overhead("Factoring", "ILP")
        .expect("Factoring -> ILP overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: Factoring::NAME.to_string(),
            variant: variant_to_map(Factoring::variant()),
            instance: serde_json::json!({
                "number": problem.target(),
                "num_bits_first": problem.m(),
                "num_bits_second": problem.n(),
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
    write_example("factoring_to_ilp", &data, &results);
}
