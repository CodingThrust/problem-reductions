//! Demonstrates 6 problem-to-QUBO reductions with practical stories.
//!
//! Run with: `cargo run --example qubo_reductions --features ilp`

use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    println!("=== Problem-to-QUBO Reductions ===\n");

    demo_independent_set();
    demo_vertex_covering();
    demo_coloring();
    demo_set_packing();
    demo_ksat();
    demo_ilp();
}

/// Wireless tower placement: find the largest set of non-interfering towers.
fn demo_independent_set() {
    println!("--- 1. IndependentSet -> QUBO ---");
    println!("Story: Place wireless towers on a 4-site grid. Adjacent towers interfere.");
    println!("       Find the maximum set of non-interfering towers.\n");

    // Path graph: sites 0-1-2-3, adjacent sites interfere
    let is = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction = ReduceTo::<QUBO>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    println!("  QUBO variables: {}", qubo.num_variables());
    println!("  Optimal solutions:");
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let sites: Vec<usize> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect();
        println!("    Tower sites: {:?} (size {})", sites, sites.len());
    }
    println!();
}

/// Security camera placement: cover all corridors with minimum cameras.
fn demo_vertex_covering() {
    println!("--- 2. VertexCovering -> QUBO ---");
    println!("Story: Place security cameras at intersections to cover all corridors.");
    println!("       Minimize the number of cameras needed.\n");

    // Cycle C4: 4 intersections, 4 corridors
    let vc = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
    let reduction = ReduceTo::<QUBO>::reduce_to(&vc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    println!("  QUBO variables: {}", qubo.num_variables());
    println!("  Optimal solutions:");
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let cameras: Vec<usize> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect();
        println!(
            "    Camera positions: {:?} ({} cameras)",
            cameras,
            cameras.len()
        );
    }
    println!();
}

/// Map coloring: color a triangle map with 3 colors so no neighbors share a color.
fn demo_coloring() {
    println!("--- 3. KColoring -> QUBO ---");
    println!("Story: Color 3 countries on a map with 3 colors so no neighbors match.\n");

    // Triangle K3: 3 countries, all share borders
    let kc = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<QUBO>::reduce_to(&kc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    let colors = ["Red", "Green", "Blue"];
    println!(
        "  QUBO variables: {} (one-hot: 3 countries x 3 colors)",
        qubo.num_variables()
    );
    println!("  Valid colorings: {}", solutions.len());
    let extracted = reduction.extract_solution(&solutions[0]);
    println!(
        "  Example: Country0={}, Country1={}, Country2={}",
        colors[extracted[0]], colors[extracted[1]], colors[extracted[2]]
    );
    println!();
}

/// Warehouse selection: pick maximum non-overlapping delivery zones.
fn demo_set_packing() {
    println!("--- 4. SetPacking -> QUBO ---");
    println!("Story: Select delivery zones that don't overlap. Maximize coverage.\n");

    // 3 zones covering different areas
    let sp = SetPacking::<i32>::new(vec![
        vec![0, 1],    // Zone A covers areas 0,1
        vec![1, 2],    // Zone B covers areas 1,2
        vec![2, 3, 4], // Zone C covers areas 2,3,4
    ]);
    let reduction = ReduceTo::<QUBO>::reduce_to(&sp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    println!("  QUBO variables: {}", qubo.num_variables());
    println!("  Optimal solutions:");
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let zones: Vec<&str> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| ["Zone-A", "Zone-B", "Zone-C"][i])
            .collect();
        println!("    Selected: {:?}", zones);
    }
    println!();
}

/// Satisfiability: find a boolean assignment satisfying maximum clauses.
fn demo_ksat() {
    println!("--- 5. KSatisfiability(K=2) -> QUBO ---");
    println!("Story: Configure 3 switches (on/off) to satisfy maximum rules.\n");

    // 4 rules over 3 switches
    let ksat = KSatisfiability::<2, i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),   // switch1 OR switch2
            CNFClause::new(vec![-1, 3]),  // NOT switch1 OR switch3
            CNFClause::new(vec![2, -3]),  // switch2 OR NOT switch3
            CNFClause::new(vec![-2, -3]), // NOT switch2 OR NOT switch3
        ],
    );
    let reduction = ReduceTo::<QUBO>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    println!("  QUBO variables: {}", qubo.num_variables());
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let switches: Vec<&str> = extracted.iter().map(|&x| if x == 1 { "ON" } else { "OFF" }).collect();
        let satisfied = ksat.solution_size(&extracted).size;
        println!(
            "  Switches: [{}] -> {}/{} rules satisfied",
            switches.join(", "),
            satisfied,
            ksat.clauses().len()
        );
    }
    println!();
}

/// Resource allocation: maximize value under budget constraints.
fn demo_ilp() {
    println!("--- 6. ILP (binary) -> QUBO ---");
    println!("Story: Select projects to maximize profit under resource constraints.\n");

    // 3 projects: values 1, 2, 3
    // Constraint 1: projects 0 and 1 share a team (at most one)
    // Constraint 2: projects 1 and 2 share equipment (at most one)
    let ilp = ILP::binary(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
            LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
        ],
        vec![(0, 1.0), (1, 2.0), (2, 3.0)],
        ObjectiveSense::Maximize,
    );
    let reduction = ReduceTo::<QUBO>::reduce_to(&ilp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    let names = ["Alpha", "Beta", "Gamma"];
    println!("  QUBO variables: {}", qubo.num_variables());
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let selected: Vec<&str> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| names[i])
            .collect();
        let value = ilp.solution_size(&extracted).size;
        println!(
            "  Selected projects: {:?} (total value: {:.0})",
            selected, value
        );
    }
    println!();
}
