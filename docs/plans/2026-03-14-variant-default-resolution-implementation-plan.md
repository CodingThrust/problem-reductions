# Variant Default Resolution Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement explicit default variants, exact node resolution across CLI and MCP, type-level `show`, bounded multi-path `path --all`, and target-aware direct reduction matching without broadening unrelated runtime variant support.

**Architecture:** Keep the existing type-level `Problem::variant()` API for problem definitions, but add a canonical runtime `VariantSpec` plus explicit `is_default` inventory metadata. Every node-level CLI/MCP problem spec should resolve through one shared resolver that starts from the registry default and applies slash-token updates; `show` stays type-level, and `path --all` becomes capped multi-path mode with explicit truncation reporting. Direct reduction-entry lookup should stop relying on name-only fallback and require an exact source+target variant match in this implementation pass.

**Tech Stack:** Rust 2021, `inventory`, `clap`, `serde_json`, `anyhow`, `petgraph`, `cargo test`

---

**Scope notes**

- This plan does **not** broaden `problemreductions-cli/src/dispatch.rs` runtime support for additional variants. Exact resolution may surface existing dispatch gaps more clearly; that is acceptable in this pass.
- Implement in a dedicated worktree. The current workspace already has unrelated local changes.
- Keep slash shorthand. Do not introduce keyword-style variant syntax.
- Treat `3SAT` / `KSAT` as node-level aliases only. Type-level `show 3SAT` should still work as a problem overview, so `show` needs a parser path that does **not** inject the implicit `K3` update.

## File Map

**Core variant metadata**

- Modify: `problemreductions-macros/src/lib.rs`
  Purpose: parse `default` in `declare_variants!`, validate exactly one default per problem, emit `is_default`, and add macro unit tests.
- Modify: `src/registry/variant.rs`
  Purpose: store explicit default metadata on each registered variant.
- Modify: `src/variant.rs`
  Purpose: add canonical runtime `VariantSpec` helpers and validation.
- Modify: `src/rules/graph.rs`
  Purpose: build/store default variants, add `default_variant_for`, keep `variants_for()` presentation-only, add capped path enumeration, and tighten direct reduction entry matching.
- Modify: `src/export.rs`
  Purpose: route variant conversion through canonical helpers and honor the target variant in direct-overhead lookup.

**Variant declaration sites**

- Modify: `src/models/algebraic/bmf.rs`
- Modify: `src/models/algebraic/closest_vector_problem.rs`
- Modify: `src/models/algebraic/ilp.rs`
- Modify: `src/models/algebraic/qubo.rs`
- Modify: `src/models/formula/circuit.rs`
- Modify: `src/models/formula/ksat.rs`
- Modify: `src/models/formula/sat.rs`
- Modify: `src/models/graph/biclique_cover.rs`
- Modify: `src/models/graph/graph_partitioning.rs`
- Modify: `src/models/graph/hamiltonian_path.rs`
- Modify: `src/models/graph/isomorphic_spanning_tree.rs`
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/models/graph/max_cut.rs`
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/models/graph/maximum_clique.rs`
- Modify: `src/models/graph/maximum_independent_set.rs`
- Modify: `src/models/graph/maximum_matching.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/models/graph/minimum_feedback_arc_set.rs`
- Modify: `src/models/graph/minimum_feedback_vertex_set.rs`
- Modify: `src/models/graph/minimum_sum_multicenter.rs`
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Modify: `src/models/graph/optimal_linear_arrangement.rs`
- Modify: `src/models/graph/partition_into_triangles.rs`
- Modify: `src/models/graph/rural_postman.rs`
- Modify: `src/models/graph/spin_glass.rs`
- Modify: `src/models/graph/subgraph_isomorphism.rs`
- Modify: `src/models/graph/traveling_salesman.rs`
- Modify: `src/models/misc/bin_packing.rs`
- Modify: `src/models/misc/factoring.rs`
- Modify: `src/models/misc/flow_shop_scheduling.rs`
- Modify: `src/models/misc/knapsack.rs`
- Modify: `src/models/misc/longest_common_subsequence.rs`
- Modify: `src/models/misc/paintshop.rs`
- Modify: `src/models/misc/shortest_common_supersequence.rs`
- Modify: `src/models/misc/subset_sum.rs`
- Modify: `src/models/set/maximum_set_packing.rs`
- Modify: `src/models/set/minimum_set_covering.rs`
  Purpose: mark one explicit default variant per problem, preserving the user-facing defaults you want (`SimpleGraph`-first, unweighted `One` where that is the desired CLI default, `KN` for generic-K families, sole variant otherwise).

