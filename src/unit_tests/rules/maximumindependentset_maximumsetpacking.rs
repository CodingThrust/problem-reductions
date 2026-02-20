use super::*;
use crate::solvers::BruteForce;
include!("../jl_helpers.rs");

#[test]
fn test_maximumindependentset_to_maximumsetpacking_closed_loop() {
    let is_problem =
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![10, 20, 30]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
    let sp_problem = reduction.target_problem();

    // Weights should be preserved
    assert_eq!(sp_problem.weights_ref(), &vec![10, 20, 30]);
}

#[test]
fn test_empty_graph() {
    // No edges means all sets are empty (or we need to handle it)
    let is_problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![]), vec![1i32; 3]);
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
    assert_eq!(is_problem.graph().num_edges(), 0);
}

#[test]
fn test_reduction_structure() {
    // Test IS to SP structure
    let is_problem =
        MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (1, 2)]), vec![1i32; 4]);
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
    assert_eq!(is.graph().num_vertices(), 2);
}

#[test]
fn test_jl_parity_is_to_setpacking() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/independentset_to_setpacking.json"
    ))
    .unwrap();
    let is_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset.json")).unwrap();
    let inst = &is_data["instances"][0]["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let source =
        MaximumIndependentSet::new(SimpleGraph::new(nv, jl_parse_edges(inst)), vec![1i32; nv]);
    let result = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_setpacking_to_is() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/setpacking_to_independentset.json"
    ))
    .unwrap();
    let sp_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/setpacking.json")).unwrap();
    let inst = &sp_data["instances"][0]["instance"];
    let source = MaximumSetPacking::<i32>::new(jl_parse_sets(&inst["sets"]));
    let result = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_rule_is_to_setpacking() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/rule_independentset_to_setpacking.json"
    ))
    .unwrap();
    let is_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset.json")).unwrap();
    let inst = &jl_find_instance_by_label(&is_data, "doc_4vertex")["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let source =
        MaximumIndependentSet::new(SimpleGraph::new(nv, jl_parse_edges(inst)), vec![1i32; nv]);
    let result = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_doc_is_to_setpacking() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/doc_independentset_to_setpacking.json"
    ))
    .unwrap();
    let is_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/independentset.json")).unwrap();
    let is_instance = jl_find_instance_by_label(&is_data, "doc_4vertex");
    let inst = &is_instance["instance"];
    let nv = inst["num_vertices"].as_u64().unwrap() as usize;
    let source =
        MaximumIndependentSet::new(SimpleGraph::new(nv, jl_parse_edges(inst)), vec![1i32; nv]);
    let result = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}
