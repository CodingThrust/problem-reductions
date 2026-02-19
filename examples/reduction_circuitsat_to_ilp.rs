// # Circuit-SAT to ILP Reduction
//
// ## Mathematical Equivalence
// Each logic gate (AND, OR, NOT, XOR) is encoded as linear constraints over
// binary variables. The expression tree is flattened by introducing an auxiliary
// variable per internal node (Tseitin-style). Any feasible ILP solution is a
// satisfying circuit assignment.
//
// ## This Example
// - Instance: 1-bit full adder circuit (a, b, cin -> sum, cout)
//   - sum = a XOR b XOR cin (via intermediate t = a XOR b)
//   - cout = (a AND b) OR (cin AND t)
//   - 5 gates (2 XOR, 2 AND, 1 OR), ~8 variables
// - Source: CircuitSAT with 3 inputs
// - Target: ILP (feasibility, trivial objective)
//
// ## Output
// Exports `docs/paper/examples/circuitsat_to_ilp.json` and `circuitsat_to_ilp.result.json`.
//
// ## Usage
// ```bash
// cargo run --example reduction_circuitsat_to_ilp --features ilp-solver
// ```

use problemreductions::export::*;
use problemreductions::models::optimization::ILP;
use problemreductions::models::specialized::{Assignment, BooleanExpr, Circuit};
use problemreductions::prelude::*;

pub fn run() {
    // 1. Create CircuitSAT instance: 1-bit full adder
    //    sum = a XOR b XOR cin, cout = (a AND b) OR (cin AND (a XOR b))
    //    Decomposed into 5 gates with intermediate variables t, ab, cin_t.
    let circuit = Circuit::new(vec![
        // Intermediate: t = a XOR b
        Assignment::new(
            vec!["t".to_string()],
            BooleanExpr::xor(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
        ),
        // sum = t XOR cin
        Assignment::new(
            vec!["sum".to_string()],
            BooleanExpr::xor(vec![BooleanExpr::var("t"), BooleanExpr::var("cin")]),
        ),
        // ab = a AND b
        Assignment::new(
            vec!["ab".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
        ),
        // cin_t = cin AND t
        Assignment::new(
            vec!["cin_t".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("cin"), BooleanExpr::var("t")]),
        ),
        // cout = ab OR cin_t
        Assignment::new(
            vec!["cout".to_string()],
            BooleanExpr::or(vec![BooleanExpr::var("ab"), BooleanExpr::var("cin_t")]),
        ),
    ]);
    let circuit_sat = CircuitSAT::new(circuit);

    println!("=== Circuit-SAT to ILP Reduction ===\n");
    println!("Source circuit: 1-bit full adder (a, b, cin -> sum, cout)");
    println!(
        "  {} variables: {:?}",
        circuit_sat.num_variables(),
        circuit_sat.variable_names()
    );

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&circuit_sat);
    let ilp = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: CircuitSAT with {} variables",
        circuit_sat.num_variables()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_variables(),
        ilp.constraints.len()
    );
    println!("  Each logic gate becomes a set of linear constraints.");
    println!("  XOR gates use 4 constraints each; AND/OR use k+1 constraints.");
    println!("  Objective is trivial (minimize 0): feasibility = satisfying assignment.");

    // 3. Solve the target ILP problem
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_all_best(ilp);
    println!("\n=== Solution ===");
    println!(
        "Target ILP feasible solutions found: {}",
        ilp_solutions.len()
    );

    // 4. Extract and verify source solutions
    println!("\nAll extracted CircuitSAT solutions:");
    let mut valid_count = 0;
    let mut solutions = Vec::new();
    for ilp_sol in &ilp_solutions {
        let circuit_sol = reduction.extract_solution(ilp_sol);
        let valid = circuit_sat.evaluate(&circuit_sol);
        let var_names = circuit_sat.variable_names();
        let assignment_str: Vec<String> = var_names
            .iter()
            .zip(circuit_sol.iter())
            .map(|(name, &val)| format!("{}={}", name, val))
            .collect();
        println!(
            "  ILP config {:?} -> Circuit: [{}], valid: {}",
            ilp_sol,
            assignment_str.join(", "),
            valid
        );
        if valid {
            valid_count += 1;
            solutions.push(SolutionPair {
                source_config: circuit_sol,
                target_config: ilp_sol.clone(),
            });
        }
    }
    println!(
        "\n{}/{} ILP solutions map to valid circuit assignments",
        valid_count,
        ilp_solutions.len()
    );
    assert!(
        valid_count > 0,
        "At least one ILP solution must be a valid circuit assignment"
    );

    println!("\nReduction verified successfully");

    // 5. Export JSON
    let source_variant = variant_to_map(CircuitSAT::variant());
    let target_variant = variant_to_map(ILP::variant());
    let overhead = lookup_overhead("CircuitSAT", &source_variant, "ILP", &target_variant)
        .expect("CircuitSAT -> ILP overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: CircuitSAT::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_gates": circuit_sat.circuit().num_assignments(),
                "num_variables": circuit_sat.num_variables(),
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
    let name = "circuitsat_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
