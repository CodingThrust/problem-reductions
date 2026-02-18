# CLI Evaluate & Reduce Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement `pred evaluate` and `pred reduce` CLI commands with JSON problem file I/O, after unifying the library's `ExecutablePath<S,T>` + `ChainedReduction<S,T>` into a single untyped `ReductionChain`.

**Architecture:** Library refactoring replaces typed `ExecutablePath<S,T>` + `ChainedReduction<S,T>` + `make_executable::<S,T>()` with a single `ReductionChain` struct and `ReductionGraph::reduce_along_path()`. Then CLI uses match-based dispatch to map problem name + variant strings to concrete Rust types for deserialization and serialization.

**Tech Stack:** clap (CLI), serde_json (JSON I/O), problemreductions library (core logic)

---

### Task 1: Complete ReductionChain refactoring — update exports and callers

The library's `graph.rs` already has `ReductionChain` and `reduce_along_path()` implemented, and `ExecutablePath`/`ChainedReduction`/`make_executable` are already removed. Now update re-exports and all callers.

**Files:**
- Modify: `src/rules/mod.rs:61-63` — update re-exports
- Modify: `src/rules/traits.rs:103` — update doc comment
- Modify: `src/unit_tests/rules/graph.rs:796-993` — migrate 3 tests
- Modify: `src/unit_tests/rules/reduction_path_parity.rs:1-221` — migrate 3 tests
- Modify: `examples/chained_reduction_ksat_to_mis.rs` — migrate example

**Step 1: Update `src/rules/mod.rs` re-exports**

Change line 61-63 from:
```rust
pub use graph::{
    ChainedReduction, ExecutablePath, ReductionGraph, ReductionPath, ReductionStep,
};
```
To:
```rust
pub use graph::{
    ReductionChain, ReductionGraph, ReductionPath, ReductionStep,
};
```

**Step 2: Update `src/rules/traits.rs` doc comment**

Change line 103 from:
```rust
/// Used internally by `ExecutablePath` and `ChainedReduction`.
```
To:
```rust
/// Used internally by `ReductionChain`.
```

**Step 3: Update `src/unit_tests/rules/graph.rs` — migrate 3 tests**

The migration pattern for all callers:
```rust
// OLD:
let path = graph.make_executable::<SourceType, TargetType>(&rpath).unwrap();
let reduction = path.reduce(&source);
let target = reduction.target_problem();
let source_sol = reduction.extract_solution(&target_sol);

// NEW:
let chain = graph.reduce_along_path(&rpath, &source as &dyn std::any::Any).unwrap();
let target: &TargetType = chain.target_problem();
let source_sol = chain.extract_solution(&target_sol);
```

Tests to update:

**`test_make_executable_direct` (line 796)** — rename to `test_reduce_along_path_direct`:
```rust
#[test]
fn test_reduce_along_path_direct() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let rpath = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "MinimumVertexCover",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();
    // Just verify the path can produce a chain with a dummy source
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let chain = graph.reduce_along_path(&rpath, &source as &dyn std::any::Any);
    assert!(chain.is_some());
}
```

**`test_chained_reduction_direct` (line 818)** — rename to `test_reduction_chain_direct`:
```rust
#[test]
fn test_reduction_chain_direct() {
    use crate::solvers::{BruteForce, Solver};
    use crate::traits::Problem;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MinimumVertexCover::<SimpleGraph, i32>::variant());
    let rpath = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "MinimumVertexCover",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let chain = graph
        .reduce_along_path(&rpath, &problem as &dyn std::any::Any)
        .unwrap();
    let target: &MinimumVertexCover<SimpleGraph, i32> = chain.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.find_best(target).unwrap();
    let source_solution = chain.extract_solution(&target_solution);
    let metric = problem.evaluate(&source_solution);
    assert!(metric.is_valid());
}
```

