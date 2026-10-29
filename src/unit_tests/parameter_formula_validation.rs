use crate::parameters::ParameterRelation;
use crate::registry::DynProblem;
use crate::rules::{registry::ReductionEntry, ReductionGraph};
use serde_json::{json, Value};
use std::collections::BTreeMap;

type SourceKey = (String, BTreeMap<String, String>);

#[test]
fn matrix_targets_with_dimension_predictions_also_bound_entries() {
    for entry in crate::rules::registry::reduction_entries() {
        let (dimensions, count): (&[&str], &str) = match entry.target_name {
            "ILP" => (&["num_vars", "num_constraints"], "num_nonzeros"),
            "QUBO" | "DecisionQUBO" => (&["num_vars"], "num_quadratic_terms"),
            _ => continue,
        };
        let contract = entry.parameter_contract().unwrap();
        let Some(transform) = contract.transform() else {
            continue;
        };
        if dimensions
            .iter()
            .all(|field| transform.get(field).is_some())
        {
            assert!(
                transform.get(count).is_some(),
                "{} -> {} has dimension predictions but no {count} bound",
                entry.source_name,
                entry.target_name
            );
        }
    }
}

fn canonical_sources() -> &'static BTreeMap<SourceKey, Vec<Value>> {
    static SOURCES: std::sync::OnceLock<BTreeMap<SourceKey, Vec<Value>>> =
        std::sync::OnceLock::new();
    SOURCES.get_or_init(|| {
        let mut sources = BTreeMap::<SourceKey, Vec<Value>>::new();
        let db = crate::example_db::build_example_db().unwrap();
        for rule in db.rules {
            let source = rule.source;
            if let (Some(name), Some(inner)) = (
                source.problem.strip_prefix("Decision"),
                source.instance.get("inner"),
            ) {
                sources
                    .entry((name.to_string(), source.variant.clone()))
                    .or_default()
                    .push(inner.clone());
            }
            sources
                .entry((source.problem, source.variant))
                .or_default()
                .push(source.instance);
        }
        // Authored rule examples are smaller than some display-model instances.
        for model in db.models {
            sources
                .entry((model.problem, model.variant))
                .or_default()
                .push(model.instance);
        }
        sources
    })
}

fn target_parameters(
    entry: &ReductionEntry,
    source: &dyn DynProblem,
) -> Result<crate::ProblemParameters, String> {
    let variant = ReductionGraph::variant_to_map(&entry.target_variant());
    if let Some(reduce) = entry.reduce_fn {
        let reduced = reduce(source.as_any()).map_err(|error| error.to_string())?;
        return Ok(ReductionGraph::compute_problem_parameters(
            entry.target_name,
            &variant,
            reduced.target_problem_any(),
        ));
    }
    if let Some(reduce) = entry.reduce_aggregate_fn {
        let reduced = reduce(source.as_any()).map_err(|error| error.to_string())?;
        return Ok(ReductionGraph::compute_problem_parameters(
            entry.target_name,
            &variant,
            reduced.target_problem_any(),
        ));
    }
    Err("no executable reduction".into())
}

fn source_for(
    entry: &ReductionEntry,
    sources: &BTreeMap<SourceKey, Vec<Value>>,
) -> Result<(Box<dyn DynProblem>, crate::ProblemParameters), String> {
    let variant = ReductionGraph::variant_to_map(&entry.source_variant());
    let registered = crate::registry::find_variant_entry(entry.source_name, &variant).unwrap();
    let key = (entry.source_name.to_string(), variant.clone());
    // Try exact-variant examples first, then compatible variants of the same model.
    let examples = sources.get(&key).into_iter().flatten().chain(
        sources
            .iter()
            .filter(|(candidate_key, _)| {
                candidate_key.0 == entry.source_name && *candidate_key != &key
            })
            .flat_map(|(_, examples)| examples),
    );
    for example in examples {
        if let Ok(source) = (registered.factory)(example.clone()) {
            if let Ok(actual) = target_parameters(entry, source.as_ref()) {
                return Ok((source, actual));
            }
        }
    }
    // Geometric variants without canonical instances use the existing seeded
    // graph generators. Do not guess values for arbitrary new generator inputs:
    // a new contract should supply a usable example or explicit generator data.
    let random = registered
        .random
        .ok_or_else(|| format!("no usable canonical source for {key:?}"))?;
    let source = (random.generate)(json!({"num_vertices": 5, "seed": 42})).map_err(|error| {
        format!("no usable canonical source for {key:?}; graph generator: {error}")
    })?;
    let actual = target_parameters(entry, source.as_ref())?;
    Ok((source, actual))
}