**CLI and MCP resolution**

- Modify: `problemreductions-cli/src/problem_name.rs`
  Purpose: add one canonical node resolver and a separate type-level parser for `show`.
- Modify: `problemreductions-cli/src/commands/create.rs`
  Purpose: reuse shared exact resolution for normal creation and `--example`.
- Modify: `problemreductions-cli/src/commands/graph.rs`
  Purpose: make `show` type-level, annotate the default variant, make `to`/`from`/`path` exact-node operations, and apply `--max-paths`.
- Modify: `problemreductions-cli/src/commands/reduce.rs`
  Purpose: resolve bare `--to` as the exact default target node instead of searching all target variants.
- Modify: `problemreductions-cli/src/cli.rs`
  Purpose: add `--max-paths`, update help text, and stop describing `--all` as exhaustive.
- Modify: `problemreductions-cli/src/main.rs`
  Purpose: thread `max_paths` through command dispatch.
- Modify: `problemreductions-cli/src/mcp/tools.rs`
  Purpose: mirror the same resolver semantics and capped multi-path behavior in MCP.

**Tests and docs**

- Modify: `src/unit_tests/variant.rs`
- Modify: `src/unit_tests/reduction_graph.rs`
- Modify: `src/unit_tests/export.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`
- Modify: `problemreductions-cli/src/mcp/tests.rs`
- Modify: `docs/src/cli.md`
  Purpose: lock in the new semantics and prevent future doc drift.

## Chunk 1: Core Variant Metadata And Graph Defaults

### Task 1: Add failing tests for default metadata and canonical variants

**Files:**

- Modify: `problemreductions-macros/src/lib.rs`
- Modify: `src/unit_tests/variant.rs`
- Modify: `src/unit_tests/reduction_graph.rs`
- Modify: `src/unit_tests/export.rs`

- [ ] **Step 1: Add macro-unit tests for `declare_variants!` default validation**

Add `#[cfg(test)] mod tests` in `problemreductions-macros/src/lib.rs` that exercises the parser/codegen helpers directly instead of building a separate compile-fail harness. Cover:

```rust
#[test]
fn declare_variants_accepts_single_default() {
    let input: DeclareVariantsInput = syn::parse_quote! {
        default Foo => "1",
    };
    assert!(generate_declare_variants(&input).is_ok());
}

#[test]
fn declare_variants_requires_one_default_per_problem() {
    let input: DeclareVariantsInput = syn::parse_quote! {
        Foo => "1",
        Bar => "1",
    };
    let err = generate_declare_variants(&input).unwrap_err();
    assert!(err.to_string().contains("exactly one default"));
}

#[test]
fn declare_variants_rejects_multiple_defaults_for_one_problem() {
    let input: DeclareVariantsInput = syn::parse_quote! {
        default Foo => "1",
        default Foo => "2",
    };
    let err = generate_declare_variants(&input).unwrap_err();
    assert!(err.to_string().contains("more than one default"));
}

#[test]
fn declare_variants_still_validates_complexity_with_default() {
    let input: DeclareVariantsInput = syn::parse_quote! {
        default Foo => "bad(getter)",
    };
    let err = generate_declare_variants(&input).unwrap_err();
    assert!(err.to_string().contains("invalid complexity expression"));
}
```

- [ ] **Step 2: Add failing runtime tests for `VariantSpec`, export normalization, and graph defaults**

Extend `src/unit_tests/variant.rs`, `src/unit_tests/export.rs`, and `src/unit_tests/reduction_graph.rs` with tests that expect:

```rust
#[test]
fn variant_spec_rejects_duplicate_dimensions() {
    let err = VariantSpec::try_from_pairs([
        ("graph", "SimpleGraph"),
        ("graph", "UnitDiskGraph"),
    ]).unwrap_err();
    assert!(err.to_string().contains("graph"));
}

#[test]
fn default_variant_for_mis_uses_declared_default() {
    let graph = ReductionGraph::new();
    let default_variant = graph.default_variant_for("MaximumIndependentSet").unwrap();
    assert_eq!(default_variant.as_map().get("graph"), Some(&"SimpleGraph".to_string()));
}

#[test]
fn variant_spec_normalizes_empty_graph_to_simple_graph() {
    let spec = VariantSpec::try_from_pairs([("graph", ""), ("weight", "One")]).unwrap();
    assert_eq!(spec.as_map().get("graph"), Some(&"SimpleGraph".to_string()));
}

#[test]
fn export_variant_to_map_normalizes_empty_graph() {
    let map = crate::export::variant_to_map(vec![("graph", ""), ("weight", "One")]);
    assert_eq!(map.get("graph"), Some(&"SimpleGraph".to_string()));
}
```

- [ ] **Step 3: Run the new tests and confirm they fail**

Run: `cargo test -p problemreductions-macros declare_variants_ -- --nocapture`
Expected: FAIL because `default` is not parsed or validated yet.

Run: `cargo test variant_spec_rejects_duplicate_dimensions -- --nocapture`
Expected: FAIL because `VariantSpec` does not exist yet.

Run: `cargo test variant_spec_normalizes_empty_graph_to_simple_graph -- --nocapture`
Expected: FAIL because `VariantSpec` does not exist yet.

Run: `cargo test export_variant_to_map_normalizes_empty_graph -- --nocapture`
Expected: FAIL because export normalization has not been routed through canonical helpers yet.

Run: `cargo test default_variant_for_mis_uses_declared_default -- --nocapture`
Expected: FAIL because `default_variant_for()` does not exist yet.

- [ ] **Step 4: Commit the red tests**

```bash
git add problemreductions-macros/src/lib.rs src/unit_tests/variant.rs src/unit_tests/reduction_graph.rs src/unit_tests/export.rs
git commit -m "test: cover variant default metadata"
```

### Task 2: Implement `default` metadata and `VariantSpec`

**Files:**

- Modify: `problemreductions-macros/src/lib.rs`
- Modify: `src/registry/variant.rs`
- Modify: `src/variant.rs`
- Modify: `src/rules/graph.rs`
- Modify: `src/export.rs`

- [ ] **Step 1: Extend `declare_variants!` parsing and generated inventory**

Update `DeclareVariantEntry` to hold `is_default: bool`, accept an optional `default` keyword before the type, and validate counts per problem name before code generation. Generate:

```rust
crate::registry::VariantEntry {
    name: <#ty as crate::traits::Problem>::NAME,
    variant_fn: || <#ty as crate::traits::Problem>::variant(),
    complexity: #complexity_str,
    complexity_eval_fn: #complexity_eval_fn,
    is_default: #is_default,
}
```

- [ ] **Step 2: Add canonical runtime variant helpers**

Implement `VariantSpec` in `src/variant.rs` as the only validated runtime form:

```rust
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariantSpec {
    dims: BTreeMap<String, String>,
}
```

Required helpers:

- `try_from_pairs`
- `try_from_map`
- `as_map`
- `into_map`
- `update_dimension`
- normalization of empty `graph` values to `"SimpleGraph"`

Use these helpers from `src/rules/graph.rs` and `src/export.rs` instead of ad hoc `collect()` calls.
Keep the type namespaced as `problemreductions::variant::VariantSpec`; do not add a new top-level `pub use`.

- [ ] **Step 3: Store and expose explicit defaults in `ReductionGraph`**

Add a `default_variants` lookup to `ReductionGraph`, populate it from `VariantEntry::is_default`, and add:

```rust
pub fn default_variant_for(&self, name: &str) -> Option<VariantSpec>;
```

