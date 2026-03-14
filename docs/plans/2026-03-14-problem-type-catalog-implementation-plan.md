# Problem Type Catalog Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Introduce a catalog-backed problem type system, typed internal problem refs, exact endpoint-based rule identity, and per-module example declarations so extending the repo requires fewer parallel metadata edits.

**Architecture:** Reuse the repo's existing local registration seams instead of inventing a second metadata world. Extend model-local schema registrations to carry aliases and variant dimensions, add typed runtime refs on top of that catalog, treat exact `(source_ref, target_ref)` pairs as primitive reduction identity, then move canonical examples from giant central builder lists into explicit per-module collectors. Remove `Problem::NAME` only after all runtime call sites use the catalog bridge.

**Tech Stack:** Rust, `inventory`, proc macros in `problemreductions-macros`, `serde`, `clap`, `cargo test`

---

## File Structure

The implementation should keep responsibilities narrow:

- Create `src/registry/problem_type.rs`
  Responsibility: runtime catalog APIs, alias lookup, dimension validation, schema-vs-reachability helpers.
- Create `src/registry/problem_ref.rs`
  Responsibility: typed internal `ProblemRef` and `VariantSpec`, plus conversions to and from export DTOs.
- Create `src/example_db/specs.rs`
  Responsibility: `ModelExampleSpec` / `RuleExampleSpec` types and shared assembly helpers.
- Create `src/unit_tests/registry/problem_type.rs`
  Responsibility: catalog validation and typed-ref unit tests.
- Modify `src/registry/schema.rs`
  Responsibility: extend model-local schema registrations with aliases and declared variant dimensions.
- Modify `src/registry/mod.rs`
  Responsibility: re-export new catalog APIs.
- Modify `src/traits.rs`
  Responsibility: add the bridge from implementation types to catalog identity, then remove `Problem::NAME` in the last phase.
- Modify `src/export.rs`
  Responsibility: preserve JSON DTOs while adding conversion helpers for typed refs.
- Modify `problemreductions-cli/src/problem_name.rs`
  Responsibility: move alias/default parsing from static tables to the catalog.
- Modify `problemreductions-cli/src/commands/create.rs`
  Responsibility: distinguish schema-valid problem specs from graph-reachable refs during create/example flows.
- Modify `problemreductions-cli/src/commands/graph.rs`
  Responsibility: use the catalog for parsing and the reduction graph for reachability.
- Modify `problemreductions-cli/src/mcp/tools.rs`
  Responsibility: same parsing/reachability split as CLI.
- Modify `src/rules/registry.rs`
  Responsibility: make exact endpoint uniqueness explicit in reduction registrations and lookup helpers.
- Modify `src/rules/graph.rs`
  Responsibility: use typed refs where appropriate and keep graph-node logic explicitly reachability-based.
- Modify `problemreductions-macros/src/lib.rs`
  Responsibility: let `#[reduction]` identify rules by exact endpoints rather than required IDs, and later switch `declare_variants!` off `Problem::NAME`.
- Modify `src/example_db/mod.rs`
  Responsibility: assemble canonical example DBs from explicit per-module specs and validate coverage/invariants.
- Modify `src/example_db/model_builders.rs`
  Responsibility: become a temporary bridge during migration, then shrink or disappear after specs are local.
- Modify `src/example_db/rule_builders.rs`
  Responsibility: same as `model_builders.rs`, but for rule examples.
- Modify `src/rules/mod.rs`
  Responsibility: aggregate per-rule example specs and rule-spec metadata through the same file that already owns rule-module inclusion.
- Modify `src/models/graph/mod.rs`, `src/models/formula/mod.rs`, `src/models/set/mod.rs`, `src/models/algebraic/mod.rs`, `src/models/misc/mod.rs`
  Responsibility: aggregate per-model canonical example specs through category module files that contributors already touch when adding a model.
- Modify every concrete model file under `src/models/**` that currently submits `ProblemSchemaEntry`
  Responsibility: declare aliases and variant dimensions in the existing local schema registration.
- Modify every concrete rule file under `src/rules/**` that currently uses `#[reduction(...)]`
  Responsibility: preserve unique exact endpoints and local canonical rule example specs.