**`test_chained_reduction_multi_step` (line 857)** — rename to `test_reduction_chain_multi_step`:
```rust
#[test]
fn test_reduction_chain_multi_step() {
    use crate::solvers::{BruteForce, Solver};
    use crate::traits::Problem;

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let dst = ReductionGraph::variant_to_map(&MaximumSetPacking::<i32>::variant());
    let rpath = graph
        .find_cheapest_path(
            "MaximumIndependentSet",
            &src,
            "MaximumSetPacking",
            &dst,
            &ProblemSize::new(vec![]),
            &MinimizeSteps,
        )
        .unwrap();

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let chain = graph
        .reduce_along_path(&rpath, &problem as &dyn std::any::Any)
        .unwrap();
    let target: &MaximumSetPacking<i32> = chain.target_problem();

    let solver = BruteForce::new();
    let target_solution = solver.find_best(target).unwrap();
    let source_solution = chain.extract_solution(&target_solution);
    let metric = problem.evaluate(&source_solution);
    assert!(metric.is_valid());
}
```

**`test_chained_reduction_with_variant_casts` (line 893)** — rename to `test_reduction_chain_with_variant_casts`:

Change all `make_executable::<S, T>(&rpath)` + `path.reduce(&source)` + `reduction.target_problem()` + `reduction.extract_solution()` to use `reduce_along_path` + `chain.target_problem()` + `chain.extract_solution()`. Same pattern as above.

**Step 4: Update `src/unit_tests/rules/reduction_path_parity.rs` — migrate 3 tests**

Same migration pattern. Tests to update:

- `test_jl_parity_maxcut_to_spinglass_path` (line 18)
- `test_jl_parity_maxcut_to_qubo_path` (line 73)
- `test_jl_parity_factoring_to_spinglass_path` (line 132)

Example for the first test — change from:
```rust
let path = graph
    .make_executable::<MaxCut<SimpleGraph, i32>, SpinGlass<SimpleGraph, f64>>(&rpath)
    .expect("Should make executable path");
// ...
let reduction = path.reduce(&source);
let target = reduction.target_problem();
// ...
let source_solution = reduction.extract_solution(&target_solution);
```
To:
```rust
let chain = graph
    .reduce_along_path(&rpath, &source as &dyn std::any::Any)
    .expect("Should build reduction chain");
let target: &SpinGlass<SimpleGraph, f64> = chain.target_problem();
// ...
let source_solution = chain.extract_solution(&target_solution);
```

Note: in parity tests, the source must be created *before* calling `reduce_along_path` (instead of between `make_executable` and `reduce`). Reorder the code so the problem is created first.

**Step 5: Update `examples/chained_reduction_ksat_to_mis.rs`**

Change from `make_executable`+`reduce` to `reduce_along_path`. The example also uses `ILPSolver::solve_reduced` which takes a `&impl Problem` — use `chain.target_problem::<MaximumIndependentSet<SimpleGraph, i32>>()` to get the typed reference.

```rust
// OLD:
let path = graph
    .make_executable::<KSatisfiability<K3>, MaximumIndependentSet<SimpleGraph, i32>>(&rpath)
    .unwrap();
let reduction = path.reduce(&ksat);
let target = reduction.target_problem();
let solver = ILPSolver::new();
let solution = solver.solve_reduced(target).unwrap();
let original = reduction.extract_solution(&solution);

// NEW:
let chain = graph
    .reduce_along_path(&rpath, &ksat as &dyn std::any::Any)
    .unwrap();
let target: &MaximumIndependentSet<SimpleGraph, i32> = chain.target_problem();
let solver = ILPSolver::new();
let solution = solver.solve_reduced(target).unwrap();
let original = chain.extract_solution(&solution);
```

**Step 6: Run tests**

Run: `make test clippy`
Expected: All tests pass, no clippy warnings.

**Step 7: Commit**

