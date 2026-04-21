use anyhow::{Context, Result};
use problemreductions::registry::{DynProblem, LoadedDynProblem};
use problemreductions::rules::{MinimizeSteps, ReductionGraph, ReductionMode};
use problemreductions::solvers::{CustomizedSolver, ILPSolver};
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
    pub fn solve_brute_force_value(&self) -> String {
        self.inner.solve_brute_force_value()
    }

    pub fn solve_brute_force_witness(&self) -> Option<WitnessSolveResult> {
        let (config, evaluation) = self.inner.solve_brute_force_witness()?;
        Some(WitnessSolveResult { config, evaluation })
    }

    pub fn solve_brute_force(&self) -> SolveResult {
        let evaluation = self.solve_brute_force_value();
        let config = self.solve_brute_force_witness().map(|result| result.config);
        SolveResult { config, evaluation }
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
                    .find_cheapest_path_mode(
                        name,
                        &variant,
                        "ILP",
                        dv,
                        ReductionMode::Witness,
                        &input_size,
                        &MinimizeSteps,
                    )
                    .is_some()
            })
        }
    }

    pub fn supports_customized_solver(&self) -> bool {
        CustomizedSolver::supports_problem(self.as_any())
    }

    pub fn solve_with_customized(&self) -> Result<WitnessSolveResult> {
        let solver = CustomizedSolver::new();
        let config = solver
            .solve_dyn(self.as_any())
            .ok_or_else(|| anyhow::anyhow!("Problem unsupported by customized solver"))?;
        let evaluation = self.evaluate_dyn(&config);
        Ok(WitnessSolveResult { config, evaluation })
    }

    #[cfg_attr(not(feature = "mcp"), allow(dead_code))]
    pub fn available_solvers(&self) -> Vec<&'static str> {
        let mut solvers = Vec::new();
        if self.supports_ilp_solver() {
            solvers.push("ilp");
        }
        solvers.push("brute-force");
        if self.supports_customized_solver() {
            solvers.push("customized");
        }
        solvers
    }

    /// Solve using the ILP solver. If the problem is not ILP, auto-reduce to ILP first.
    pub fn solve_with_ilp(&self) -> Result<WitnessSolveResult> {
        let name = self.problem_name();
        let variant = self.variant_map();
        let solver = ILPSolver::new();
        let config = solver
            .try_solve_via_reduction(name, &variant, self.as_any())
            .map_err(|err| anyhow::anyhow!(err))?;
        let evaluation = self.evaluate_dyn(&config);
        Ok(WitnessSolveResult { config, evaluation })
    }
}

/// A validated reduction bundle ready to replay:
/// source, target, and the reconstructed reduction chain. Construct via
/// [`BundleReplay::prepare`]. All three CLI/MCP bundle workflows
/// (`pred solve <bundle>`, `pred extract <bundle>`, MCP `solve_problem`)
/// share this setup so validation and error text stay in sync.
pub struct BundleReplay {
    pub(crate) source: LoadedProblem,
    pub(crate) source_name: String,
    pub(crate) target: LoadedProblem,
    pub(crate) target_name: String,
    pub(crate) chain: problemreductions::rules::ReductionChain,
}

