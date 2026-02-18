use anyhow::{bail, Result};
use problemreductions::prelude::*;
use problemreductions::models::optimization::ILP;
use problemreductions::topology::{
    KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph,
};
use problemreductions::variant::{K2, K3, KN};
use serde::Serialize;
use serde_json::Value;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;

use crate::problem_name::resolve_alias;

/// Type-erased problem for CLI dispatch.
pub trait DynProblem: Any {
    fn evaluate_dyn(&self, config: &[usize]) -> String;
    fn serialize_json(&self) -> Value;
    fn as_any(&self) -> &dyn Any;
    fn dims_dyn(&self) -> Vec<usize>;
    fn problem_name(&self) -> &'static str;
    fn variant_map(&self) -> BTreeMap<String, String>;
}

impl<T> DynProblem for T
where
    T: Problem + Serialize + 'static,
    T::Metric: fmt::Debug,
{
    fn evaluate_dyn(&self, config: &[usize]) -> String {
        format!("{:?}", self.evaluate(config))
    }
    fn serialize_json(&self) -> Value {
        serde_json::to_value(self).expect("serialize failed")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn dims_dyn(&self) -> Vec<usize> {
        self.dims()
    }
    fn problem_name(&self) -> &'static str {
        T::NAME
    }
    fn variant_map(&self) -> BTreeMap<String, String> {
        T::variant()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
}

fn deser<T: Problem + Serialize + serde::de::DeserializeOwned + 'static>(
    data: Value,
) -> Result<Box<dyn DynProblem>>
where
    T::Metric: fmt::Debug,
{
    let problem: T = serde_json::from_value(data)?;
    Ok(Box::new(problem))
}

fn graph_variant<'a>(variant: &'a BTreeMap<String, String>) -> &'a str {
    variant
        .get("graph")
        .map(|s| s.as_str())
        .unwrap_or("SimpleGraph")
}

/// Load a problem from JSON type/variant/data.
pub fn load_problem(
    name: &str,
    variant: &BTreeMap<String, String>,
    data: Value,
) -> Result<Box<dyn DynProblem>> {
    let canonical = resolve_alias(name);
    match canonical.as_str() {
        "MaximumIndependentSet" => match graph_variant(variant) {
            "KingsSubgraph" => deser::<MaximumIndependentSet<KingsSubgraph, i32>>(data),
            "TriangularSubgraph" => deser::<MaximumIndependentSet<TriangularSubgraph, i32>>(data),
            "UnitDiskGraph" => deser::<MaximumIndependentSet<UnitDiskGraph, i32>>(data),
            _ => deser::<MaximumIndependentSet<SimpleGraph, i32>>(data),
        },
        "MinimumVertexCover" => deser::<MinimumVertexCover<SimpleGraph, i32>>(data),
        "MaximumClique" => deser::<MaximumClique<SimpleGraph, i32>>(data),
        "MaximumMatching" => deser::<MaximumMatching<SimpleGraph, i32>>(data),
        "MinimumDominatingSet" => deser::<MinimumDominatingSet<SimpleGraph, i32>>(data),
        "MaxCut" => deser::<MaxCut<SimpleGraph, i32>>(data),
        "TravelingSalesman" => deser::<TravelingSalesman<SimpleGraph, i32>>(data),
        "KColoring" => match variant.get("k").map(|s| s.as_str()) {
            Some("K3") => deser::<KColoring<K3, SimpleGraph>>(data),
            _ => deser::<KColoring<KN, SimpleGraph>>(data),
        },
        "MaximumSetPacking" => deser::<MaximumSetPacking<i32>>(data),
        "MinimumSetCovering" => deser::<MinimumSetCovering<i32>>(data),
        "QUBO" => deser::<QUBO<f64>>(data),
        "SpinGlass" => match variant.get("weight").map(|s| s.as_str()) {
            Some("f64") => deser::<SpinGlass<SimpleGraph, f64>>(data),
            _ => deser::<SpinGlass<SimpleGraph, i32>>(data),
        },
        "Satisfiability" => deser::<Satisfiability>(data),
        "KSatisfiability" => match variant.get("k").map(|s| s.as_str()) {
            Some("K2") => deser::<KSatisfiability<K2>>(data),
            Some("K3") => deser::<KSatisfiability<K3>>(data),
            _ => deser::<KSatisfiability<KN>>(data),
        },
        "CircuitSAT" => deser::<CircuitSAT>(data),
        "Factoring" => deser::<Factoring>(data),
        "ILP" => deser::<ILP>(data),
        "BicliqueCover" => deser::<BicliqueCover>(data),
        "BMF" => deser::<BMF>(data),
        "PaintShop" => deser::<PaintShop>(data),
        _ => bail!("Unknown problem type: {canonical}"),
    }
}

