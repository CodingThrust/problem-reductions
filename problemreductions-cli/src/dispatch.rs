use anyhow::{Context, Result};
use problemreductions::models::algebraic::ILP;
use problemreductions::registry::{DynProblem, LoadedDynProblem};
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::solvers::ILPSolver;
use problemreductions::traits::Problem;
use problemreductions::types::ProblemSize;
use serde_json::Value;
use std::any::Any;
use std::collections::BTreeMap;
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
        std::fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))
    }
}

/// Loaded problem with type-erased solve capability.
pub struct LoadedProblem {
    inner: LoadedDynProblem,
}

impl std::ops::Deref for LoadedProblem {
    type Target = dyn DynProblem;
    fn deref(&self) -> &(dyn DynProblem + 'static) {
        &*self.inner
    }
}

impl LoadedProblem {
    pub fn solve_brute_force(&self) -> Result<SolveResult> {
        let (config, evaluation) = self
            .inner
            .solve_brute_force()
            .ok_or_else(|| anyhow::anyhow!("No solution found"))?;
        Ok(SolveResult { config, evaluation })
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

        let reduction_path = best_path.ok_or_else(|| {
            anyhow::anyhow!(
                "No reduction path from {} to ILP. Try `--solver brute-force`, or reduce to a problem that supports ILP.",
                name
            )
        })?;

        let chain = graph
            .reduce_along_path(&reduction_path, self.as_any())
            .ok_or_else(|| anyhow::anyhow!("Failed to execute reduction chain to ILP"))?;

        let ilp_result = solve_ilp(chain.target_problem_any())?;
        let config = chain.extract_solution(&ilp_result.config);
        let evaluation = self.evaluate_dyn(&config);
        Ok(SolveResult { config, evaluation })
    }
}

/// Load a problem from JSON type/variant/data.
pub fn load_problem(
    name: &str,
    variant: &BTreeMap<String, String>,
    data: Value,
) -> Result<LoadedProblem> {
    let canonical = resolve_alias(name);
    let inner = problemreductions::registry::load_dyn(&canonical, variant, data)
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(LoadedProblem { inner })
}

/// Serialize a `&dyn Any` target problem given its name and variant.
pub fn serialize_any_problem(
    name: &str,
    variant: &BTreeMap<String, String>,
    any: &dyn Any,
) -> Result<Value> {
    let canonical = resolve_alias(name);
    problemreductions::registry::serialize_any(&canonical, variant, any).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to serialize {} with variant {:?}",
            canonical,
            variant
        )
    })
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

/// Solve an ILP problem directly. The input must be an `ILP<bool>` or `ILP<i32>` instance.
fn solve_ilp(any: &dyn Any) -> Result<SolveResult> {
    if let Some(problem) = any.downcast_ref::<ILP<bool>>() {
        let solver = ILPSolver::new();
        let config = solver
            .solve(problem)
            .ok_or_else(|| anyhow::anyhow!("ILP solver found no feasible solution"))?;
        let evaluation = format!("{:?}", problem.evaluate(&config));
        return Ok(SolveResult { config, evaluation });
    }
    if let Some(problem) = any.downcast_ref::<ILP<i32>>() {
        let solver = ILPSolver::new();
        let config = solver
            .solve(problem)
            .ok_or_else(|| anyhow::anyhow!("ILP solver found no feasible solution"))?;
        let evaluation = format!("{:?}", problem.evaluate(&config));
        return Ok(SolveResult { config, evaluation });
    }
    Err(anyhow::anyhow!(
        "Internal error: expected ILP<bool> or ILP<i32> problem instance"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use problemreductions::models::graph::MaximumIndependentSet;
    use problemreductions::models::misc::BinPacking;
    use problemreductions::topology::SimpleGraph;
    use serde_json::json;

    #[test]
    fn test_load_problem_alias_uses_registry_dispatch() {
        let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
        let variant = BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]);
        let loaded =
            load_problem("MIS", &variant, serde_json::to_value(&problem).unwrap()).unwrap();
        assert_eq!(loaded.problem_name(), "MaximumIndependentSet");
    }

    #[test]
    fn test_load_problem_rejects_unresolved_weight_variant() {
        let problem = BinPacking::new(vec![3i32, 3, 2, 2], 5i32);
        let loaded = load_problem(
            "BinPacking",
            &BTreeMap::new(),
            serde_json::to_value(&problem).unwrap(),
        );
        assert!(loaded.is_err());
    }

    #[test]
    fn test_load_problem_rejects_invalid_strong_connectivity_augmentation_instance() {
        let variant = BTreeMap::from([("weight".to_string(), "i32".to_string())]);
        let data = json!({
            "graph": {
                "inner": {
                    "edge_property": "directed",
                    "nodes": [null, null, null],
                    "node_holes": [],
                    "edges": [[0, 1, null], [1, 2, null]]
                }
            },
            "candidate_arcs": [[0, 3, 1]],
            "bound": 1
        });

        let loaded = load_problem("StrongConnectivityAugmentation", &variant, data);
        assert!(loaded.is_err());
        let err = loaded.err().unwrap().to_string();
        assert!(err.contains("candidate arc"), "err: {err}");
        assert!(err.contains("num_vertices"), "err: {err}");
    }

    #[test]
    fn test_serialize_any_problem_round_trips_bin_packing() {
        let problem = BinPacking::new(vec![3i32, 3, 2, 2], 5i32);
        let variant = BTreeMap::from([("weight".to_string(), "i32".to_string())]);
        let json = serialize_any_problem("BinPacking", &variant, &problem as &dyn Any).unwrap();
        assert_eq!(json, serde_json::to_value(&problem).unwrap());
    }
}
