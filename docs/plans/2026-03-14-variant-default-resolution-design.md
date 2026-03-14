# Variant Default Resolution Redesign

## Summary

The current variant system has a sound type-level core, but the runtime and CLI layers still rely on loose string maps and a few heuristics. This redesign keeps the short slash-style CLI syntax, but changes its meaning from "search for a matching variant" to "start from the declared default variant and apply updates". It also strengthens the internal model so defaults, variant identity, and reduction entry matching are explicit instead of inferred from ordering or fallback behavior.

The accepted direction is:

- Keep slash shorthand such as `MIS`, `MIS/UnitDiskGraph`, and `MIS/UnitDiskGraph/One`.
- Mark one explicit default variant per problem inside `declare_variants!`.
- Resolve shorthand by loading the default full variant, then applying slash tokens as dimension updates.
- Use exact default-to-default semantics everywhere a problem spec denotes a graph node.
- Throw errors on ambiguity, unknown tokens, duplicate updates to the same dimension, invalid final combinations, and missing defaults.
- Keep `show` as a type-level command and annotate the declared default variant in its variant listing.
- Replace loose internal variant handling with a canonical representation that enforces one value per dimension.
- Tighten reduction entry matching so it is exact and target-aware before any hierarchy-aware fallback.

## Current Problems

### 1. Variant identity is still stringly typed

`Problem::variant()` returns `Vec<(&str, &str)>`, which is later converted into `BTreeMap<String, String>`. This makes runtime handling simple, but it does not enforce the real invariant of the system: one value per variant dimension. Duplicate categories are representable in the source form, and the conversion silently collapses them.

This is acceptable for display, but weak as the internal representation that drives path finding, export, and CLI resolution.

### 2. Default variant behavior is inferred, not declared

Today the graph exposes variants in a preferred order, and some callers treat the first variant as the semantic default. That couples CLI behavior to sorting logic and hard-coded values such as `SimpleGraph`, `One`, and `KN`.

This is brittle for two reasons:

- Ordering is presentation logic, not semantic metadata.
- Future variant dimensions may not fit the current preference heuristic.

### 3. CLI shorthand resolution is based on global value matching

The current resolver looks at all known variants of a problem and tries to find ones containing all supplied values. This behaves like a fuzzy search over registered variants. It is convenient, but it does not reflect the user model you validated:

- `MIS` should mean the default MIS variant.
- `MIS/UnitDiskGraph` should mean "take the default MIS variant and change the graph dimension".

That is an update model, not a matching model.

### 4. Direct reduction entry matching is too permissive

`find_best_entry()` currently matches exact source variants first, then falls back to the first same-name reduction and ignores the target variant. That is workable only while same-name reductions happen to share overhead. It is not a strong contract.

### 5. Variant metadata and runtime graph fallback can drift

The registry already distinguishes between declared variants and reduction entries, but graph construction still allows nodes to appear from reduction edges even when full declared metadata is missing. That weakens the system precisely where canonical resolution depends on complete variant metadata.

## Goals

- Preserve the short slash-style CLI syntax.
- Make the default variant explicit and required.
- Make slash resolution deterministic and easy to explain.
- Enforce one value per variant dimension in the internal representation.
- Remove semantic dependence on variant ordering.
- Make reduction entry matching exact and target-aware.
- Fail loudly when variant metadata is incomplete.

## Non-Goals

- Replacing slash shorthand with keyword arguments.
- Changing user-facing variant value names such as `UnitDiskGraph` or `One`.
- Auto-generating all variant cast reductions in this change.
- Redesigning the type-level `VariantParam` trait hierarchy.
- Changing human-facing ordering of variants for display unless needed for clarity.

## Design

### 1. Canonical internal variant model

Introduce a canonical runtime type for full resolved variants. The exact name can vary, but the model should behave like a `VariantSpec` or `VariantKey` with these properties:

- One entry per variant dimension.
- Stable ordering for serialization and display.
- Validation at construction time.
- Equality and hashing based on the full resolved dimension set.

Conceptually:

