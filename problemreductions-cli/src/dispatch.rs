use anyhow::{Context, Result};
use problemreductions::registry::{DynProblem, LoadedDynProblem};
use problemreductions::rules::ReductionGraph;
use problemreductions::solvers::{
    solve_deterministically, solver_capabilities, DeterministicSolveResult, ExactProblemKey,
    SolveOutcome, SolverRequest,
};
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
    pub fn solve_deterministically(
        &self,
        request: SolverRequest,
    ) -> Result<DeterministicSolveResult> {
        solve_deterministically(&self.inner, request).map_err(anyhow::Error::from)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct CustomizedSolverCapabilityView {
    pub implementation: &'static str,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct IlpSolverCapabilityView {
    pub reduction_path: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SolverCapabilityDetailsView {
    pub customized: Option<CustomizedSolverCapabilityView>,
    pub ilp: Option<IlpSolverCapabilityView>,
    pub brute_force: bool,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SolverCapabilitiesView {
    pub solvers: Vec<&'static str>,
    pub default_solver: &'static str,
    pub capabilities: SolverCapabilityDetailsView,
}

pub fn solver_capabilities_view(problem: &LoadedProblem) -> Result<SolverCapabilitiesView> {
    let key = ExactProblemKey::new(problem.problem_name(), problem.variant_map());
    let registered = solver_capabilities(&key)
        .map_err(|error| anyhow::anyhow!("solver capability registry is invalid: {error}"))?;
    let customized = registered
        .customized
        .map(|entry| CustomizedSolverCapabilityView {
            implementation: entry.implementation,
        });
    let ilp = registered.ilp.map(|pipeline| IlpSolverCapabilityView {
        reduction_path: pipeline.path_labels(),
    });
    let default_solver = if customized.is_some() {
        "customized"
    } else if ilp.is_some() {
        "ilp"
    } else {
        "brute-force"
    };
    let mut solvers = Vec::with_capacity(3);
    if customized.is_some() {
        solvers.push("customized");
    }
    if ilp.is_some() {
        solvers.push("ilp");
    }
    solvers.push("brute-force");

    Ok(SolverCapabilitiesView {
        solvers,
        default_solver,
        capabilities: SolverCapabilityDetailsView {
            customized,
            ilp,
            brute_force: true,
        },
    })
}

pub fn solver_request(solver_name: Option<&str>) -> Result<SolverRequest> {
    match solver_name {
        None => Ok(SolverRequest::Default),
        Some("customized") => Ok(SolverRequest::Customized),
        Some("ilp") => Ok(SolverRequest::Ilp),
        Some("brute-force") => Ok(SolverRequest::BruteForce),
        Some(other) => {
            anyhow::bail!(
                "Unknown solver: {other}. Available solver overrides: customized, ilp, brute-force"
            )
        }
    }
}

pub fn solve_result_json(problem: &str, result: &DeterministicSolveResult) -> serde_json::Value {
    #[derive(serde::Serialize)]
    struct SolveOutput<'a> {
        problem: &'a str,
        solver: &'a problemreductions::solvers::SolverExecution,
        #[serde(flatten)]
        outcome: &'a SolveOutcome,
    }

    serde_json::to_value(SolveOutput {
        problem,
        solver: &result.solver,
        outcome: &result.outcome,
    })
    .expect("solve output is serializable")
}

pub(crate) struct BundleSolveResult {
    pub(crate) source_name: String,
    pub(crate) target_name: String,
    pub(crate) solver: problemreductions::solvers::SolverExecution,
    pub(crate) source_outcome: SolveOutcome,
    pub(crate) target_outcome: SolveOutcome,
}

impl BundleSolveResult {
    pub(crate) fn to_json(&self) -> serde_json::Value {
        #[derive(serde::Serialize)]
        struct Intermediate<'a> {
            problem: &'a str,
            #[serde(flatten)]
            outcome: &'a SolveOutcome,
        }

        #[derive(serde::Serialize)]
        struct BundleOutput<'a> {
            problem: &'a str,
            solver: &'a problemreductions::solvers::SolverExecution,
            #[serde(flatten)]
            outcome: &'a SolveOutcome,
            intermediate: Intermediate<'a>,
        }

        serde_json::to_value(BundleOutput {
            problem: &self.source_name,
            solver: &self.solver,
            outcome: &self.source_outcome,
            intermediate: Intermediate {
                problem: &self.target_name,
                outcome: &self.target_outcome,
            },
        })
        .expect("bundle solve output is serializable")
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
    pub fn extract(&self, target_config: &[usize]) -> Result<(Vec<usize>, String)> {
        let source_config = self.chain.extract_solution(target_config)?;
        let source_eval = self.source.evaluate_dyn(&source_config);
        Ok((source_config, source_eval))
    }

    /// Solve the target and map the result back to the source problem.
    ///
    pub(crate) fn solve(&self, request: SolverRequest) -> Result<BundleSolveResult> {
        let target_result = self.target.solve_deterministically(request)?;
        let solver = target_result.solver;
        let (source_outcome, target_outcome) = match target_result.outcome {
            SolveOutcome::Optimal {
                config: Some(target_config),
                evaluation: target_evaluation,
            } => {
                let (source_config, source_evaluation) = self.extract(&target_config)?;
                (
                    SolveOutcome::Optimal {
                        config: Some(source_config),
                        evaluation: source_evaluation,
                    },
                    SolveOutcome::Optimal {
                        config: Some(target_config),
                        evaluation: target_evaluation,
                    },
                )
            }
            SolveOutcome::Optimal { config: None, .. } => anyhow::bail!(
                "Bundle solving requires a witness-capable target problem and witness-capable reduction path; {} only supports aggregate-value solving.",
                self.target_name
            ),
            SolveOutcome::Infeasible => (SolveOutcome::Infeasible, SolveOutcome::Infeasible),
        };

        Ok(BundleSolveResult {
            source_name: self.source_name.clone(),
            target_name: self.target_name.clone(),
            solver,
            source_outcome,
            target_outcome,
        })
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

        let result = loaded
            .solve_deterministically(SolverRequest::BruteForce)
            .unwrap();
        assert_eq!(
            result.outcome,
            SolveOutcome::Optimal {
                config: None,
                evaluation: "Sum(56)".to_string(),
            }
        );
    }

    #[test]
    fn test_default_uses_brute_force_without_registered_backend() {
        let loaded = load_problem(
            AGGREGATE_SOURCE_NAME,
            &BTreeMap::new(),
            serde_json::to_value(AggregateValueSource::sample()).unwrap(),
        )
        .unwrap();

        let result = loaded
            .solve_deterministically(SolverRequest::Default)
            .unwrap();
        assert_eq!(
            result.solver,
            problemreductions::solvers::SolverExecution::BruteForce
        );
    }

    #[test]
    fn test_explicit_ilp_requires_registered_pipeline() {
        let loaded = load_problem(
            AGGREGATE_SOURCE_NAME,
            &BTreeMap::new(),
            serde_json::to_value(AggregateValueSource::sample()).unwrap(),
        )
        .unwrap();

        let err = loaded
            .solve_deterministically(SolverRequest::Ilp)
            .unwrap_err();
        assert!(
            err.to_string().contains("No ILP pipeline is registered"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn solver_request_accepts_only_documented_overrides() {
        assert_eq!(solver_request(None).unwrap(), SolverRequest::Default);
        assert_eq!(
            solver_request(Some("customized")).unwrap(),
            SolverRequest::Customized
        );
        assert_eq!(solver_request(Some("ilp")).unwrap(), SolverRequest::Ilp);
        assert_eq!(
            solver_request(Some("brute-force")).unwrap(),
            SolverRequest::BruteForce
        );
        for rejected in ["auto", "native", "implementation-id"] {
            let error = solver_request(Some(rejected)).unwrap_err();
            assert!(error.to_string().contains(rejected), "{error}");
        }
    }

    #[test]
    fn solve_result_json_preserves_structured_solver_contract() {
        let result = DeterministicSolveResult {
            solver: problemreductions::solvers::SolverExecution::Ilp {
                reduction_path: vec!["Source".to_string(), "ILP<bool>".to_string()],
            },
            outcome: SolveOutcome::Optimal {
                config: Some(vec![1, 0]),
                evaluation: "Max(1)".to_string(),
            },
        };
        let json = solve_result_json("Source", &result);

        assert_eq!(json["problem"], "Source");
        assert_eq!(json["solver"]["kind"], "ilp");
        assert_eq!(json["status"], "optimal");
        assert_eq!(
            json["solver"]["reduction_path"],
            serde_json::json!(["Source", "ILP<bool>"])
        );
        assert_eq!(json["solution"], serde_json::json!([1, 0]));
        assert!(json.get("reduced_to").is_none());
    }

    #[test]
    fn solver_capabilities_view_centralizes_default_and_available_order() {
        use problemreductions::models::graph::RootedTreeArrangement;
        use problemreductions::Problem;

        let problem = RootedTreeArrangement::new(SimpleGraph::new(2, vec![(0, 1)]), 1);
        let loaded = load_problem(
            RootedTreeArrangement::<SimpleGraph>::NAME,
            &BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
            serde_json::to_value(problem).unwrap(),
        )
        .unwrap();
        let view = solver_capabilities_view(&loaded).unwrap();

        assert_eq!(view.default_solver, "customized");
        assert_eq!(view.solvers, ["customized", "ilp", "brute-force"]);
        assert!(view.capabilities.customized.is_some());
        assert!(view.capabilities.ilp.is_some());
        assert!(view.capabilities.brute_force);
    }
}
