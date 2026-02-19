use anyhow::{bail, Context, Result};
use problemreductions::models::optimization::ILP;
use problemreductions::prelude::*;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::solvers::{BruteForce, ILPSolver, Solver};
use problemreductions::topology::{KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph};
use problemreductions::types::ProblemSize;
use problemreductions::variant::{K2, K3, KN};
use serde::Serialize;
use serde_json::Value;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Deref;
use std::path::Path;

use crate::problem_name::resolve_alias;

/// Read input from a file, or from stdin if the path is "-".
pub fn read_input(path: &Path) -> Result<String> {
    if path.as_os_str() == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("Failed to read from stdin")?;
        Ok(buf)
    } else {
        std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))
    }
}

/// Type-erased problem for CLI dispatch.
#[allow(dead_code)]
pub trait DynProblem: Any {
    fn evaluate_dyn(&self, config: &[usize]) -> String;
    fn serialize_json(&self) -> Value;
    fn as_any(&self) -> &dyn Any;
    fn dims_dyn(&self) -> Vec<usize>;
    fn problem_name(&self) -> &'static str;
    fn variant_map(&self) -> BTreeMap<String, String>;
    fn problem_size_names_dyn(&self) -> &'static [&'static str];
    fn problem_size_values_dyn(&self) -> Vec<usize>;
    fn num_variables_dyn(&self) -> usize;
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
    fn problem_size_names_dyn(&self) -> &'static [&'static str] {
        T::problem_size_names()
    }
    fn problem_size_values_dyn(&self) -> Vec<usize> {
        self.problem_size_values()
    }
    fn num_variables_dyn(&self) -> usize {
        self.num_variables()
    }
}

fn deser_opt<T>(data: Value) -> Result<LoadedProblem>
where
    T: OptimizationProblem + Serialize + serde::de::DeserializeOwned + 'static,
    T::Metric: fmt::Debug,
{
    let problem: T = serde_json::from_value(data)?;
    Ok(LoadedProblem {
        inner: Box::new(problem),
        brute_force_fn: bf_opt::<T>,
    })
}

fn deser_sat<T>(data: Value) -> Result<LoadedProblem>
where
    T: Problem<Metric = bool> + Serialize + serde::de::DeserializeOwned + 'static,
{
    let problem: T = serde_json::from_value(data)?;
    Ok(LoadedProblem {
        inner: Box::new(problem),
        brute_force_fn: bf_sat::<T>,
    })
}

fn bf_opt<T>(any: &dyn Any) -> Option<SolveResult>
where
    T: OptimizationProblem + 'static,
    T::Metric: fmt::Debug,
{
    let p = any.downcast_ref::<T>()?;
    let config = BruteForce::new().find_best(p)?;
    let evaluation = format!("{:?}", p.evaluate(&config));
    Some(SolveResult { config, evaluation })
}

fn bf_sat<T>(any: &dyn Any) -> Option<SolveResult>
where
    T: Problem<Metric = bool> + 'static,
{
    let p = any.downcast_ref::<T>()?;
    let config = BruteForce::new().find_satisfying(p)?;
    let evaluation = format!("{:?}", p.evaluate(&config));
    Some(SolveResult { config, evaluation })
}

/// Loaded problem with type-erased solve capability.
pub struct LoadedProblem {
    inner: Box<dyn DynProblem>,
    brute_force_fn: fn(&dyn Any) -> Option<SolveResult>,
}

impl Deref for LoadedProblem {
    type Target = dyn DynProblem;
    fn deref(&self) -> &(dyn DynProblem + 'static) {
        &*self.inner
    }
}

impl LoadedProblem {
    pub fn solve_brute_force(&self) -> Result<SolveResult> {
        (self.brute_force_fn)(self.inner.as_any())
            .ok_or_else(|| anyhow::anyhow!("No solution found"))
    }

    /// Solve using the ILP solver. If the problem is not ILP, auto-reduce to ILP first.
    pub fn solve_with_ilp(&self) -> Result<SolveResult> {
        let name = self.problem_name();
        if name == "ILP" {
            return solve_ilp(self.as_any());
        }

        // Auto-reduce to ILP, solve, and map solution back
        let source_variant = self.variant_map();
        let graph = ReductionGraph::new();
        let ilp_variants = graph.variants_for("ILP");
        let input_size = ProblemSize::new(vec![]);

        let mut best_path = None;
        for dv in &ilp_variants {
            if let Some(p) = graph.find_cheapest_path(
                name,
                &source_variant,
                "ILP",
                dv,
                &input_size,
                &MinimizeSteps,
            ) {
                let is_better = best_path
                    .as_ref()
                    .is_none_or(|bp: &problemreductions::rules::ReductionPath| p.len() < bp.len());
                if is_better {
                    best_path = Some(p);
                }
            }
        }

        let reduction_path =
            best_path.ok_or_else(|| anyhow::anyhow!("No reduction path from {} to ILP", name))?;

        let chain = graph
            .reduce_along_path(&reduction_path, self.as_any())
            .ok_or_else(|| anyhow::anyhow!("Failed to execute reduction chain to ILP"))?;

        let ilp_result = solve_ilp(chain.target_problem_any())?;
        let config = chain.extract_solution(&ilp_result.config);
        let evaluation = self.evaluate_dyn(&config);
        Ok(SolveResult { config, evaluation })
    }
}