- Modify `src/unit_tests/example_db.rs`, `src/unit_tests/reduction_graph.rs`, `src/unit_tests/rules/registry.rs`, `src/unit_tests/rules/graph.rs`, `src/unit_tests/trait_consistency.rs`, `src/unit_tests/export.rs`, `problemreductions-cli/tests/cli_tests.rs`, `problemreductions-cli/src/mcp/tests.rs`
  Responsibility: replace brittle count checks with catalog/rule/example invariants and cover new parsing behavior.

## Chunk 1: Catalog Foundation And CLI Bridge

### Task 1: Add the problem type catalog and typed internal refs

**Files:**
- Create: `src/registry/problem_type.rs`
- Create: `src/registry/problem_ref.rs`
- Create: `src/unit_tests/registry/problem_type.rs`
- Modify: `src/registry/schema.rs`
- Modify: `src/registry/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/variant.rs`
- Test: `src/unit_tests/registry/problem_type.rs`

- [ ] **Step 1: Write the failing catalog and typed-ref tests**

```rust
#[test]
fn typed_problem_ref_fills_declared_defaults() {
    let problem = crate::registry::find_problem_type("MaximumIndependentSet").unwrap();
    let problem_ref = crate::registry::ProblemRef::from_values(problem, ["i32"]).unwrap();
    assert_eq!(problem_ref.variant().get("graph"), Some("SimpleGraph"));
    assert_eq!(problem_ref.variant().get("weight"), Some("i32"));
}

#[test]
fn catalog_rejects_unknown_dimension_values() {
    let problem = crate::registry::find_problem_type("MaximumIndependentSet").unwrap();
    let err = crate::registry::ProblemRef::from_values(problem, ["HyperGraph"]).unwrap_err();
    assert!(err.to_string().contains("Known variants"));
}

#[test]
fn catalog_alias_lookup_is_case_insensitive() {
    let problem = crate::registry::find_problem_type_by_alias("mis").unwrap();
    assert_eq!(problem.canonical_name, "MaximumIndependentSet");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test typed_problem_ref_fills_declared_defaults catalog_alias_lookup_is_case_insensitive --lib`
Expected: FAIL with unresolved items such as `find_problem_type`, `ProblemRef::from_values`, or missing catalog metadata on schema entries.

- [ ] **Step 3: Implement the catalog core**

Create `src/registry/problem_type.rs` with:

```rust
pub struct VariantDimension {
    pub key: &'static str,
    pub default_value: &'static str,
    pub allowed_values: &'static [&'static str],
}

pub struct ProblemType<'a> {
    pub canonical_name: &'a str,
    pub display_name: &'a str,
    pub aliases: &'a [&'a str],
    pub dimensions: &'a [VariantDimension],
}
```

Implementation requirements:

- Build the runtime catalog from `inventory::iter::<ProblemSchemaEntry>()`.
- Extend `ProblemSchemaEntry` so each model-local registration includes `display_name`, `aliases`, and `dimensions`.
- Add lookup helpers:
  - `find_problem_type(name: &str) -> Option<ProblemType>`
  - `find_problem_type_by_alias(input: &str) -> Option<ProblemType>`
  - `problem_types() -> Vec<ProblemType>`
- Create `src/registry/problem_ref.rs` with typed `VariantSpec` and typed internal `ProblemRef`.
- Keep `VariantSpec` map-backed, but validate keys and values against the owning `ProblemType`.
- Add conversion helpers to and from `BTreeMap<String, String>` so the rest of the repo can migrate incrementally.

- [ ] **Step 4: Add schema-vs-reachability helpers**

Implement two explicit helpers:

```rust
pub fn parse_catalog_problem_ref(input: &str) -> anyhow::Result<ProblemRef>;
pub fn require_graph_variant(
    graph: &crate::rules::ReductionGraph,
    problem_ref: &ProblemRef,
) -> anyhow::Result<crate::export::ProblemRef>;
```

The first validates only against catalog schema. The second checks whether the concrete variant currently exists in the reduction graph.

- [ ] **Step 5: Run the new unit tests**