Keep `variants_for()` for display only. It may still order the default first for convenience, but all semantic call sites must use `default_variant_for()`.

- [ ] **Step 4: Run the targeted tests and confirm they pass**

Run: `cargo test -p problemreductions-macros declare_variants_ -- --nocapture`
Expected: PASS.

Run: `cargo test variant_spec_rejects_duplicate_dimensions -- --nocapture`
Expected: PASS.

Run: `cargo test variant_spec_normalizes_empty_graph_to_simple_graph -- --nocapture`
Expected: PASS.

Run: `cargo test export_variant_to_map_normalizes_empty_graph -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit the core metadata implementation**

```bash
git add problemreductions-macros/src/lib.rs src/registry/variant.rs src/variant.rs src/rules/graph.rs src/export.rs
git commit -m "feat: add explicit variant defaults"
```

### Task 3: Mark defaults in every declared problem variant set

**Files:**

- Modify: `src/models/algebraic/bmf.rs`
- Modify: `src/models/algebraic/closest_vector_problem.rs`
- Modify: `src/models/algebraic/ilp.rs`
- Modify: `src/models/algebraic/qubo.rs`
- Modify: `src/models/formula/circuit.rs`
- Modify: `src/models/formula/ksat.rs`
- Modify: `src/models/formula/sat.rs`
- Modify: `src/models/graph/biclique_cover.rs`
- Modify: `src/models/graph/graph_partitioning.rs`
- Modify: `src/models/graph/hamiltonian_path.rs`
- Modify: `src/models/graph/isomorphic_spanning_tree.rs`
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/models/graph/max_cut.rs`
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/models/graph/maximum_clique.rs`
- Modify: `src/models/graph/maximum_independent_set.rs`
- Modify: `src/models/graph/maximum_matching.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/models/graph/minimum_feedback_arc_set.rs`
- Modify: `src/models/graph/minimum_feedback_vertex_set.rs`
- Modify: `src/models/graph/minimum_sum_multicenter.rs`
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Modify: `src/models/graph/optimal_linear_arrangement.rs`
- Modify: `src/models/graph/partition_into_triangles.rs`
- Modify: `src/models/graph/rural_postman.rs`
- Modify: `src/models/graph/spin_glass.rs`
- Modify: `src/models/graph/subgraph_isomorphism.rs`
- Modify: `src/models/graph/traveling_salesman.rs`
- Modify: `src/models/misc/bin_packing.rs`
- Modify: `src/models/misc/factoring.rs`
- Modify: `src/models/misc/flow_shop_scheduling.rs`
- Modify: `src/models/misc/knapsack.rs`
- Modify: `src/models/misc/longest_common_subsequence.rs`
- Modify: `src/models/misc/paintshop.rs`
- Modify: `src/models/misc/shortest_common_supersequence.rs`
- Modify: `src/models/misc/subset_sum.rs`
- Modify: `src/models/set/maximum_set_packing.rs`
- Modify: `src/models/set/minimum_set_covering.rs`
- Modify: `src/unit_tests/reduction_graph.rs`

- [ ] **Step 1: Add one `default` marker to every `declare_variants!` block**

Choose defaults intentionally, not by prior sort order:

- graph families: prefer `SimpleGraph` when available
- weighted/unweighted pairs: prefer `One` where the bare CLI should act unweighted by default
- `K`-families: prefer `KN` as the generic default
- integer-vs-float families without `One`: prefer the currently established integer variant (`ILP<bool>`, `ClosestVectorProblem<i32>`, `BinPacking<i32>`, `SpinGlass<SimpleGraph, i32>`)
- single-variant problems: mark the only variant as `default`

For example:

```rust
crate::declare_variants! {
    default MaximumIndependentSet<SimpleGraph, One> => "1.1996^num_vertices",
    MaximumIndependentSet<SimpleGraph, i32> => "1.1996^num_vertices",
    // ...
}
```

- [ ] **Step 2: Add regression tests that assert the chosen defaults**

In `src/unit_tests/reduction_graph.rs`, replace the existing ordering-based `variants()[0]` default assertions with explicit `default_variant_for(...)` assertions for the problem families the CLI relies on most:

- `MaximumIndependentSet`
- `MinimumVertexCover`
- `QUBO`
- `KSatisfiability`

Do not keep tests that infer default semantics from `variants()[0]` alone.

- [ ] **Step 3: Run the affected graph tests**

Run: `cargo test reduction_graph:: -- --nocapture`
Expected: PASS with explicit default lookups.

Run: `cargo test default_variant_for_mis_uses_declared_default -- --nocapture`
Expected: PASS once the declarations are marked.

- [ ] **Step 4: Commit the declaration updates**

```bash
git add src/models src/unit_tests/reduction_graph.rs
git commit -m "feat: mark default problem variants"
```

## Chunk 2: Shared Resolver And CLI/MCP Semantics

### Task 4: Add failing resolver and command-semantic tests

**Files:**

- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`
- Modify: `problemreductions-cli/src/mcp/tests.rs`
- Modify: `src/unit_tests/reduction_graph.rs`

