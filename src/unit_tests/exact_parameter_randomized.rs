use crate::parameters::ParameterRelation;
use crate::registry::{load_dyn, DynProblem};
use crate::rules::{registry::ReductionEntry, ReductionGraph};
use serde_json::{json, Value};
use std::collections::BTreeMap;

type SourceKey = (String, BTreeMap<String, String>);

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

fn seed_for(entry: &ReductionEntry) -> u64 {
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for byte in format!(
        "{}{:?}{}{:?}",
        entry.source_name,
        entry.source_variant(),
        entry.target_name,
        entry.target_variant()
    )
    .bytes()
    {
        seed = seed.wrapping_mul(0x100_0000_01b3) ^ u64::from(byte);
    }
    seed
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
        if let Some(inner) = source.instance.get("inner") {
            if let Some(name) = source.problem.strip_prefix("Decision") {
                sources
                    .entry((name.to_string(), source.variant.clone()))
                    .or_default()
                    .push(inner.clone());
            }
        }
        sources
            .entry((source.problem, source.variant))
            .or_default()
            .push(source.instance);
    }
    sources
}

// These changes preserve the element type and start from an independently
// constructed canonical instance. Deserialization and reduction must both accept
// a candidate before it is used as a test instance.
fn variations(value: &Value) -> Vec<Value> {
    match value {
        Value::Array(items) => {
            let mut result = Vec::new();
            if items.len() > 1 {
                let mut shorter = items.clone();
                shorter.pop();
                result.push(Value::Array(shorter));
                let mut reordered = items.clone();
                reordered.rotate_left(1);
                result.push(Value::Array(reordered));
            }
            for (index, item) in items.iter().enumerate() {
                for changed in variations(item) {
                    let mut copy = items.clone();
                    copy[index] = changed;
                    result.push(Value::Array(copy));
                }
            }
            result
        }
        Value::Object(fields) => fields
            .iter()
            .flat_map(|(key, child)| {
                variations(child).into_iter().map(move |changed| {
                    let mut copy = fields.clone();
                    copy.insert(key.clone(), changed);
                    Value::Object(copy)
                })
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn random_source(
    entry: &ReductionEntry,
    sources: &BTreeMap<SourceKey, Vec<Value>>,
) -> Result<Box<dyn DynProblem>, String> {
    let variant = ReductionGraph::variant_to_map(&entry.source_variant());
    let registered = crate::registry::find_variant_entry(entry.source_name, &variant).unwrap();
    let mut seed = seed_for(entry);
    if let Some(random) = registered.random {
        let mut args = serde_json::Map::new();
        for input in (random.inputs)() {
            let value = match input.name {
                "num_vertices" => json!(4 + next(&mut seed) % 3),
                "seed" => json!((next(&mut seed) >> 1) as i64),
                "k" => json!(if variant.get("k").is_some_and(|value| value == "K3") {
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
        return (random.generate)(Value::Object(args)).map_err(|error| error.to_string());
    }
    if entry.source_name == "MaximumLikelihoodRanking" {
        let n = 3 + next(&mut seed) as usize % 3;
        let mut matrix = vec![vec![0; n]; n];
        for i in 0..n {
            let (earlier, later) = matrix.split_at_mut(i + 1);
            for (offset, row) in later.iter_mut().enumerate() {
                let j = i + offset + 1;
                earlier[i][j] = (next(&mut seed) % 6) as i64;
                row[i] = 5 - earlier[i][j];
            }
        }
        return Ok(Box::new(
            crate::models::misc::MaximumLikelihoodRanking::new(matrix),
        ));
    }
    if entry.source_name == "OptimumCommunicationSpanningTree" {
        let n = 3 + next(&mut seed) as usize % 2;
        let mut weights = vec![vec![0; n]; n];
        let mut requirements = vec![vec![0; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                weights[i][j] = 1 + (next(&mut seed) % 4) as i64;
                weights[j][i] = weights[i][j];
                requirements[i][j] = 1 + (next(&mut seed) % 3) as i64;
                requirements[j][i] = requirements[i][j];
            }
        }
        return Ok(Box::new(
            crate::models::misc::OptimumCommunicationSpanningTree::new(weights, requirements),
        ));
    }
    if entry.source_name == "DecisionOpenShopScheduling" {
        let time = 1 + (next(&mut seed) % 3) as i64;
        return Ok(Box::new(crate::models::decision::Decision::new(
            crate::models::misc::OpenShopScheduling::new(2, vec![vec![time, 1], vec![2, time]]),
            4,
        )));
    }
    if entry.source_name == "ExpectedRetrievalCost" {
        let records = 2 + next(&mut seed) as usize % 3;
        return Ok(Box::new(
            crate::models::misc::ExpectedRetrievalCost::new(vec![1.0 / records as f64; records], 2)
                .unwrap(),
        ));
    }
    if entry.source_name == "ILP" && variant.get("variable").is_some_and(|v| v == "i64") {
        use crate::models::algebraic::{IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
        let upper = 2 + (next(&mut seed) % 4) as i64;
        return Ok(Box::new(
            ILP::<i64>::with_variables(
                vec![IntegerVariable::new(Some(0), Some(upper)).unwrap()],
                vec![LinearConstraint::le(vec![(0, 1)], upper)],
                vec![(0, 1)],
                ObjectiveSense::Minimize,
            )
            .unwrap(),
        ));
    }

    let key = (entry.source_name.to_string(), variant.clone());
    let examples = sources
        .get(&key)
        .ok_or_else(|| format!("no canonical source for {key:?}"))?;
    let base = &examples[next(&mut seed) as usize % examples.len()];
    let base_params = load_dyn(entry.source_name, &variant, base.clone())
        .map_err(|error| error.to_string())?
        .parameters_dyn();
    let use_constructor = (registered.construct_fn)(base.clone()).is_ok();
    let mut candidates = variations(base);
    // Check a seeded permutation of the candidates, preferring an instance
    // with different measured source parameters.
    let mut same_size = None;
    while !candidates.is_empty() {
        let index = next(&mut seed) as usize % candidates.len();
        let candidate = candidates.swap_remove(index);
        if candidate == *base {
            continue;
        }
        let problem = if use_constructor {
            (registered.construct_fn)(candidate).ok()
        } else {
            (registered.factory)(candidate).ok()
        };
        let Some(problem) = problem else {
            continue;
        };
        if target_parameters(entry, problem.as_ref()).is_err() {
            continue;
        }
        if problem.parameters_dyn() != base_params {
            return Ok(problem);
        }
        same_size.get_or_insert(problem);
    }
    same_size.ok_or_else(|| format!("no valid variation for {key:?}: {base}"))
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
    let target =
        load_dyn(entry.target_name, &variant, target_json).map_err(|error| error.to_string())?;
    Ok(target.parameters_dyn())
}

#[test]
fn every_exact_field_matches_a_random_reduced_instance() {
    let sources = canonical_sources();
    let mut checked = 0;
    let mut expected = 0;
    let mut failures = Vec::new();
    for entry in crate::rules::registry::reduction_entries() {
        let contract = entry.parameter_contract().unwrap();
        let Some(transform) = contract.transform() else {
            continue;
        };
        if transform.relation() != ParameterRelation::Exact {
            continue;
        }
        expected += transform.expressions().count();
        let label = format!(
            "{} {:?} -> {} {:?}",
            entry.source_name,
            entry.source_variant(),
            entry.target_name,
            entry.target_variant()
        );
        let result = (|| {
            let source = random_source(entry, &sources)?;
            let actual = target_parameters(entry, source.as_ref())?;
            let predicted = transform
                .evaluate(&source.parameters_dyn())
                .map_err(|error| error.to_string())?;
            for (field, _) in transform.expressions() {
                if predicted.get(field) != actual.get(field) {
                    return Err(format!(
                        "{field}: predicted {:?}, measured {:?}; source {}",
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
    assert_eq!(checked, expected, "some exact fields were not checked");
}