#[test]
fn bounded_ilp_reductions_support_binary_encoding() {
    use crate::models::algebraic::{Bounded, ILP};
    use crate::rules::ReduceTo;
    use crate::traits::Problem;

    let sources = canonical_sources();
    let mut failures = Vec::new();
    let mut checked = 0;
    for entry in crate::rules::registry::reduction_entries() {
        if entry.target_name != "ILP"
            || entry.target_variant() != ILP::<i64, i64, Bounded>::variant()
        {
            continue;
        }
        let result = (|| {
            let (source, _) = source_for(entry, sources)?;
            let reduced =
                entry.reduce_fn.unwrap()(source.as_any()).map_err(|error| error.to_string())?;
            let integer = reduced
                .target_problem_any()
                .downcast_ref::<ILP<i64, i64, Bounded>>()
                .unwrap();
            ReduceTo::<ILP<bool>>::reduce_to(integer).map_err(|error| error.to_string())?;
            Ok::<_, String>(())
        })();
        if let Err(error) = result {
            failures.push(format!(
                "{} {:?}: {error}",
                entry.source_name,
                entry.source_variant()
            ));
        }
        checked += 1;
    }
    assert!(checked > 0);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn every_executable_parameter_formula_matches_a_constructed_target() {
    let sources = canonical_sources();
    let mut checked = 0;
    let mut expected = 0;
    let mut failures = Vec::new();
    for entry in crate::rules::registry::reduction_entries() {
        // Metadata-only and Turing edges have no single target constructor.
        // Their declarations are checked by symbolic_parameter_contracts; do not
        // manufacture a target here and claim it verifies a real executor.
        if entry.reduce_fn.is_none() && entry.reduce_aggregate_fn.is_none() {
            eprintln!(
                "no executable size check: {} -> {}",
                entry.source_name, entry.target_name
            );
            continue;
        }
        let contract = entry.parameter_contract().unwrap();
        let Some(transform) = contract.transform() else {
            continue;
        };
        expected += transform.expressions().count();
        let label = format!(
            "{} {:?} -> {} {:?}",
            entry.source_name,
            entry.source_variant(),
            entry.target_name,
            entry.target_variant()
        );
        let result = (|| {
            let (source, actual) = source_for(entry, sources)?;
            let predicted = transform
                .evaluate(&source.parameters_dyn())
                .map_err(|error| error.to_string())?;
            for (field, _) in transform.expressions() {
                let valid = match (predicted.get(field), actual.get(field)) {
                    (Some(predicted), Some(actual)) => match transform.relation(field).unwrap() {
                        ParameterRelation::Exact => predicted == actual,
                        ParameterRelation::UpperBound => predicted >= actual,
                    },
                    _ => false,
                };
                if !valid {
                    return Err(format!(
                        "{field} ({:?}): predicted {:?}, measured {:?}; source {}",
                        transform.relation(field).unwrap(),
                        predicted.get(field),
                        actual.get(field),
                        source.serialize_json()
                    ));
                }
                checked += 1;
            }
            Ok::<_, String>(())
        })();
        if let Err(error) = result {
            failures.push(format!("{label}: {error}"));
        }
    }
    assert!(expected > 0);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert_eq!(checked, expected, "some formula fields were not checked");
}

#[test]
fn sparse_ilp_predictions_use_construction_counts() {
    use crate::models::{BiconnectivityAugmentation, Factoring, StrongConnectivityAugmentation};
    use crate::topology::{MultiDigraph, SimpleDigraph, SimpleGraph};
    use crate::traits::Problem;

    use crate::models::graph::EulerianPath;

    // Independent row-count budgets for these small constructions, before cancellations:
    // biconnectivity: 3 budget terms + 20 commodities * (12 base + 24 candidate terms);
    // strong connectivity: 4 candidate/budget terms + 3 commodities * (8 base + 16 candidate);
    // factoring: 48 product terms + 5 factor bounds + at most 36 carry-related terms;
    // Eulerian path: 28 arc terms + at most 12 ordered pairs * 6 terms.
    let cases = [
        (
            "BiconnectivityAugmentation",
            BiconnectivityAugmentation::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![(0, 2, 1_i64), (0, 3, 2), (1, 3, 1)],
                3,
            )
            .parameters(),
            723_u64,
        ),
        (
            "StrongConnectivityAugmentation",
            StrongConnectivityAugmentation::new(
                SimpleDigraph::new(3, vec![(0, 1), (1, 2)]),
                vec![(2, 0, 1_i64), (1, 0, 2)],
                2,
            )
            .parameters(),
            76,
        ),
        (
            "Factoring",
            Factoring::with_factor_bits(15, 2, 3).parameters(),
            89,
        ),
        (
            "EulerianPath",
            EulerianPath::new(MultiDigraph::new(3, vec![(0, 1), (0, 1), (1, 2), (2, 0)]))
                .parameters(),
            100,
        ),
    ];
    let entries = crate::rules::registry::reduction_entries();
    for (name, parameters, budget) in cases {
        let entry = entries
            .iter()
            .find(|e| e.source_name == name && e.target_name == "ILP")
            .unwrap();
        let contract = entry.parameter_contract().unwrap();
        let prediction = contract.transform().unwrap().evaluate(&parameters).unwrap();
        assert!(
            prediction.get("num_nonzeros").unwrap() <= budget,
            "{name}: sparse count budget exceeded"
        );
    }
}

