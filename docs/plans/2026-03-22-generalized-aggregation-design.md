# Generalized Aggregation -- Unified Problem Trait Hierarchy

**Date:** 2026-03-22
**Status:** Revised design, approved for implementation planning
**Supersedes:** `2026-03-22-counting-problem-trait-design.md`

## Problem

The current trait hierarchy hard-codes two witness-oriented problem families:

- `OptimizationProblem` (`Metric = SolutionSize<V>`, plus `direction()`)
- `SatisfactionProblem` (`Metric = bool`)

That works for "find one config" workflows, but it does not scale to `#P` and probability problems where the answer is an aggregate over the whole configuration space. Adding a third parallel leaf trait for counting would unblock the immediate issues, but it would also duplicate the same branching in solvers, macros, registry dispatch, and reduction execution.

The goal of this design is to unify value aggregation while preserving the existing witness-oriented workflows that the repo already depends on:

- brute-force witness search
- solution extraction through reduction chains
- `pred reduce` bundles
- `pred solve bundle.json`
- ILP solve-via-reduction

## Core idea

Unify the **value layer**, not the **witness layer**.

Each problem exposes a single aggregate value type. Solvers always know how to compute the final value by folding over all configurations. Some aggregate types also support recovering representative witness configurations; others do not.

This keeps the mathematical core small while making the runtime honest about which operations are valid.

## `Aggregate` trait

`Aggregate` remains a monoid at its core, but it also exposes optional witness hooks with safe defaults. That is the minimal extra surface needed to keep dynamic witness APIs working without re-introducing a full parallel trait hierarchy.

```rust
// src/types.rs
pub trait Aggregate: Clone + fmt::Debug + Serialize + DeserializeOwned {
    /// Neutral element for folding over the configuration space.
    fn identity() -> Self;

    /// Associative combine operation.
    fn combine(self, other: Self) -> Self;

    /// Whether this aggregate admits representative witness configurations.
    fn supports_witnesses() -> bool {
        false
    }

    /// Whether a per-configuration value belongs to the witness set
    /// for the final aggregate value.
    fn contributes_to_witnesses(_config_value: &Self, _total: &Self) -> bool {
        false
    }
}
```

The default witness behavior is deliberately conservative:

- `Sum` and `And` remain value-only
- `Max`, `Min`, and `Or` opt in to witness recovery

## Aggregate types

Five concrete aggregate wrappers replace the current leaf-trait split:

```rust
// src/types.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Max<V>(pub Option<V>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Min<V>(pub Option<V>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sum<W>(pub W);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Or(pub bool);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct And(pub bool);
```

| Type | Identity | Combine | Witness support | Replaces |
|------|----------|---------|-----------------|----------|
| `Max<V>` | `Max(None)` | keep larger `Some` | yes | `SolutionSize<V>` + `Direction::Maximize` |
| `Min<V>` | `Min(None)` | keep smaller `Some` | yes | `SolutionSize<V>` + `Direction::Minimize` |
| `Sum<W>` | `Sum(W::zero())` | numeric addition | no | counting / probability totals |
| `Or` | `Or(false)` | logical or | yes | `bool` existential problems |
| `And` | `And(true)` | logical and | no | universal / tautology-style problems |

Witness semantics:

- `Max` / `Min`: a config is a witness iff its aggregate value equals the final optimum and is feasible
- `Or`: a config is a witness iff it evaluates to `Or(true)` and the final total is `Or(true)`
- `Sum` / `And`: no single config is a representative witness, so witness APIs return `None` / empty

## Unified `Problem` trait

```rust
// src/traits.rs
pub trait Problem: Clone {
    const NAME: &'static str;
    type Value: Aggregate;

    fn dims(&self) -> Vec<usize>;
    fn evaluate(&self, config: &[usize]) -> Self::Value;

    fn num_variables(&self) -> usize {
        self.dims().len()
    }

    fn variant() -> Vec<(&'static str, &'static str)>;

    fn problem_type() -> crate::registry::ProblemType {
        crate::registry::find_problem_type(Self::NAME)
            .unwrap_or_else(|| panic!("no catalog entry for Problem::NAME = {:?}", Self::NAME))
    }
}
```

Removed:

- `OptimizationProblem`
- `SatisfactionProblem`
- `type Metric`
- `SolutionSize`
- `Direction`

Unchanged:

- `DeclaredVariant`
- `Problem::NAME`
- `dims()`
- `variant()`
- catalog bridge via `problem_type()`

## Solvers

### Value solving

All problems support value solving through one fold:

```rust
// src/solvers/mod.rs
pub trait Solver {
    fn solve<P: Problem>(&self, problem: &P) -> P::Value;
}
```

