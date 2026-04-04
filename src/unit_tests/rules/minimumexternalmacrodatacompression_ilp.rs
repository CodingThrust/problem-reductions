use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_emdc_to_ilp_closed_loop() {
    // s = "ab" (len 2), alphabet {a,b}, h=2
    // Optimal: uncompressed, cost = 2
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid(), "Extracted solution should be valid");
    assert_eq!(value, Min(Some(2)));
}

#[test]
fn test_emdc_to_ilp_compression_wins() {
    // alphabet {a,b,c,d,e,f} (6), s="abcdefabcdefabcdef" (18), h=2
    // Optimal: D="abcdef"(6), C=3 pointers of (0,6), cost=6+3+1*3=12
    // (pointer cost h=2, so (h-1)*3 = 3, total = 6+3+3 = 12)
    // Uncompressed: 18
    let s: Vec<usize> = (0..6).cycle().take(18).collect();
    let problem = MinimumExternalMacroDataCompression::new(6, s, 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid(), "Extracted solution should be valid");
    assert_eq!(value, Min(Some(12)));
}

#[test]
fn test_emdc_to_ilp_structure() {
    // s = "ab" (len 2), alphabet {a,b} (k=2), h=2
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let _n = 2;
    let _k = 2;
    // d[j][c]: 2*2 = 4
    // d_used[j]: 2
    // lit[i]: 2
    // ptr triples: (0,1,0),(0,1,1),(0,2,0),(1,1,0),(1,1,1) = 5
    // Total = 4 + 2 + 2 + 5 = 13
    assert_eq!(ilp.num_vars, 13);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // Constraints:
    // one-hot: n = 2
    // linking: n*k = 4
    // contiguous: n-1 = 1
    // flow: n+1 = 3
    // ptr matching: each ptr triple's matching constraints
    //   (0,1,0): 1, (0,1,1): 1, (0,2,0): 2, (1,1,0): 1, (1,1,1): 1 = 6
    // Total = 2 + 4 + 1 + 3 + 6 = 16
    assert_eq!(ilp.constraints.len(), 16);
}

#[test]
fn test_emdc_to_ilp_empty() {
    // Empty string: cost should be 0
    let problem = MinimumExternalMacroDataCompression::new(2, vec![], 1);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 0);
    assert!(ilp.constraints.is_empty());

    // For empty ILP, the solution is empty
    let extracted = reduction.extract_solution(&[]);
    let value = problem.evaluate(&extracted);
    assert_eq!(value, Min(Some(0)));
}

#[test]
fn test_emdc_to_ilp_bf_vs_ilp() {
    // Small instance: s="ab", alphabet {a,b}, h=2
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_emdc_to_ilp_single_char() {
    // s = "a" (len 1), alphabet {a} (k=1), h=1
    // Uncompressed: cost = 0+1+0 = 1. With D="a"(1), C=ptr(0,1)(1, 1 ptr): cost = 1+1+0 = 2.
    // So uncompressed is optimal.
    let problem = MinimumExternalMacroDataCompression::new(1, vec![0], 1);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid());
    assert_eq!(value, Min(Some(1)));
}

#[test]
fn test_emdc_to_ilp_repeated_string() {
    // s = "aaa" (len 3), alphabet {a} (k=1), h=1
    // Uncompressed: cost = 3. D="a"(1), C=ptr(0,1)*3: cost = 1+3+0=4.
    // D="aaa"(3), C=ptr(0,3): cost = 3+1+0 = 4.
    // D="aa"(2), C=ptr(0,1) ptr(0,2): cost = 2+2+0 = 4.
    // Uncompressed is best at 3.
    let problem = MinimumExternalMacroDataCompression::new(1, vec![0, 0, 0], 1);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid());
    assert_eq!(value, Min(Some(3)));
}

#[cfg(feature = "example-db")]
#[test]
fn test_emdc_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "minimumexternalmacrodatacompression_to_ilp")
        .expect("missing canonical EMDC -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(
        example.source.problem,
        "MinimumExternalMacroDataCompression"
    );
    assert_eq!(example.target.problem, "ILP");
    assert_eq!(example.source.instance["alphabet_size"], 2);
    assert_eq!(example.source.instance["string"], serde_json::json!([0, 1]));
}
