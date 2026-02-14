// # Spin Glass to QUBO Reduction
//
// ## Mathematical Equivalence
// The substitution s_i = 2x_i - 1 transforms Ising spins s in {-1,+1} to binary
// variables x in {0,1}. Expanding the Ising Hamiltonian H(s) under this substitution
// yields a QUBO objective Q(x) plus a constant offset.
//
// ## This Example
// - Instance: Petersen graph with 10 spins, 15 frustrated ±1 couplings, zero fields
// - Source SpinGlass: 10 spins on Petersen topology
// - Target QUBO: 10 binary variables
//
// ## Output
// Exports `docs/paper/examples/spinglass_to_qubo.json` and
// `docs/paper/examples/spinglass_to_qubo.result.json` for use in paper code blocks.
//
// See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

pub fn run() {
    let (n, edges) = petersen();
    // Alternating +/-1 couplings create frustration on odd cycles
    let couplings: Vec<((usize, usize), f64)> = edges
        .iter()
        .enumerate()
        .map(|(i, &(u, v))| ((u, v), if i % 2 == 0 { 1.0 } else { -1.0 }))
        .collect();
    let sg = SpinGlass::<SimpleGraph, f64>::new(n, couplings, vec![0.0; n]);

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: SpinGlass with {} variables", sg.num_variables());
    println!("Target: QUBO with {} variables", qubo.num_variables());

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", qubo_solutions.len());

    let sg_solution = reduction.extract_solution(&qubo_solutions[0]);
    println!("Source SpinGlass solution: {:?}", sg_solution);

    let energy = sg.evaluate(&sg_solution);
    println!("Solution energy: {:?}", energy);
    assert!(energy.is_valid()); // Valid solution
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
    let source_variant = variant_to_map(SpinGlass::<SimpleGraph, f64>::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead =
        lookup_overhead("SpinGlass", &source_variant, "QUBO", &target_variant)
            .expect("SpinGlass -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: SpinGlass::<SimpleGraph, f64>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_spins": sg.num_variables(),
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
    let name = "spinglass_to_qubo";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