Run: `cargo test typed_problem_ref_fills_declared_defaults catalog_rejects_unknown_dimension_values catalog_alias_lookup_is_case_insensitive --lib`
Expected: PASS with `test result: ok`.

- [ ] **Step 6: Commit**

```bash
git add src/registry/problem_type.rs src/registry/problem_ref.rs src/registry/schema.rs src/registry/mod.rs src/lib.rs src/unit_tests/registry/problem_type.rs
git commit -m "feat(registry): add problem type catalog and typed refs"
```

### Task 2: Move CLI and MCP parsing to the catalog

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/commands/graph.rs`
- Modify: `problemreductions-cli/src/mcp/tools.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`
- Modify: `problemreductions-cli/src/mcp/tests.rs`
- Test: `problemreductions-cli/src/problem_name.rs`
- Test: `problemreductions-cli/tests/cli_tests.rs`

- [ ] **Step 1: Write the failing parser tests**

Add tests covering:

```rust
#[test]
fn resolve_problem_ref_bare_mis_uses_catalog_default() { /* ... */ }

#[test]
fn parse_problem_type_rejects_variant_suffixes_before_graph_lookup() { /* ... */ }

#[test]
fn resolve_problem_ref_rejects_schema_invalid_variant_before_graph_query() { /* ... */ }
```

Add CLI tests covering:

```rust
// `pred to MIS/PlanarGraph/i32` should fail with a graph-reachability error
// after schema parsing succeeds.
```

- [ ] **Step 2: Run the focused tests to verify failure**

Run: `cargo test -p problemreductions-cli resolve_problem_ref_bare_mis resolve_problem_ref_rejects_schema_invalid_variant_before_graph_query -- --exact`
Expected: FAIL because `problem_name.rs` still depends on `ALIASES`, graph ordering, and string-map heuristics.

- [ ] **Step 3: Implement the catalog-backed parser**

In `problemreductions-cli/src/problem_name.rs`:

- Delete the hand-maintained `ALIASES` table after the catalog-backed implementation passes.
- Keep `ProblemSpec` as a lightweight parsed slash-token structure, but resolve names through the registry catalog.
- Split responsibilities:
  - `parse_problem_spec(input)` parses raw tokens only.
  - `resolve_catalog_problem_ref(input)` returns a typed internal ref validated against schema.
  - `resolve_problem_ref(input, graph)` becomes the graph-reachability version used by graph/path tools.
- Keep the `3SAT -> K3` shorthand, but implement it as a catalog-aware normalization rule rather than alias-table special casing.
- Update shell completion and suggestions to enumerate names and aliases from the catalog.

In `create.rs`, `graph.rs`, and `mcp/tools.rs`:

- Use catalog parsing for user input normalization.
- Use graph reachability only in flows that truly require an existing graph node.
- Keep `pred create --example` schema-driven for model/rule example lookup, then separately require reachability only where needed.

- [ ] **Step 4: Run the parser and CLI tests**

Run: `cargo test -p problemreductions-cli resolve_problem_ref_bare_mis parse_problem_type_rejects_variant_suffixes_before_graph_lookup -- --exact`
Expected: PASS.

Run: `cargo test -p problemreductions-cli test_create_`
Expected: PASS with `test result: ok`.

- [ ] **Step 5: Commit**

```bash
git add problemreductions-cli/src/problem_name.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/src/commands/graph.rs problemreductions-cli/src/mcp/tools.rs problemreductions-cli/tests/cli_tests.rs problemreductions-cli/src/mcp/tests.rs
git commit -m "refactor(cli): resolve problem specs through the catalog"
```

### Task 3: Populate model-local catalog metadata and enforce catalog invariants

**Files:**
- Modify: every concrete model file under `src/models/**` that submits `ProblemSchemaEntry`
- Modify: `src/unit_tests/trait_consistency.rs`
- Modify: `src/unit_tests/reduction_graph.rs`
- Modify: `src/unit_tests/registry/schema.rs`
- Test: `src/unit_tests/registry/problem_type.rs`
- Test: `src/unit_tests/reduction_graph.rs`

- [ ] **Step 1: Write failing invariant tests**

Add tests for:

```rust
#[test]
fn every_public_problem_schema_has_dimension_defaults() { /* ... */ }