- [ ] **Step 1: Add unit tests for the new resolver contract**

In `problemreductions-cli/src/problem_name.rs`, add tests for:

- bare `MIS` resolves to the declared default full variant
- `MIS/UnitDiskGraph` updates only the `graph` dimension
- `MIS/One/i32` errors as duplicate dimension updates
- ambiguous token errors mention the colliding dimensions
- invalid final combinations error after updates are applied
- type-level parsing of `show 3SAT` resolves to `KSatisfiability` **without** injecting `K3`

Use a real `ReductionGraph::new()` in these tests so they follow registered metadata.

- [ ] **Step 2: Add CLI and MCP regression tests for the user-visible behavior**

In `problemreductions-cli/tests/cli_tests.rs` and `problemreductions-cli/src/mcp/tests.rs`, add failing coverage for:

- `pred show MIS` includes `(default)` beside the default variant
- `pred show MIS/UnitDiskGraph` errors because `show` is type-level
- `pred show 3SAT` succeeds as a type overview
- `pred create MIS` uses the declared default MIS node
- `pred to MIS` and `pred from MIS` use the declared default MIS node
- `pred path MIS QUBO` uses exact default nodes
- `pred path MIS QUBO --all --max-paths 5` truncates and prints a truncation note
- `pred path MIS QUBO --all` returns at most 20 paths by default
- `pred reduce <problem.json> --to QUBO` targets the declared default QUBO node
- `pred reduce <problem.json> --via path.json --to <spec>` rejects mismatched target variants
- MCP `show_problem_inner("MIS/UnitDiskGraph")` errors
- MCP `neighbors_inner("MIS", 1, "out")` uses the declared default MIS node
- MCP `create_problem_inner("MIS", ...)` uses the declared default MIS node
- MCP `reduce_inner(..., "QUBO")` uses the declared default QUBO node
- MCP `find_path_inner("MIS", "QUBO", ..., true)` returns a structured capped response
- `find_paths_up_to(..., limit)` returns at most `limit + 1` paths so truncation can be detected without full enumeration

For `pred create --example MIS`, add an assertion that the command no longer asks for an explicit variant. If the chosen default example does not exist, assert the resolved-node error instead of expecting success.

- [ ] **Step 3: Run the new tests and confirm they fail**

Run: `cargo test -p problemreductions-cli problem_name::tests -- --nocapture`
Expected: FAIL because the shared resolver does not exist yet.

Run: `cargo test -p problemreductions-cli --test cli_tests test_show_rejects_slash_spec -- --nocapture`
Expected: FAIL because `show` still accepts slash specs silently.

Run: `cargo test -p problemreductions-cli --test cli_tests test_path_all_max_paths_truncates -- --nocapture`
Expected: FAIL because `path --all` still enumerates without a cap or truncation note.

Run: `cargo test find_paths_up_to_stops_after_limit_plus_one -- --nocapture`
Expected: FAIL because the capped graph helper does not exist yet.

Run: `cargo test -p problemreductions-cli test_show_problem_rejects_slash_spec -- --nocapture`
Expected: FAIL for the same semantic reasons in MCP.