```rust
pub struct VariantSpec {
    dims: BTreeMap<String, String>,
}
```

The important part is not the container type, but the invariant: duplicate dimensions are impossible once a value reaches the canonical representation.

`Problem::variant()` can continue to return the current lightweight form for now if that minimizes churn, but all runtime consumers should normalize into the canonical type immediately.

### 2. Explicit default variant in `declare_variants!`

Extend `declare_variants!` with an inline `default` marker:

```rust
crate::declare_variants! {
    default MaximumIndependentSet<SimpleGraph, i32> => "1.1996^num_vertices",
    MaximumIndependentSet<SimpleGraph, One> => "1.1996^num_vertices",
    MaximumIndependentSet<KingsSubgraph, i32> => "2^sqrt(num_vertices)",
}
```

Each parsed entry gains `is_default: bool`.

The macro must validate:

- Exactly one default per problem.
- Zero defaults is a macro error.
- More than one default is a macro error.

The generated registry metadata should carry `is_default` directly:

```rust
pub struct VariantEntry {
    pub name: &'static str,
    pub variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    pub complexity: &'static str,
    pub complexity_eval_fn: fn(&dyn Any) -> f64,
    pub is_default: bool,
}
```

This removes the need to infer default semantics from ordering.

### 3. Slash shorthand resolves by updating the default variant

The CLI syntax stays short, but the resolution model changes completely.

#### Resolution rule

1. Parse the problem alias or name.
2. Load the problem's declared default full variant.
3. Interpret each extra slash token as a request to update one dimension of that default.
4. Apply updates one by one.
5. Validate that the final assembled variant is a declared variant for that problem.

#### Examples

If the default MIS variant is `{graph=SimpleGraph, weight=i32}`:

- `MIS` -> `{graph=SimpleGraph, weight=i32}`
- `MIS/UnitDiskGraph` -> `{graph=UnitDiskGraph, weight=i32}`
- `MIS/One` -> `{graph=SimpleGraph, weight=One}`
- `MIS/UnitDiskGraph/One` -> `{graph=UnitDiskGraph, weight=One}`

This is not "choose the best match from all known variants". It is "start from the default and apply updates".

#### Token-to-dimension mapping

To apply a token like `UnitDiskGraph`, the resolver needs to know which dimension it updates. It should determine this from declared variants of that problem, not from global hard-coded tables.

For a given problem, gather the declared values that appear in each dimension across all registered variants. Then:

- If a token appears in exactly one dimension, update that dimension.
- If it appears in zero dimensions, error.
- If it appears in multiple dimensions, error as ambiguous.

This keeps the syntax short without introducing keyword-heavy input.

#### Duplicate updates are errors

If the user supplies two values that both map to the same dimension, resolution fails:

- `MIS/One/i32` -> error
- `MIS/SimpleGraph/UnitDiskGraph` -> error

The resolver should not use "last token wins". Conflicting inputs should be surfaced immediately.

### 4. Missing defaults are hard errors

If no default variant is registered for a problem, that is an error.

This should fail in two places:

- At macro expansion time for code that uses `declare_variants!`.
- At runtime in CLI or graph helpers if metadata is incomplete or legacy registrations are encountered.

There should be no fallback to "first variant in sorted order" and no silent recovery.

### 5. Exact and target-aware reduction entry matching

Direct reduction entry lookup should stop using name-only fallback that ignores the target variant.

Recommended matching order:

1. Exact source variant and exact target variant.
2. Exact source variant with validated target generalization, only if the reduction contract explicitly allows it.
3. Hierarchy-aware generalization based on declared variant relationships, if introduced for that caller.
4. Otherwise, no match.

The export path should use the same rule. It should not discard the target variant argument.

This makes matching semantics explicit and prevents the current situation where correctness depends on undocumented uniformity across entries with the same problem names.

### 6. Registry and graph invariants

The runtime graph should treat declared variant metadata as authoritative for canonical resolution.

Recommended invariant:

- Any problem that participates in CLI shorthand resolution must have complete `VariantEntry` metadata, including exactly one default.

