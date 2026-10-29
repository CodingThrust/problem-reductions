use crate::models::graph::*;
use crate::models::misc::{BinPacking, MinimumTardinessSequencing};
use crate::models::set::ComparativeContainment;
use crate::models::Decision;
use crate::rules::{ReductionError, ReductionGraph};
use crate::solvers::{BruteForce, BruteForceProblem};
use crate::topology::{MultiDigraph, MultiMixedGraph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{One, MAX_EXACT_F64_INTEGER};
use crate::variant::KN;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fmt::Debug;

#[test]
fn registered_numeric_variants_have_direct_promotions() {
    let variants: BTreeSet<_> = crate::registry::variant_entries()
        .into_iter()
        .map(|v| (v.name, v.variant_map()))
        .collect();
    let edges: BTreeSet<_> = crate::rules::registry::reduction_entries()
        .into_iter()
        .filter(|e| e.source_name == e.target_name && e.reduce_fn.is_some())
        .map(|e| {
            (
                e.source_name,
                ReductionGraph::variant_to_map(&e.source_variant()),
                ReductionGraph::variant_to_map(&e.target_variant()),
            )
        })
        .collect();
    let mut missing = Vec::new();
    for (name, source) in &variants {
        for field in ["weight", "coefficient"] {
            let to = match source.get(field).map(String::as_str) {
                Some("One") => "i64",
                Some("i64") => "f64",
                _ => continue,
            };
            let mut target = source.clone();
            target.insert(field.into(), to.into());
            if variants.contains(&(*name, target.clone()))
                && !edges.contains(&(*name, source.clone(), target.clone()))
            {
                missing.push(format!("{name} {source:?} -> {target:?}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "missing numeric promotions:\n{}",
        missing.join("\n")
    );
}

fn assert_cast<S, T>(source: S, maps_values: bool)
where
    S: Problem + BruteForceProblem + Serialize + 'static,
    T: Problem<Solution = S::Solution>
        + BruteForceProblem
        + Serialize
        + serde::de::DeserializeOwned
        + 'static,
    S::Solution: Eq + std::hash::Hash + Debug,
    S::Value: PartialEq + Debug + Serialize + crate::types::SolutionAggregate,
    T::Value: Serialize + crate::types::SolutionAggregate,
{
    let graph = ReductionGraph::new();
    let paths = graph.find_paths_up_to(
        S::NAME,
        &ReductionGraph::variant_to_map(&S::variant()),
        T::NAME,
        &ReductionGraph::variant_to_map(&T::variant()),
        1,
    );
    assert_eq!(
        paths.len(),
        1,
        "{} {:?} -> {:?}",
        S::NAME,
        S::variant(),
        T::variant()
    );
    assert_eq!(
        paths[0].steps.len(),
        2,
        "{} promotion must be direct",
        S::NAME
    );
    let chain = graph
        .reduce_along_path(&paths[0], &source)
        .unwrap()
        .unwrap();
    let target = chain.target_problem::<T>();
    assert_eq!(target.parameters(), source.parameters());
    let expected: T = serde_json::from_value(serde_json::to_value(&source).unwrap()).unwrap();
    assert_eq!(
        serde_json::to_value(target).unwrap(),
        serde_json::to_value(expected).unwrap()
    );
    let solver = BruteForce::new();
    let source_solution = solver.solve(&source).unwrap();
    let target_solutions = solver.find_all_witnesses(target).unwrap();
    if let Some(source_solution) = source_solution {
        let optimum = source.evaluate(&source_solution).unwrap();
        assert!(!target_solutions.is_empty());
        for solution in target_solutions {
            let recovered = chain.extract_solution::<S::Solution, _>(&solution).unwrap();
            assert_eq!(recovered, solution);
            let recovered_value = source.evaluate(&recovered).unwrap();
            assert_eq!(recovered_value, optimum);
            if maps_values {
                assert!(chain.has_value_mapping());
                let value = serde_json::to_value(target.evaluate(&solution).unwrap()).unwrap();
                assert_eq!(
                    chain.extract_value(value).unwrap(),
                    serde_json::to_value(recovered_value).unwrap()
                );
            }
        }
    } else {
        assert!(target_solutions.is_empty());
    }
}

#[test]
fn unit_weight_promotions_preserve_instances_and_optima() {
    let g = SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2)]);
    assert_cast::<MaxCut<_, One>, MaxCut<SimpleGraph, i64>>(
        MaxCut::new(g.clone(), vec![One; 3]),
        true,
    );
    assert_cast::<MaximumClique<_, One>, MaximumClique<SimpleGraph, i64>>(
        MaximumClique::new(g.clone(), vec![One; 4]),
        true,
    );
    assert_cast::<MaximumCoKPlex<_, One, KN>, MaximumCoKPlex<SimpleGraph, i64, KN>>(
        MaximumCoKPlex::with_k(g.clone(), vec![One; 4], 2),
        true,
    );
    assert_cast::<MinimumDominatingSet<_, One>, MinimumDominatingSet<SimpleGraph, i64>>(
        MinimumDominatingSet::new(g.clone(), vec![One; 4]),
        true,
    );
    assert_cast::<MinimumFeedbackVertexSet<One>, MinimumFeedbackVertexSet<i64>>(
        MinimumFeedbackVertexSet::new(
            MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
            vec![One; 3],
        ),
        true,
    );
    assert_cast::<LongestPath<_, One>, LongestPath<SimpleGraph, i64>>(
        LongestPath::new(g.clone(), vec![One; 3], 0, 2),
        true,
    );
    assert_cast::<MinMaxMulticenter<_, One>, MinMaxMulticenter<SimpleGraph, i64>>(
        MinMaxMulticenter::new(g.clone(), vec![One; 4], vec![One; 3], 2),
        true,
    );
    assert_cast::<SteinerTree<_, One>, SteinerTree<SimpleGraph, i64>>(
        SteinerTree::unit_weights(g, vec![0, 2]),
        true,
    );
    assert_cast::<MixedChinesePostman<One>, MixedChinesePostman<i64>>(
        MixedChinesePostman::new(
            MultiMixedGraph::new(2, vec![(0, 1)], vec![(0, 1)]),
            vec![One],
            vec![One],
        ),
        true,
    );
    assert_cast::<MinimumTardinessSequencing<One>, MinimumTardinessSequencing<i64>>(
        MinimumTardinessSequencing::new(3, vec![1, 1, 2], vec![(0, 2)]),
        true,
    );
    assert_cast::<ComparativeContainment<One>, ComparativeContainment<i64>>(
        ComparativeContainment::new(2, vec![vec![0]], vec![vec![1], vec![0, 1]]).unwrap(),
        true,
    );
}

#[test]
fn decision_weight_promotions_preserve_bounds_and_answers() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    for bound in [-1, 1, 2, i64::MAX] {
        assert_cast::<
            Decision<MaximumIndependentSet<_, One>>,
            Decision<MaximumIndependentSet<SimpleGraph, i64>>,
        >(
            Decision::new(MaximumIndependentSet::new(g.clone(), vec![One; 3]), bound),
            true,
        );
        assert_cast::<
            Decision<MinimumDominatingSet<_, One>>,
            Decision<MinimumDominatingSet<SimpleGraph, i64>>,
        >(
            Decision::new(MinimumDominatingSet::new(g.clone(), vec![One; 3]), bound),
            true,
        );
        assert_cast::<
            Decision<MinimumVertexCover<_, One>>,
            Decision<MinimumVertexCover<SimpleGraph, i64>>,
        >(
            Decision::new(MinimumVertexCover::new(g.clone(), vec![One; 3]), bound),
            true,
        );
    }
}

#[test]
fn integer_weight_promotions_preserve_optima_and_feasibility() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    assert_cast::<MaximumIndependentSet<_, i64>, MaximumIndependentSet<SimpleGraph, f64>>(
        MaximumIndependentSet::new(g.clone(), vec![3, -2, 5]),
        false,
    );
    assert_cast::<MaximumEdgeWeightedKClique<i64>, MaximumEdgeWeightedKClique<f64>>(
        MaximumEdgeWeightedKClique::new(g.clone(), vec![-2, 3], 2).unwrap(),
        false,
    );
    assert_cast::<ComparativeContainment<i64>, ComparativeContainment<f64>>(
        ComparativeContainment::with_weights(
            2,
            vec![vec![0], vec![0, 1]],
            vec![vec![1], vec![0, 1]],
            vec![2, 5],
            vec![3, 6],
        )
        .unwrap(),
        true,
    );
    assert_cast::<BinPacking<i64>, BinPacking<f64>>(
        BinPacking::new(vec![2, 3, 4], 5).unwrap(),
        true,
    );
    assert_cast::<
        PrizeCollectingSteinerForest<_, i64>,
        PrizeCollectingSteinerForest<SimpleGraph, f64>,
    >(
        PrizeCollectingSteinerForest::new(g, vec![5, 2, 5], vec![1, 6], 1, 2).unwrap(),
        false,
    );
}