- [ ] **Step 4: Commit the red resolver tests**

```bash
git add problemreductions-cli/src/problem_name.rs problemreductions-cli/tests/cli_tests.rs problemreductions-cli/src/mcp/tests.rs
git commit -m "test: cover exact variant resolution semantics"
```

### Task 5: Implement one canonical node resolver and adopt it in CLI commands

**Files:**

- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/commands/graph.rs`
- Modify: `problemreductions-cli/src/commands/reduce.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/main.rs`
- Modify: `src/rules/graph.rs`

- [ ] **Step 1: Introduce two parsing paths in `problem_name.rs`**

Keep `parse_problem_spec()` for node-level commands, but add:

- a type-level parser for `show` that resolves aliases and rejects slash suffixes
- a shared exact resolver that returns a fully resolved `ProblemRef`

Recommended shape:

```rust
pub fn parse_problem_type(input: &str) -> anyhow::Result<String>;
pub fn resolve_problem_ref(
    input: &str,
    graph: &ReductionGraph,
) -> anyhow::Result<ProblemRef>;
```

`resolve_problem_ref()` should:

1. resolve alias
2. load `default_variant_for()`
3. build a per-dimension token index from declared variants
4. apply slash-token updates
5. reject unknown/ambiguous/duplicate tokens
6. reject final combinations that are not declared

- [ ] **Step 2: Switch CLI commands to exact-node semantics**

Apply the shared resolver in:

- `create`
- `create --example`
- `to`
- `from`
- `path`
- `reduce --to`

Important command-specific rules:

- `show` must use `parse_problem_type()` instead of the node resolver
- `show MIS/UnitDiskGraph` must error
- `show MIS` should annotate the default variant in its variant list
- `path MIS QUBO` must search only default-to-default
- `reduce --to QUBO` must target the default `QUBO` node, not scan all `QUBO` variants

- [ ] **Step 3: Add capped multi-path support**

In `problemreductions-cli/src/cli.rs`, add:

```rust
#[arg(long, default_value_t = 20)]
max_paths: usize,
```

In `problemreductions-cli/src/main.rs` and `problemreductions-cli/src/commands/graph.rs`, thread `max_paths` into `path()`. In `src/rules/graph.rs`, add a helper that stops after `max_paths + 1` paths so the CLI can detect truncation without enumerating the entire graph:

```rust
pub fn find_paths_up_to(
    &self,
    source: &str,
    source_variant: &VariantSpec,
    target: &str,
    target_variant: &VariantSpec,
    limit: usize,
) -> Vec<ReductionPath>;
```

CLI behavior:

- `pred path A B` => one cheapest path
- `pred path A B --all` => up to `max_paths`
- if more exist, succeed and print a truncation note

For non-text outputs, use structured metadata instead of a bare array:

```json
{
  "paths": [],
  "truncated": false,
  "returned": 0,
  "max_paths": 20
}
```

If `-o <dir>` is used, keep per-path files and write a `manifest.json` with the same metadata plus the generated filenames.

- [ ] **Step 4: Run the targeted CLI tests and confirm they pass**

Run: `cargo test -p problemreductions-cli problem_name::tests -- --nocapture`
Expected: PASS.

Run: `cargo test -p problemreductions-cli --test cli_tests test_show_rejects_slash_spec -- --nocapture`
Expected: PASS.

Run: `cargo test -p problemreductions-cli --test cli_tests test_path_all_max_paths_truncates -- --nocapture`
Expected: PASS.

Run: `cargo test find_paths_up_to_stops_after_limit_plus_one -- --nocapture`
Expected: PASS.

Run: `cargo test -p problemreductions-cli --test cli_tests test_reduce_uses_default_target_variant -- --nocapture`
Expected: PASS.

- [ ] **Step 5: Commit the CLI resolver conversion**

```bash
git add problemreductions-cli/src/problem_name.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/src/commands/graph.rs problemreductions-cli/src/commands/reduce.rs problemreductions-cli/src/cli.rs problemreductions-cli/src/main.rs src/rules/graph.rs
git commit -m "feat: unify CLI problem resolution"
```

### Task 6: Mirror the same semantics in MCP and docs

**Files:**

- Modify: `problemreductions-cli/src/mcp/tools.rs`
- Modify: `problemreductions-cli/src/mcp/tests.rs`
- Modify: `docs/src/cli.md`
- Modify: `problemreductions-cli/src/cli.rs`

- [ ] **Step 1: Reuse the same resolver helpers in MCP**

`McpServer` should not keep its own resolution rules. Apply the exact same node/type split as the CLI:

- `show_problem_inner()` is type-level
- `neighbors_inner()`, `find_path_inner()`, `create_problem_inner()`, and `reduce_inner()` are node-level
- multi-path mode should return at most `max_paths` results and expose truncation in JSON
- add an optional `max_paths` input to the MCP path tool schema/handler, defaulting to `20`

Use one explicit JSON shape for MCP and CLI `--json` multi-path responses:

```json
{
  "paths": [],
  "truncated": false,
  "returned": 0,
  "max_paths": 20
}
```

Prefer a small private formatter/helper for this response instead of adding more branching inline to `mcp/tools.rs`.

- [ ] **Step 2: Update help text and user docs**

In `problemreductions-cli/src/cli.rs` and `docs/src/cli.md`, change wording from “all paths” to “multiple paths” / “up to N paths”, document `--max-paths`, and document `show` as type-level with default annotation. Include examples like:

```bash
pred show MIS
pred path MIS QUBO --all
pred path MIS QUBO --all --max-paths 100
```

- [ ] **Step 3: Run the MCP and doc-adjacent tests**

Run: `cargo test -p problemreductions-cli test_show_problem_rejects_slash_spec -- --nocapture`
Expected: PASS.

Run: `cargo test -p problemreductions-cli test_find_path_all_max_paths_structured_response -- --nocapture`
Expected: PASS.

Run: `cargo test -p problemreductions-cli --test cli_tests test_help -- --nocapture`
Expected: PASS with updated help and output text.

- [ ] **Step 4: Commit the MCP and docs sync**

```bash
git add problemreductions-cli/src/mcp/tools.rs problemreductions-cli/src/mcp/tests.rs problemreductions-cli/src/cli.rs docs/src/cli.md
git commit -m "docs: align CLI and MCP variant semantics"
```

## Chunk 3: Direct Reduction Matching And Final Verification

### Task 7: Add failing tests for exact target-aware direct reduction lookup

**Files:**

- Modify: `src/unit_tests/export.rs`
- Modify: `src/unit_tests/reduction_graph.rs`

- [ ] **Step 1: Add regression tests that expose the current fallback bug**

Add tests that prove the target variant matters. Use one export-level regression and one graph-level regression with concrete assertions:

```rust
#[test]
fn lookup_overhead_rejects_target_variant_mismatch() {
    let source = BTreeMap::from([("weight".to_string(), "f64".to_string())]);
    let wrong_target = BTreeMap::from([("weight".to_string(), "i32".to_string())]);
    let result = lookup_overhead(
        "MaximumSetPacking",
        &source,
        "QUBO",
        &wrong_target,
    );
    assert!(result.is_none());
}
```

- [ ] **Step 2: Run the focused tests and confirm they fail**

Run: `cargo test lookup_overhead_rejects_target_variant_mismatch -- --nocapture`
Expected: FAIL because the current implementation ignores the target variant.

- [ ] **Step 3: Commit the red matching tests**

```bash
git add src/unit_tests/export.rs src/unit_tests/reduction_graph.rs
git commit -m "test: cover exact reduction entry lookup"
```

### Task 8: Implement exact direct matching and keep example/export callers working

**Files:**

- Modify: `src/rules/graph.rs`
- Modify: `src/export.rs`
- Inspect only if needed: `src/example_db/rule_builders.rs`

- [ ] **Step 1: Tighten `find_best_entry()` to exact source+target matching**

Change the signature so the caller passes both variants explicitly:

```rust
pub fn find_best_entry(
    &self,
    source_name: &str,
    source_variant: &BTreeMap<String, String>,
    target_name: &str,
    target_variant: &BTreeMap<String, String>,
) -> Option<MatchedEntry>;
```

For this implementation pass, use the simplest safe rule:

1. exact source variant match
2. exact target variant match
3. otherwise `None`

Do **not** keep the current name-only fallback. If a later hierarchy-aware generalization is needed, it should be added explicitly in a follow-up change, not silently preserved here.

- [ ] **Step 2: Honor the target variant in `lookup_overhead()`**

Change `lookup_overhead()` to pass both source and target variants through and normalize via `VariantSpec`/map helpers. Any caller that asks for a nonexistent direct edge should now get `None`.

- [ ] **Step 3: Add exact-match graph tests against the new signature**

Once `find_best_entry()` accepts both variants, add both a mismatch regression and a positive exact-match regression in `src/unit_tests/reduction_graph.rs` using `BTreeMap::from([...])`:

```rust
#[test]
fn find_best_entry_rejects_wrong_target_variant() { /* expect None */ }