#[test]
fn simple_graph_create_contracts_propagate_edge_validation() {
    let names = [
        "BiconnectivityAugmentation",
        "BottleneckTravelingSalesman",
        "BoundedDiameterSpanningTree",
        "DisjointConnectingPaths",
        "KClique",
        "KColoring",
        "KthBestSpanningTree",
        "LengthBoundedDisjointPaths",
        "LongestCircuit",
        "LongestPath",
        "MaxCut",
        "MaximumIndependentSet",
        "MaximumMatching",
        "MinMaxMulticenter",
        "MinimumSumMulticenter",
        "MultipleCopyFileAllocation",
        "PrizeCollectingSteinerForest",
        "RuralPostman",
        "SpinGlass",
        "TravelingSalesman",
        "UndirectedTwoCommodityIntegralFlow",
    ];
    let sources = canonical_sources();
    let mut checked = std::collections::BTreeSet::new();
    for entry in crate::registry::variant_entries() {
        if !names.contains(&entry.name)
            || entry
                .variant_map()
                .get("graph")
                .is_some_and(|g| g != "SimpleGraph")
        {
            continue;
        }
        let inputs = entry.inputs();
        let instance = sources
            .iter()
            .find(|((name, variant), _)| name == entry.name && variant == &entry.variant_map())
            .or_else(|| sources.iter().find(|((name, _), _)| name == entry.name))
            .unwrap()
            .1
            .first()
            .unwrap();
        let mut data = serde_json::Map::new();
        for input in &inputs {
            if input.required && input.name != "graph" {
                data.insert(
                    input.name.to_string(),
                    instance
                        .get(input.name)
                        .or_else(|| {
                            (entry.name == "KColoring" && input.name == "k")
                                .then(|| &instance["num_colors"])
                        })
                        .unwrap_or_else(|| {
                            panic!("{}: missing canonical input {}", entry.name, input.name)
                        })
                        .clone(),
                );
            }
        }
        data.insert("graph".into(), instance["graph"]["edges"].clone());
        if inputs.iter().any(|input| input.name == "num_vertices") {
            data.insert(
                "num_vertices".into(),
                instance["graph"]["num_vertices"].clone(),
            );
        }
        (entry.construct_fn)(Value::Object(data.clone()))
            .unwrap_or_else(|error| panic!("{} {:?}: {error}", entry.name, entry.variant()));
        for (edges, offending) in [
            (json!([[0, 0]]), "edge (0, 0)"),
            (json!([[0, 1], [0, 1]]), "edge (0, 1)"),
            (json!([[0, 1], [1, 0]]), "edge (1, 0)"),
        ] {
            data.insert("graph".into(), edges);
            let error = (entry.construct_fn)(Value::Object(data.clone()))
                .err()
                .unwrap();
            assert!(
                error.to_string().contains(offending),
                "{}: {error}",
                entry.name
            );
        }
        if inputs.iter().any(|input| input.name == "num_vertices") {
            data.remove("num_vertices");
            data.insert("graph".into(), json!([]));
            let error = (entry.construct_fn)(Value::Object(data.clone()))
                .err()
                .unwrap();
            assert!(
                error.to_string().contains("num_vertices"),
                "{}: {error}",
                entry.name
            );
            data.insert("num_vertices".into(), json!(0));
            data.insert("graph".into(), json!([[0, 1]]));
            let error = (entry.construct_fn)(Value::Object(data.clone()))
                .err()
                .unwrap();
            assert!(
                error.to_string().contains("too small"),
                "{}: {error}",
                entry.name
            );
            data.insert("graph".into(), json!([[usize::MAX, 0]]));
            let error = (entry.construct_fn)(Value::Object(data.clone()))
                .err()
                .unwrap();
            assert!(
                error.to_string().contains("overflow"),
                "{}: {error}",
                entry.name
            );
        }
        checked.insert(entry.name);
    }
    assert_eq!(checked, names.into_iter().collect());
}