#[test]
fn every_alias_is_globally_unique() { /* ... */ }

#[test]
fn graph_defaults_are_catalog_defaults_for_registered_variants() { /* ... */ }
```

- [ ] **Step 2: Run the new invariant tests to verify failure**

Run: `cargo test every_alias_is_globally_unique graph_defaults_are_catalog_defaults_for_registered_variants --lib`
Expected: FAIL because existing model schema entries do not yet provide aliases/dimensions.

- [ ] **Step 3: Extend every model-local schema entry**

For each model file that already submits `ProblemSchemaEntry`, add:

- `display_name`
- `aliases`
- `dimensions`

Example shape:

```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumIndependentSet",
        display_name: "Maximum Independent Set",
        aliases: &["MIS"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph", "UnitDiskGraph"]),
            VariantDimension::new("weight", "One", &["One", "i32", "f64", "BigUint"]),
        ],
        module_path: module_path!(),
        description: "...",
        fields: &[...],
    }
}
```

Implementation notes:

- Keep the dimension sets mathematically valid for the problem type, not limited to graph-reachable nodes.
- Add a catalog-vs-variant-entry cross-check so CI fails if a model's declared dimensions do not cover its `declare_variants!` registrations.
- Update `trait_consistency.rs` to validate problem-type catalog coverage, not just trait smoke behavior.

- [ ] **Step 4: Run registry and reduction-graph tests**

Run: `cargo test every_public_problem_schema_has_dimension_defaults every_alias_is_globally_unique graph_defaults_are_catalog_defaults_for_registered_variants --lib`
Expected: PASS.

Run: `cargo test default_variant_for_mis_uses_declared_default --lib`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/models src/unit_tests/trait_consistency.rs src/unit_tests/reduction_graph.rs src/unit_tests/registry/schema.rs src/unit_tests/registry/problem_type.rs
git commit -m "feat(models): declare catalog metadata alongside schemas"
```

## Chunk 2: Rules, Example Specs, And Final Cleanup

### Task 4: Make exact endpoint identity explicit in reduction registration

**Files:**
- Modify: `problemreductions-macros/src/lib.rs`
- Modify: `src/rules/registry.rs`
- Modify: `src/rules/graph.rs`
- Modify: every rule file under `src/rules/**` that uses `#[reduction(...)]`
- Modify: `src/unit_tests/rules/registry.rs`
- Modify: `src/unit_tests/rules/graph.rs`
- Test: `problemreductions-macros/src/lib.rs`
- Test: `src/unit_tests/rules/registry.rs`

- [ ] **Step 1: Write the failing macro and registry tests**

Add macro tests for:

```rust
#[test]
fn reduction_accepts_overhead_without_id() { /* parse success */ }

#[test]
fn reduction_accepts_optional_id_attribute() { /* parse success */ }
```

Add runtime tests for:

```rust
#[test]
fn every_registered_reduction_has_unique_exact_endpoints() { /* ... */ }

#[test]
fn every_registered_reduction_has_non_empty_names() { /* ... */ }
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test -p problemreductions-macros reduction_accepts_overhead_without_id reduction_accepts_optional_id_attribute -- --exact`
Expected: FAIL because `ReductionAttrs` still requires `id`.

Run: `cargo test every_registered_reduction_has_unique_exact_endpoints --lib`
Expected: FAIL because the registry tests do not yet validate endpoint uniqueness explicitly.

- [ ] **Step 3: Implement exact endpoint identity**

In `problemreductions-macros/src/lib.rs`:

- Make `id = "..."` optional compatibility syntax rather than required metadata.
- Generate `ReductionEntry` values without a separate rule-ID field.
- Rely on endpoint uniqueness validation in the library tests.

In `src/rules/registry.rs`:

- Keep `ReductionEntry` keyed by `source_name`, `target_name`, and exact variants.
- Add or retain lookup helpers needed for endpoint-based validation and tooling.
  - `reduction_entries()`

In each concrete rule file:

- Ensure there is at most one primitive reduction registration per exact endpoint pair.
- Shared implementations should be wrapped rather than registered multiple times for the same endpoints.

- [ ] **Step 4: Run the macro and registry tests**

