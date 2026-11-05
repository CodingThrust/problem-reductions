use super::{Parameters, construction_parameters};
use crate::models::decision::Decision;
use crate::models::graph::{MinimumCutIntoBoundedSets, MinimumVertexCover};
use crate::rules::{ReduceTo, ReductionError, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One, Or};

type Source = Decision<MinimumVertexCover<SimpleGraph, One>>;
type Target = MinimumCutIntoBoundedSets<SimpleGraph, i64>;
type Reduction = super::ReductionDecisionMinimumVertexCoverToMinimumCutIntoBoundedSets;

const EXAMPLE_EDGES: [(usize, usize); 7] = [(0, 1), (0, 2), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5)];

fn source(n: usize, edges: &[(usize, usize)], bound: i64) -> Source {
    Decision::new(
        MinimumVertexCover::new(SimpleGraph::new(n, edges.to_vec()), vec![One; n]),
        bound,
    )
}

fn reduce(n: usize, edges: &[(usize, usize)], bound: i64) -> Reduction {
    ReduceTo::<Target>::reduce_to(&source(n, edges, bound)).expect("reduction should succeed")
}

/// The target of the issue's example graph on six vertices.
fn example(bound: i64) -> (Source, Reduction) {
    let source = source(6, &EXAMPLE_EDGES, bound);
    let reduction = ReduceTo::<Target>::reduce_to(&source).unwrap();
    (source, reduction)
}

fn value_map(reduction: &Reduction, value: Min<i64>) -> Or {
    crate::rules::AggregateReductionResult::extract_value(reduction, value)
}

