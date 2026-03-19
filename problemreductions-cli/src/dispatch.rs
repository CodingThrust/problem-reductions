use anyhow::{Context, Result};
use problemreductions::registry::{DynProblem, LoadedDynProblem};
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::solvers::ILPSolver;
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

    pub fn supports_ilp_solver(&self) -> bool {
        let name = self.problem_name();
        let variant = self.variant_map();
        name == "ILP" || {
            let graph = ReductionGraph::new();
            let ilp_variants = graph.variants_for("ILP");
            let input_size = ProblemSize::new(vec![]);
            ilp_variants.iter().any(|dv| {
                graph
                    .find_cheapest_path(name, &variant, "ILP", dv, &input_size, &MinimizeSteps)
                    .is_some()
            })
        }
    }

    /// Solve using the ILP solver. If the problem is not ILP, auto-reduce to ILP first.
    pub fn solve_with_ilp(&self) -> Result<SolveResult> {
        let name = self.problem_name();
        let variant = self.variant_map();
        let solver = ILPSolver::new();
        let config = solver
            .solve_via_reduction(name, &variant, self.as_any())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No reduction path from {} to ILP or ILP solver found no solution. \
                     Try `--solver brute-force`.",
                    name
                )
            })?;
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
                "num_vertices": 3,
                "arcs": [[0, 1], [1, 2]]
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

    #[test]
    fn test_load_problem_rejects_zero_processor_multiprocessor_scheduling() {
        let loaded = load_problem(
            "MultiprocessorScheduling",
            &BTreeMap::new(),
            serde_json::json!({
                "lengths": [1, 2],
                "num_processors": 0,
                "deadline": 5
            }),
        );
        assert!(
            loaded.is_err(),
            "zero-processor instance should be rejected"
        );
        let err = loaded.err().unwrap();
        assert!(
            err.to_string().contains("expected positive integer, got 0"),
            "unexpected error: {err}"
        );
    }
}
