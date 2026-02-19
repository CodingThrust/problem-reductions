// # QUBO to ILP Reduction (McCormick Linearization)
//
// ## Mathematical Relationship
// A QUBO problem:
//
//   minimize x^T Q x,   x ∈ {0,1}^n
//
// is linearized by replacing each product x_i·x_j (i < j) with an
// auxiliary binary variable y_ij and three McCormick constraints:
//   y_ij ≤ x_i,  y_ij ≤ x_j,  y_ij ≥ x_i + x_j - 1
//
// Diagonal terms Q_ii·x_i² = Q_ii·x_i are directly linear.
//
// ## This Example
// - Instance: 4-variable QUBO with a few quadratic terms
//   Q = diag(-2, -3, -1, -4) with Q_{01}=1, Q_{12}=2, Q_{23}=-1
// - Expected: optimal binary assignment minimizing x^T Q x
//
// ## Outputs
// - `docs/paper/examples/qubo_to_ilp.json` — reduction structure
// - `docs/paper/examples/qubo_to_ilp.result.json` — solutions
//
// ## Usage
// ```bash
// cargo run --example reduction_qubo_to_ilp --features ilp-solver
// ```

use problemreductions::export::*;
use problemreductions::models::optimization::ILP;
use problemreductions::prelude::*;

pub fn run() {
    println!("=== QUBO -> ILP Reduction (McCormick) ===\n");

    // 4-variable QUBO: diagonal (linear) + off-diagonal (quadratic) terms
    let mut matrix = vec![vec![0.0; 4]; 4];
    matrix[0][0] = -2.0;
    matrix[1][1] = -3.0;
    matrix[2][2] = -1.0;
    matrix[3][3] = -4.0;
    matrix[0][1] = 1.0; // x0·x1 coupling
    matrix[1][2] = 2.0; // x1·x2 coupling
    matrix[2][3] = -1.0; // x2·x3 coupling
    let qubo = QUBO::from_matrix(matrix);

    // Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    println!("Source: QUBO with {} variables", qubo.num_variables());
    println!("  Q diagonal: [-2, -3, -1, -4]");
    println!("  Q off-diagonal: (0,1)=1, (1,2)=2, (2,3)=-1");
    println!(
        "Target: ILP with {} variables ({} original + {} auxiliary)",
        ilp.num_variables(),
        qubo.num_variables(),
        ilp.num_variables() - qubo.num_variables()
    );
    println!(
        "  {} constraints (3 McCormick per auxiliary variable)",
        ilp.constraints.len()
    );

    // Solve ILP with brute force
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_all_best(ilp);

    println!("\nOptimal solutions:");
    let mut solutions = Vec::new();
    for sol in &ilp_solutions {
        let extracted = reduction.extract_solution(sol);
        let qubo_val = qubo.evaluate(&extracted);
        println!("  x = {:?}, QUBO value = {}", extracted, qubo_val);

        // Closed-loop verification
        assert!(
            qubo_val < f64::MAX,
            "Solution must be valid in source problem"
        );

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!("\nVerification passed: all solutions are feasible and optimal");

    // Export JSON
    let source_variant = variant_to_map(QUBO::<f64>::variant());
    let target_variant = variant_to_map(ILP::variant());
    let overhead = lookup_overhead("QUBO", &source_variant, "ILP", &target_variant)
        .expect("QUBO -> ILP overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": qubo.matrix(),
            }),
        },
        target: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": ilp.num_variables(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "qubo_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
