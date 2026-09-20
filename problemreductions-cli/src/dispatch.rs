use anyhow::{Context, Result};
use problemreductions::registry::{DynProblem, LoadedDynProblem};
use problemreductions::rules::ReductionGraph;
use problemreductions::solvers::{
    brute_force_dimensions, solve, solver_capabilities, ExactProblemKey, SolveOutcome, SolveResult,
    SolverRequest,
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
    pub fn brute_force_num_variables(&self) -> Result<Option<usize>> {
        brute_force_dimensions(&self.inner)
            .map(|dimensions| dimensions.map(|dimensions| dimensions.len()))
            .map_err(|error| anyhow::anyhow!("solver capability registry is invalid: {error}"))
    }

    pub fn solve(&self, request: SolverRequest) -> Result<SolveResult> {
        solve(&self.inner, request).map_err(anyhow::Error::from)
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
    } else if registered.brute_force {
        "brute-force"
    } else {
        anyhow::bail!("no solver is registered for {}", key.label());
    };
    let mut solvers = Vec::with_capacity(3);
    if customized.is_some() {
        solvers.push("customized");
    }
    if ilp.is_some() {
        solvers.push("ilp");
    }
    if registered.brute_force {
        solvers.push("brute-force");
    }

    Ok(SolverCapabilitiesView {
        solvers,
        default_solver,
        capabilities: SolverCapabilityDetailsView {
            customized,
            ilp,
            brute_force: registered.brute_force,
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

pub fn solve_result_json(problem: &str, result: &SolveResult) -> serde_json::Value {
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
    steps: Vec<WitnessStep>,
}

struct WitnessStep {
    chain: problemreductions::rules::ReductionChain,
    source_variant: &'static problemreductions::registry::VariantEntry,
}

fn load_bundle_endpoints(
    bundle: &ReductionBundle,
) -> Result<(
    LoadedProblem,
    LoadedProblem,
    problemreductions::rules::ReductionPath,
)> {
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

    let target = load_problem(
        &bundle.target.problem_type,
        &bundle.target.variant,
        bundle.target.data.clone(),
    )?;

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

    Ok((source, target, reduction_path))
}

fn validate_replayed_target(bundle: &ReductionBundle, target_any: &dyn Any) -> Result<()> {
    let replayed_target_data = serialize_any_problem(
        &bundle.target.problem_type,
        &bundle.target.variant,
        target_any,
    )?;
    if replayed_target_data != bundle.target.data {
        anyhow::bail!(
            "Malformed bundle: `target.data` does not match the result of replaying \
                 `source` along `path`. The bundle is tampered or was produced by \
                 incompatible code."
        );
    }

    Ok(())
}

pub(crate) fn extract_bundle_value(
    bundle: &ReductionBundle,
    value: serde_json::Value,
) -> Result<serde_json::Value> {
    let (source, _target, path) = load_bundle_endpoints(bundle)?;
    let chain = ReductionGraph::new()
        .reduce_aggregate_along_path(&path, source.as_any())?
        .context("Bundle requires an aggregate-capable reduction path")?;
    validate_replayed_target(bundle, chain.target_problem_any())?;
    Ok(chain.extract_value(value)?)
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
    /// Returns an error (not a panic) for malformed bundles or paths without witness extraction.
    pub fn prepare(bundle: &ReductionBundle) -> Result<Self> {
        let (source, target, reduction_path) = load_bundle_endpoints(bundle)?;
        let graph = ReductionGraph::new();
        let mut steps: Vec<WitnessStep> = Vec::new();
        for edge in reduction_path.steps.windows(2) {
            let input = steps
                .last()
                .map_or(source.as_any(), |step| step.chain.target_problem_any());
            let path = problemreductions::rules::ReductionPath {
                steps: edge.to_vec(),
            };
            let chain = graph.reduce_along_path(&path, input)?.ok_or_else(|| {
                anyhow::anyhow!("Bundle requires a witness-capable reduction path")
            })?;
            let source_variant =
                problemreductions::registry::find_variant_entry(&edge[0].name, &edge[0].variant)
                    .context("missing intermediate problem registration")?;
            steps.push(WitnessStep {
                chain,
                source_variant,
            });
        }

        validate_replayed_target(bundle, steps.last().unwrap().chain.target_problem_any())?;
        Ok(Self {
            source_name: source.problem_name().to_string(),
            target_name: target.problem_name().to_string(),
            source,
            target,
            steps,
        })
    }

    /// Map a target-space configuration back to the source space and evaluate it.
    pub fn extract(
        &self,
        target_config: &serde_json::Value,
    ) -> Result<(serde_json::Value, String)> {
        let source_config = self
            .steps
            .iter()
            .rev()
            .try_fold(target_config.clone(), |solution, step| {
                step.chain.extract_solution_json(solution)
            })?;
        let source_eval = self.source.evaluate_witness_dyn(&source_config)?.ok_or_else(|| {
            problemreductions::rules::ExtractionError::invalid(format!(
                "extracted solution is infeasible for {}; the reduction did not establish a source solution",
                self.source_name
            ))
        })?;
        Ok((source_config, source_eval))
    }

    /// Execute recovery of a completed result. The caller establishes optimality
    /// or infeasibility under its solver contract, including numerical tolerances;
    /// evaluating a candidate cannot establish it.
    pub(crate) fn extract_result(&self, result: &SolveOutcome) -> Result<SolveOutcome> {
        use problemreductions::rules::ExtractionError;
        let steps = &self.steps;
        let (mut witness, mut value) = match result {
            SolveOutcome::Optimal {
                solution,
                evaluation,
            } => {
                let value = self.target.evaluate_json(solution)?;
                let actual = self
                    .target
                    .aggregate_witness_evaluation(&value)?
                    .ok_or_else(|| ExtractionError::invalid("target witness is infeasible"))?;
                if evaluation != &actual {
                    return Err(ExtractionError::invalid(
                        "target evaluation does not match the witness",
                    )
                    .into());
                }
                (Some(solution.clone()), value)
            }
            SolveOutcome::Infeasible => (None, self.target.empty_aggregate_json()?),
        };
        let mut evaluation = None;
        for (index, step) in steps.iter().enumerate().rev() {
            let input: &dyn DynProblem = if index == 0 {
                &*self.source
            } else {
                (step.source_variant.borrow_fn)(steps[index - 1].chain.target_problem_any())
                    .context("intermediate problem type mismatch")?
            };
            let mapped = if step.chain.has_value_mapping() {
                Some(step.chain.extract_value(value)?)
            } else {
                None
            };
            if let Some(mapped_value) = &mapped {
                if input.aggregate_witness_evaluation(mapped_value)?.is_none() {
                    witness = None;
                    value = mapped_value.clone();
                    evaluation = None;
                    continue;
                }
            }
            let target_witness = witness.take().ok_or_else(|| {
                ExtractionError::invalid(format!(
                    "cannot recover a {} witness from this value-only result",
                    input.problem_name()
                ))
            })?;
            let solution = step.chain.extract_solution_json(target_witness)?;
            value = input.evaluate_json(&solution)?;
            evaluation = Some(
                input
                    .aggregate_witness_evaluation(&value)?
                    .ok_or_else(|| ExtractionError::invalid("extracted solution is infeasible"))?,
            );
            if mapped.is_some_and(|mapped| mapped != value) {
                return Err(ExtractionError::invalid(
                    "extracted witness does not realize the mapped aggregate",
                )
                .into());
            }
            witness = Some(solution);
        }
        Ok(match witness {
            Some(solution) => SolveOutcome::Optimal {
                solution,
                evaluation: evaluation.expect("a recovered witness has an evaluation"),
            },
            None => SolveOutcome::Infeasible,
        })
    }

    /// Solve the target and map the result back to the source problem.
    ///
    pub(crate) fn solve(&self, request: SolverRequest) -> Result<BundleSolveResult> {
        let target_result = self.target.solve(request)?;
        let solver = target_result.solver;
        let target_outcome = target_result.outcome;
        let source_outcome = self.extract_result(&target_outcome)?;

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
    let problem_type = problemreductions::registry::find_problem_type(&canonical)
        .ok_or_else(|| anyhow::anyhow!("Unknown problem type: {canonical}"))?;
    let normalized =
        problemreductions::registry::ProblemRef::from_prefix_map(&problem_type, variant.clone())?;
    let inner = problemreductions::registry::load_dyn(&canonical, normalized.variant(), data)
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

    fn problem_step<P: problemreductions::Problem>() -> problemreductions::rules::ReductionStep {
        problemreductions::rules::ReductionStep {
            name: P::NAME.into(),
            variant: ReductionGraph::variant_to_map(&P::variant()),
        }
    }

    fn replay<P: problemreductions::Problem + serde::Serialize>(
        source: &P,
        targets: Vec<problemreductions::rules::ReductionStep>,
    ) -> BundleReplay {
        use problemreductions::rules::ReductionPath;
        let mut steps = vec![problem_step::<P>()];
        steps.extend(targets);
        let bundle = crate::commands::reduce::execute_route(
            ProblemJson {
                problem_type: P::NAME.into(),
                variant: ReductionGraph::variant_to_map(&P::variant()),
                data: serde_json::to_value(source).unwrap(),
            },
            ReductionPath { steps },
        )
        .unwrap();
        BundleReplay::prepare(&bundle).unwrap()
    }

    #[test]
    fn completed_recovery_composes_solution_only_and_value_mapping_steps() {
        use problemreductions::models::{Decision, MinimumVertexCover};
        type Cover = MinimumVertexCover<SimpleGraph, i64>;
        type Independent = MaximumIndependentSet<SimpleGraph, i64>;
        for bound in [1, 2] {
            let source = Decision::new(
                Cover::new(
                    SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
                    vec![1; 3],
                ),
                bound,
            );
            let replay = replay(
                &source,
                vec![problem_step::<Cover>(), problem_step::<Independent>()],
            );
            for solution in [
                json!([true, false, false]),
                json!([false, true, false]),
                json!([false, false, true]),
            ] {
                let recovered = replay
                    .extract_result(&SolveOutcome::Optimal {
                        solution,
                        evaluation: "Max(1)".into(),
                    })
                    .unwrap();
                assert_eq!(matches!(recovered, SolveOutcome::Infeasible), bound == 1);
                if let SolveOutcome::Optimal {
                    solution,
                    evaluation,
                } = recovered
                {
                    assert_eq!(evaluation, "Or(true)");
                    assert_eq!(
                        replay.source.evaluate_witness_dyn(&solution).unwrap(),
                        Some(evaluation)
                    );
                }
            }
            assert!(replay.extract_result(&SolveOutcome::Infeasible).is_err());
            for (solution, evaluation) in [
                (json!([true]), "Max(1)"),
                (json!([true, true, true]), "Max(None)"),
                (json!([true, false, false]), "Max(99)"),
            ] {
                assert!(replay
                    .extract_result(&SolveOutcome::Optimal {
                        solution,
                        evaluation: evaluation.into()
                    })
                    .is_err());
            }
        }
    }

    #[test]
    fn completed_recovery_carries_negative_answers_through_value_mappings() {
        use problemreductions::models::formula::{CNFClause, NAESatisfiability, Satisfiability};
        use problemreductions::models::graph::MaxCut;
        use problemreductions::solvers::BruteForce;
        use problemreductions::Problem;
        for unsatisfiable in [false, true] {
            let clauses = if unsatisfiable {
                vec![vec![1], vec![-1]]
            } else {
                vec![vec![1]]
            };
            let source = Satisfiability::new(1, clauses.into_iter().map(CNFClause::new).collect());
            let replay = replay(
                &source,
                vec![
                    problem_step::<NAESatisfiability>(),
                    problem_step::<
                        problemreductions::models::decision::Decision<MaxCut<SimpleGraph, i64>>,
                    >(),
                    problem_step::<MaxCut<SimpleGraph, i64>>(),
                ],
            );
            let target = replay
                .target
                .as_any()
                .downcast_ref::<MaxCut<SimpleGraph, i64>>()
                .unwrap();
            for solution in BruteForce::new().find_all_witnesses(target).unwrap() {
                let recovered = replay
                    .extract_result(&SolveOutcome::Optimal {
                        evaluation: target.evaluate(&solution).unwrap().to_string(),
                        solution: json!(solution),
                    })
                    .unwrap();
                assert_eq!(matches!(recovered, SolveOutcome::Infeasible), unsatisfiable);
            }
        }
    }

    #[test]
    fn completed_recovery_checks_value_and_solution_agreement() {
        use problemreductions::models::algebraic::{ObjectiveSense, ILP, QUBO};
        use problemreductions::Problem;
        let source = ILP::<bool>::new(1, vec![], vec![(0, 1)], ObjectiveSense::Maximize).unwrap();
        let mut replay = replay(&source, vec![problem_step::<QUBO<i64>>()]);
        let target_result = replay
            .target
            .solve(SolverRequest::BruteForce)
            .unwrap()
            .outcome;
        assert!(matches!(
            replay.extract_result(&target_result).unwrap(),
            SolveOutcome::Optimal { .. }
        ));
        // A different source objective must not agree with the executed value mapping.
        let different =
            ILP::<bool>::new(1, vec![], vec![(0, 2)], ObjectiveSense::Maximize).unwrap();
        replay.source = load_problem(
            ILP::<bool>::NAME,
            &ReductionGraph::variant_to_map(&ILP::<bool>::variant()),
            serde_json::to_value(different).unwrap(),
        )
        .unwrap();
        assert!(replay
            .extract_result(&target_result)
            .unwrap_err()
            .to_string()
            .contains("does not realize the mapped aggregate"));
        assert!(replay
            .source
            .aggregate_witness_evaluation(&json!(true))
            .is_err());
    }

    #[test]
    fn decision_chain_carries_no_through_identity_and_threshold_maps() {
        use problemreductions::models::{
            formula::{CNFClause, KSatisfiability, Satisfiability},
            MinimumVertexCover,
        };
        use problemreductions::variant::K3;
        for second in [1, -1] {
            let source = Satisfiability::new(
                1,
                vec![CNFClause::new(vec![1; 3]), CNFClause::new(vec![second; 3])],
            );
            let replay = replay(
                &source,
                vec![
                    problem_step::<KSatisfiability<K3>>(),
                    problem_step::<
                        problemreductions::models::decision::Decision<
                            MinimumVertexCover<SimpleGraph, i64>,
                        >,
                    >(),
                    problem_step::<MinimumVertexCover<SimpleGraph, i64>>(),
                ],
            );
            for solver in [SolverRequest::BruteForce, SolverRequest::Ilp] {
                let result = replay.solve(solver).unwrap();
                assert_eq!(
                    matches!(result.source_outcome, SolveOutcome::Optimal { .. }),
                    second == 1
                );
                if second == -1 {
                    assert!(matches!(result.source_outcome, SolveOutcome::Infeasible));
                }
            }
        }
    }

    #[test]
    fn ilp_bundle_recovers_target_infeasibility_under_backend_contract() {
        use problemreductions::models::formula::{CNFClause, NAESatisfiability, Satisfiability};
        let source =
            Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);
        let replay = replay(&source, vec![problem_step::<NAESatisfiability>()]);
        let result = replay.solve(SolverRequest::Ilp).unwrap();
        assert!(matches!(result.target_outcome, SolveOutcome::Infeasible));
        assert!(matches!(result.source_outcome, SolveOutcome::Infeasible));
    }

    #[test]
    fn aggregate_only_bundle_executes_and_recovers_without_witnesses() {
        use problemreductions::rules::{ReductionPath, ReductionStep};
        let bundle = crate::test_support::aggregate_bundle();
        let path = ReductionPath {
            steps: bundle
                .path
                .iter()
                .map(|step| ReductionStep {
                    name: step.name.clone(),
                    variant: step.variant.clone(),
                })
                .collect(),
        };
        let source = ProblemJson {
            problem_type: bundle.source.problem_type.clone(),
            variant: bundle.source.variant.clone(),
            data: bundle.source.data.clone(),
        };
        let executed = crate::commands::reduce::execute_aggregate_route(source, path).unwrap();
        assert_eq!(executed.target.data, serde_json::json!({"base":14}));
        assert_eq!(
            extract_bundle_value(&executed, serde_json::json!(12)).unwrap(),
            serde_json::json!(12)
        );
        assert!(extract_bundle_value(&executed, serde_json::json!(true)).is_err());
        assert!(BundleReplay::prepare(&executed).is_err());
    }

    #[test]
    fn decision_bundle_recovers_yes_and_no_without_invalid_witnesses() {
        for (clauses, feasible) in [
            (vec![vec![1, 1, 1], vec![-1, -1, -1]], false),
            (vec![vec![1, 1, 1], vec![1, 1, 1]], true),
        ] {
            let source = problemreductions::models::formula::KSatisfiability::<
                problemreductions::variant::K3,
            >::new(
                1,
                clauses
                    .into_iter()
                    .map(problemreductions::models::formula::CNFClause::new)
                    .collect(),
            );
            let source = ProblemJson {
                problem_type: "KSatisfiability".into(),
                variant: BTreeMap::from([("k".into(), "K3".into())]),
                data: serde_json::to_value(source).unwrap(),
            };
            let route = crate::commands::reduce::parse_path_json(
                r#"{"path":[{
                    "from":{"name":"KSatisfiability","variant":{"k":"K3"}},
                    "to":{"name":"DecisionMinimumVertexCover","variant":{"graph":"SimpleGraph","weight":"i64"}}
                },{
                    "from":{"name":"DecisionMinimumVertexCover","variant":{"graph":"SimpleGraph","weight":"i64"}},
                    "to":{"name":"MinimumVertexCover","variant":{"graph":"SimpleGraph","weight":"i64"}}
                }]}"#,
            ).unwrap();
            let bundle = crate::commands::reduce::execute_route(source, route).unwrap();
            let replay = BundleReplay::prepare(&bundle).unwrap();

            {
                let source = ProblemJson {
                    problem_type: bundle.source.problem_type.clone(),
                    variant: bundle.source.variant.clone(),
                    data: bundle.source.data.clone(),
                };
                let route = problemreductions::rules::ReductionPath {
                    steps: bundle
                        .path
                        .iter()
                        .map(|step| problemreductions::rules::ReductionStep {
                            name: step.name.clone(),
                            variant: step.variant.clone(),
                        })
                        .collect(),
                };
                assert!(crate::commands::reduce::execute_aggregate_route(source, route).is_ok());
            }
            let result = replay.solve(SolverRequest::BruteForce).unwrap();
            let SolveOutcome::Optimal { solution, .. } = &result.target_outcome else {
                panic!("vertex cover always has a feasible target solution")
            };
            assert_eq!(
                extract_bundle_value(&bundle, replay.target.evaluate_json(solution).unwrap())
                    .unwrap(),
                serde_json::json!(feasible)
            );
            if feasible {
                assert!(matches!(result.source_outcome,
                    SolveOutcome::Optimal { evaluation, .. } if evaluation == "Or(true)"));
            } else {
                assert!(matches!(result.source_outcome, SolveOutcome::Infeasible));
                assert!(replay.extract(solution).is_err());
            }
        }
    }

    #[test]
    fn test_float_bundle_round_trip_corpus_regression() {
        let source = serde_json::from_value(json!({
            "type": "ExpectedRetrievalCost",
            "variant": {},
            "data": {
                "num_sectors": 3,
                "probabilities": [
                    0.08974358974358974, 0.24358974358974358,
                    0.23076923076923078, 0.23076923076923078,
                    0.038461538461538464, 0.16666666666666666
                ]
            }
        }))
        .unwrap();
        let route = crate::commands::reduce::parse_path_json(
            r#"{"path":[{
                "from":{"name":"ExpectedRetrievalCost","variant":{}},
                "to":{"name":"ILP","variant":{"coefficient":"f64","variable":"bool"}}
            }]}"#,
        )
        .unwrap();
        let bundle = crate::commands::reduce::execute_route(source, route).unwrap();
        let encoded = serde_json::to_vec(&bundle).unwrap();
        let mut restored: ReductionBundle = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(restored.source.data, bundle.source.data);
        assert_eq!(restored.target.data, bundle.target.data);
        BundleReplay::prepare(&restored).expect("an unchanged JSON bundle must replay exactly");

        // A one-ULP change remains tampering; replay must not use a float tolerance.
        let coefficient = restored.target.data["objective"][0][1].as_f64().unwrap();
        restored.target.data["objective"][0][1] = json!(f64::from_bits(coefficient.to_bits() + 1));
        let error = BundleReplay::prepare(&restored).err().unwrap();
        assert!(error
            .to_string()
            .contains("does not match the result of replaying"));
    }

    #[test]
    fn test_load_problem_alias_uses_registry_dispatch() {
        let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i64; 3]);
        let variant = BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i64".to_string()),
        ]);
        let loaded =
            load_problem("MIS", &variant, serde_json::to_value(&problem).unwrap()).unwrap();
        assert_eq!(loaded.problem_name(), "MaximumIndependentSet");
    }

    #[test]
    fn test_load_problem_rejects_unresolved_weight_variant() {
        let problem = BinPacking::new(vec![3i64, 3, 2, 2], 5i64).unwrap();
        let loaded = load_problem(
            "BinPacking",
            &BTreeMap::new(),
            serde_json::to_value(&problem).unwrap(),
        );
        assert!(loaded.is_err());
    }

    #[test]
    fn test_load_problem_fills_trailing_ilp_coefficient_variant() {
        let data = json!({
            "variables": [{"lower_bound": 0, "upper_bound": 1}],
            "constraints": [],
            "objective": [[0, 1]],
            "sense": "Minimize"
        });
        let variant = BTreeMap::from([("variable".to_string(), "bool".to_string())]);
        let loaded = load_problem("ILP", &variant, data).unwrap();
        assert_eq!(loaded.variant_map()["variable"], "bool");
        assert_eq!(loaded.variant_map()["coefficient"], "i64");
    }

    #[test]
    fn test_load_problem_rejects_non_prefix_ilp_variant() {
        let variant = BTreeMap::from([("coefficient".to_string(), "i64".to_string())]);
        assert!(load_problem("ILP", &variant, json!({})).is_err());
    }

    #[test]
    fn test_load_problem_rejects_invalid_strong_connectivity_augmentation_instance() {
        let variant = BTreeMap::from([("weight".to_string(), "i64".to_string())]);
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
        let problem = BinPacking::new(vec![3i64, 3, 2, 2], 5i64).unwrap();
        let variant = BTreeMap::from([("weight".to_string(), "i64".to_string())]);
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
            err.to_string().contains("num_processors must be positive"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_solve_brute_force_problem_returns_solution() {
        let loaded = load_problem(
            AGGREGATE_SOURCE_NAME,
            &BTreeMap::new(),
            serde_json::to_value(AggregateValueSource::sample()).unwrap(),
        )
        .unwrap();

        let result = loaded.solve(SolverRequest::BruteForce).unwrap();
        assert_eq!(
            result.outcome,
            SolveOutcome::Optimal {
                solution: serde_json::json!([true, true, true]),
                evaluation: "Max(14)".to_string(),
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

        let result = loaded.solve(SolverRequest::Default).unwrap();
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

        let err = loaded.solve(SolverRequest::Ilp).unwrap_err();
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
        for rejected in ["auto", "implementation-id"] {
            let error = solver_request(Some(rejected)).unwrap_err();
            assert!(error.to_string().contains(rejected), "{error}");
        }
    }

    #[test]
    fn solve_result_json_preserves_structured_solver_contract() {
        let result = SolveResult {
            solver: problemreductions::solvers::SolverExecution::Ilp {
                reduction_path: vec!["Source".to_string(), "ILP<i64, bool>".to_string()],
            },
            outcome: SolveOutcome::Optimal {
                solution: serde_json::json!([true, false]),
                evaluation: "Max(1)".to_string(),
            },
        };
        let json = solve_result_json("Source", &result);

        assert_eq!(json["problem"], "Source");
        assert_eq!(json["solver"]["kind"], "ilp");
        assert_eq!(json["status"], "optimal");
        assert_eq!(
            json["solver"]["reduction_path"],
            serde_json::json!(["Source", "ILP<i64, bool>"])
        );
        assert_eq!(json["solution"], serde_json::json!([true, false]));
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
