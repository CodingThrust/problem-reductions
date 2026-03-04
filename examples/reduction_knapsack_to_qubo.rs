// # Knapsack to QUBO Reduction
//
// ## Reduction Overview
// The 0-1 Knapsack capacity constraint sum(w_i * x_i) <= C is converted to equality
// using B = floor(log2(C)) + 1 binary slack variables. The QUBO objective combines
// -sum(v_i * x_i) with penalty P * (sum(w_i * x_i) + sum(2^j * s_j) - C)^2 where P > sum(v_i).
//
// ## This Example
// - 4 items: weights=[2,3,4,5], values=[3,4,5,7], capacity=7
// - QUBO: 7 variables (4 items + 3 slack bits)
// - Optimal: items {0,3} (weight=7, value=10)
//
// ## Output
// Exports `docs/paper/examples/knapsack_to_qubo.json` and `knapsack_to_qubo.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;

pub fn run() {
    // Source: Knapsack with 4 items, capacity 7
    let knapsack = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: Knapsack with {} items, capacity {}",
        knapsack.num_items(),
        knapsack.capacity()
    );
    println!("Target: QUBO with {} variables", qubo.num_vars());

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", qubo_solutions.len());

    let mut solutions = Vec::new();
    for target_sol in &qubo_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let eval = knapsack.evaluate(&source_sol);
        assert!(eval.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol.clone(),
            target_config: target_sol.clone(),
        });
    }

    let source_sol = reduction.extract_solution(&qubo_solutions[0]);
    println!("Source solution: {:?}", source_sol);
    println!("Source value: {:?}", knapsack.evaluate(&source_sol));
    println!("\nReduction verified successfully");

    // Export JSON
    let source_variant = variant_to_map(Knapsack::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead = lookup_overhead("Knapsack", &source_variant, "QUBO", &target_variant)
        .expect("Knapsack -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: Knapsack::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_items": knapsack.num_items(),
                "weights": knapsack.weights(),
                "values": knapsack.values(),
                "capacity": knapsack.capacity(),
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("knapsack_to_qubo", &data, &results);
}

fn main() {
    run()
}