#[test]
fn find_best_entry_accepts_exact_source_and_target_variant() { /* expect Some */ }
```

- [ ] **Step 4: Run export and graph unit tests**

Run: `cargo test lookup_overhead_rejects_target_variant_mismatch -- --nocapture`
Expected: PASS.

Run: `cargo test find_best_entry_rejects_wrong_target_variant -- --nocapture`
Expected: PASS.

Run: `cargo test find_best_entry_accepts_exact_source_and_target_variant -- --nocapture`
Expected: PASS.

If any example-db code fails because it depended on the unsafe fallback, stop and inspect `src/example_db/rule_builders.rs` in the worktree. Prefer adding the missing exact declaration or updating the test expectation; do not reintroduce the name-only fallback.

- [ ] **Step 5: Commit the matching cleanup**

```bash
git add src/rules/graph.rs src/export.rs src/unit_tests/export.rs src/unit_tests/reduction_graph.rs src/example_db/rule_builders.rs
git commit -m "fix: require exact reduction entry matches"
```

### Task 9: Run the full verification matrix and prepare the branch for execution handoff

**Files:**

- Modify if needed after failures: any file changed in previous tasks

- [ ] **Step 1: Run the focused crate test suites**

Run: `cargo test -p problemreductions-macros -- --nocapture`
Expected: PASS.

Run: `cargo test --lib -- --nocapture`
Expected: PASS.

Run: `cargo test -p problemreductions-cli -- --nocapture`
Expected: PASS.

- [ ] **Step 2: Run targeted high-signal commands manually**

Run: `cargo run -p problemreductions-cli --bin pred -- show MIS`
Expected: output lists variants and marks one as `(default)`.

Run: `cargo run -p problemreductions-cli --bin pred -- show MIS/UnitDiskGraph`
Expected: non-zero exit with a type-level `show` error.

Run: `cargo run -p problemreductions-cli --bin pred -- path MIS QUBO --all --max-paths 5`
Expected: success, 5 paths max, and a truncation note if more exist.

Run: `cargo run -p problemreductions-cli --bin pred -- create --example MIS`
Expected: resolved-default behavior; either a canonical example or a clear resolved-node error, but never “explicit variant required”.

- [ ] **Step 3: Update any stale docs/tests surfaced by verification**

Keep changes narrowly scoped to semantics introduced in this plan. Do not broaden unrelated CLI wording or dispatch support.

- [ ] **Step 4: Make the final verification commit**

```bash
git add problemreductions-macros/src/lib.rs src/registry/variant.rs src/variant.rs src/rules/graph.rs src/export.rs src/models problemreductions-cli/src problemreductions-cli/tests/cli_tests.rs problemreductions-cli/src/mcp/tests.rs src/unit_tests docs/src/cli.md src/example_db/rule_builders.rs
git commit -m "feat: implement explicit variant defaults"
```

- [ ] **Step 5: Record verification evidence in the handoff note**

Capture the exact commands run and their exit status in the final handoff or PR description so the next worker does not have to guess what was already verified.