#[test]
fn direct_qubo_sparse_support_bounds_avoid_dense_cliques() {
    use crate::models::algebraic::QUBO;
    use crate::models::graph::OptimalLinearArrangement;
    use crate::models::misc::{
        CyclicOrdering, SequencingWithReleaseTimesAndDeadlines, ShortestCommonSuperstring,
        StackerCrane, StaffScheduling, ThreePartition,
    };
    use crate::models::Decision;
    use crate::rules::{ReduceTo, ReductionResult};
    use crate::topology::SimpleGraph;
    use crate::Problem;

    fn check<S: Problem + ReduceTo<T> + 'static, T: Problem>(source: S, budget: u64) {
        let entry = crate::rules::registry::reduction_entries()
            .into_iter()
            .find(|entry| {
                entry.source_name == S::NAME
                    && entry.source_variant() == S::variant()
                    && entry.target_name == T::NAME
                    && entry.target_variant() == T::variant()
            })
            .unwrap();
        let contract = entry.parameter_contract().unwrap();
        let predicted = contract
            .transform()
            .unwrap()
            .evaluate(&source.parameters())
            .unwrap();
        let reduced = <S as ReduceTo<T>>::reduce_to(&source).unwrap();
        let actual = reduced.target_problem().parameters();
        assert!(
            actual.get("num_quadratic_terms").unwrap()
                <= predicted.get("num_quadratic_terms").unwrap()
        );
        assert!(
            predicted.get("num_quadratic_terms").unwrap() <= budget,
            "{}: sparse support budget exceeded",
            S::NAME
        );
    }
    // Budget clique plus separate per-period surplus blocks, two bits for W = 3:
    // binom(8, 2) + 3 * (2^2 * 3 + binom(2, 2)).
    check::<_, Decision<QUBO<i64>>>(
        StaffScheduling::new(
            2,
            vec![
                vec![true, true, false],
                vec![false, true, true],
                vec![true, false, true],
            ],
            vec![2, 2, 2],
            3,
        ),
        67,
    );
    // Three tasks with at most three start times each: binom(9, 2) couplings.
    check::<_, Decision<QUBO<i64>>>(
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]),
        36,
    );
    // Triangle terms, path/cycle transitions, and eligible value-triple multiplicities.
    // Vanishing factors must remain nonnegative and valid on empty/singleton inputs.
    for n in 1..=2 {
        check::<_, Decision<QUBO<i64>>>(CyclicOrdering::new(n, vec![]), 0);
    }
    check::<_, QUBO<i64>>(
        OptimalLinearArrangement::new(SimpleGraph::new(0, vec![])),
        0,
    );
    check::<_, QUBO<i64>>(
        OptimalLinearArrangement::new(SimpleGraph::new(1, vec![])),
        0,
    );
    check::<_, QUBO<i64>>(ShortestCommonSuperstring::new(1, vec![vec![]]), 0);
    check::<_, QUBO<i64>>(StackerCrane::new(0, vec![], vec![], vec![], vec![]), 0);
    check::<_, QUBO<i64>>(
        StackerCrane::new(1, vec![(0, 0)], vec![], vec![1], vec![]),
        0,
    );
    check::<_, Decision<QUBO<i64>>>(ThreePartition::new(vec![2; 3], 6), 1);
    check::<_, Decision<QUBO<i64>>>(CyclicOrdering::new(5, vec![(0, 1, 2)]), 30);
    check::<_, QUBO<i64>>(
        OptimalLinearArrangement::new(SimpleGraph::new(5, vec![(0, 1), (1, 2)])),
        30,
    );
    check::<_, QUBO<i64>>(
        ShortestCommonSuperstring::new(2, vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]),
        84,
    );
    check::<_, QUBO<i64>>(
        StackerCrane::new(3, vec![(0, 1), (1, 2), (2, 0)], vec![], vec![1; 3], vec![]),
        36,
    );
    check::<_, Decision<QUBO<i64>>>(ThreePartition::new(vec![2; 6], 6), 15);
}