The current graph fallback that synthesizes nodes from reduction edges can remain temporarily for backward compatibility in low-level graph construction, but commands that depend on canonical full variants should error if declared metadata is missing.

In practice, the system should move toward:

- `VariantEntry` defines valid nodes and their metadata.
- `ReductionEntry` defines valid edges between nodes.

That separation is already present conceptually and should become stricter operationally.

### 7. `variants_for()` becomes presentation-only

`ReductionGraph::variants_for()` can still return variants in a stable display order, but callers must stop treating `variants[0]` as the semantic default.

Instead, add an explicit helper:

```rust
pub fn default_variant_for(&self, name: &str) -> Option<VariantSpec>;
```

This is the only supported default lookup for CLI resolution and similar workflows.

## Error Model

Slash resolution should fail with clear, user-facing errors in these cases:

- Unknown problem name.
- No declared default variant for the resolved problem.
- Unknown variant token for that problem.
- Variant token ambiguous across dimensions.
- Duplicate updates to the same dimension.
- Final assembled variant is not registered.

Suggested examples:

- `Unknown variant token "FooGraph" for MaximumIndependentSet`
- `Token "One" is ambiguous for ProblemX; matches dimensions weight and cost_model`
- `Variant dimension "weight" was specified more than once`
- `Resolved variant {graph=KingsSubgraph, weight=f64} is not declared for MaximumIndependentSet`
- `No default variant declared for MaximumIndependentSet`

## Macro And Registry Changes

### `declare_variants!`

Parser changes:

- Support optional `default` keyword before an entry.
- Preserve existing complexity string validation.
- Group entries by problem name during validation so "exactly one default" can be enforced per problem.

Generated changes:

- `DeclaredVariant` impl remains.
- `VariantEntry` submission gains `is_default`.

### `VariantEntry`

Add:

- `is_default: bool`

Possible later additions, if helpful:

- Canonical full variant value precomputed at registration time.
- Optional dimension metadata if a future CLI helper wants direct access without rebuilding it from variants.

## CLI Resolution Algorithm

Given a parsed spec like `MIS/UnitDiskGraph/One`:

1. Resolve alias to canonical problem name.
2. Fetch declared variants for that problem.
3. Fetch the declared default full variant.
4. Build per-dimension token sets from declared variants.
5. Start from the default full variant.
6. For each supplied token:
   - Determine which dimension it maps to.
   - Error if zero or multiple dimensions match.
   - Error if that dimension was already updated.
   - Update the dimension.
7. Check that the final full variant exists in the declared variant set.
8. Return the canonical resolved variant.

This algorithm is deterministic, short to explain, and aligned with user expectations.

## CLI Command Semantics

The CLI should make a clean distinction between commands that operate on exact graph nodes and commands that operate on problem types.

### Node-level commands

These commands take problem specs that resolve to exact `ProblemRef` values:

- `create`
- `create --example`
- `to`
- `from`
- `path`
- `reduce --to`
- MCP tools that accept problem specs

For these commands, bare specs use exact default-to-default semantics. A bare `MIS` means the declared default MIS node, not "all MIS variants" and not "best match among variants". Examples:

- `pred create MIS` uses the default MIS variant.
- `pred create --example MIS` resolves to the default MIS variant, then looks up the exact canonical example for that node.
- `pred path MIS QUBO` searches from the default MIS node to the default QUBO node.
- `pred reduce problem.json --to QUBO` targets the default QUBO node unless the user supplies updates.

This means node-level commands should share one canonical resolver. They should not implement separate variant rules for normal creation, example creation, graph traversal, or MCP.

### Type-level commands

These commands operate on the problem type rather than a single resolved node:

- `list`
- `show`

`show` should remain a type overview command. It should accept only a problem name or alias, not a slash-qualified node spec. If the user passes `MIS/UnitDiskGraph`, that should be a clear error rather than silently ignoring the suffix.

Within the `show` output, the variants section should annotate the declared default variant explicitly, for example:

```text
MaximumIndependentSet

Variants (3):
  MIS/SimpleGraph/One (default)
  MIS/SimpleGraph/i32
  MIS/UnitDiskGraph/One
```