fn graph_variant(variant: &BTreeMap<String, String>) -> &str {
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
) -> Result<LoadedProblem> {
    let canonical = resolve_alias(name);
    match canonical.as_str() {
        "MaximumIndependentSet" => match graph_variant(variant) {
            "KingsSubgraph" => deser_opt::<MaximumIndependentSet<KingsSubgraph, i32>>(data),
            "TriangularSubgraph" => {
                deser_opt::<MaximumIndependentSet<TriangularSubgraph, i32>>(data)
            }
            "UnitDiskGraph" => deser_opt::<MaximumIndependentSet<UnitDiskGraph, i32>>(data),
            _ => deser_opt::<MaximumIndependentSet<SimpleGraph, i32>>(data),
        },
        "MinimumVertexCover" => deser_opt::<MinimumVertexCover<SimpleGraph, i32>>(data),
        "MaximumClique" => deser_opt::<MaximumClique<SimpleGraph, i32>>(data),
        "MaximumMatching" => deser_opt::<MaximumMatching<SimpleGraph, i32>>(data),
        "MinimumDominatingSet" => deser_opt::<MinimumDominatingSet<SimpleGraph, i32>>(data),
        "MaxCut" => deser_opt::<MaxCut<SimpleGraph, i32>>(data),
        "TravelingSalesman" => deser_opt::<TravelingSalesman<SimpleGraph, i32>>(data),
        "KColoring" => match variant.get("k").map(|s| s.as_str()) {
            Some("K3") => deser_sat::<KColoring<K3, SimpleGraph>>(data),
            _ => deser_sat::<KColoring<KN, SimpleGraph>>(data),
        },
        "MaximumSetPacking" => deser_opt::<MaximumSetPacking<i32>>(data),
        "MinimumSetCovering" => deser_opt::<MinimumSetCovering<i32>>(data),
        "QUBO" => deser_opt::<QUBO<f64>>(data),
        "SpinGlass" => match variant.get("weight").map(|s| s.as_str()) {
            Some("f64") => deser_opt::<SpinGlass<SimpleGraph, f64>>(data),
            _ => deser_opt::<SpinGlass<SimpleGraph, i32>>(data),
        },
        "Satisfiability" => deser_sat::<Satisfiability>(data),
        "KSatisfiability" => match variant.get("k").map(|s| s.as_str()) {
            Some("K2") => deser_sat::<KSatisfiability<K2>>(data),
            Some("K3") => deser_sat::<KSatisfiability<K3>>(data),
            _ => deser_sat::<KSatisfiability<KN>>(data),
        },
        "CircuitSAT" => deser_sat::<CircuitSAT>(data),
        "Factoring" => deser_opt::<Factoring>(data),
        "ILP" => deser_opt::<ILP>(data),
        "BicliqueCover" => deser_opt::<BicliqueCover>(data),
        "BMF" => deser_opt::<BMF>(data),
        "PaintShop" => deser_opt::<PaintShop>(data),
        _ => bail!("{}", crate::problem_name::unknown_problem_error(&canonical)),
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
        _ => bail!("{}", crate::problem_name::unknown_problem_error(&canonical)),
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

/// Result of solving a problem.
pub struct SolveResult {
    /// The solution configuration.
    pub config: Vec<usize>,
    /// Evaluation of the solution.
    pub evaluation: String,
}

/// Solve an ILP problem directly. The input must be an `ILP` instance.
fn solve_ilp(any: &dyn Any) -> Result<SolveResult> {
    let problem = any
        .downcast_ref::<ILP>()
        .ok_or_else(|| anyhow::anyhow!("Internal error: expected ILP problem instance"))?;
    let solver = ILPSolver::new();
    let config = solver
        .solve(problem)
        .ok_or_else(|| anyhow::anyhow!("ILP solver found no feasible solution"))?;
    let evaluation = format!("{:?}", problem.evaluate(&config));
    Ok(SolveResult { config, evaluation })
}
