use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::SolutionSize;
include!("../jl_helpers.rs");

#[test]
fn test_matching_to_setpacking_structure() {
    // Path graph 0-1-2
    let matching = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    // Should have 2 sets (one for each edge)
    assert_eq!(sp.num_sets(), 2);

    // Sets should contain edge endpoints
    let sets = sp.sets();
    assert_eq!(sets[0], vec![0, 1]);
    assert_eq!(sets[1], vec![1, 2]);
}

#[test]
fn test_matching_to_setpacking_weighted() {
    // Weighted edges: heavy edge should win over multiple light edges
    let matching =
        MaximumMatching::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3)]), vec![100, 1, 1]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    // Weights should be preserved
    assert_eq!(sp.weights_ref(), &vec![100, 1, 1]);

    let solver = BruteForce::new();
    let sp_solutions = solver.find_all_best(sp);

    // Edge 0-1 (weight 100) alone beats edges 0-2 + 1-3 (weight 2)
    assert!(sp_solutions.contains(&vec![1, 0, 0]));

    // Verify through direct MaximumMatching solution
    let direct_solutions = solver.find_all_best(&matching);
    assert_eq!(
        matching.evaluate(&sp_solutions[0]),
        SolutionSize::Valid(100)
    );
    assert_eq!(
        matching.evaluate(&direct_solutions[0]),
        SolutionSize::Valid(100)
    );
}

#[test]
fn test_matching_to_setpacking_solution_extraction() {
    let matching = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&matching);

    // Test solution extraction is 1:1
    let sp_solution = vec![1, 0, 1];
    let matching_solution = reduction.extract_solution(&sp_solution);
    assert_eq!(matching_solution, vec![1, 0, 1]);

    // Verify the extracted solution is valid for original MaximumMatching
    assert!(matching.evaluate(&matching_solution).is_valid());
}

#[test]
fn test_matching_to_setpacking_empty() {
    // Graph with no edges
    let matching = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(3, vec![]));
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    assert_eq!(sp.num_sets(), 0);
}

#[test]
fn test_matching_to_setpacking_single_edge() {
    let matching = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(2, vec![(0, 1)]));
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    assert_eq!(sp.num_sets(), 1);
    assert_eq!(sp.sets()[0], vec![0, 1]);

    let solver = BruteForce::new();
    let sp_solutions = solver.find_all_best(sp);

    // Should select the only set
    assert_eq!(sp_solutions, vec![vec![1]]);
}

#[test]
fn test_matching_to_setpacking_disjoint_edges() {
    // Two disjoint edges: 0-1 and 2-3
    let matching = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(4, vec![(0, 1), (2, 3)]));
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_all_best(sp);

    // Both edges can be selected (they don't share vertices)
    assert_eq!(sp_solutions, vec![vec![1, 1]]);
}

#[test]
fn test_reduction_structure() {
    let matching = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    // SP should have same number of sets as edges in matching
    assert_eq!(sp.num_sets(), 3);
}

#[test]
fn test_matching_to_setpacking_star() {
    // Star graph: center vertex 0 connected to 1, 2, 3
    let matching = MaximumMatching::<_, i32>::unit_weights(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]));
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&matching);
    let sp = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_all_best(sp);

    // All edges share vertex 0, so max matching = 1
    for sol in &sp_solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
    // Should have 3 optimal solutions
    assert_eq!(sp_solutions.len(), 3);
}

#[test]
fn test_jl_parity_matching_to_setpacking() {
    let match_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/matching.json")).unwrap();
    let fixtures: &[(&str, &str)] = &[
        (
            include_str!("../../../tests/data/jl/matching_to_setpacking.json"),
            "petersen",
        ),
        (
            include_str!("../../../tests/data/jl/rule_matching_to_setpacking.json"),
            "rule_4vertex",
        ),
        (
            include_str!("../../../tests/data/jl/rule_matchingw_to_setpacking.json"),
            "rule_4vertex_weighted",
        ),
    ];
    for (fixture_str, label) in fixtures {
        let data: serde_json::Value = serde_json::from_str(fixture_str).unwrap();
        let inst = &jl_find_instance_by_label(&match_data, label)["instance"];
        let weighted_edges = jl_parse_weighted_edges(inst);
        let edges: Vec<(usize, usize)> = weighted_edges.iter().map(|&(u, v, _)| (u, v)).collect();
        let weights: Vec<i32> = weighted_edges.into_iter().map(|(_, _, w)| w).collect();
        let source = MaximumMatching::new(
            SimpleGraph::new(inst["num_vertices"].as_u64().unwrap() as usize, edges),
            weights,
        );
        let result = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&source);
        let solver = BruteForce::new();
        let best_target = solver.find_all_best(result.target_problem());
        let best_source: HashSet<Vec<usize>> = solver.find_all_best(&source).into_iter().collect();
        let extracted: HashSet<Vec<usize>> = best_target
            .iter()
            .map(|t| result.extract_solution(t))
            .collect();
        assert!(
            extracted.is_subset(&best_source),
            "Matching->SP [{label}]: extracted not subset"
        );
        for case in data["cases"].as_array().unwrap() {
            assert_eq!(
                best_source,
                jl_parse_configs_set(&case["best_source"]),
                "Matching->SP [{label}]: best source mismatch"
            );
        }
    }
}