fn optimum(target: &Target) -> Option<i64> {
    BruteForce::new()
        .find_all_witnesses(target)
        .unwrap()
        .first()
        .map(|solution| target.evaluate(solution).unwrap().0.unwrap())
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_structure() {
    let (_, reduction) = example(3);
    let target = reduction.target_problem();
    // 6 original vertices, s = 6, t = 7 and q = n - T = 3 padding vertices.
    assert_eq!(target.num_vertices(), 11);
    assert_eq!((target.source(), target.sink()), (6, 7));
    assert_eq!(target.size_bound(), 7);
    // The 8 complement edges, then {i, s}, {i, t} and the padding edges {t, pad}.
    let complement = [
        (0, 3),
        (2, 3),
        (0, 4),
        (1, 4),
        (0, 5),
        (1, 5),
        (2, 5),
        (3, 5),
    ];
    let mut edges = complement.to_vec();
    edges.extend((0..6).map(|i| (i, 6)));
    edges.extend((0..6).map(|i| (i, 7)));
    edges.extend((8..11).map(|pad| (7, pad)));
    assert_eq!(target.graph().edges(), edges);
    assert_eq!(target.num_edges(), 23);
    // Phi = 2 * 8 + 2 * 6 = 28, M = 29; deg_H = (3, 2, 2, 3, 2, 4).
    let mut weights = vec![1; 8];
    weights.extend([1; 6]);
    weights.extend([5, 4, 4, 5, 4, 6]);
    weights.extend([29; 3]);
    assert_eq!(target.edge_weights(), weights.as_slice());
    assert!(target.edge_weights().iter().all(|&weight| weight > 0));
    // Threshold Phi - T^2 = 28 - 9.
    assert_eq!(reduction.threshold, 19);
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_closed_loop() {
    let (source, reduction) = example(3);
    let target = reduction.target_problem();
    crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "DecisionMinimumVertexCover -> MinimumCutIntoBoundedSets",
    );
    let witnesses = BruteForce::new().find_all_witnesses(target).unwrap();
    assert!(!witnesses.is_empty());
    for witness in witnesses {
        let value = target.evaluate(&witness).unwrap();
        assert_eq!(value, Min(Some(19)));
        assert_eq!(value_map(&reduction, value), Or(true));
        let cover = reduction.extract_solution(&witness).unwrap();
        assert_eq!(cover, witness[..6].iter().map(|&b| !b).collect::<Vec<_>>());
    }
    // The cover {1, 2, 4} comes from the independent set {0, 3, 5} on the sink's side.
    let partition = vec![
        true, false, false, true, false, true, false, true, true, true, true,
    ];
    assert_eq!(target.evaluate(&partition).unwrap(), Min(Some(19)));
    assert_eq!(
        reduction.extract_solution(&partition).unwrap(),
        vec![false, true, true, false, true, false]
    );
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_no_instance() {
    // The minimum cover has size 3, so the bound 2 is a NO instance: T = 4 and the threshold is
    // 28 - 16 = 12, below the optimal cut.
    let (source, reduction) = example(2);
    let target = reduction.target_problem();
    assert_eq!(reduction.threshold, 12);
    assert_eq!(BruteForce::new().solve(&source).unwrap(), None);
    let best = optimum(target).unwrap();
    assert!(best > 12, "optimal cut {best} must exceed the threshold");
    for witness in BruteForce::new().find_all_witnesses(target).unwrap() {
        let value = target.evaluate(&witness).unwrap();
        assert_eq!(value_map(&reduction, value), Or(false));
        assert!(reduction.extract_solution(&witness).is_err());
    }
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_infeasible_bound() {
    // A negative bound has no cover; T = n + 1, so no cut reaches the threshold Phi - (n + 1)^2.
    for bound in [-1, i64::MIN] {
        let (source, reduction) = example(bound);
        let target = reduction.target_problem();
        assert_eq!(BruteForce::new().solve(&source).unwrap(), None);
        // No padding: the target is the bare construction with q = 0.
        assert_eq!(target.num_vertices(), 8);
        assert_eq!(reduction.threshold, 28 - 49);
        let best = optimum(target).unwrap();
        assert!(best > reduction.threshold);
        assert_eq!(value_map(&reduction, Min(Some(best))), Or(false));
    }
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_empty_graph() {
    // n = 0: only s = 0 and t = 1, one side of size at most 1, no edges.
    let reduction = reduce(0, &[], 0);
    let target = reduction.target_problem();
    assert_eq!(target.num_vertices(), 2);
    assert_eq!(target.num_edges(), 0);
    assert_eq!(target.size_bound(), 1);
    assert_eq!(reduction.threshold, 0);
    let partition = vec![false, true];
    assert_eq!(target.evaluate(&partition).unwrap(), Min(Some(0)));
    assert_eq!(
        reduction.extract_solution(&partition).unwrap(),
        Vec::<bool>::new()
    );
    // A negative bound has no cover even for the empty graph: T = 1, threshold -1.
    let reduction = reduce(0, &[], -1);
    assert_eq!(reduction.threshold, -1);
    assert_eq!(reduction.target_problem().num_vertices(), 2);
    assert!(reduction.extract_solution(&vec![false, true]).is_err());
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_bound_equals_vertex_count() {
    // k = n gives T = 0, so n padding vertices; k = 0 on one vertex gives T = n = 1, q = 0.
    let reduction = reduce(1, &[], 0);
    let target = reduction.target_problem();
    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.graph().edges(), vec![(0, 1), (0, 2)]);
    assert_eq!(target.edge_weights(), &[1, 2]);
    assert_eq!(reduction.threshold, 1); // Phi = 2, T = 1
    // The lone vertex on the sink's side cuts only {0, s} and is the empty cover.
    assert_eq!(
        reduction
            .extract_solution(&vec![true, false, true])
            .unwrap(),
        vec![false]
    );
    // Putting it on the source's side cuts {0, t} of weight 2 > 1: not certifying.
    assert!(
        reduction
            .extract_solution(&vec![false, false, true])
            .is_err()
    );

    // For k >= n every bound gives the same instance, with T = 0 and n padding vertices.
    let edges = [(0, 1), (1, 2)];
    let at_n = reduce(3, &edges, 3);
    for bound in [4, 100, i64::MAX] {
        let other = reduce(3, &edges, bound);
        assert_eq!(other.target.graph().edges(), at_n.target.graph().edges());
        assert_eq!(other.target.edge_weights(), at_n.target.edge_weights());
        assert_eq!(other.threshold, at_n.threshold);
    }
    assert_eq!(at_n.target.num_vertices(), 3 + 2 + 3);
    assert_eq!(at_n.threshold, 8); // |E(H)| = 1, Phi = 2 + 6, T = 0
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_intermediate_bound() {
    // Path 0-1-2, k = 1: T = 2, one padding vertex, the cover {1}.
    let (source, reduction) = (
        source(3, &[(0, 1), (1, 2)], 1),
        reduce(3, &[(0, 1), (1, 2)], 1),
    );
    let target = reduction.target_problem();
    assert_eq!(target.num_vertices(), 6);
    // H has the single edge {0, 2}; Phi = 2 + 6 = 8, M = 9, threshold 8 - 4 = 4.
    assert_eq!(target.graph().edges()[0], (0, 2));
    assert_eq!(reduction.threshold, 4);
    // I = {0, 2} and the padding vertex on t's side: cut = Phi - 2 * 1 - 2 = 4.
    let partition = vec![true, false, true, false, true, true];
    assert_eq!(target.evaluate(&partition).unwrap(), Min(Some(4)));
    let cover = reduction.extract_solution(&partition).unwrap();
    assert_eq!(cover, vec![false, true, false]);
    assert_eq!(source.evaluate(&cover).unwrap(), Or(true));
    // The padding vertex on the source's side costs M and is rejected.
    let padding_on_source = vec![true, false, true, false, true, false];
    assert_eq!(
        target.evaluate(&padding_on_source).unwrap(),
        Min(Some(4 + 9))
    );
    assert!(reduction.extract_solution(&padding_on_source).is_err());
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_edge_order_is_irrelevant() {
    let forward = reduce(4, &[(0, 1), (2, 3)], 2);
    let shuffled = reduce(4, &[(3, 2), (1, 0)], 2);
    assert_eq!(
        forward.target.graph().edges(),
        shuffled.target.graph().edges()
    );
    assert_eq!(
        forward.target.edge_weights(),
        shuffled.target.edge_weights()
    );
    assert_eq!(forward.threshold, shuffled.threshold);
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_extraction_rejects() {
    let (_, reduction) = example(3);
    let target = reduction.target_problem();
    let good = vec![
        true, false, false, true, false, true, false, true, true, true, true,
    ];
    assert!(reduction.extract_solution(&good).is_ok());
    // Wrong lengths.
    assert!(reduction.extract_solution(&good[..10].to_vec()).is_err());
    let mut longer = good.clone();
    longer.push(true);
    assert!(reduction.extract_solution(&longer).is_err());
    assert!(reduction.extract_solution(&Vec::new()).is_err());
    // The source on the sink's side, or the sink on the source's side: infeasible.
    let mut bad = good.clone();
    bad[6] = true;
    assert_eq!(target.evaluate(&bad).unwrap(), Min(None));
    assert!(reduction.extract_solution(&bad).is_err());
    let mut bad = good.clone();
    bad[7] = false;
    assert!(reduction.extract_solution(&bad).is_err());
    // A side larger than the size bound 7 is infeasible: all three paddings and everything else
    // on the sink's side.
    let mut oversized = vec![true; 11];
    oversized[6] = false;
    assert_eq!(target.evaluate(&oversized).unwrap(), Min(None));
    assert!(reduction.extract_solution(&oversized).is_err());
    // Feasible but above the threshold: the padding on the source's side.
    let mut above = good.clone();
    above[8] = false;
    assert!(target.evaluate(&above).unwrap().0.unwrap() > 19);
    assert!(reduction.extract_solution(&above).is_err());
    // Feasible, within the threshold, but not an independent set: nothing within the bound
    // exists, so every other feasible partition is above the threshold (checked exhaustively).
    for mask in 0usize..1 << 11 {
        let partition: Vec<bool> = (0..11).map(|i| mask & (1 << i) != 0).collect();
        if let Ok(cover) = reduction.extract_solution(&partition) {
            assert_eq!(
                Source::evaluate(&source(6, &EXAMPLE_EDGES, 3), &cover).unwrap(),
                Or(true)
            );
        }
    }
}

// Every graph on at most four vertices, every bound around the interesting range, and EVERY
// target assignment (feasible or not): both directions of the equivalence, and extraction never
// returns a false witness.
#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_all_small_graphs() {
    for n in 0..=4usize {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for graph_mask in 0usize..1 << pairs.len() {
            let edges: Vec<_> = pairs
                .iter()
                .enumerate()
                .filter(|(i, _)| graph_mask & (1 << i) != 0)
                .map(|(_, &edge)| edge)
                .collect();
            let bounds = [i64::MIN, -1, i64::MAX]
                .into_iter()
                .chain(0..=i64::try_from(n).unwrap() + 1);
            for bound in bounds {
                let source = source(n, &edges, bound);
                let reduction = ReduceTo::<Target>::reduce_to(&source).unwrap();
                let target = reduction.target_problem();
                let vertices = target.num_vertices();
                assert!(vertices <= 2 * n + 2);
                assert!(target.num_edges() <= n * n.saturating_sub(1) / 2 - edges.len() + 3 * n);
                let source_yes = BruteForce::new().solve(&source).unwrap().is_some();
                let mut best: Option<i64> = None;
                for mask in 0usize..1 << vertices {
                    let partition: Vec<bool> =
                        (0..vertices).map(|i| mask & (1 << i) != 0).collect();
                    let value = target.evaluate(&partition).unwrap();
                    if let Some(cut) = value.0 {
                        best = Some(best.map_or(cut, |previous| previous.min(cut)));
                    }
                    let accepted = value_map(&reduction, value).0;
                    match reduction.extract_solution(&partition) {
                        Ok(cover) => {
                            assert!(accepted);
                            assert_eq!(source.evaluate(&cover).unwrap(), Or(true));
                        }
                        Err(_) => assert!(!accepted),
                    }
                }
                assert_eq!(
                    value_map(&reduction, Min(best)),
                    Or(source_yes),
                    "n={n}, edges={edges:?}, bound={bound}"
                );
            }
        }
    }
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_larger_graphs() {
    // The five-cycle (minimum cover 3) and the star K_{1,4} (minimum cover 1).
    let cycle = [(0, 1), (1, 2), (2, 3), (3, 4), (0, 4)];
    let star = [(0, 1), (0, 2), (0, 3), (0, 4)];
    for (edges, yes_from) in [(&cycle[..], 3), (&star[..], 1)] {
        for bound in 0..=5 {
            let reduction = reduce(5, edges, bound);
            let best = optimum(reduction.target_problem()).unwrap();
            let yes = best <= reduction.threshold;
            assert_eq!(yes, bound >= yes_from, "edges={edges:?}, bound={bound}");
            for witness in BruteForce::new()
                .find_all_witnesses(reduction.target_problem())
                .unwrap()
            {
                assert_eq!(reduction.extract_solution(&witness).is_ok(), yes);
            }
        }
    }
}

fn parameters(n: usize, m: usize, bound: i64) -> Result<Parameters, ReductionError> {
    construction_parameters(n, m, bound)
}

#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_parameters() {
    assert_eq!(
        parameters(6, 7, 3).unwrap(),
        Parameters {
            independent_size: 3,
            padding: 3,
            target_vertices: 11,
            complement_edges: 8,
            target_edges: 23,
            heavy_weight: 29,
            threshold: 19,
        }
    );
    // Clamping of T = n - bound to 0..=n + 1 at every boundary of the bound.
    for (bound, t, q) in [
        (i64::MIN, 7, 0),
        (-1, 7, 0),
        (0, 6, 0),
        (5, 1, 5),
        (6, 0, 6),
        (7, 0, 6),
        (i64::MAX, 0, 6),
    ] {
        let p = parameters(6, 0, bound).unwrap();
        assert_eq!((p.independent_size, p.padding), (t, q), "bound={bound}");
    }
    // The number of edges of the source cannot exceed the number of pairs.
    assert!(parameters(4, 6, 1).is_ok());
    assert!(matches!(
        parameters(4, 7, 1),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    // Counts that do not fit in `usize`.
    assert!(matches!(
        parameters(usize::MAX, 0, 0),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    assert!(matches!(
        parameters(usize::MAX - 1, 0, -1),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_parameters_64_bit_overflow() {
    for (n, bound) in [(1usize << 63, i64::MAX), (1 << 32, 0), ((1 << 32) + 1, 0)] {
        assert!(
            matches!(
                parameters(n, 0, bound),
                Err(ReductionError::IntegerOverflow { .. })
            ),
            "n={n}, bound={bound}"
        );
    }
}

/// The total weight of the construction, computed in `u128` without any overflow.
#[cfg(target_pointer_width = "64")]
fn total_weight(n: u128, m: u128, bound: i64) -> u128 {
    let n_signed = i128::try_from(n).unwrap();
    let t = u128::try_from((n_signed - i128::from(bound)).clamp(0, n_signed + 1)).unwrap();
    let q = n.saturating_sub(t);
    let complement = n * n.saturating_sub(1) / 2 - m;
    let phi = 2 * complement + 2 * n;
    complement + n + (phi) + q * (phi + 1)
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_decisionminimumvertexcover_to_minimumcutintoboundedsets_weight_boundary() {
    // For each regime of the bound, find by bisection the largest n whose total weight (an upper
    // bound on every cut) fits in `i64`, and require the parameters to succeed there and fail
    // one vertex later.
    for bound in [i64::MAX, 0, -1] {
        let (mut low, mut high) = (1u128, 1u128 << 40);
        while low + 1 < high {
            let mid = (low + high) / 2;
            if total_weight(mid, 0, bound) <= u128::try_from(i64::MAX).unwrap() {
                low = mid;
            } else {
                high = mid;
            }
        }
        let n = usize::try_from(low).unwrap();
        assert!(parameters(n, 0, bound).is_ok());
        assert!(i64::try_from(total_weight(low, 0, bound)).is_ok());
        assert!(
            matches!(
                parameters(n + 1, 0, bound),
                Err(ReductionError::IntegerOverflow { .. })
            ),
            "bound={bound}, n={n}"
        );
    }
}
