# Plan: MinimumMultiwayCut to ILP Reduction (#185)

## Overview

Add a reduction from `MinimumMultiwayCut<SimpleGraph, i32>` to `ILP<bool>` using the standard vertex-assignment + edge-cut indicator ILP formulation from Chopra & Owen (1996).

## Reduction Algorithm

**Variables** (kn + m binary):
- `y_{iv}` at index `i*n + v` for i in 0..k, v in 0..n: vertex v assigned to component of terminal t_i
- `x_e` at index `kn + e_idx` for each edge: 1 if edge is in the cut

**Bounds (terminal fixing):**
- `y_{i, t_i} = 1` (terminal t_i is in its own component)
- `y_{j, t_i} = 0` for j != i

**Constraints:**
1. Partition (n equality): `sum_i y_{iv} = 1` for each vertex v
2. Edge-cut linking (2km inequality): `x_e >= y_{iu} - y_{iv}` and `x_e >= y_{iv} - y_{iu}` for each edge (u,v), each terminal i

**Objective:** minimize `sum_e w_e * x_e`

**Solution extraction:** For each edge e, source config[e] = target_solution[kn + e_idx] (the x_e value).

## Steps

### Batch 1: Implementation + Tests + Example-DB

#### Step 1: Implement reduction (`src/rules/minimummultiwaycut_ilp.rs`)

Follow `binpacking_ilp.rs` pattern:

```rust
// ReductionMMCToILP struct with fields: target: ILP<bool>, n: usize, m: usize, k: usize
// ReductionResult impl: extract_solution reads x_e values at indices kn..kn+m
// #[reduction(overhead = {
//     num_vars = "num_terminals * num_vertices + num_edges",
//     num_constraints = "num_vertices + 2 * num_terminals * num_edges",
// })]
// impl ReduceTo<ILP<bool>> for MinimumMultiwayCut<SimpleGraph, i32>
```

Key implementation details:
- Use `LinearConstraint::eq` for partition constraints
- Use `LinearConstraint::ge` for edge-cut linking (rearranged: `x_e - y_{iu} + y_{iv} >= 0`)
- Terminal fixing: set bounds via additional equality constraints `y_{i,t_i} = 1` and `y_{j,t_i} = 0`
  - Alternative: use `LinearConstraint::eq` with single term to fix variables (simpler than custom bounds)
- Objective: sparse vec of `(kn + e_idx, w_e as f64)` for each edge

#### Step 2: Register in `src/rules/mod.rs`

Add under the `#[cfg(feature = "ilp-solver")]` block:
```rust
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimummultiwaycut_ilp;
```

#### Step 3: Write unit tests (`src/unit_tests/rules/minimummultiwaycut_ilp.rs`)

Tests (following `binpacking_ilp.rs` pattern):
1. `test_reduction_creates_valid_ilp` — check num_vars = kn+m, num_constraints = n+2km + terminal fixes, sense = Minimize
2. `test_minimummultiwaycut_to_ilp_closed_loop` — use the canonical 5-vertex example, compare brute-force vs ILP solver
3. `test_triangle_with_3_terminals` — triangle graph, all vertices are terminals, optimal = cut 2 cheapest edges
4. `test_solution_extraction` — manually construct ILP solution, verify extracted config
5. `test_solve_reduced` — use `ILPSolver.solve_reduced(&problem)`
6. `test_two_terminals` — degenerate case with k=2 (min s-t cut)

#### Step 4: Add canonical example (`src/example_db/rule_builders.rs`)

Add `canonical_rule_example_specs()` in the reduction file, using the issue's 5-vertex example:
```rust
MinimumMultiwayCut::new(
    SimpleGraph::new(5, vec![(0,1),(1,2),(2,3),(3,4),(0,4),(1,3)]),
    vec![0, 2, 4],
    vec![2, 3, 1, 2, 4, 5],
)
```
Register in `src/rules/mod.rs` `canonical_rule_example_specs()` function under the `#[cfg(feature = "ilp-solver")]` block.

### Batch 2: Paper Documentation

#### Step 5: Document in `docs/paper/reductions.typ`

Add `reduction-rule("MinimumMultiwayCut", "ILP", example: true, ...)` entry. Include:
- Theorem statement: the ILP formulation with variable/constraint counts
- Proof: construction details (variable mapping, constraints, terminal fixing, objective), correctness argument, solution extraction
- Worked example from the canonical fixture JSON

### Batch 3: Export + Verify

#### Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make test
make clippy
make paper
```
