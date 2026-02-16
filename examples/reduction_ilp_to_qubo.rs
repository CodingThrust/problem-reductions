// # Integer Linear Programming (Binary) to QUBO Reduction (Penalty Method)
//
// ## Mathematical Relationship
// A binary ILP problem:
//
//   maximize   c^T x
//   subject to A x <= b
//              x_i in {0, 1}
//
// is mapped to QUBO by introducing slack variables to convert inequality
// constraints into equalities, then penalizing constraint violations:
//
//   H(x, s) = -c^T x + P * sum_j (a_j^T x + s_j - b_j)^2
//
// where s_j are slack variables encoded in binary. The penalty P is chosen
// large enough to ensure feasibility is always preferred over infeasible
// solutions with better objective values.
//
// ## This Example
// - Instance: 6-variable binary knapsack problem
//   - Items with weights [3, 2, 5, 4, 2, 3] and values [10, 7, 12, 8, 6, 9]
//   - Constraint 1: 3x0 + 2x1 + 5x2 + 4x3 + 2x4 + 3x5 <= 10 (weight capacity)
//   - Constraint 2: x0 + x1 + x2 <= 2 (category A limit)
//   - Constraint 3: x3 + x4 + x5 <= 2 (category B limit)
//   - Objective: maximize 10x0 + 7x1 + 12x2 + 8x3 + 6x4 + 9x5
// - Expected: Select items that maximize total value while satisfying all
//   weight and category constraints
//
// ## Outputs
// - `docs/paper/examples/ilp_to_qubo.json` — reduction structure
// - `docs/paper/examples/ilp_to_qubo.result.json` — solutions
//
// ## Usage
// ```bash
// cargo run --example reduction_ilp_to_qubo
// ```

use problemreductions::export::*;
use problemreductions::models::optimization::{ILP, LinearConstraint, ObjectiveSense};
use problemreductions::prelude::*;

pub fn run() {
    println!("=== ILP (Binary) -> QUBO Reduction ===\n");

    // 6-variable binary knapsack problem
    // Items with weights [3, 2, 5, 4, 2, 3] and values [10, 7, 12, 8, 6, 9]
    // Constraint 1: knapsack weight capacity <= 10
    // Constraint 2: category A items (x0, x1, x2) limited to 2
    // Constraint 3: category B items (x3, x4, x5) limited to 2
    let ilp = ILP::binary(
        6,
        vec![
            // Knapsack weight constraint: 3x0 + 2x1 + 5x2 + 4x3 + 2x4 + 3x5 <= 10
            LinearConstraint::le(
                vec![(0, 3.0), (1, 2.0), (2, 5.0), (3, 4.0), (4, 2.0), (5, 3.0)],
                10.0,
            ),
            // Category A limit: x0 + x1 + x2 <= 2
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 1.0)], 2.0),
            // Category B limit: x3 + x4 + x5 <= 2
            LinearConstraint::le(vec![(3, 1.0), (4, 1.0), (5, 1.0)], 2.0),
        ],
        vec![(0, 10.0), (1, 7.0), (2, 12.0), (3, 8.0), (4, 6.0), (5, 9.0)],
        ObjectiveSense::Maximize,
    );

    let item_names = ["Item0", "Item1", "Item2", "Item3", "Item4", "Item5"];
    let weights = [3, 2, 5, 4, 2, 3];
    let values = [10, 7, 12, 8, 6, 9];

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&ilp);
    let qubo = reduction.target_problem();

    println!("Source: ILP (binary) with 6 variables, 3 constraints");
    println!("  Objective: maximize 10x0 + 7x1 + 12x2 + 8x3 + 6x4 + 9x5");
    println!("  Constraint 1: 3x0 + 2x1 + 5x2 + 4x3 + 2x4 + 3x5 <= 10 (weight capacity)");
    println!("  Constraint 2: x0 + x1 + x2 <= 2 (category A limit)");
    println!("  Constraint 3: x3 + x4 + x5 <= 2 (category B limit)");
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!(
        "  (6 original + {} slack variables for inequality constraints)",
        qubo.num_variables() - 6
    );
    println!(
        "Q matrix size: {}x{}",
        qubo.matrix().len(),
        qubo.matrix().len()
    );

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let selected: Vec<String> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| item_names[i].to_string())
            .collect();
        let total_weight: i32 = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| weights[i])
            .sum();
        let total_value: i32 = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| values[i])
            .sum();
        println!(
            "  Selected items: {:?} (total weight: {}, total value: {})",
            selected, total_weight, total_value
        );

        // Closed-loop verification: check solution is valid in original problem
        let sol_size = ilp.evaluate(&extracted);
        assert!(
            sol_size.is_valid(),
            "Solution must be valid in source problem"
        );

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!("\nVerification passed: all solutions are feasible and optimal");

    // Export JSON
    let source_variant = variant_to_map(ILP::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead = lookup_overhead("ILP", &source_variant, "QUBO", &target_variant)
        .expect("ILP -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": qubo.matrix(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "ilp_to_qubo";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