Run: `cargo test -p problemreductions-macros reduction_accepts_overhead_without_id reduction_accepts_optional_id_attribute -- --exact`
Expected: PASS.

Run: `cargo test every_registered_reduction_has_unique_exact_endpoints every_registered_reduction_has_non_empty_names --lib`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add problemreductions-macros/src/lib.rs src/rules/registry.rs src/rules/graph.rs src/rules src/unit_tests/rules/registry.rs src/unit_tests/rules/graph.rs
git commit -m "refactor(rules): use exact endpoint identity"
```

### Task 5: Move canonical examples to explicit per-module specs

**Files:**
- Create: `src/example_db/specs.rs`
- Modify: `src/example_db/mod.rs`
- Modify: `src/example_db/model_builders.rs`
- Modify: `src/example_db/rule_builders.rs`
- Modify: `src/rules/mod.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/formula/mod.rs`
- Modify: `src/models/set/mod.rs`
- Modify: `src/models/algebraic/mod.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: concrete model files that currently own canonical examples in `src/example_db/model_builders.rs`
- Modify: concrete rule files that currently own canonical examples in `src/example_db/rule_builders.rs`
- Modify: `src/unit_tests/example_db.rs`
- Modify: `src/unit_tests/export.rs`
- Test: `src/unit_tests/example_db.rs`

- [ ] **Step 1: Write the failing example-db tests**

Replace brittle count-based assertions with invariants such as:

```rust
#[test]
fn every_model_example_spec_points_to_a_valid_catalog_problem_ref() { /* ... */ }

#[test]
fn canonical_model_example_ids_are_unique() { /* ... */ }

#[test]
fn canonical_rule_example_ids_are_unique() { /* ... */ }
```

- [ ] **Step 2: Run the example-db tests to verify failure**

Run: `cargo test example_db:: --features 'ilp-highs example-db'`
Expected: FAIL because examples are still assembled from central builder lists and there are no per-module spec inventories or coverage checks.

- [ ] **Step 3: Introduce shared example spec types**

Create `src/example_db/specs.rs`:

```rust
pub struct ModelExampleSpec {
    pub id: &'static str,
    pub problem: crate::registry::ProblemRef,
    pub build: fn() -> crate::export::ModelExample,
}

pub struct RuleExampleSpec {
    pub id: &'static str,
    pub source: crate::registry::ProblemRef,
    pub target: crate::registry::ProblemRef,
    pub build: fn() -> crate::export::RuleExample,
}
```

- [ ] **Step 4: Move model examples next to their owning model modules**

For each model that currently contributes a canonical example:

- add a local `pub(crate) fn canonical_model_example_specs() -> Vec<ModelExampleSpec>` in the model file
- move the builder function out of `src/example_db/model_builders.rs` into that model file
- have the category module (`src/models/<category>/mod.rs`) concatenate specs from its child modules

Do not use `inventory` here. Use explicit per-module collection through module files contributors already touch.

- [ ] **Step 5: Move rule examples next to their owning rule modules**

For each rule that currently contributes a canonical example:

- add a local `pub(crate) fn canonical_rule_example_specs() -> Vec<RuleExampleSpec>` in the rule file
- move the example builder function out of `src/example_db/rule_builders.rs` into that rule file
- have `src/rules/mod.rs` concatenate rule example specs from its child modules

Each rule example spec must reference a registered exact `(source_ref, target_ref)` pair.

- [ ] **Step 6: Rebuild the example DB assembly**

In `src/example_db/mod.rs`:

- build model and rule DBs from the aggregated per-module spec lists
- validate:
  - unique example IDs
  - valid typed problem refs
  - rule examples reference registered exact `(source_ref, target_ref)` pairs
  - no duplicate canonical `(problem_ref)` for models
  - no duplicate canonical `(source_ref, target_ref)` for rules

Keep the exported JSON schema unchanged.

After the new assembly is green:

- delete the old hard-coded `Vec<ModelExample>` / `Vec<RuleExample>` construction lists from `src/example_db/model_builders.rs` and `src/example_db/rule_builders.rs`, or reduce those files to thin compatibility shims that simply call the new per-module collectors
- do not leave two independent canonical example sources in the repo