The `(default)` annotation comes from registry metadata, not from list position. Display order may still place the default first for convenience, but ordering is no longer semantic.

## Implementation Plan

### Phase 1: Registry and macro support

- Extend `VariantEntry` with `is_default`.
- Extend `declare_variants!` parser with `default`.
- Validate exactly one default per problem.
- Update existing variant declarations to mark one default per problem.

### Phase 2: Canonical runtime variant type

- Introduce a canonical full variant representation.
- Normalize graph/export/CLI logic onto it.
- Keep current map-based display helpers as adapters if needed.

### Phase 3: CLI resolver rewrite

- Replace match-by-values logic with one shared default-plus-updates resolver.
- Reuse that resolver in `create`, `create --example`, graph node commands, `reduce --to`, and MCP tools.
- Add explicit error handling for ambiguity and duplicate updates.
- Make bare node specs exact default-to-default operations instead of variant searches.
- Keep slash syntax unchanged.

### Phase 4: CLI command semantics cleanup

- Keep `show` type-level and reject slash-qualified specs there.
- Annotate the default variant in `show` output.
- Remove remaining command-specific variant resolution rules.

### Phase 5: Reduction entry matching cleanup

- Make `find_best_entry()` exact and target-aware.
- Update export lookup to pass and honor both source and target variants.
- Remove or sharply limit name-only fallback.

### Phase 6: Tighten invariants

- Audit callers that assume `variants[0]` is the default.
- Convert them to explicit default lookup.
- Restrict legacy fallback behavior where it interferes with canonical resolution.

## Test Matrix

### Macro tests

- One default entry succeeds.
- Zero defaults fails.
- Multiple defaults fail.
- Existing complexity validation still works with `default`.

### Graph and registry tests

- `default_variant_for(name)` returns the marked default.
- `variants_for(name)` ordering no longer affects semantic resolution.
- Missing default metadata is reported as an error in default-dependent paths.

### CLI resolver tests

- `MIS` resolves to the marked default.
- `MIS/UnitDiskGraph` updates only the graph dimension.
- `MIS/One` updates only the weight dimension.
- `MIS/UnitDiskGraph/One` updates both dimensions.
- `MIS/One/i32` errors on duplicate weight updates.
- Unknown token errors.
- Ambiguous token-to-dimension mapping errors.
- Final invalid variant combination errors.
- `pred create --example MIS` uses the same resolved default variant as other node-level commands.
- `pred path MIS QUBO` resolves source and target as exact default nodes instead of expanding across all variants.
- `pred reduce problem.json --to QUBO` resolves `QUBO` to the declared default target node.

### CLI command semantics tests

- `pred show MIS` succeeds and lists all declared variants for the problem type.
- `pred show MIS/UnitDiskGraph` errors because `show` is type-level.
- `pred show MIS` marks the declared default variant with `(default)`.
- Node-level commands no longer treat bare specs as existential searches over all variants.

### Reduction lookup tests

- Exact source and target variant match succeeds.
- Mismatched target variant does not silently succeed.
- Export overhead lookup respects both source and target variants.

## Risks And Tradeoffs

### Pros

- Short CLI input is preserved.
- Semantics become explicit and explainable.
- Defaults become stable metadata instead of ordering accidents.
- Internal variant handling becomes safer and easier to extend.
- Reduction entry lookup becomes less fragile.

### Costs

- `declare_variants!` needs a parser update and repository-wide annotation changes.
- Existing tests that rely on first-variant semantics will need updates.
- Some legacy fallback paths may need to become errors.

### Deferred work

- Auto-generating variant cast reductions.
- Richer public APIs around dimension metadata.
- A typed `VariantSpec` exposed publicly rather than only internally.

## Recommendation

Implement this redesign in one coherent pass centered on explicit defaults.

The highest-value change is not the new syntax marker by itself. The real win is changing the meaning of CLI shorthand from "search the set of variants" to "edit the default variant". Once that contract is in place, the rest of the system can align around a canonical full variant representation and explicit metadata rather than heuristic matching.
