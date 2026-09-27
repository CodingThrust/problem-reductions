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

fn canonical_sources() -> BTreeMap<SourceKey, Vec<Value>> {
    let mut sources = BTreeMap::<SourceKey, Vec<Value>>::new();
    let db = crate::example_db::build_example_db().unwrap();
    for model in db.models {
        sources
            .entry((model.problem, model.variant))
            .or_default()
            .push(model.instance);
    }
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
    sources
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
    let source_json = source.serialize_json();
    let target_json = if entry.turing {
        json!({"inner": source_json, "bound": 2})
    } else if entry.source_name == "MinimumVertexCover"
        && entry.target_name == "MinimumMaximalMatching"
    {
        json!({"graph": source_json["graph"]})
    } else if entry.source_name == "SubsetSum" && entry.target_name == "IntegerKnapsack" {
        let sizes: Vec<i64> = source_json["sizes"]
            .as_array()
            .ok_or("SubsetSum sizes are not an array")?
            .iter()
            .map(|item| {
                item.as_str()
                    .ok_or("size is not a string")?
                    .parse()
                    .map_err(|error| format!("{error}"))
            })
            .collect::<Result<_, String>>()?;
        let capacity: i64 = source_json["target"]
            .as_str()
            .ok_or("target is not a string")?
            .parse()
            .map_err(|error| format!("{error}"))?;
        json!({"sizes": sizes, "values": sizes, "capacity": capacity})
    } else {
        return Err("no executable reduction or test construction".into());
    };
    let target = crate::registry::load_dyn(entry.target_name, &variant, target_json)
        .map_err(|error| error.to_string())?;
    Ok(target.parameters_dyn())
}

fn source_for(
    entry: &ReductionEntry,
    sources: &BTreeMap<SourceKey, Vec<Value>>,
) -> Result<Box<dyn DynProblem>, String> {
    let variant = ReductionGraph::variant_to_map(&entry.source_variant());
    let registered = crate::registry::find_variant_entry(entry.source_name, &variant).unwrap();
    let key = (entry.source_name.to_string(), variant.clone());
    if let Some(examples) = sources.get(&key) {
        for example in examples {
            if let Ok(source) = (registered.factory)(example.clone()) {
                if target_parameters(entry, source.as_ref()).is_ok() {
                    return Ok(source);
                }
            }
        }
    }
    if entry.source_name == "ILP" && variant.get("variable").is_some_and(|v| v == "i64") {
        use crate::models::algebraic::{IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
        return Ok(Box::new(
            ILP::<i64>::with_variables(
                vec![IntegerVariable::new(Some(0), Some(3)).unwrap()],
                vec![LinearConstraint::le(vec![(0, 1)], 3)],
                vec![(0, 1)],
                ObjectiveSense::Minimize,
            )
            .unwrap(),
        ));
    }
    let random = registered
        .random
        .ok_or_else(|| format!("no usable canonical source for {key:?}"))?;
    let mut args = serde_json::Map::new();
    for input in (random.inputs)() {
        let value = match input.name {
            "num_vertices" => json!(5),
            "seed" => json!(42),
            "k" => json!(if variant.get("k").is_some_and(|v| v == "K3") {
                3
            } else {
                2
            }),
            "bound" => json!(2),
            _ if !input.required => continue,
            name => return Err(format!("unsupported random input {name}")),
        };
        args.insert(input.name.to_string(), value);
    }
    (random.generate)(Value::Object(args)).map_err(|error| error.to_string())
}

#[test]
fn integer_ilp_reductions_support_binary_encoding() {
    use crate::models::algebraic::ILP;
    use crate::rules::ReduceTo;
    use crate::traits::Problem;

    let sources = canonical_sources();
    let mut failures = Vec::new();
    let mut checked = 0;
    for entry in crate::rules::registry::reduction_entries() {
        if entry.target_name != "ILP" || entry.target_variant() != ILP::<i64>::variant() {
            continue;
        }
        let result = (|| {
            let source = source_for(entry, &sources)?;
            let reduced =
                entry.reduce_fn.unwrap()(source.as_any()).map_err(|error| error.to_string())?;
            let integer = reduced
                .target_problem_any()
                .downcast_ref::<ILP<i64>>()
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
fn every_parameter_formula_matches_a_constructed_target() {
    let sources = canonical_sources();
    let mut checked = 0;
    let mut expected = 0;
    let mut failures = Vec::new();
    for entry in crate::rules::registry::reduction_entries() {
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
            let source = source_for(entry, &sources)?;
            let actual = target_parameters(entry, source.as_ref())?;
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