```rust
// src/solvers/brute_force.rs
impl Solver for BruteForce {
    fn solve<P: Problem>(&self, problem: &P) -> P::Value {
        DimsIterator::new(problem.dims())
            .map(|config| problem.evaluate(&config))
            .fold(P::Value::identity(), P::Value::combine)
    }
}
```

### Witness solving

Witness APIs remain available, but only when the aggregate type opts in through the default hooks above:

```rust
impl BruteForce {
    pub fn find_witness<P: Problem>(&self, problem: &P) -> Option<Vec<usize>>;
    pub fn find_all_witnesses<P: Problem>(&self, problem: &P) -> Vec<Vec<usize>>;
    pub fn solve_with_witnesses<P: Problem>(&self, problem: &P)
        -> (P::Value, Vec<Vec<usize>>);
}
```

Behavior:

- `Max` / `Min`: witnesses are the optimal configs
- `Or`: witnesses are satisfying configs
- `Sum` / `And`: `find_witness()` returns `None`, `find_all_witnesses()` returns `[]`

This is the key distinction from the counting-only design: value aggregation is unified, but witness recovery is explicitly optional.

## Dynamic solve surfaces

The dynamic registry needs two solve entry points, not one:

```rust
// src/registry/dyn_problem.rs
pub type SolveValueFn = fn(&dyn Any) -> String;
pub type SolveWitnessFn = fn(&dyn Any) -> Option<(Vec<usize>, String)>;
```

`VariantEntry` stores both:

- `solve_value_fn` always exists
- `solve_witness_fn` always exists, but returns `None` for aggregate-only values (`Sum`, `And`)

`LoadedDynProblem` mirrors that split:

- `solve_brute_force_value() -> String`
- `solve_brute_force_witness() -> Option<(Vec<usize>, String)>`

This keeps `declare_variants!` simple:

- the `opt` / `sat` keywords disappear
- the generated value-solve closure always calls `Solver::solve()`
- the generated witness-solve closure always calls `BruteForce::find_witness()`

No solver-kind branching is needed at variant registration time.

## CLI behavior

### `pred solve problem.json`

Always computes the aggregate value.

- If a witness exists, print both `Solution` and `Evaluation`
- If no witness exists, print only `Evaluation`

Examples:

- `Max(Some(42))` -> solution config + `Maximum: 42`
- `Or(true)` -> solution config + `Satisfiable: true`
- `Sum(0.9832)` -> no single solution config, print `Sum: 0.9832`
- `And(false)` -> no single solution config, print `Tautology: false`

### `pred solve bundle.json`

Remains a **witness-only** workflow in this design.

Bundles exist to solve a target problem and map a target configuration back through `extract_solution`. That makes sense only for witness-capable problems and witness-capable reduction paths.

If the target variant or the path is aggregate-only, bundle solving is rejected early with a clear error.

### `--solver ilp`

Also remains **witness-only**.

ILP support in this repo is a witness-producing solve-via-reduction path. Aggregate-only problems (`Sum`, `And`) do not have an ILP mode unless a future design introduces a threshold or certificate-bearing witness formulation.

The immediate design change is:

- keep the ILP solver internals unchanged
- require witness-capable source problems
- require a witness-capable path from source to `ILP`

## Reductions

Two reduction traits remain necessary because config mapping and aggregate-value mapping are genuinely different operations.

```rust
// src/rules/traits.rs
pub trait ReductionResult {
    type Source: Problem;
    type Target: Problem;
    fn target_problem(&self) -> &Self::Target;
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize>;
}

pub trait ReduceTo<T: Problem>: Problem {
    type Result: ReductionResult<Source = Self, Target = T>;
    fn reduce_to(&self) -> Self::Result;
}

pub trait AggregateReductionResult {
    type Source: Problem;
    type Target: Problem;
    fn target_problem(&self) -> &Self::Target;
    fn extract_value(
        &self,
        target_value: <Self::Target as Problem>::Value,
    ) -> <Self::Source as Problem>::Value;
}

pub trait ReduceToAggregate<T: Problem>: Problem {
    type Result: AggregateReductionResult<Source = Self, Target = T>;
    fn reduce_to_aggregate(&self) -> Self::Result;
}
```

Type-erased runtime support likewise splits:

- `DynReductionResult` for witness/config reductions
- `DynAggregateReductionResult` for aggregate/value reductions

## `EdgeCapabilities`