impl BundleReplay {
    /// Validate the bundle and replay the reduction chain.
    ///
    /// Checks:
    /// - `path` has at least two steps
    /// - `path[0]` matches `source` (name + variant)
    /// - `path[-1]` matches `target` (name + variant)
    /// - serializing the chain's replayed target equals `bundle.target.data`
    ///   (tampered/stale bundles where `target.data` disagrees with what
    ///   `reduce_along_path` actually produced are rejected)
    ///
    /// Returns an error (not a panic) for malformed bundles or aggregate-only paths.
    pub fn prepare(bundle: &ReductionBundle) -> Result<Self> {
        if bundle.path.len() < 2 {
            anyhow::bail!(
                "Malformed bundle: `path` must contain at least two steps (source and target), got {}",
                bundle.path.len()
            );
        }
        let first = bundle.path.first().unwrap();
        let last = bundle.path.last().unwrap();
        if first.name != bundle.source.problem_type || first.variant != bundle.source.variant {
            anyhow::bail!(
                "Malformed bundle: path starts with {} but source is {}",
                format_step(&first.name, &first.variant),
                format_step(&bundle.source.problem_type, &bundle.source.variant),
            );
        }
        if last.name != bundle.target.problem_type || last.variant != bundle.target.variant {
            anyhow::bail!(
                "Malformed bundle: path ends with {} but target is {}",
                format_step(&last.name, &last.variant),
                format_step(&bundle.target.problem_type, &bundle.target.variant),
            );
        }

        let source = load_problem(
            &bundle.source.problem_type,
            &bundle.source.variant,
            bundle.source.data.clone(),
        )?;
        let source_name = source.problem_name().to_string();

        let target = load_problem(
            &bundle.target.problem_type,
            &bundle.target.variant,
            bundle.target.data.clone(),
        )?;
        let target_name = target.problem_name().to_string();

        let reduction_path = problemreductions::rules::ReductionPath {
            steps: bundle
                .path
                .iter()
                .map(|s| problemreductions::rules::ReductionStep {
                    name: s.name.clone(),
                    variant: s.variant.clone(),
                })
                .collect(),
        };

        let graph = ReductionGraph::new();
        let chain = graph
            .reduce_along_path(&reduction_path, source.as_any())
            .ok_or_else(|| anyhow::anyhow!(
                "Bundle requires a witness-capable reduction path; this bundle cannot map a target solution back to the source."
            ))?;

        // Coherence check: `bundle.target.data` must equal what replaying
        // `source` along `path` actually produces. Without this, a caller
        // could solve/validate against the bundle's stated target but then
        // extract through a completely different chain target.
        let replayed_target_data =
            serialize_any_problem(&last.name, &last.variant, chain.target_problem_any())?;
        if replayed_target_data != bundle.target.data {
            anyhow::bail!(
                "Malformed bundle: `target.data` does not match the result of replaying \
                 `source` along `path`. The bundle is tampered or was produced by \
                 incompatible code."
            );
        }

        Ok(Self {
            source,
            source_name,
            target,
            target_name,
            chain,
        })
    }

    /// Map a target-space configuration back to the source space and evaluate it.
    pub fn extract(&self, target_config: &[usize]) -> (Vec<usize>, String) {
        let source_config = self.chain.extract_solution(target_config);
        let source_eval = self.source.evaluate_dyn(&source_config);
        (source_config, source_eval)
    }
}

fn format_step(name: &str, variant: &BTreeMap<String, String>) -> String {
    if variant.is_empty() {
        name.to_string()
    } else {
        let parts: Vec<String> = variant
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!("{}{{{}}}", name, parts.join(", "))
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolveResult {
    /// The solution configuration when the problem supports witness extraction.
    pub config: Option<Vec<usize>>,
    /// Evaluation of the solution.
    pub evaluation: String,
}

/// Result of solving a witness-capable problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessSolveResult {
    /// The solution configuration.
    pub config: Vec<usize>,
    /// Evaluation of the solution.
    pub evaluation: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{AggregateValueSource, AGGREGATE_SOURCE_NAME};
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

    #[test]
    fn test_solve_brute_force_value_only_problem_has_no_witness() {
        let loaded = load_problem(
            AGGREGATE_SOURCE_NAME,
            &BTreeMap::new(),
            serde_json::to_value(AggregateValueSource::sample()).unwrap(),
        )
        .unwrap();

        let result = loaded.solve_brute_force();
        assert_eq!(result.config, None);
        assert_eq!(result.evaluation, "Sum(56)");
    }

    #[test]
    fn test_available_solvers_excludes_customized_for_unsupported_problem() {
        let loaded = load_problem(
            AGGREGATE_SOURCE_NAME,
            &BTreeMap::new(),
            serde_json::to_value(AggregateValueSource::sample()).unwrap(),
        )
        .unwrap();

        assert!(!loaded.available_solvers().contains(&"customized"));
    }

    #[test]
    fn test_solve_with_customized_rejects_unsupported_problem() {
        let loaded = load_problem(
            AGGREGATE_SOURCE_NAME,
            &BTreeMap::new(),
            serde_json::to_value(AggregateValueSource::sample()).unwrap(),
        )
        .unwrap();

        let err = loaded.solve_with_customized().unwrap_err();
        assert!(
            err.to_string().contains("unsupported by customized solver"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_solve_with_ilp_rejects_aggregate_only_problem() {
        let loaded = load_problem(
            AGGREGATE_SOURCE_NAME,
            &BTreeMap::new(),
            serde_json::to_value(AggregateValueSource::sample()).unwrap(),
        )
        .unwrap();

        let err = loaded.solve_with_ilp().unwrap_err();
        assert!(
            err.to_string().contains("witness-capable"),
            "unexpected error: {err}"
        );
    }
}
