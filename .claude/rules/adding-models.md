---
paths:
  - "src/models/**/*.rs"
---

# Adding a Model (Problem Type)

**Reference implementations — read these first:**
- **Optimization problem:** `src/models/graph/maximum_independent_set.rs` — `Problem` + `OptimizationProblem` with `Metric = SolutionSize<W>`
- **Satisfaction problem:** `src/models/satisfiability/sat.rs` — `Problem` with `Metric = bool`
- **Reference test:** `src/unit_tests/models/graph/maximum_independent_set.rs`

## Steps

1. **Create** `src/models/<category>/<name>.rs` — follow the reference for struct definition, `Problem` impl, and `OptimizationProblem` impl (if applicable).
2. **Register** in `src/models/<category>/mod.rs`.
3. **Add tests** in `src/unit_tests/models/<category>/<name>.rs` (linked via `#[path]`).
4. **Document** in `docs/paper/reductions.typ`: add `display-name` entry and `#problem-def("Name")[definition...]`.

## Trait Implementations

Every problem must implement `Problem` (see `src/traits.rs`). Key points:

- **`type Metric`** — `SolutionSize<W>` for optimization, `bool` for satisfaction
- **`fn dims()`** — configuration space dimensions (e.g., `vec![2; n]` for n binary variables)
- **`fn evaluate()`** — return `SolutionSize::Valid(value)` / `SolutionSize::Invalid` for optimization, or `bool` for satisfaction
- **`fn variant()`** — graph and weight type metadata for the reduction registry

Optimization problems additionally implement `OptimizationProblem` (see `src/traits.rs`):
- **`fn direction()`** — `Direction::Maximize` or `Direction::Minimize`
- **`fn is_better()`** — delegates to `SolutionSize::is_better()`
- **`fn is_feasible()`** — delegates to `SolutionSize::is_valid()`

Weight management (`weights()`, `set_weights()`, `is_weighted()`) goes on inherent `impl` blocks, not traits. See the reference implementation for the pattern.

## Categories

- `src/models/satisfiability/` — Satisfiability, KSatisfiability, CircuitSAT
- `src/models/graph/` — MaximumIndependentSet, MinimumVertexCover, KColoring, etc.
- `src/models/set/` — MinimumSetCovering, MaximumSetPacking
- `src/models/optimization/` — SpinGlass, QUBO, ILP
- `src/models/specialized/` — Factoring

## Naming

Use explicit optimization prefixes: `Maximum` for maximization, `Minimum` for minimization (e.g., `MaximumIndependentSet`, `MinimumVertexCover`).