The reduction graph needs explicit edge-mode metadata so path search can reject incompatible paths before execution.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeCapabilities {
    pub witness: bool,
    pub aggregate: bool,
}
```

Capability assignment:

- `ReduceTo<T>` edges -> `{ witness: true, aggregate: false }`
- `ReduceToAggregate<T>` edges -> `{ witness: false, aggregate: true }`
- natural subtype / `ReductionAutoCast` edges -> `{ witness: true, aggregate: true }`

Why the natural edges are both:

- witness mode: the config mapping is identity
- aggregate mode: the value mapping is also identity because the problem semantics do not change

## Mode-aware pathfinding

Pathfinding stays on one graph, but it now receives a required capability:

```rust
pub enum ReductionMode {
    Witness,
    Aggregate,
}
```

`ReductionGraph::find_cheapest_path(...)` becomes capability-aware:

- witness callers traverse only edges with `capabilities.witness`
- aggregate callers traverse only edges with `capabilities.aggregate`

This prevents "valid graph path, invalid runtime execution" failures.

Mode usage:

- `pred reduce` -> witness
- `pred solve bundle.json` -> witness
- ILP solve-via-reduction -> witness
- future aggregate chain execution -> aggregate
- graph export / inspection -> all edges, with capability metadata shown

## Aggregate reduction chains

Witness execution stays on `ReductionChain`.

Aggregate execution gets its own chain:

```rust
pub struct AggregateReductionChain {
    steps: Vec<Box<dyn DynAggregateReductionResult>>,
}
```

with:

- `target_problem_any()`
- backwards composition of `extract_value_dyn(...)`

The important point is that witness execution and aggregate execution are separate entry points over the same graph, selected by `ReductionMode`.

## Registry and graph changes

### `ReductionEntry`

`ReductionEntry` gains:

- `reduce_fn: Option<ReduceFn>`
- `reduce_aggregate_fn: Option<AggregateReduceFn>`
- `capabilities: EdgeCapabilities`

### `ReductionEdgeData`

`ReductionEdgeData` gains:

- `capabilities: EdgeCapabilities`
- optional witness executor
- optional aggregate executor

### Graph export

The JSON export includes:

- `witness: bool`
- `aggregate: bool`

instead of a single coarse edge-kind label.

## Model migration examples

### Optimization

```rust
impl Problem for MaximumIndependentSet<G, W> {
    type Value = Max<W::Sum>;

    fn evaluate(&self, config: &[usize]) -> Max<W::Sum> {
        if invalid {
            Max(None)
        } else {
            Max(Some(size))
        }
    }
}
```

### Satisfaction

```rust
impl Problem for Satisfiability {
    type Value = Or;

    fn evaluate(&self, config: &[usize]) -> Or {
        Or(satisfies)
    }
}
```

### Counting

```rust
impl Problem for NetworkReliability {
    type Value = Sum<f64>;

    fn evaluate(&self, config: &[usize]) -> Sum<f64> {
        if terminals_connected {
            Sum(probability_weight)
        } else {
            Sum(0.0)
        }
    }
}
```

## Migration scope

| Area | Change |
|------|--------|
| `src/types.rs` | replace `SolutionSize` / `Direction` with aggregate wrappers and witness hooks |
| `src/traits.rs` | unify on `Problem<Value>` |
| `src/solvers/` | one value fold plus generic witness helpers |
| `src/registry/` | split value solve from witness solve |
| `problemreductions-macros/` | remove `opt` / `sat`, emit both dynamic solve closures |
| `src/rules/` | add aggregate reductions and capability-aware path execution |
| `problemreductions-cli/` | differentiate value-only vs witness-capable solve output |
| existing model/test files | mechanical `Metric -> Value` migration |

## What is not changed

- problem names, aliases, and variant resolution
- the overall CLI command set
- the catalog bridge via `ProblemType`
- the fact that ILP is a witness-oriented backend
- the paper format in `docs/paper/reductions.typ`

## Deferred follow-up work

Out of scope for this design revision:

- threshold-specific decision wrappers for `Sum` problems
- a new aggregate-only bundle format
- universal counterexample extraction for `And`
- choosing default reduction modes in graph-inspection UX

## Alternatives considered

1. **Minimal `CountingProblem` extension**
   - Lowest short-term diff
   - Repeats the branching in solvers, registry dispatch, macros, and reductions

2. **Unify value aggregation but keep witness-oriented runtime explicit** (chosen)
   - Solves the architectural duplication
   - Preserves the witness assumptions already embedded in the repo

3. **Single edge kind with runtime rejection**
   - Smaller patch
   - Bad UX and bad API: pathfinding would still return paths that cannot be executed

## Related issues

- #737 -- original aggregation architecture issue
- #748 -- default solver per problem (future, orthogonal)
- #235, #237, #404, #405 -- counting models enabled by this refactor
- #256, #257, #394, #395 -- aggregate-value reductions enabled by this refactor