#[test]
fn ilp_nonzero_bounds_hold_on_dense_sources() {
    use crate::models::{
        ConsecutiveOnesSubmatrix, DirectedHamiltonianPath, LongestCircuit, SteinerTree,
        SubgraphIsomorphism,
    };
    use crate::topology::{SimpleDigraph, SimpleGraph};
    use crate::traits::Problem;

    // Dense inputs exercise every constraint family at full size, so a family
    // missing from a per-family nonzero count shows up as an exceeded bound.
    fn check<P: Problem + 'static>(source: P) {
        let variant = P::variant();
        let entry = crate::rules::registry::reduction_entries()
            .into_iter()
            .find(|e| {
                e.source_name == P::NAME && e.target_name == "ILP" && e.source_variant() == variant
            })
            .unwrap();
        let reduced = entry.reduce_fn.unwrap()(&source).unwrap();
        let actual = ReductionGraph::compute_problem_parameters(
            "ILP",
            &ReductionGraph::variant_to_map(&entry.target_variant()),
            reduced.target_problem_any(),
        );
        let contract = entry.parameter_contract().unwrap();
        let predicted = contract
            .transform()
            .unwrap()
            .evaluate(&source.parameters())
            .unwrap();
        let (predicted, actual) = (
            predicted.get("num_nonzeros").unwrap(),
            actual.get("num_nonzeros").unwrap(),
        );
        assert!(
            predicted >= actual,
            "{}: predicted {predicted:?} nonzeros, measured {actual:?}",
            P::NAME
        );
    }
    let complete = |n: usize| {
        (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect::<Vec<_>>()
    };

    check(ConsecutiveOnesSubmatrix::new(vec![vec![true; 4]; 3], 4));
    check(DirectedHamiltonianPath::new(SimpleDigraph::new(
        4,
        (0..4)
            .flat_map(|u| (0..4).filter(move |&v| v != u).map(move |v| (u, v)))
            .collect(),
    )));
    check(SubgraphIsomorphism::new(
        SimpleGraph::new(4, complete(4)),
        SimpleGraph::new(3, complete(3)),
    ));
    check(SteinerTree::new(
        SimpleGraph::new(5, complete(5)),
        vec![1_i64; 10],
        vec![0, 2, 4],
    ));
    check(LongestCircuit::new(
        SimpleGraph::new(5, complete(5)),
        vec![1_i64; 10],
    ));
}