```
refactor(rules): replace ExecutablePath+ChainedReduction with ReductionChain

Unify ExecutablePath<S,T>, ChainedReduction<S,T>, and make_executable
into a single untyped ReductionChain and reduce_along_path(). Callers
downcast when they need typed access.
```

---

### Task 2: Create `dispatch.rs` — DynProblem trait and match table

**Files:**
- Create: `problemreductions-cli/src/dispatch.rs`
- Modify: `problemreductions-cli/src/main.rs` (add `mod dispatch;`)

**Step 1: Create `dispatch.rs` with DynProblem trait and load_problem**

```rust
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
```

**Step 2: Add `mod dispatch;` to `problemreductions-cli/src/main.rs`**

Add after the existing `mod` declarations:
```rust
mod dispatch;
```

**Step 3: Run `cargo check -p problemreductions-cli`**

Expected: compiles without errors.

**Step 4: Commit**

```
feat(cli): add DynProblem dispatch table for runtime problem loading
```

---

### Task 3: Implement `pred evaluate` command

**Files:**
- Create: `problemreductions-cli/src/commands/evaluate.rs`
- Modify: `problemreductions-cli/src/commands/mod.rs`
- Modify: `problemreductions-cli/src/main.rs`

**Step 1: Create `commands/evaluate.rs`**

```rust
use crate::dispatch::{load_problem, ProblemJson};
use crate::output::OutputConfig;
use anyhow::Result;
use std::path::Path;

pub fn evaluate(input: &Path, config_str: &str, out: &OutputConfig) -> Result<()> {
    let content = std::fs::read_to_string(input)?;
    let problem_json: ProblemJson = serde_json::from_str(&content)?;

    let problem = load_problem(
        &problem_json.problem_type,
        &problem_json.variant,
        problem_json.data,
    )?;

    let config: Vec<usize> = config_str
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Invalid config value '{}': {}", s.trim(), e))
        })
        .collect::<Result<Vec<_>>>()?;

    let dims = problem.dims_dyn();
    if config.len() != dims.len() {
        anyhow::bail!(
            "Config has {} values but problem has {} variables",
            config.len(),
            dims.len()
        );
    }

    let result = problem.evaluate_dyn(&config);

    let text = format!("{}", result);
    let json = serde_json::json!({
        "problem": problem.problem_name(),
        "config": config,
        "result": result,
    });

    out.emit_with_default_name("pred_evaluate.json", &text, &json)
}
```

**Step 2: Add to `commands/mod.rs`**

```rust
pub mod evaluate;
```

**Step 3: Wire up in `main.rs`**

Replace `Commands::Evaluate(_args) => todo!("evaluate")` with:
```rust
Commands::Evaluate(args) => commands::evaluate::evaluate(&args.input, &args.config, &out),
```

**Step 4: Run `cargo check -p problemreductions-cli`**

**Step 5: Commit**

```
feat(cli): implement 'pred evaluate' command
```

---

### Task 4: Implement `pred reduce` command

**Files:**
- Create: `problemreductions-cli/src/commands/reduce.rs`
- Modify: `problemreductions-cli/src/commands/mod.rs`
- Modify: `problemreductions-cli/src/main.rs`

**Step 1: Create `commands/reduce.rs`**

