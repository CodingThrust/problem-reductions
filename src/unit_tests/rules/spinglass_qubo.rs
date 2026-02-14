use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
include!("../jl_helpers.rs");

#[test]
fn test_qubo_to_spinglass() {
    // Simple 2-variable QUBO: minimize x0 + x1 - 2*x0*x1
    // Optimal at x = [0, 0] (value 0) or x = [1, 1] (value 0)
    let qubo = QUBO::from_matrix(vec![vec![1.0, -2.0], vec![0.0, 1.0]]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&qubo);
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let sg_solutions = solver.find_all_best(sg);
    let qubo_solutions: Vec<_> = sg_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Verify solutions are valid
    assert!(!qubo_solutions.is_empty());

    // Original QUBO at [0,0]: 0, at [1,1]: 1 + 1 - 2 = 0, at [0,1]: 1, at [1,0]: 1
    // So [0,0] and [1,1] are optimal with value 0
    for sol in &qubo_solutions {
        let val = qubo.evaluate(sol);
        assert!(
            val <= 0.0 + 1e-6,
            "Expected optimal value near 0, got {}",
            val
        );
    }
}

#[test]
fn test_spinglass_to_qubo() {
    // Simple SpinGlass: J_01 = -1 (ferromagnetic: prefers aligned spins)
    // Energy: J_01 * s0 * s1 = -s0 * s1
    // Aligned spins give -1, anti-aligned give +1
    // Minimum is -1 at [0,0] or [1,1] (both give s=-1,-1 or s=+1,+1)
    let sg = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), -1.0)], vec![0.0, 0.0]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    // Ferromagnetic: aligned spins are optimal
    for sol in &qubo_solutions {
        assert_eq!(sol[0], sol[1], "Ferromagnetic should have aligned spins");
    }
}

#[test]
fn test_roundtrip_qubo_sg_qubo() {
    let original = QUBO::from_matrix(vec![vec![-1.0, 2.0], vec![0.0, -1.0]]);
    let solver = BruteForce::new();
    let original_solutions = solver.find_all_best(&original);
    let _original_val = original.evaluate(&original_solutions[0]);

    // QUBO -> SG -> QUBO
    let reduction1 = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&original);
    let sg = reduction1.target_problem().clone();
    let reduction2 = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let roundtrip = reduction2.target_problem();

    let roundtrip_solutions = solver.find_all_best(roundtrip);
    let _roundtrip_val = roundtrip.evaluate(&roundtrip_solutions[0]);

    // The solutions should have the same configuration
    // (optimal configs should match)
    let orig_configs: std::collections::HashSet<_> = original_solutions.iter().collect();
    let rt_configs: std::collections::HashSet<_> = roundtrip_solutions.iter().collect();
    assert!(
        orig_configs.intersection(&rt_configs).count() > 0,
        "At least one optimal solution should match"
    );
}

#[test]
fn test_antiferromagnetic() {
    // Antiferromagnetic: J > 0, prefers anti-aligned spins
    let sg = SpinGlass::<SimpleGraph, f64>::new(2, vec![((0, 1), 1.0)], vec![0.0, 0.0]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(qubo);

    // Anti-ferromagnetic: opposite spins are optimal
    for sol in &solutions {
        assert_ne!(
            sol[0], sol[1],
            "Antiferromagnetic should have opposite spins"
        );
    }
}

#[test]
fn test_with_onsite_fields() {
    // SpinGlass with only on-site field h_0 = 1
    // Energy = h_0 * s_0 = s_0
    // Minimum at s_0 = -1, i.e., x_0 = 0
    let sg = SpinGlass::<SimpleGraph, f64>::new(1, vec![], vec![1.0]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(qubo);

    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0], "Should prefer x=0 (s=-1)");
}

#[test]
fn test_reduction_structure() {
    // Test QUBO to SpinGlass structure
    let qubo = QUBO::from_matrix(vec![vec![1.0, -2.0], vec![0.0, 1.0]]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&qubo);
    let sg = reduction.target_problem();

    // SpinGlass should have same number of spins as QUBO variables
    assert_eq!(sg.num_spins(), 2);

    // Test SpinGlass to QUBO structure
    let sg2 = SpinGlass::<SimpleGraph, f64>::new(3, vec![((0, 1), -1.0)], vec![0.0, 0.0, 0.0]);
    let reduction2 = ReduceTo::<QUBO<f64>>::reduce_to(&sg2);
    let qubo2 = reduction2.target_problem();

    assert_eq!(qubo2.num_variables(), 3);
}

#[test]
fn test_jl_parity_spinglass_to_qubo() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/spinglass_to_qubo.json")).unwrap();
    let sg_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/spinglass.json")).unwrap();
    let inst = &sg_data["instances"][0]["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let edges = jl_parse_edges(inst);
    let j_values: Vec<f64> = inst["J"].as_array().unwrap().iter().map(|v| v.as_i64().unwrap() as f64).collect();
    let h_values: Vec<f64> = inst["h"].as_array().unwrap().iter().map(|v| v.as_i64().unwrap() as f64).collect();
    let interactions: Vec<((usize, usize), f64)> = edges.into_iter().zip(j_values).collect();
    let source = SpinGlass::<SimpleGraph, f64>::new(nv, interactions, h_values);
    let result = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_qubo_to_spinglass() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/qubo_to_spinglass.json")).unwrap();
    let q_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/qubo.json")).unwrap();
    let jl_matrix: Vec<Vec<f64>> = q_data["instances"][0]["instance"]["matrix"]
        .as_array().unwrap().iter()
        .map(|row| row.as_array().unwrap().iter().map(|v| v.as_i64().unwrap() as f64).collect())
        .collect();
    let n = jl_matrix.len();
    let mut rust_matrix = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        rust_matrix[i][i] = jl_matrix[i][i];
        for j in (i + 1)..n { rust_matrix[i][j] = jl_matrix[i][j] + jl_matrix[j][i]; }
    }
    let source = QUBO::from_matrix(rust_matrix);
    let result = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_rule_qubo_to_spinglass() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/rule_qubo_to_spinglass.json")).unwrap();
    let q_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/qubo.json")).unwrap();
    let jl_matrix: Vec<Vec<f64>> = jl_find_instance_by_label(&q_data, "rule_3x3")["instance"]["matrix"]
        .as_array().unwrap().iter()
        .map(|row| row.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect())
        .collect();
    let n = jl_matrix.len();
    let mut rust_matrix = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        rust_matrix[i][i] = jl_matrix[i][i];
        for j in (i + 1)..n { rust_matrix[i][j] = jl_matrix[i][j] + jl_matrix[j][i]; }
    }
    let source = QUBO::from_matrix(rust_matrix);
    let result = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}
