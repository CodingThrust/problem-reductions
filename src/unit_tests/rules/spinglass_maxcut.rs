use super::*;
use crate::solvers::BruteForce;
include!("../jl_helpers.rs");

#[test]
fn test_maxcut_to_spinglass() {
    // Simple triangle MaxCut
    let mc = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&mc);
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    assert!(!solutions.is_empty());
}

#[test]
fn test_spinglass_to_maxcut_no_onsite() {
    // SpinGlass without onsite terms
    let sg = SpinGlass::<SimpleGraph, i32>::new(3, vec![((0, 1), 1), ((1, 2), 1)], vec![0, 0, 0]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
    let mc = reduction.target_problem();

    assert_eq!(mc.num_vertices(), 3); // No ancilla needed
    assert!(reduction.ancilla.is_none());
}

#[test]
fn test_spinglass_to_maxcut_with_onsite() {
    // SpinGlass with onsite terms
    let sg = SpinGlass::<SimpleGraph, i32>::new(2, vec![((0, 1), 1)], vec![1, 0]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
    let mc = reduction.target_problem();

    assert_eq!(mc.num_vertices(), 3); // Ancilla added
    assert_eq!(reduction.ancilla, Some(2));
}

#[test]
fn test_solution_extraction_no_ancilla() {
    let sg = SpinGlass::<SimpleGraph, i32>::new(2, vec![((0, 1), 1)], vec![0, 0]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);

    let mc_sol = vec![0, 1];
    let extracted = reduction.extract_solution(&mc_sol);
    assert_eq!(extracted, vec![0, 1]);
}

#[test]
fn test_solution_extraction_with_ancilla() {
    let sg = SpinGlass::<SimpleGraph, i32>::new(2, vec![((0, 1), 1)], vec![1, 0]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);

    // If ancilla is 0, don't flip
    let mc_sol = vec![0, 1, 0];
    let extracted = reduction.extract_solution(&mc_sol);
    assert_eq!(extracted, vec![0, 1]);

    // If ancilla is 1, flip all
    let mc_sol = vec![0, 1, 1];
    let extracted = reduction.extract_solution(&mc_sol);
    assert_eq!(extracted, vec![1, 0]); // flipped and ancilla removed
}

#[test]
fn test_weighted_maxcut() {
    let mc = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 10), (1, 2, 20)]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&mc);
    let sg = reduction.target_problem();

    // Verify interactions have correct weights
    let interactions = sg.interactions();
    assert_eq!(interactions.len(), 2);
}

#[test]
fn test_reduction_structure() {
    // Test MaxCut to SpinGlass structure
    let mc = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&mc);
    let sg = reduction.target_problem();

    // SpinGlass should have same number of spins as vertices
    assert_eq!(sg.num_spins(), 3);

    // Test SpinGlass to MaxCut structure
    let sg2 = SpinGlass::<SimpleGraph, i32>::new(3, vec![((0, 1), 1)], vec![0, 0, 0]);
    let reduction2 = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg2);
    let mc2 = reduction2.target_problem();

    assert_eq!(mc2.num_vertices(), 3);
}

#[test]
fn test_jl_parity_spinglass_to_maxcut() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/spinglass_to_maxcut.json")).unwrap();
    let sg_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/spinglass.json")).unwrap();
    let inst = &sg_data["instances"][0]["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let edges = jl_parse_edges(inst);
    let j_values = jl_parse_i32_vec(&inst["J"]);
    let h_values = jl_parse_i32_vec(&inst["h"]);
    let interactions: Vec<((usize, usize), i32)> = edges.into_iter().zip(j_values).collect();
    let source = SpinGlass::<SimpleGraph, i32>::new(nv, interactions, h_values);
    let result = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
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
fn test_jl_parity_maxcut_to_spinglass() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/maxcut_to_spinglass.json")).unwrap();
    let mc_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/maxcut.json")).unwrap();
    let inst = &mc_data["instances"][0]["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let weighted_edges = jl_parse_weighted_edges(inst);
    let source = MaxCut::<SimpleGraph, i32>::new(nv, weighted_edges);
    let result = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&source);
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
fn test_jl_parity_rule_maxcut_to_spinglass() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/rule_maxcut_to_spinglass.json")).unwrap();
    let mc_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/maxcut.json")).unwrap();
    let inst = &jl_find_instance_by_label(&mc_data, "rule_4vertex")["instance"];
    let source = MaxCut::<SimpleGraph, i32>::new(
        inst["num_vertices"].as_u64().unwrap() as usize, jl_parse_weighted_edges(inst));
    let result = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&source);
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
fn test_jl_parity_rule_spinglass_to_maxcut() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/rule_spinglass_to_maxcut.json")).unwrap();
    let sg_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/spinglass.json")).unwrap();
    let inst = &jl_find_instance_by_label(&sg_data, "rule_4vertex")["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let edges = jl_parse_edges(inst);
    let j_values = jl_parse_i32_vec(&inst["J"]);
    let h_values = jl_parse_i32_vec(&inst["h"]);
    let interactions: Vec<((usize, usize), i32)> = edges.into_iter().zip(j_values).collect();
    let source = SpinGlass::<SimpleGraph, i32>::new(nv, interactions, h_values);
    let result = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}
