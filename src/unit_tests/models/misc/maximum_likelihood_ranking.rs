use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_maximum_likelihood_ranking_creation() {
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    let problem = MaximumLikelihoodRanking::new(matrix.clone());
    assert_eq!(problem.num_items(), 4);
    assert_eq!(problem.matrix(), &matrix);
    assert_eq!(problem.comparison_count(), 5);
    assert_eq!(problem.dims(), vec![4; 4]);
    assert_eq!(
        <MaximumLikelihoodRanking as Problem>::NAME,
        "MaximumLikelihoodRanking"
    );
    assert_eq!(<MaximumLikelihoodRanking as Problem>::variant(), vec![]);
}

#[test]
fn test_maximum_likelihood_ranking_evaluate_optimal() {
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    let problem = MaximumLikelihoodRanking::new(matrix);
    // Identity ranking: config[i] = i (item i is at position i)
    // Disagreement pairs where config[a] > config[b]:
    // (1,0): matrix[1][0] = 1
    // (2,0): matrix[2][0] = 2
    // (2,1): matrix[2][1] = 1
    // (3,0): matrix[3][0] = 0
    // (3,1): matrix[3][1] = 2
    // (3,2): matrix[3][2] = 1
    // Total = 1 + 2 + 1 + 0 + 2 + 1 = 7
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), Min(Some(7)));
}

#[test]
fn test_maximum_likelihood_ranking_evaluate_non_permutation() {
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    let problem = MaximumLikelihoodRanking::new(matrix);
    // Duplicate rank
    assert_eq!(problem.evaluate(&[0, 0, 2, 3]), Min(None));
    // Rank out of range
    assert_eq!(problem.evaluate(&[0, 1, 2, 4]), Min(None));
    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 0]), Min(None));
}

#[test]
fn test_maximum_likelihood_ranking_evaluate_suboptimal() {
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    let problem = MaximumLikelihoodRanking::new(matrix);
    // Reversed ranking: config = [3, 2, 1, 0]
    // (item 0 at pos 3, item 1 at pos 2, item 2 at pos 1, item 3 at pos 0)
    // Pairs where config[a] > config[b]:
    // (0,1): config[0]=3 > config[1]=2 -> matrix[0][1] = 4
    // (0,2): config[0]=3 > config[2]=1 -> matrix[0][2] = 3
    // (0,3): config[0]=3 > config[3]=0 -> matrix[0][3] = 5
    // (1,2): config[1]=2 > config[2]=1 -> matrix[1][2] = 4
    // (1,3): config[1]=2 > config[3]=0 -> matrix[1][3] = 3
    // (2,3): config[2]=1 > config[3]=0 -> matrix[2][3] = 4
    // Total = 4 + 3 + 5 + 4 + 3 + 4 = 23
    assert_eq!(problem.evaluate(&[3, 2, 1, 0]), Min(Some(23)));
}

#[test]
fn test_maximum_likelihood_ranking_solver() {
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    let problem = MaximumLikelihoodRanking::new(matrix);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let value = problem.evaluate(&solution);
    assert_eq!(value, Min(Some(7)));
}

#[test]
fn test_maximum_likelihood_ranking_serialization() {
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    let problem = MaximumLikelihoodRanking::new(matrix.clone());
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MaximumLikelihoodRanking = serde_json::from_value(json).unwrap();
    assert_eq!(restored.matrix(), &matrix);
    assert_eq!(restored.num_items(), 4);
}

#[test]
fn test_maximum_likelihood_ranking_two_items() {
    // 2 items: a_01 = 3, a_10 = 2
    let matrix = vec![vec![0, 3], vec![2, 0]];
    let problem = MaximumLikelihoodRanking::new(matrix);
    // config [0,1]: item 0 at pos 0, item 1 at pos 1
    // Only pair where config[a] > config[b]: (1,0) -> matrix[1][0] = 2
    assert_eq!(problem.evaluate(&[0, 1]), Min(Some(2)));
    // config [1,0]: item 0 at pos 1, item 1 at pos 0
    // Only pair where config[a] > config[b]: (0,1) -> matrix[0][1] = 3
    assert_eq!(problem.evaluate(&[1, 0]), Min(Some(3)));
    // Optimal is [0,1] with cost 2
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(2)));
}

#[test]
fn test_maximum_likelihood_ranking_single_item() {
    let problem = MaximumLikelihoodRanking::new(vec![vec![0]]);
    assert_eq!(problem.num_items(), 1);
    assert_eq!(problem.comparison_count(), 0);
    assert_eq!(problem.dims(), vec![1]);
    assert_eq!(problem.evaluate(&[0]), Min(Some(0)));
}

#[test]
#[should_panic(expected = "matrix must be square")]
fn test_maximum_likelihood_ranking_non_square_panics() {
    MaximumLikelihoodRanking::new(vec![vec![0, 1], vec![2, 0], vec![1, 2]]);
}

#[test]
#[should_panic(expected = "diagonal entries must be zero")]
fn test_maximum_likelihood_ranking_nonzero_diagonal_panics() {
    MaximumLikelihoodRanking::new(vec![vec![1, 2], vec![3, 0]]);
}

#[test]
#[should_panic(expected = "off-diagonal entries must be non-negative")]
fn test_maximum_likelihood_ranking_negative_off_diagonal_panics() {
    MaximumLikelihoodRanking::new(vec![vec![0, -1], vec![6, 0]]);
}

#[test]
#[should_panic(expected = "all off-diagonal pairs must have the same comparison count")]
fn test_maximum_likelihood_ranking_inconsistent_pair_sum_panics() {
    MaximumLikelihoodRanking::new(vec![vec![0, 4, 3], vec![1, 0, 4], vec![1, 2, 0]]);
}

#[cfg(feature = "example-db")]
#[test]
fn test_maximum_likelihood_ranking_canonical_example() {
    let specs = canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];
    assert_eq!(spec.id, "maximum_likelihood_ranking");
    assert_eq!(spec.optimal_config, vec![0, 1, 2, 3]);
    assert_eq!(spec.optimal_value, serde_json::json!(7));
}
