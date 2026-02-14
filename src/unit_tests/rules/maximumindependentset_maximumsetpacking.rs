use super::*;
use crate::solvers::BruteForce;
include!("../jl_helpers.rs");

#[test]
fn test_is_to_setpacking() {
    // Triangle graph
    let is_problem =
        MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
    let sp_problem = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_all_best(sp_problem);

    // Extract back
    let is_solutions: Vec<_> = sp_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Max IS in triangle = 1
    for sol in &is_solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 1);
    }
}

#[test]
fn test_setpacking_to_is() {
    // Two disjoint sets and one overlapping
    let sets = vec![
        vec![0, 1],
        vec![2, 3],
        vec![1, 2], // overlaps with both
    ];
    let sp_problem = MaximumSetPacking::<i32>::new(sets);
    let reduction: ReductionSPToIS<i32> =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp_problem);
    let is_problem = reduction.target_problem();

    let solver = BruteForce::new();
    let is_solutions = solver.find_all_best(is_problem);

    // Max packing = 2 (sets 0 and 1)
    for sol in &is_solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 2);
    }
}

#[test]
fn test_roundtrip_is_sp_is() {
    let original = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let solver = BruteForce::new();
    let original_solutions = solver.find_all_best(&original);

    // IS -> SP -> IS
    let reduction1 = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&original);
    let sp = reduction1.target_problem().clone();
    let reduction2: ReductionSPToIS<i32> =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp);
    let roundtrip = reduction2.target_problem();

    let roundtrip_solutions = solver.find_all_best(roundtrip);

    // Solutions should have same objective value
    let orig_size: usize = original_solutions[0].iter().sum();
    let rt_size: usize = roundtrip_solutions[0].iter().sum();
    assert_eq!(orig_size, rt_size);
}

#[test]
fn test_weighted_reduction() {
    let is_problem = MaximumIndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 20, 30]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
    let sp_problem = reduction.target_problem();

    // Weights should be preserved
    assert_eq!(sp_problem.weights_ref(), &vec![10, 20, 30]);
}

#[test]
fn test_empty_graph() {
    // No edges means all sets are empty (or we need to handle it)
    let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
    let sp_problem = reduction.target_problem();

    // All sets should be empty (no edges to include)
    assert_eq!(sp_problem.num_sets(), 3);

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sp_problem);

    // With no overlaps, we can select all sets
    assert_eq!(solutions[0].iter().sum::<usize>(), 3);
}

#[test]
fn test_disjoint_sets() {
    // Completely disjoint sets
    let sets = vec![vec![0], vec![1], vec![2]];
    let sp_problem = MaximumSetPacking::<i32>::new(sets);
    let reduction: ReductionSPToIS<i32> =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp_problem);
    let is_problem = reduction.target_problem();

    // No edges in the intersection graph
    assert_eq!(is_problem.num_edges(), 0);
}

#[test]
fn test_reduction_structure() {
    // Test IS to SP structure
    let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
    let sp = reduction.target_problem();

    // SP should have same number of sets as vertices in IS
    assert_eq!(sp.num_sets(), 4);

    // Test SP to IS structure
    let sets = vec![vec![0, 1], vec![2, 3]];
    let sp_problem = MaximumSetPacking::<i32>::new(sets);
    let reduction2: ReductionSPToIS<i32> =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp_problem);
    let is = reduction2.target_problem();

    // IS should have same number of vertices as sets in SP
    assert_eq!(is.num_vertices(), 2);
}

#[test]
fn test_jl_parity_is_to_setpacking() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset_to_setpacking.json")).unwrap();
    let is_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset.json")).unwrap();
    let inst = &is_data["instances"][0]["instance"];
    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(
        inst["num_vertices"].as_u64().unwrap() as usize, jl_parse_edges(inst));
    let result = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&source);
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
fn test_jl_parity_setpacking_to_is() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/setpacking_to_independentset.json")).unwrap();
    let sp_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/setpacking.json")).unwrap();
    let inst = &sp_data["instances"][0]["instance"];
    let source = MaximumSetPacking::<i32>::new(jl_parse_sets(&inst["sets"]));
    let result = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&source);
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
fn test_jl_parity_rule_is_to_setpacking() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/rule_independentset_to_setpacking.json")).unwrap();
    let is_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset.json")).unwrap();
    let inst = &jl_find_instance_by_label(&is_data, "doc_4vertex")["instance"];
    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(
        inst["num_vertices"].as_u64().unwrap() as usize, jl_parse_edges(inst));
    let result = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&source);
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
fn test_jl_parity_doc_is_to_setpacking() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/doc_independentset_to_setpacking.json")).unwrap();
    let is_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset.json")).unwrap();
    let is_instance = jl_find_instance_by_label(&is_data, "doc_4vertex");
    let inst = &is_instance["instance"];
    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(
        inst["num_vertices"].as_u64().unwrap() as usize, jl_parse_edges(inst));
    let result = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}
