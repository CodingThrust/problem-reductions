# Design: ILP Type Parameter for Variable Domain

## Motivation

The current `ILP` struct carries a `bounds: Vec<VarBounds>` field, but every reduction into ILP produces binary variables (`VarBounds::binary()`), except Factoring which uses bounded integers for carries. Meanwhile, ILP → QUBO requires binary variables and checks this with a runtime assert. The `bounds` field adds complexity without value for the common case, and makes the ILP → QUBO overhead expression inaccurate (slack bits depend on coefficient magnitudes, not expressible symbolically).

## Design

### Type Parameter

Replace `ILP` with `ILP<V>` where `V` determines the variable domain:

```rust
pub struct ILP<V> {
    pub num_vars: usize,
    pub constraints: Vec<LinearConstraint>,
    pub objective: Vec<(usize, f64)>,
    pub sense: ObjectiveSense,
    _marker: PhantomData<V>,
}
```

- `ILP<bool>`: Binary variables. `dims() = vec![2; n]`. Config `0 → 0`, `1 → 1`.
- `ILP<i32>`: Non-negative integers `0..2_147_483_647`. `dims() = vec![(i32::MAX as usize) + 1; n]`. Config index = value. Bounded ranges expressed as constraints (e.g., `x_i <= 5`).

The `bounds` field is removed entirely.

### Reduction Graph

Two variant nodes following the existing pattern (like `MIS {graph: "SimpleGraph"}` / `MIS {graph: "KingsSubgraph"}`):

- `ILP {variable: "bool"}` — binary integer linear programming
- `ILP {variable: "i32"}` — general integer linear programming

Natural cast edge: `ILP<bool>` → `ILP<i32>` (zero cost — every binary program is a valid integer program).

### Impact on Reductions

| Reduction | Before | After |
|---|---|---|
| MIS → ILP | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| MVC → ILP (if re-added) | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| MaxClique → ILP | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| MaxMatching → ILP | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| MinDS → ILP | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| MinSetCovering → ILP | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| KColoring → ILP | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| TSP → ILP | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| CircuitSAT → ILP | `ILP` with `VarBounds::binary()` | `ILP<bool>` |
| Factoring → ILP | `ILP` with mixed bounds | `ILP<i32>` with carry ranges as constraints |
| ILP → QUBO | Runtime assert binary | `impl ReduceTo<QUBO<f64>> for ILP<bool>` — compile-time guarantee |
| QUBO → ILP | Produces binary ILP | `ILP<bool>` |

### ILP → QUBO Overhead Fix

With `ILP<bool>`, all source variables are binary. Slack variables from inequality constraints have a worst-case count bounded by `num_constraints * ceil(log2(num_vars + 1))`. The overhead expression becomes:

```rust
#[reduction(overhead = {
    num_vars = "num_vars + num_constraints * num_vars",  // worst-case slack
})]
impl ReduceTo<QUBO<f64>> for ILP<bool> { ... }
```

This removes ILP → QUBO from the `UNTRUSTED_EDGES` list in `analysis.rs`.

### ILP Solver

The `ILPSolver` works with both `ILP<bool>` and `ILP<i32>`:
- `ILP<bool>`: HiGHS variables set as binary `[0, 1]`
- `ILP<i32>`: HiGHS variables set as integer `[0, i32::MAX]`, constraints provide effective bounds

No `VarBounds` needed in the solver interface.

### VarBounds

Removed from `ILP`. Retained for `ClosestVectorProblem<T>` which genuinely needs per-variable bounds for lattice enumeration.

### Constructors

- `ILP::<bool>::new(num_vars, constraints, objective, sense)` — replaces `ILP::binary()`
- `ILP::<i32>::new(num_vars, constraints, objective, sense)` — general integer
- `ILP::binary()` convenience removed (redundant with `ILP::<bool>::new`)

### variant() Implementation

```rust
impl<V: VariableDomain> Problem for ILP<V> {
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("variable", V::NAME)]
    }
}
```

Where `VariableDomain` is a sealed trait implemented for `bool` ("bool") and `i32` ("i32").

### Complexity

```rust
declare_variants! {
    ILP<bool> => "2^num_vars",
    ILP<i32> => "num_vars^num_vars",
}
```

Binary ILP is `O(2^n)` brute-force; general ILP is `O(n^n)` since each variable can take up to `n^n` combinations.
