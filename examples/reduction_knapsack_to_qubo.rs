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
// Exports `docs/paper/examples/generated/knapsack_to_qubo.json` by default.

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

    // Export JSON using the merged rule-example format.
    let source = ProblemSide::from_problem(&knapsack);
    let target = ProblemSide::from_problem(qubo);
    let overhead = lookup_overhead(
        &source.problem,
        &source.variant,
        &target.problem,
        &target.variant,
    )
    .expect("Knapsack -> QUBO overhead not found");

    let example = RuleExample {
        source,
        target,
        overhead: overhead_to_json(&overhead),
        solutions,
    };
    write_rule_example("knapsack_to_qubo", &example);
}

fn main() {
    run()
}