fn assert_float_rejected<S: Problem + 'static, T: Problem>(source: S) {
    let graph = ReductionGraph::new();
    let paths = graph.find_paths_up_to(
        S::NAME,
        &ReductionGraph::variant_to_map(&S::variant()),
        T::NAME,
        &ReductionGraph::variant_to_map(&T::variant()),
        1,
    );
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].steps.len(), 2);
    assert!(matches!(
        graph.reduce_along_path(&paths[0], &source),
        Err(ReductionError::InexactFloatConversion { .. })
            | Err(ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn float_promotions_reject_inexact_accumulations_and_products() {
    let big = MAX_EXACT_F64_INTEGER;
    assert_float_rejected::<MaximumIndependentSet<_, i64>, MaximumIndependentSet<SimpleGraph, f64>>(
        MaximumIndependentSet::new(SimpleGraph::new(3, vec![]), vec![big, 1, 1]),
    );
    assert_float_rejected::<MaximumEdgeWeightedKClique<i64>, MaximumEdgeWeightedKClique<f64>>(
        MaximumEdgeWeightedKClique::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
            vec![big, 1, 1],
            3,
        )
        .unwrap(),
    );
    assert_float_rejected::<ComparativeContainment<i64>, ComparativeContainment<f64>>(
        ComparativeContainment::with_weights(
            1,
            vec![vec![0]; 3],
            vec![vec![0]],
            vec![big, 1, 1],
            vec![big],
        )
        .unwrap(),
    );
    assert_float_rejected::<BinPacking<i64>, BinPacking<f64>>(
        BinPacking::new(vec![big, 1, 1], big).unwrap(),
    );
    assert_float_rejected::<
        PrizeCollectingSteinerForest<_, i64>,
        PrizeCollectingSteinerForest<SimpleGraph, f64>,
    >(
        PrizeCollectingSteinerForest::new(SimpleGraph::new(1, vec![]), vec![big], vec![], 2, 0)
            .unwrap(),
    );
}

#[test]
fn float_promotions_reject_unrepresentable_input_fields() {
    let inexact = MAX_EXACT_F64_INTEGER + 2;
    for weight in [inexact, i64::MIN] {
        assert_float_rejected::<
            MaximumIndependentSet<_, i64>,
            MaximumIndependentSet<SimpleGraph, f64>,
        >(MaximumIndependentSet::new(
            SimpleGraph::new(1, vec![]),
            vec![weight],
        ));
        assert_float_rejected::<MaximumEdgeWeightedKClique<i64>, MaximumEdgeWeightedKClique<f64>>(
            MaximumEdgeWeightedKClique::new(SimpleGraph::new(2, vec![(0, 1)]), vec![weight], 2)
                .unwrap(),
        );
        assert_float_rejected::<BinPacking<i64>, BinPacking<f64>>(
            BinPacking::new(vec![1], weight).unwrap(),
        );
    }
    assert_float_rejected::<ComparativeContainment<i64>, ComparativeContainment<f64>>(
        ComparativeContainment::with_weights(
            1,
            vec![vec![0]],
            vec![vec![0]],
            vec![1],
            vec![inexact],
        )
        .unwrap(),
    );
    assert_float_rejected::<
        PrizeCollectingSteinerForest<_, i64>,
        PrizeCollectingSteinerForest<SimpleGraph, f64>,
    >(
        PrizeCollectingSteinerForest::new(SimpleGraph::new(1, vec![]), vec![1], vec![], inexact, 0)
            .unwrap(),
    );
    assert_float_rejected::<BinPacking<i64>, BinPacking<f64>>(
        BinPacking::new(vec![inexact], 1).unwrap(),
    );
    assert_float_rejected::<
        PrizeCollectingSteinerForest<_, i64>,
        PrizeCollectingSteinerForest<SimpleGraph, f64>,
    >(
        PrizeCollectingSteinerForest::new(SimpleGraph::new(1, vec![]), vec![inexact], vec![], 1, 0)
            .unwrap(),
    );
}

#[test]
fn float_forest_promotion_rejects_overflowing_scaled_costs() {
    let big = MAX_EXACT_F64_INTEGER;
    assert_float_rejected::<
        PrizeCollectingSteinerForest<_, i64>,
        PrizeCollectingSteinerForest<SimpleGraph, f64>,
    >(
        PrizeCollectingSteinerForest::new(SimpleGraph::new(1, vec![]), vec![big], vec![], big, 0)
            .unwrap(),
    );
}
