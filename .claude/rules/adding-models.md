---
paths:
  - "src/models/**/*.rs"
---

# Adding a Model (Problem Type)

**Reference implementation:** `src/models/graph/kcoloring.rs`

## Steps

1. **Create** `src/models/<category>/<name>.rs` — follow the reference for struct definition, `Problem` impl, and optionally `ConstraintSatisfactionProblem` impl.
2. **Register** in `src/models/<category>/mod.rs`.
3. **Add tests** in `src/unit_tests/models/<category>/<name>.rs` (linked via `#[path]`).
4. **Document** in `docs/paper/reductions.typ`: add `display-name` entry and `#problem-def("Name")[definition...]`.

## Categories

- `src/models/satisfiability/` — Satisfiability, KSatisfiability, CircuitSAT
- `src/models/graph/` — MaximumIndependentSet, MinimumVertexCover, KColoring, etc.
- `src/models/set/` — MinimumSetCovering, MaximumSetPacking
- `src/models/optimization/` — SpinGlass, QUBO, ILP
- `src/models/specialized/` — Factoring

## Naming

Use explicit optimization prefixes: `Maximum` for maximization, `Minimum` for minimization (e.g., `MaximumIndependentSet`, `MinimumVertexCover`).
