use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn source(
    num_vertices: usize,
    edges: Vec<(usize, usize)>,
) -> MinimumDominatingSet<SimpleGraph, One> {
    MinimumDominatingSet::new(
        SimpleGraph::new(num_vertices, edges),
        vec![One; num_vertices],
    )
}

fn reduce(problem: &MinimumDominatingSet<SimpleGraph, One>) -> ReductionDSToHS {
    ReduceTo::<MinimumHittingSet>::reduce_to(problem).expect("reduction should succeed")
}

fn all_configs(n: usize) -> impl Iterator<Item = Vec<bool>> {
    (0usize..1 << n).map(move |mask| (0..n).map(|i| mask >> i & 1 == 1).collect())
}

#[test]
fn test_minimumdominatingset_to_minimumhittingset_closed_loop() {
    let problem = source(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction = reduce(&problem);
    let target = reduction.target_problem();
    assert_eq!(target.universe_size(), 5);
    assert_eq!(
        target.sets(),
        &[
            vec![0, 1],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![3, 4]
        ]
    );
    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "DS(One)->HittingSet path",
    );
    // {1,3} is optimal; {0,2,4} and {0,1,3} are feasible but larger; no singleton works.
    for (chosen, value) in [
        (vec![1, 3], Some(2)),
        (vec![0, 2, 4], Some(3)),
        (vec![0, 1, 3], Some(3)),
    ] {
        let config: Vec<bool> = (0..5).map(|v| chosen.contains(&v)).collect();
        assert_eq!(target.evaluate(&config).unwrap(), Min(value));
        assert_eq!(
            problem
                .evaluate(&reduction.extract_solution(&config).unwrap())
                .unwrap(),
            Min(value)
        );
    }
    for v in 0..5 {
        let config: Vec<bool> = (0..5).map(|u| u == v).collect();
        assert_eq!(target.evaluate(&config).unwrap(), Min(None));
    }
    let optimum = BruteForce::new().find_all_witnesses(target).unwrap();
    assert!(
        optimum
            .iter()
            .all(|c| c.iter().filter(|&&b| b).count() == 2)
    );
}

#[test]
fn test_minimumdominatingset_to_minimumhittingset_exhaustive_up_to_four_vertices() {
    let solver = BruteForce::new();
    let mut graphs = 0;
    for n in 0..=4usize {
        let possible: Vec<(usize, usize)> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0usize..1 << possible.len() {
            let edges: Vec<(usize, usize)> = possible
                .iter()
                .enumerate()
                .filter(|(i, _)| mask >> i & 1 == 1)
                .map(|(_, &e)| e)
                .collect();
            let problem = source(n, edges.clone());
            let reduction = reduce(&problem);
            let target = reduction.target_problem();
            assert_eq!(target.universe_size(), n);
            assert_eq!(target.num_sets(), n);
            assert_eq!(
                target.sets().iter().map(Vec::len).sum::<usize>(),
                n + 2 * edges.len()
            );
            // Feasibility and value agree on every configuration.
            for config in all_configs(n) {
                assert_eq!(
                    problem.evaluate(&config).unwrap(),
                    target.evaluate(&config).unwrap(),
                    "n={n} edges={edges:?} config={config:?}"
                );
            }
            // Identity extraction of every optimal target configuration.
            let source_best = problem
                .evaluate(&solver.solve(&problem).unwrap().unwrap())
                .unwrap();
            for witness in solver.find_all_witnesses(target).unwrap() {
                let extracted = reduction.extract_solution(&witness).unwrap();
                assert_eq!(extracted, witness);
                assert_eq!(problem.evaluate(&extracted).unwrap(), source_best);
            }
            graphs += 1;
        }
    }
    assert_eq!(graphs, 1 + 1 + 2 + 8 + 64);
}

