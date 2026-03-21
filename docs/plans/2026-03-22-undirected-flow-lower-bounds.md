# UndirectedFlowLowerBounds Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `UndirectedFlowLowerBounds` graph model, wire it into the registry/CLI/example-db/paper, and verify the canonical YES/NO instances from issue #294.

**Architecture:** Represent the problem as an undirected `SimpleGraph` plus per-edge capacities/lower bounds, source/sink terminals, and a required net flow. Use one binary configuration variable per edge to choose its orientation relative to the stored edge order; `evaluate()` then checks whether that orientation admits a feasible directed lower-bound flow with net sink inflow at least `R` by reducing to a feasible circulation instance and solving the resulting max-flow subproblem. Keep rule work out of scope for this PR; related rule issues already exist (`#367` inbound, `#735` outbound).

**Tech Stack:** Rust workspace (`problemreductions`, `problemreductions-cli`), serde/inventory registry metadata, existing brute-force solver, Typst paper, GitHub pipeline scripts.

---

## Batch 1: Model, Tests, Registry, CLI, Example DB

### Task 1: Add failing model tests first

**Files:**
- Create: `src/unit_tests/models/graph/undirected_flow_lower_bounds.rs`
- Reference: `src/unit_tests/models/graph/undirected_two_commodity_integral_flow.rs`
- Reference: `src/models/graph/undirected_two_commodity_integral_flow.rs`

**Step 1: Write the failing tests**

Cover these behaviors before production code exists:
- Constructor/accessors/dims shape for the issue YES instance
- `evaluate()` accepts the canonical YES orientation
- `evaluate()` rejects the canonical NO instance
- Serialization round-trip
- Brute-force solver finds a satisfying orientation on the YES instance and none on the NO instance
- Paper example test uses the same canonical YES instance

Use the issue’s fixed examples:
- YES graph edges: `(0,1) (0,2) (1,3) (2,3) (1,4) (3,5) (4,5)`
- YES lower bounds: `1,1,0,0,1,0,1`
- YES capacities: `2,2,2,2,1,3,2`
- YES source/sink/requirement: `0,5,3`
- YES orientation config: `[0,0,0,0,0,0,0]`
- NO graph edges: `(0,1) (0,2) (1,3) (2,3)`
- NO lower bounds: `2,2,1,1`
- NO capacities: `2,2,1,1`
- NO source/sink/requirement: `0,3,2`

**Step 2: Run the targeted test to verify RED**

Run:
```bash
cargo test undirected_flow_lower_bounds --lib
```

Expected:
- FAIL to compile because `UndirectedFlowLowerBounds` is not registered yet.

**Step 3: Commit the red test scaffold**

```bash
git add src/unit_tests/models/graph/undirected_flow_lower_bounds.rs
git commit -m "test: add UndirectedFlowLowerBounds model coverage"
```

### Task 2: Implement the model and binary-orientation feasibility check

**Files:**
- Create: `src/models/graph/undirected_flow_lower_bounds.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement the model skeleton**

Add `inventory::submit!` metadata plus:
- `ProblemSchemaEntry` with fields `graph`, `capacities`, `lower_bounds`, `source`, `sink`, `requirement`
- `ProblemSizeFieldEntry` for `num_vertices`, `num_edges`
- `#[derive(Debug, Clone, Serialize, Deserialize)]` struct using `SimpleGraph`
- Constructor validation:
  - capacities/lower-bounds lengths match `graph.num_edges()`
  - `source`/`sink` are in range
  - `requirement >= 1`
  - every `lower_bounds[i] <= capacities[i]`
- Accessors: `graph()`, `capacities()`, `lower_bounds()`, `source()`, `sink()`, `requirement()`, `num_vertices()`, `num_edges()`

**Step 2: Define the configuration space**

Implement:
- `dims() -> vec![2; num_edges]`
- `variant() -> crate::variant_params![]`
- `declare_variants! { default sat UndirectedFlowLowerBounds => "2^num_edges", }`

Interpret each config bit as:
- `0`: orient stored edge `(u,v)` as `u -> v`
- `1`: orient it as `v -> u`

Document this clearly in the model doc comment and in the paper/example text because it differs from the issue’s earlier “2|E| flow variables” draft.

**Step 3: Implement the feasibility algorithm**

Inside the model file:
- Convert the chosen orientation into a directed lower/upper-bound network
- Add a synthetic arc `sink -> source` with lower bound `requirement` and upper bound equal to the sum of edge capacities
- Reduce lower-bound feasibility to ordinary max-flow by:
  - replacing each edge capacity with `upper - lower`
  - computing vertex imbalances from the lower bounds
  - adding super-source/super-sink edges for positive/negative imbalance
- Implement a small local Edmonds-Karp max-flow helper (keep it private to the model file unless later reuse is obvious)
- Return `true` iff the max-flow saturates every edge from the super-source