/// Serialize a `&dyn Any` target problem given its name and variant.
pub fn serialize_any_problem(
    name: &str,
    variant: &BTreeMap<String, String>,
    any: &dyn Any,
) -> Result<Value> {
    let canonical = resolve_alias(name);
    match canonical.as_str() {
        "MaximumIndependentSet" => match graph_variant(variant) {
            "KingsSubgraph" => try_ser::<MaximumIndependentSet<KingsSubgraph, i32>>(any),
            "TriangularSubgraph" => try_ser::<MaximumIndependentSet<TriangularSubgraph, i32>>(any),
            "UnitDiskGraph" => try_ser::<MaximumIndependentSet<UnitDiskGraph, i32>>(any),
            _ => try_ser::<MaximumIndependentSet<SimpleGraph, i32>>(any),
        },
        "MinimumVertexCover" => try_ser::<MinimumVertexCover<SimpleGraph, i32>>(any),
        "MaximumClique" => try_ser::<MaximumClique<SimpleGraph, i32>>(any),
        "MaximumMatching" => try_ser::<MaximumMatching<SimpleGraph, i32>>(any),
        "MinimumDominatingSet" => try_ser::<MinimumDominatingSet<SimpleGraph, i32>>(any),
        "MaxCut" => try_ser::<MaxCut<SimpleGraph, i32>>(any),
        "TravelingSalesman" => try_ser::<TravelingSalesman<SimpleGraph, i32>>(any),
        "KColoring" => match variant.get("k").map(|s| s.as_str()) {
            Some("K3") => try_ser::<KColoring<K3, SimpleGraph>>(any),
            _ => try_ser::<KColoring<KN, SimpleGraph>>(any),
        },
        "MaximumSetPacking" => match variant.get("weight").map(|s| s.as_str()) {
            Some("f64") => try_ser::<MaximumSetPacking<f64>>(any),
            _ => try_ser::<MaximumSetPacking<i32>>(any),
        },
        "MinimumSetCovering" => try_ser::<MinimumSetCovering<i32>>(any),
        "QUBO" => try_ser::<QUBO<f64>>(any),
        "SpinGlass" => match variant.get("weight").map(|s| s.as_str()) {
            Some("f64") => try_ser::<SpinGlass<SimpleGraph, f64>>(any),
            _ => try_ser::<SpinGlass<SimpleGraph, i32>>(any),
        },
        "Satisfiability" => try_ser::<Satisfiability>(any),
        "KSatisfiability" => match variant.get("k").map(|s| s.as_str()) {
            Some("K2") => try_ser::<KSatisfiability<K2>>(any),
            Some("K3") => try_ser::<KSatisfiability<K3>>(any),
            _ => try_ser::<KSatisfiability<KN>>(any),
        },
        "CircuitSAT" => try_ser::<CircuitSAT>(any),
        "Factoring" => try_ser::<Factoring>(any),
        "ILP" => try_ser::<ILP>(any),
        "BicliqueCover" => try_ser::<BicliqueCover>(any),
        "BMF" => try_ser::<BMF>(any),
        "PaintShop" => try_ser::<PaintShop>(any),
        _ => bail!("Unknown problem type: {canonical}"),
    }
}

fn try_ser<T: Serialize + 'static>(any: &dyn Any) -> Result<Value> {
    let problem = any
        .downcast_ref::<T>()
        .ok_or_else(|| anyhow::anyhow!("Type mismatch during serialization"))?;
    Ok(serde_json::to_value(problem)?)
}

/// JSON wrapper format for problem files.
#[derive(serde::Deserialize)]
pub struct ProblemJson {
    #[serde(rename = "type")]
    pub problem_type: String,
    #[serde(default)]
    pub variant: BTreeMap<String, String>,
    pub data: Value,
}

/// JSON wrapper format for reduction bundles.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReductionBundle {
    pub source: ProblemJsonOutput,
    pub target: ProblemJsonOutput,
    pub path: Vec<PathStep>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProblemJsonOutput {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub variant: BTreeMap<String, String>,
    pub data: Value,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PathStep {
    pub name: String,
    pub variant: BTreeMap<String, String>,
}