- [ ] **Step 7: Run the example DB and export tests**

Run: `cargo test example_db:: --features 'ilp-highs example-db'`
Expected: PASS.

Run: `cargo test test_write_canonical_example_dbs --features 'ilp-highs example-db' --lib`
Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add src/example_db src/models src/rules/mod.rs src/unit_tests/example_db.rs src/unit_tests/export.rs
git commit -m "refactor(example-db): collect canonical examples from owning modules"
```

### Task 6: Bridge export and runtime code to typed refs, then remove `Problem::NAME`

**Files:**
- Modify: `src/traits.rs`
- Modify: `src/export.rs`
- Modify: `src/registry/variant.rs`
- Modify: `src/registry/problem_ref.rs`
- Modify: `problemreductions-macros/src/lib.rs`
- Modify: every concrete model file under `src/models/**`
- Modify: `src/example_db/mod.rs`
- Modify: `src/example_db/specs.rs`
- Modify: `src/unit_tests/export.rs`
- Modify: `src/unit_tests/traits.rs`
- Modify: `src/unit_tests/rules/traits.rs`
- Test: `src/unit_tests/export.rs`
- Test: `src/unit_tests/traits.rs`
- Test: `problemreductions-cli/tests/cli_tests.rs`

- [ ] **Step 1: Add the bridge method before removing `NAME`**

In `src/traits.rs`, add a temporary default method:

```rust
fn problem_type() -> crate::registry::ProblemType<'static> {
    crate::registry::find_problem_type(Self::NAME).expect("missing problem type")
}
```

Migrate runtime call sites from `Problem::NAME` to `Problem::problem_type().canonical_name` before removing the const.

- [ ] **Step 2: Write the failing cleanup tests**

Add tests for:

```rust
#[test]
fn export_from_problem_uses_problem_type_identity() { /* ... */ }

#[test]
fn declare_variants_codegen_no_longer_depends_on_problem_name_const() { /* ... */ }
```

- [ ] **Step 3: Switch runtime call sites off `NAME`**

Update:

- `src/export.rs`
- `src/registry/variant.rs`
- `problemreductions-macros/src/lib.rs` (`declare_variants!`)
- any example-db or CLI helper still reading `Problem::NAME`

so they read the canonical name through the catalog bridge.

- [ ] **Step 4: Remove `const NAME` from `Problem` and from every concrete model implementation**

After the bridge is green:

- delete `const NAME` from `src/traits.rs`
- update all model impls to provide `fn problem_type() -> crate::registry::ProblemType<'static>`
- update proc-macro code generation to emit variant registrations using `problem_type().canonical_name`
- update unit tests that define fake problems to implement the new method

- [ ] **Step 5: Run the cleanup test suite**

Run: `cargo test export_from_problem_uses_problem_type_identity declare_variants_codegen_no_longer_depends_on_problem_name_const --lib`
Expected: PASS.

Run: `cargo test -p problemreductions-cli test_create_`
Expected: PASS.

Run: `cargo test example_db:: --features 'ilp-highs example-db'`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/traits.rs src/export.rs src/registry/variant.rs src/registry/problem_ref.rs problemreductions-macros/src/lib.rs src/models src/example_db src/unit_tests/export.rs src/unit_tests/traits.rs src/unit_tests/rules/traits.rs problemreductions-cli/tests/cli_tests.rs
git commit -m "refactor(core): remove Problem::NAME in favor of catalog identity"
```

## Final Verification

- [ ] **Step 1: Run the focused library checks**

Run: `cargo test typed_problem_ref_fills_declared_defaults every_registered_reduction_has_unique_exact_endpoints --features 'ilp-highs example-db' --lib`
Expected: PASS.

- [ ] **Step 2: Run the example DB suite**

Run: `cargo test example_db:: --features 'ilp-highs example-db'`
Expected: PASS.

- [ ] **Step 3: Run the CLI regression suite**

Run: `cargo test -p problemreductions-cli test_create_`
Expected: PASS.

- [ ] **Step 4: Run the full default test command used by the repo**

Run: `cargo test --features "ilp-highs example-db"`
Expected: PASS with `test result: ok`.
