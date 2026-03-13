# Plan: MinimumFeedbackVertexSet to ILP Reduction

**Issue:** #141
**Type:** Rule
**Source:** `MinimumFeedbackVertexSet<i32>`
**Target:** `ILP<i32>` (mixed binary + integer variables)

## Reduction Algorithm

MTZ-style topological ordering formulation:
- **Variables:** n binary x_i (vertex removal) + n integer o_i in {0,...,n-1} (topological order) = 2n total
- **Constraints:** For each arc (u->v): o_v - o_u >= 1 - n*(x_u + x_v). Plus binary bounds (x_i <= 1) and order bounds (o_i <= n-1) = m + 2n total
- **Objective:** minimize sum w_i * x_i
- **Solution extraction:** first n variables of ILP solution are the x_i values

## Steps

### Step 1: Implement the reduction rule
- Create `src/rules/minimumfeedbackvertexset_ilp.rs`
- ReductionResult struct: `ReductionMFVSToILP` with target `ILP<i32>` and `num_vertices: usize`
- `reduce_to()`: construct 2n variables, m arc constraints + n binary constraints + n order bound constraints
- `extract_solution()`: take first n values from ILP solution
- Overhead: `num_vars = "2 * num_vertices"`, `num_constraints = "num_arcs + 2 * num_vertices"`
- Feature-gate under `#[cfg(feature = "ilp-solver")]`

### Step 2: Register in mod.rs
- Add `#[cfg(feature = "ilp-solver")] mod minimumfeedbackvertexset_ilp;` to `src/rules/mod.rs`

### Step 3: Write unit tests
- Create `src/unit_tests/rules/minimumfeedbackvertexset_ilp.rs`
- `test_minimumfeedbackvertexset_to_ilp_closed_loop`: 3-cycle, verify BF vs ILP
- `test_reduction_structure`: verify num_vars = 2n, num_constraints = m + 2n
- `test_cycle_of_triangles`: the example from the issue (n=9, m=15, FVS=3)
- `test_dag_no_removal`: DAG input requires FVS=0
- `test_single_vertex`: trivial case
- `test_weighted`: verify weight transfer to objective

### Step 4: Write example program
- Create `examples/reduction_minimumfeedbackvertexset_to_ilp.rs`
- Use the cycle-of-triangles instance from the issue
- Register in `tests/suites/examples.rs`

### Step 5: Document in paper
- Add `reduction-rule` entry in `docs/paper/reductions.typ`

### Step 6: Regenerate exports and verify
- `cargo run --example export_graph`
- `cargo run --example export_schemas`
- `make test clippy`
