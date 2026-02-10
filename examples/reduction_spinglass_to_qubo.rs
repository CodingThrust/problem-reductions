//! # Spin Glass to QUBO Reduction
//!
//! ## Mathematical Equivalence
//! The substitution s_i = 2x_i - 1 transforms Ising spins s in {-1,+1} to binary
//! variables x in {0,1}. Expanding the Ising Hamiltonian H(s) under this substitution
//! yields a QUBO objective Q(x) plus a constant offset.
//!
//! ## This Example
//! - Instance: 3-spin antiferromagnetic chain with fields
//!   - Couplings: J_{01} = -1.0, J_{12} = -1.0
//!   - Fields: h = [0.5, -0.5, 0.5]
//! - Source SpinGlass: 3 spins
//! - Target QUBO: 3 binary variables
//!
//! ## Output
//! Exports `docs/paper/examples/spinglass_to_qubo.json` and
//! `docs/paper/examples/spinglass_to_qubo.result.json` for use in paper code blocks.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    let sg = SpinGlass::<SimpleGraph, f64>::new(
        3,
        vec![((0, 1), -1.0), ((1, 2), -1.0)],
        vec![0.5, -0.5, 0.5],
    );

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: SpinGlass with {} variables", sg.num_variables());
    println!("Target: QUBO with {} variables", qubo.num_variables());

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", qubo_solutions.len());

    let sg_solution = reduction.extract_solution(&qubo_solutions[0]);
    println!("Source SpinGlass solution: {:?}", sg_solution);

    let size = sg.solution_size(&sg_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // Collect all solutions
    let mut solutions = Vec::new();
    for target_sol in &qubo_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }

    // Export JSON
    let overhead = lookup_overhead("SpinGlass", "QUBO")
        .expect("SpinGlass -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: SpinGlass::<SimpleGraph, f64>::NAME.to_string(),
            variant: variant_to_map(SpinGlass::<SimpleGraph, f64>::variant()),
            instance: serde_json::json!({
                "num_spins": sg.num_variables(),
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: variant_to_map(QUBO::<f64>::variant()),
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": qubo.matrix(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
