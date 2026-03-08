use super::*;
use crate::solvers::BruteForce;
use crate::topology::Graph;
use crate::traits::Problem;

#[test]
fn test_lcs_to_maximumindependentset_closed_loop() {
    // ABAC / BACA -> LCS length = 3 (e.g., "BAC" or "AAC" or "ACA")
    let lcs = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'A', b'C'],
        vec![b'B', b'A', b'C', b'A'],
    ]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let is_problem = reduction.target_problem();

    // Solve the IS problem
    let solver = BruteForce::new();
    let best_solutions = solver.find_all_best(is_problem);
    assert!(!best_solutions.is_empty());

    // IS size should be 3 (= LCS length)
    for sol in &best_solutions {
        assert_eq!(sol.iter().sum::<usize>(), 3);
    }

    // Extract and verify each solution is valid for the original LCS problem
    for sol in &best_solutions {
        let lcs_config = reduction.extract_solution(sol);
        let metric = lcs.evaluate(&lcs_config);
        match metric {
            crate::types::SolutionSize::Valid(len) => assert_eq!(len, 3),
            crate::types::SolutionSize::Invalid => panic!("Extracted solution is invalid"),
        }
    }
}

#[test]
fn test_lcs_to_is_graph_structure() {
    // ABAC / BACA
    // Common chars: A (positions [0,2] and [1,3]), B ([1] and [0]), C ([3] and [2])
    // Match nodes for A: (0,1),(0,3),(2,1),(2,3) = 4 nodes
    // Match nodes for B: (1,0) = 1 node
    // Match nodes for C: (3,2) = 1 node
    // Total: 6 nodes
    let lcs = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'A', b'C'],
        vec![b'B', b'A', b'C', b'A'],
    ]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let is_problem = reduction.target_problem();

    assert_eq!(is_problem.graph().num_vertices(), 6);
    assert_eq!(is_problem.graph().num_edges(), 9);
}

#[test]
fn test_lcs_to_is_three_strings() {
    // Three strings with LCS = "ABCD" (length 4)
    // s1 = XABCDY, s2 = ABCDZ, s3 = WABCD
    let lcs = LongestCommonSubsequence::new(vec![
        vec![b'X', b'A', b'B', b'C', b'D', b'Y'],
        vec![b'A', b'B', b'C', b'D', b'Z'],
        vec![b'W', b'A', b'B', b'C', b'D'],
    ]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let is_problem = reduction.target_problem();

    let solver = BruteForce::new();
    let best_solutions = solver.find_all_best(is_problem);
    assert!(!best_solutions.is_empty());

    // IS size should be 4 (= LCS length)
    for sol in &best_solutions {
        assert_eq!(sol.iter().sum::<usize>(), 4);
    }

    // Verify extracted solutions
    for sol in &best_solutions {
        let lcs_config = reduction.extract_solution(sol);
        let metric = lcs.evaluate(&lcs_config);
        match metric {
            crate::types::SolutionSize::Valid(len) => assert_eq!(len, 4),
            crate::types::SolutionSize::Invalid => panic!("Extracted solution is invalid"),
        }
    }
}

#[test]
fn test_lcs_to_is_no_common_chars() {
    // No common characters -> empty graph
    let lcs = LongestCommonSubsequence::new(vec![vec![b'A', b'B'], vec![b'C', b'D']]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let is_problem = reduction.target_problem();

    assert_eq!(is_problem.graph().num_vertices(), 0);
    assert_eq!(is_problem.graph().num_edges(), 0);
}

#[test]
fn test_lcs_to_is_identical_strings() {
    // Identical strings -> LCS = full string
    let lcs = LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'A', b'B', b'C']]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let is_problem = reduction.target_problem();

    let solver = BruteForce::new();
    let best_solutions = solver.find_all_best(is_problem);
    assert!(!best_solutions.is_empty());

    // IS size should be 3 (= full string length)
    for sol in &best_solutions {
        assert_eq!(sol.iter().sum::<usize>(), 3);
    }

    for sol in &best_solutions {
        let lcs_config = reduction.extract_solution(sol);
        let metric = lcs.evaluate(&lcs_config);
        match metric {
            crate::types::SolutionSize::Valid(len) => assert_eq!(len, 3),
            crate::types::SolutionSize::Invalid => panic!("Extracted solution is invalid"),
        }
    }
}

#[test]
fn test_tuples_conflict_function() {
    // Same position in one dimension -> conflict
    assert!(tuples_conflict(&[0, 1], &[0, 2]));
    assert!(tuples_conflict(&[1, 0], &[1, 2]));

    // Crossing: a_0 < b_0 but a_1 > b_1 -> conflict
    assert!(tuples_conflict(&[0, 2], &[1, 0]));

    // Consistent ordering: all a_i < b_i -> no conflict
    assert!(!tuples_conflict(&[0, 0], &[1, 1]));
    assert!(!tuples_conflict(&[0, 1], &[2, 3]));

    // Reverse consistent ordering: all b_i < a_i -> no conflict
    assert!(!tuples_conflict(&[2, 3], &[0, 1]));

    // Three dimensions
    assert!(!tuples_conflict(&[0, 0, 0], &[1, 1, 1]));
    assert!(tuples_conflict(&[0, 1, 0], &[1, 0, 1]));

    // Equal tuples -> conflict (a_i = b_i for all i)
    assert!(tuples_conflict(&[1, 2], &[1, 2]));
}