#[test]
fn test_minimumdominatingset_to_minimumhittingset_special_cases() {
    let optimum = |problem: &MinimumDominatingSet<SimpleGraph, One>| {
        let reduction = reduce(problem);
        BruteForce::new()
            .solve(reduction.target_problem())
            .unwrap()
            .map(|c| c.iter().filter(|&&b| b).count())
    };
    // Empty graph: empty universe, empty family, empty configuration.
    let empty = source(0, vec![]);
    let reduction = reduce(&empty);
    assert_eq!(reduction.target_problem().universe_size(), 0);
    assert_eq!(reduction.target_problem().num_sets(), 0);
    assert!(reduction.extract_solution(&vec![false]).is_err());
    assert_eq!(
        reduction.extract_solution(&vec![]).unwrap(),
        Vec::<bool>::new()
    );
    assert_eq!(optimum(&empty), Some(0));
    // Isolated vertices: singleton rows force every vertex.
    let isolated = source(3, vec![]);
    assert_eq!(
        reduce(&isolated).target_problem().sets(),
        &[vec![0], vec![1], vec![2]]
    );
    assert_eq!(optimum(&isolated), Some(3));
    // Paths, cycles, stars, complete graphs.
    for n in 3..=8usize {
        let path: Vec<_> = (0..n - 1).map(|v| (v, v + 1)).collect();
        let mut cycle = path.clone();
        cycle.push((0, n - 1));
        let star: Vec<_> = (1..n).map(|v| (0, v)).collect();
        let complete: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        assert_eq!(optimum(&source(n, path)), Some(n.div_ceil(3)));
        assert_eq!(optimum(&source(n, cycle)), Some(n.div_ceil(3)));
        assert_eq!(optimum(&source(n, star)), Some(1));
        assert_eq!(optimum(&source(n, complete)), Some(1));
    }
    // Disconnected graph with an isolated vertex.
    assert_eq!(
        optimum(&source(7, vec![(0, 1), (1, 2), (3, 4), (4, 5)])),
        Some(3)
    );
    // Equal closed neighborhoods keep one row per vertex.
    let edge = source(2, vec![(0, 1)]);
    assert_eq!(
        reduce(&edge).target_problem().sets(),
        &[vec![0, 1], vec![0, 1]]
    );
}

#[test]
fn test_minimumdominatingset_to_minimumhittingset_bounded_no_instance() {
    // Path P5 has optimum 2, so bound 1 is a NO instance on both sides.
    let problem = source(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction = reduce(&problem);
    for config in all_configs(5).filter(|c| c.iter().filter(|&&b| b).count() <= 1) {
        assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
        assert_eq!(
            reduction.target_problem().evaluate(&config).unwrap(),
            Min(None)
        );
    }
}

#[test]
fn test_minimumdominatingset_to_minimumhittingset_extraction_rejects_invalid() {
    let problem = source(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction = reduce(&problem);
    // Does not hit every set.
    assert!(reduction.extract_solution(&vec![false; 5]).is_err());
    assert!(
        reduction
            .extract_solution(&vec![true, false, false, false, false])
            .is_err()
    );
    // Wrong lengths.
    assert!(reduction.extract_solution(&vec![true; 4]).is_err());
    assert!(reduction.extract_solution(&vec![true; 6]).is_err());
    assert!(reduction.extract_solution(&vec![]).is_err());
    // A valid witness is returned unchanged.
    let valid = vec![false, true, false, true, false];
    assert_eq!(reduction.extract_solution(&valid).unwrap(), valid);
}

#[test]
fn test_minimumdominatingset_to_minimumhittingset_rejects_non_simple_graphs() {
    // The source model only admits simple graphs, so the construction never sees these.
    assert!(SimpleGraph::try_new(3, vec![(1, 1)]).is_err());
    assert!(SimpleGraph::try_new(3, vec![(0, 1), (0, 1)]).is_err());
    assert!(SimpleGraph::try_new(3, vec![(0, 1), (1, 0)]).is_err());
    assert!(SimpleGraph::try_new(3, vec![(0, 3)]).is_err());
}

#[cfg(feature = "example-db")]
#[test]
fn test_minimumdominatingset_to_minimumhittingset_canonical_example() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "minimumdominatingset_to_minimumhittingset")
        .expect("example spec should be registered")
        .build)();
    assert_eq!(example.source.problem, "MinimumDominatingSet");
    assert_eq!(example.target.problem, "MinimumHittingSet");
    assert_eq!(example.solutions.len(), 1);
    let solution = &example.solutions[0];
    assert_eq!(solution.source_config, solution.target_config);

    let source: MinimumDominatingSet<SimpleGraph, One> =
        serde_json::from_value(example.source.instance.clone()).unwrap();
    let target: MinimumHittingSet =
        serde_json::from_value(example.target.instance.clone()).unwrap();
    let source_config: Vec<bool> = serde_json::from_value(solution.source_config.clone()).unwrap();
    let target_config: Vec<bool> = serde_json::from_value(solution.target_config.clone()).unwrap();
    assert_eq!(source.evaluate(&source_config).unwrap(), Min(Some(2)));
    assert_eq!(target.evaluate(&target_config).unwrap(), Min(Some(2)));
}