```rust
use crate::dispatch::{
    load_problem, serialize_any_problem, PathStep, ProblemJson, ProblemJsonOutput, ReductionBundle,
};
use crate::output::OutputConfig;
use crate::problem_name::parse_problem_spec;
use anyhow::Result;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::types::ProblemSize;
use std::collections::BTreeMap;
use std::path::Path;

pub fn reduce(input: &Path, target: &str, out: &OutputConfig) -> Result<()> {
    // 1. Load source problem
    let content = std::fs::read_to_string(input)?;
    let problem_json: ProblemJson = serde_json::from_str(&content)?;

    let source = load_problem(
        &problem_json.problem_type,
        &problem_json.variant,
        problem_json.data.clone(),
    )?;

    let source_name = source.problem_name();
    let source_variant = source.variant_map();

    // 2. Parse target spec
    let dst_spec = parse_problem_spec(target)?;
    let graph = ReductionGraph::new();

    // Resolve target variant (use default if not specified)
    let graph_json_str = graph.to_json_string()?;
    let graph_json: serde_json::Value = serde_json::from_str(&graph_json_str)?;
    let nodes = graph_json["nodes"].as_array().unwrap();

    let dst_variants: Vec<BTreeMap<String, String>> = nodes
        .iter()
        .filter(|n| n["name"].as_str() == Some(&dst_spec.name))
        .map(|n| {
            n["variant"]
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect()
                })
                .unwrap_or_default()
        })
        .collect();

    if dst_variants.is_empty() {
        anyhow::bail!("Unknown target problem: {}", dst_spec.name);
    }

    // 3. Find reduction path
    let input_size = ProblemSize::new(vec![]);
    let mut best_path = None;

    for dv in &dst_variants {
        if let Some(p) = graph.find_cheapest_path(
            source_name,
            &source_variant,
            &dst_spec.name,
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
            "No reduction path from {} to {}",
            source_name,
            dst_spec.name
        )
    })?;

    // 4. Execute reduction chain via reduce_along_path
    let chain = graph
        .reduce_along_path(&reduction_path, source.as_any())
        .ok_or_else(|| anyhow::anyhow!("Failed to execute reduction chain"))?;

    // 5. Serialize target
    let target_step = reduction_path.steps.last().unwrap();
    let target_data = serialize_any_problem(
        &target_step.name,
        &target_step.variant,
        chain.target_problem_any(),
    )?;

    // 6. Build full reduction bundle
    let bundle = ReductionBundle {
        source: ProblemJsonOutput {
            problem_type: source_name.to_string(),
            variant: source_variant,
            data: problem_json.data,
        },
        target: ProblemJsonOutput {
            problem_type: target_step.name.clone(),
            variant: target_step.variant.clone(),
            data: target_data,
        },
        path: reduction_path
            .steps
            .iter()
            .map(|s| PathStep {
                name: s.name.clone(),
                variant: s.variant.clone(),
            })
            .collect(),
    };

    let text = format!(
        "Reduced {} to {} ({} steps)\nBundle written with source + target + path.",
        source_name,
        target_step.name,
        reduction_path.len(),
    );

    let json = serde_json::to_value(&bundle)?;
    let default_name = format!("pred_reduce_{}_to_{}.json", source_name, dst_spec.name);
    out.emit_with_default_name(&default_name, &text, &json)
}
```

**Step 2: Add to `commands/mod.rs`**

```rust
pub mod reduce;
```

**Step 3: Wire up in `main.rs`**

Replace `Commands::Reduce(_args) => todo!("reduce")` with:
```rust
Commands::Reduce(args) => commands::reduce::reduce(&args.input, &args.to, &out),
```

**Step 4: Run `cargo check -p problemreductions-cli`**

**Step 5: Commit**

```
feat(cli): implement 'pred reduce' command with full reduction bundle
```

---

### Task 5: Add integration tests

**Files:**
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Add evaluate and reduce tests**