Guardrails:
- Use `u128` / checked arithmetic for aggregate capacities and convert safely back to the max-flow scalar type used internally
- Treat wrong-length configs as `false`
- Do not depend on optional ILP features; the model must work in default and non-ILP builds

**Step 4: Register the model**

Wire the new type into:
- `src/models/graph/mod.rs` module list, re-export list, and `canonical_model_example_specs()` chain
- `src/models/mod.rs` graph re-export list
- `src/lib.rs` prelude exports

**Step 5: Run the targeted test to verify GREEN**

Run:
```bash
cargo test undirected_flow_lower_bounds --lib
```

Expected:
- PASS for the new model tests

**Step 6: Commit**

```bash
git add src/models/graph/undirected_flow_lower_bounds.rs src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/graph/undirected_flow_lower_bounds.rs
git commit -m "feat: add UndirectedFlowLowerBounds model"
```

### Task 3: Add CLI discovery and creation support

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`

**Step 1: Extend CLI flags and help text**

Add:
- `CreateArgs.lower_bounds: Option<String>` with `--lower-bounds`
- help-table line for `UndirectedFlowLowerBounds`
- example command using the issue YES instance
- `all_data_flags_empty()` coverage for `lower_bounds`

**Step 2: Add alias + parser support**

Update `resolve_alias()` with the lowercase pass-through mapping:
- `"undirectedflowlowerbounds" => "UndirectedFlowLowerBounds".to_string()`

Do not invent a short alias.

**Step 3: Add `pred create` support**

Add a `create()` arm using:
- `--graph`
- `--capacities`
- `--lower-bounds`
- `--source`
- `--sink`
- `--requirement`

Implement a small parsing helper mirroring `parse_capacities()` so:
- the list length matches `graph.num_edges()`
- every lower bound parses as `u64`
- constructor-level `lower <= capacity` validation remains the source of truth

**Step 4: Add focused CLI tests**

In `problemreductions-cli/src/commands/create.rs` tests, add:
- JSON creation test for the YES instance
- missing `--lower-bounds` usage error test

In `problemreductions-cli/src/problem_name.rs` tests, add:
- alias resolution pass-through test for `UndirectedFlowLowerBounds`

**Step 5: Run targeted CLI tests**

Run:
```bash
cargo test -p problemreductions-cli undirected_flow_lower_bounds
```

Expected:
- PASS for the new CLI-specific tests

**Step 6: Commit**

```bash
git add problemreductions-cli/src/problem_name.rs problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs
git commit -m "feat: add CLI support for UndirectedFlowLowerBounds"
```

## Batch 2: Paper Entry

### Task 4: Add canonical example and paper documentation

**Files:**
- Modify: `src/models/graph/undirected_flow_lower_bounds.rs`
- Modify: `docs/paper/reductions.typ`
- Reference: `docs/paper/reductions.typ` `problem-def("UndirectedTwoCommodityIntegralFlow")`

**Step 1: Add canonical model example metadata**

In the model file, add `canonical_model_example_specs()` gated by `#[cfg(feature = "example-db")]` using the issue YES instance:
- instance = the 6-vertex YES graph
- `optimal_config = vec![0, 0, 0, 0, 0, 0, 0]`
- `optimal_value = serde_json::json!(true)`

Keep the canonical example aligned with the paper example and the unit test.

**Step 2: Add the paper display name and `problem-def`**

Update `docs/paper/reductions.typ` with:
- display-name entry for `UndirectedFlowLowerBounds`
- a `problem-def("UndirectedFlowLowerBounds")` section

The paper entry should explain:
- the formal definition from issue #294 / Garey-Johnson ND37
- why the implementation uses edge orientations as the configuration space
- the `2^m` brute-force-over-orientations interpretation
- the canonical YES example, including one explicit feasible flow witness derived from the chosen orientation
- `pred-commands()` using `pred create --example UndirectedFlowLowerBounds`

Do not claim the ILP rule is implemented here; mention it as a natural outbound reduction already tracked separately.

**Step 3: Run paper verification**

Run:
```bash
make paper
```

Expected:
- PASS with no Typst errors

**Step 4: Re-run the paper example unit test**

Run:
```bash
cargo test test_undirected_flow_lower_bounds_paper_example --lib
```

Expected:
- PASS

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ src/models/graph/undirected_flow_lower_bounds.rs
git commit -m "docs: add UndirectedFlowLowerBounds paper entry"
```

## Final Verification

### Task 5: Run full verification and clean up

**Files:**
- Verify: workspace root

**Step 1: Run repository verification**

Run:
```bash
make test
make clippy
```

Expected:
- PASS for the full workspace

**Step 2: Inspect the tree**

Run:
```bash
git status --short
```

Expected:
- Clean tree, or only ignored/generated docs outputs

**Step 3: Final commit if needed**

If verification changed tracked files:
```bash
git add -A
git commit -m "chore: finalize UndirectedFlowLowerBounds verification fixes"
```

**Step 4: Push**

Run:
```bash
git push
```

Expected:
- Branch updated and ready for review pipeline