```rust
#[test]
fn test_evaluate() {
    let problem_json = r#"{
        "type": "MaximumIndependentSet",
        "variant": {"graph": "SimpleGraph", "weight": "i32"},
        "data": {
            "graph": {"num_vertices": 4, "edges": [[0,1],[1,2],[2,3]]},
            "weights": [1, 1, 1, 1]
        }
    }"#;
    let tmp = std::env::temp_dir().join("pred_test_evaluate.json");
    std::fs::write(&tmp, problem_json).unwrap();

    let output = pred()
        .args(["evaluate", tmp.to_str().unwrap(), "--config", "1,0,1,0"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Valid"));
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_evaluate_sat() {
    let problem_json = r#"{
        "type": "Satisfiability",
        "data": {
            "num_vars": 3,
            "clauses": [[{"variable": 0, "negated": false}, {"variable": 1, "negated": false}]]
        }
    }"#;
    let tmp = std::env::temp_dir().join("pred_test_eval_sat.json");
    std::fs::write(&tmp, problem_json).unwrap();

    let output = pred()
        .args(["evaluate", tmp.to_str().unwrap(), "--config", "1,1,0"])
        .output()
        .unwrap();
    assert!(output.status.success());
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_reduce() {
    let problem_json = r#"{
        "type": "MIS",
        "variant": {"graph": "SimpleGraph", "weight": "i32"},
        "data": {
            "graph": {"num_vertices": 4, "edges": [[0,1],[1,2],[2,3]]},
            "weights": [1, 1, 1, 1]
        }
    }"#;
    let input = std::env::temp_dir().join("pred_test_reduce_in.json");
    let output_file = std::env::temp_dir().join("pred_test_reduce_out.json");
    std::fs::write(&input, problem_json).unwrap();

    let output = pred()
        .args([
            "--json",
            "-o",
            output_file.to_str().unwrap(),
            "reduce",
            input.to_str().unwrap(),
            "--to",
            "QUBO",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_file.exists());

    let content = std::fs::read_to_string(&output_file).unwrap();
    let bundle: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(bundle["source"]["type"], "MaximumIndependentSet");
    assert_eq!(bundle["target"]["type"], "QUBO");
    assert!(bundle["path"].is_array());

    std::fs::remove_file(&input).ok();
    std::fs::remove_file(&output_file).ok();
}
```

**Step 2: Run tests**

Run: `cargo test -p problemreductions-cli`

**Step 3: Commit**

```
test(cli): add integration tests for evaluate and reduce commands
```

---

### Task 6: Update CLI documentation

**Files:**
- Modify: `docs/src/cli.md`

**Step 1: Add evaluate and reduce sections to `docs/src/cli.md`**

Add after the `pred schema` section:

````markdown
### `pred evaluate` — Evaluate a configuration

Evaluate a configuration against a problem instance from a JSON file:

```bash
$ pred evaluate problem.json --config 1,0,1,0
Valid(2)
```

The JSON file uses a wrapper format:

```json
{
  "type": "MaximumIndependentSet",
  "variant": {"graph": "SimpleGraph", "weight": "i32"},
  "data": {
    "graph": {"num_vertices": 4, "edges": [[0,1],[1,2],[2,3]]},
    "weights": [1, 1, 1, 1]
  }
}
```

- `type`: Problem name (aliases like `MIS` accepted)
- `variant`: Optional, defaults to base variant
- `data`: The problem struct fields as JSON

### `pred reduce` — Reduce a problem

Reduce a problem to a target type. Outputs a full reduction bundle with source, target, and path:

```bash
$ pred reduce problem.json --to QUBO
Reduced MaximumIndependentSet to QUBO (1 steps)
Bundle written with source + target + path.
```

Save the bundle as JSON for later use:

```bash
pred reduce problem.json --to QUBO --json -o bundle.json
```

The bundle contains everything needed to map solutions back:

```json
{
  "source": { "type": "MaximumIndependentSet", "variant": {...}, "data": {...} },
  "target": { "type": "QUBO", "variant": {...}, "data": {...} },
  "path": [
    {"name": "MaximumIndependentSet", "variant": {"graph": "SimpleGraph", "weight": "i32"}},
    {"name": "QUBO", "variant": {"weight": "f64"}}
  ]
}
```
````

**Step 2: Commit**

```
docs: document evaluate and reduce CLI commands
```

---

### Task 7: Run clippy and full test suite

**Step 1: Run `make test clippy`**

**Step 2: Fix any issues**

**Step 3: Final commit if needed**
