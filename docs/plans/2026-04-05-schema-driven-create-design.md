# Schema-Driven `pred create` Refactor

**Date:** 2026-04-05
**Goal:** Replace the 11K-line `create.rs` with a schema-driven generic dispatch that uses the existing registry `factory` function, reducing the file by ~50%.
**Reviewed:** 2026-04-05 by Codex (see Appendix A for review findings)

## Problem

`problemreductions-cli/src/commands/create.rs` is 11,049 lines. The bulk is a 5,400-line `match canonical { ... }` that manually builds JSON for each of ~189 problems, plus 480 lines for `create_random`, plus 330 lines of lookup tables (`example_for`, `help_flag_name`, `help_flag_hint`, `type_format_hint`).

The registry already has a `factory: fn(serde_json::Value) -> Result<Box<dyn DynProblem>>` per variant that calls `serde_json::from_value()`. The 5,400 lines are manually doing what the factory can do generically.

## Design

### Phase 1: Align CLI Flags to Struct Field Names

Rename ~20 CLI flags in `CreateArgs` so every flag name matches its problem struct field name via `snake_case → kebab-case`. This makes convention-based mapping 100% mechanical with zero exceptions.

**Renames required:**

| Current flag | Struct field | New flag | Problems affected |
|---|---|---|---|
| `--job-tasks` | `jobs` | `--jobs` | JobShopScheduling |
| `--source-string` | `source` | (keep, add alias `--source-string`) | StringToStringCorrection |
| `--target-string` | `target` | (keep, add alias `--target-string`) | StringToStringCorrection |
| `--sets` | `subsets` | `--subsets` | SetPacking, MinimumHittingSet, etc. (~8) |
| `--universe` | `universe_size` | `--universe-size` | SetBasis, Betweenness, etc. (~5) |
| `--arc-costs` | `arc_weights` / `arc_lengths` | `--arc-weights` / `--arc-lengths` | MixedChinesePostman, StackerCrane |
| `--deps` | `dependencies` | `--dependencies` | PrimeAttributeName |
| `--query` | `query_attribute` | `--query-attribute` | PrimeAttributeName |
| `--precedence-pairs` | `precedences` | `--precedences` (already has alias) | MinimumTardinessSequencing, etc. (~4) |
| `--sizes` (for lengths) | `lengths` | `--lengths` (already exists!) | MultiprocessorScheduling, etc. (~5) |
| `--n` (for num_tasks) | `num_tasks` | `--num-tasks` (already exists!) | TimetableDesign, etc. (~4) |
| `--potential-edges` | `potential_weights` | `--potential-weights` | BiconnectivityAugmentation |
| `--bound` (various) | `max_length` / `max_weight` / `bound_k` / `threshold` | match each field | ~6 problems |

**Backward compat:** Add `#[arg(alias = "old-name")]` for renamed flags so existing scripts don't break.

**Note on `--source`/`--sink`/`--target`:** These flags are shared across many problems with different field names (`source`, `source_vertex`, `target`, `sink`). For fields like `GeneralizedHex.target` (which currently uses `--sink`), we keep `--sink` as an alias after renaming. The `source`/`sink`/`target` flags already match field names for most graph problems. StringToStringCorrection's `source`/`target` fields conflict with the graph vertex `--source`/`--sink` flags, so we keep `--source-string`/`--target-string` as aliases while the field-matched flag takes precedence during schema dispatch.

### Phase 2: Generic Type Parser Registry

A small registry that maps resolved concrete type names to parse functions. These parse functions already exist — we're just organizing them for generic dispatch.

```rust
/// Parse a CLI string value into a serde_json::Value based on the resolved concrete type.
fn parse_field_value(
    concrete_type: &str,
    field_name: &str,
    raw: &str,
    context: &CreateContext,  // holds graph info for size validation
) -> Result<serde_json::Value>
```

**Type dispatch table:**

There are 52 unique `type_name` values across all schema registrations. They fall into three categories:

**Category 1: Generic types (resolve via variant map first)**
These appear as literal type_name strings and must be resolved before dispatch:
- `G` → resolve via `variant["graph"]` → concrete graph type
- `W` → resolve via `variant["weight"]` → `One`, `i32`, or `f64`
- `W::Sum` → resolve `W` first, then map: `One` → `usize`, `i32` → `i32`, `f64` → `f64`
- `Vec<W>`, `Vec<Vec<W>>`, `Vec<Vec<T>>`, `Vec<(usize, usize, W)>` → substitute generic param, then dispatch on resolved type

**Note:** Whitespace in type_name strings is inconsistent (e.g., `Vec<(usize,usize)>` vs `Vec<(usize, usize)>`). Normalize by stripping spaces before matching.

**Category 2: Concrete types with direct CLI parsing (~20 entries)**

| Concrete type pattern | Parse strategy | Existing helper |
|---|---|---|
| `SimpleGraph` | edge list → `{num_vertices, edges}` | `parse_graph()` |
| `BipartiteGraph` | bipartite edge list | `parse_bipartite_graph()` |
| `KingsSubgraph` / `TriangularSubgraph` | positions → grid subgraph | `parse_grid_subgraph()` |
| `UnitDiskGraph` | positions + radius | `parse_unit_disk_graph()` |
| `DirectedGraph` | arc list → `{num_vertices, arcs}` | `parse_directed_graph()` |
| `MixedGraph` | graph + arcs | `parse_mixed_graph()` |
| `Vec<i32>` / `Vec<f64>` / `Vec<u64>` / `Vec<i64>` / `Vec<usize>` | comma-separated numbers | `parse_numeric_list::<T>()` |
| `Vec<One>` | auto-fill unit weights (length from context) | fill with `1`s |
| `Vec<bool>` | comma-separated 0/1 or true/false | `parse_bool_list()` |
| `Vec<Vec<usize>>` / `Vec<Vec<u64>>` / `Vec<Vec<i32>>` / `Vec<Vec<i64>>` / `Vec<Vec<bool>>` | semicolon-separated rows | `parse_nested_list::<T>()` |
| `Vec<Vec<Vec<usize>>>` | pipe-separated matrices | `parse_3d_list()` |
| `Vec<[usize; 3]>` | semicolon-separated triples | `parse_triple_list()` |
| `Vec<CNFClause>` | semicolon-separated signed literals | `parse_clauses()` |
| `Vec<(usize, usize)>` | pair list (comma or `>` separated) | `parse_pair_list()` |
| `Vec<(usize, f64)>` | pair list | `parse_typed_pair_list()` |
| `Vec<(usize, usize, usize)>` / `Vec<(usize, usize, usize, usize)>` | tuple list | `parse_tuple_list()` |
| `Vec<(usize, Vec<usize>)>` / `Vec<(Vec<usize>, Vec<usize>)>` / `Vec<(Vec<usize>, usize)>` | nested pair list | `parse_complex_pair_list()` |
| `Vec<Vec<(usize, u64)>>` | job-task format | `parse_job_shop_jobs()` |
| `Vec<String>` | semicolon-separated strings | `split(';')` |
| `Vec<BigUint>` / `BigUint` | comma-separated decimal strings | custom decimal parse (uses biguint_serde) |
| `Vec<Option<bool>>` | comma-separated with "?" for None | `parse_optional_bool_list()` |
| `usize` / `u64` / `i32` / `i64` / `f64` | single number parse | `str::parse::<T>()` |
| `One` (scalar unit weight) | skip field / default to `null` | (handled by serde default) |
| `bool` | "true"/"false" parse | `str::parse::<bool>()` |

**Category 3: Complex domain types (passthrough as JSON)**
These types have custom serde and are best handled by passing the raw CLI string through the existing problem-specific parser, or by skipping CLI creation entirely (like ILP/CircuitSAT):
- `Circuit` → CircuitSAT (already excluded from CLI create)
- `IntExpr` → IntegerExpressionMembership (JSON expression tree via `--expression`)
- `ObjectiveSense` → ILP (already excluded from CLI create)
- `Vec<LinearConstraint>` → ILP (already excluded from CLI create)
- `Vec<Quantifier>` → QBF (comma-separated E/A via `--quantifiers`)
- `Vec<Relation>` → ConjunctiveBooleanQuery (custom format via `--relations`)
- `Vec<FrequencyTable>` → ConsistencyOfDatabaseFrequencyTables (custom format via `--frequency-tables`)
- `Vec<KnownValue>` → ConsistencyOfDatabaseFrequencyTables (custom format via `--known-values`)
- `Vec<VarBounds>` → ClosestVectorProblem (custom format via `--bounds`)
- `Vec<(usize, Vec<QueryArg>)>` → ConjunctiveBooleanQuery (custom format via `--conjuncts-spec`)
- `Vec<(usize, Vec<Term>)>` → ConjunctiveQueryFoldability (tagged enum JSON)

For Category 3, the parse function dispatches on `(problem_name, field_name)` rather than type alone, since these formats are problem-specific. This is a small lookup table (~10 entries) separate from the generic type dispatch.

### Phase 3: Generic `create()` Function

Replace the 5,400-line match with:

```rust
pub fn create(args: &CreateArgs, out: &OutputConfig) -> Result<()> {
    // Existing: example path, ILP/CircuitSAT rejection, random path
    if args.example.is_some() { return create_from_example(args, out); }
    // ... resolve canonical name, variant ...
    if args.random { return create_random(args, canonical, &resolved_variant, out); }

    // NEW: schema-driven path
    let schema = find_schema(canonical)
        .ok_or_else(|| anyhow!("No schema for {canonical}"))?;
    let variant_entry = find_variant_entry(canonical, &resolved_variant)?;

    // Show help if no data flags provided
    if all_data_flags_empty(args) {
        print_schema_help(canonical, &schema, &resolved_variant)?;
        std::process::exit(2);
    }

    // Build JSON from schema fields
    let mut json_map = serde_json::Map::new();
    let mut context = CreateContext::default();

    for field in &schema.fields {
        let flag_name = field.name.replace('_', "-");  // convention
        let raw_value = get_flag_value(args, &flag_name);
        let concrete_type = resolve_type(&field.type_name, &resolved_variant);

        let value = parse_field_value(&concrete_type, field.name, raw_value, &context)?;

        // Track graph context for downstream validation
        if is_graph_type(&concrete_type) {
            context.num_vertices = extract_num_vertices(&value);
            context.num_edges = extract_num_edges(&value);
        }

        json_map.insert(field.name.to_string(), value);
    }

    // Run optional per-problem validator
    if let Some(validator) = find_validator(canonical) {
        validator(&json_map, args)?;
    }

    // Factory deserializes JSON → concrete problem type
    let json = serde_json::Value::Object(json_map);
    let problem = (variant_entry.factory)(json)
        .map_err(|e| anyhow!("Failed to construct {canonical}: {e}"))?;

    emit_dyn_problem_output(&problem, canonical, &resolved_variant, out)
}
```

### Phase 4: `get_flag_value()` — Reflective Flag Access

The `CreateArgs` struct has ~120 `Option<String>` fields. We need to look up a field by name at runtime. Two approaches:

**Option A (recommended): Build a `HashMap<&str, Option<&str>>` from CreateArgs.**

Add a method to `CreateArgs`:
```rust
impl CreateArgs {
    fn flag_map(&self) -> HashMap<&str, Option<&str>> {
        let mut m = HashMap::new();
        m.insert("graph", self.graph.as_deref());
        m.insert("weights", self.weights.as_deref());
        m.insert("edge-weights", self.edge_weights.as_deref());
        // ... all string flags
        m
    }
}
```

This is ~120 lines but purely mechanical and can be generated by a macro or build script. It replaces 5,400 lines.

**Option B: Use serde to serialize CreateArgs to JSON, then look up fields by name.**

Derive `Serialize` on `CreateArgs`, serialize to `serde_json::Value`, then access fields by name. Zero boilerplate but adds serde dependency to the CLI args struct.

**Recommendation:** Option A for explicitness. Option B as fallback if the mechanical list becomes a maintenance burden.

### Phase 5: Help Text Generation

Replace the 330-line lookup tables (`example_for`, `help_flag_name`, `help_flag_hint`, `type_format_hint`) with schema-driven help:

```rust
fn print_schema_help(canonical: &str, schema: &ProblemSchemaEntry, variant: &BTreeMap<String, String>) -> Result<()> {
    eprintln!("Usage: pred create {canonical} [FLAGS]\n");
    eprintln!("Fields:");
    for field in &schema.fields {
        let flag = field.name.replace('_', "-");
        let concrete = resolve_type(&field.type_name, variant);
        let format = type_format_hint_generic(&concrete);
        eprintln!("  --{flag:<25} {:<20} {}", concrete, field.description);
        if !format.is_empty() {
            eprintln!("  {:<27} Format: {format}", "");
        }
    }
    // Show canonical example from example_db
    if let Some(example) = find_model_example(canonical) {
        eprintln!("\nExample:\n  pred create {canonical} {}", example.cli_string());
    }
    Ok(())
}
```

**`example_for()` elimination:** Delegate to existing `canonical_model_example_specs()` from `src/example_db/model_builders.rs` instead of maintaining a parallel 300-line string table.

### Phase 6: `create_random` Simplification

The 480-line `create_random` also has a giant match. For most problems, random creation follows a pattern:
1. Create random graph (with `util::create_random_graph()`)
2. Create random weights (if needed)
3. Construct the problem

This can be partially genericized using the same schema-driven approach, but random creation involves more problem-specific logic (e.g., SteinerTree needs random terminal selection). Keep the match for now but reduce it by extracting shared patterns into helpers for graph-only, graph+vertex-weight, and graph+edge-weight categories. Target: reduce from 480 to ~200 lines.

### Phase 7: Per-Problem Validators

~15-20 problems need custom validation beyond type parsing:

```rust
type ValidatorFn = fn(&serde_json::Map<String, Value>, &CreateArgs) -> Result<()>;

fn find_validator(canonical: &str) -> Option<ValidatorFn> {
    match canonical {
        "GeneralizedHex" => Some(|json, _| {
            let source = json["source"].as_u64().unwrap();
            let target = json["target"].as_u64().unwrap();
            if source == target { bail!("source and target must be distinct"); }
            Ok(())
        }),
        "LengthBoundedDisjointPaths" => Some(validate_lbdp),
        // ~15 more
        _ => None,
    }
}
```

This is ~200 lines — the genuinely unique validation logic that can't be eliminated.

### Phase 8: Non-String Flag Handling

Some `CreateArgs` fields are non-string types (`Option<usize>`, `Option<u64>`, `Option<f64>`, `bool`). These need special handling in `get_flag_value()`:

- `source: Option<usize>` → convert to string for the generic path
- `k: Option<usize>` → same
- `bound: Option<i64>` → same
- `random: bool`, `seed: Option<u64>`, `edge_prob: Option<f64>` → only used by `create_random`, not the schema path

The `flag_map()` can include these by converting to string: `m.insert("source", self.source.map(|v| v.to_string()))`. Slight ugliness but keeps the generic path uniform.

Alternatively, keep these as special-case lookups outside the generic loop (they affect <10 problems).

## File Structure After Refactor

```
problemreductions-cli/src/commands/create.rs  (~3,000 lines → from 11,049)
├── create()                    — generic schema-driven dispatch (~80 lines)
├── create_from_example()       — unchanged (~40 lines)
├── create_random()             — simplified (~200 lines, down from 480)
├── CreateContext               — tracking struct for cross-field validation (~20 lines)
├── Type parsers                — parse_field_value() + ~15 type handlers (~400 lines)
├── Flag access                 — flag_map() or equivalent (~130 lines)
├── Help generation             — schema-driven help (~60 lines)
├── Validators                  — per-problem validation (~200 lines)
├── Existing helpers            — parse_graph, parse_clauses, etc. (~1,500 lines, kept)
└── Graph parsing utilities     — parse_edge_list, etc. (~400 lines, kept)
```

**Estimated reduction:** 11,049 → ~3,000 lines (~73% reduction).

## Serde Edge Cases

Several problems use serde customization that affects the factory JSON path:

**`#[serde(try_from)]` — validation wrappers (6 problems):**
`NAESatisfiability`, `StackerCrane`, `SetSplitting`, `RootedTreeStorageAssignment`, `EnsembleComputation`, `ConsecutiveBlockMinimization`. These keep the same JSON keys but reject invalid input after parsing. The factory will return an error with a validation message — this is correct behavior and needs no special handling.

**`#[serde(from)]` + skip/default — cache fields:**
`BalancedCompleteBipartiteSubgraph` uses `#[serde(from)]` with internal cache fields. `KColoring` has `#[serde(default)]` + `#[serde(skip)]` on internal fields. These are transparent to the JSON input — the schema-exposed fields are the only ones needed.

**`#[serde(skip)]` — phantom/const fields:**
`KSatisfiability` and `ILP` skip phantom type fields. These don't appear in JSON and need no CLI parsing.

**Custom serde for `BigUint`:**
`SubsetSum` and `SubsetProduct` use custom decimal-string serde for `BigUint` fields (`biguint_serde` module). The CLI parser should produce decimal strings matching this format.

**Tagged enum types:**
`Term` in `ConjunctiveQueryFoldability` serializes as a tagged JSON object, not a plain scalar. The CLI flag passes raw JSON for this type.

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| JSON shape mismatch (field order, missing defaults) | Factory uses `serde_json::from_value` which handles field order. Add integration tests comparing old vs new output for all 177 problems. |
| Generic type resolution fails for complex types | Start with a whitelist of known type patterns. Fall back to problem-specific match arm for unrecognized types. |
| Flag rename breaks external scripts | Add `#[arg(alias = "old-name")]` for all renames. |
| Error messages degrade (generic vs problem-specific) | Include problem name and field name in all error messages. Per-problem validators can add context. |
| `create_random` is harder to genericize | Phase 6 is conservative — extract helpers but keep the match. Revisit later. |

## Testing Strategy

1. **Regression tests:** For each of the 177 problems, compare `pred create <Problem> <args>` output before and after the refactor. Use the existing `example_for()` args as test inputs.
2. **Round-trip tests:** `pred create X --args | pred solve -` must still work for all problems with ILP paths.
3. **Help text tests:** Verify `pred create <Problem>` (no args) produces useful help for 10+ diverse problems.
4. **Flag alias tests:** Verify old flag names still work via aliases.
5. **CLI demo:** `make cli-demo` must pass (exercises all commands).

## Implementation Order

1. **Write regression test harness** — capture current output for all 189 problems
2. **Rename CLI flags** — add aliases for backward compat (see Phase 1 + Appendix A for complete list)
3. **Implement `flag_map()`** — reflective flag access
4. **Implement type parser registry** — `parse_field_value()` with all 3 categories of type handlers
5. **Implement generic `create()`** — schema-driven dispatch
6. **Implement schema-driven help** — replace lookup tables
7. **Add per-problem validators** — ~15-20 problem-specific checks
8. **Simplify `create_random`** — extract shared patterns
9. **Run regression tests** — verify all 189 problems produce identical output
10. **Remove dead code** — old match arms, old lookup tables

## Appendix A: Codex Review Findings (2026-04-05)

### Additional Flag→Field Mismatches (not in Phase 1 table)

The following field names have no matching CLI flag via `snake_case → kebab-case`. Each needs either a new flag or a rename:

**Algebraic:** `ILP` (`constraints`, `objective`, `sense` — already excluded from CLI), `QuadraticCongruences` (`a`, `b`, `c`), `QuadraticDiophantineEquations` (`a`, `b`, `c`)

**Graph:** `AcyclicPartition` (`vertex_weights`), `BicliqueCover` (`left_size`, `right_size`, `edges`), `BoundedComponentSpanningForest` (`max_components`), `DegreeConstrainedSpanningTree` (`max_degree`), `LengthBoundedDisjointPaths` (`max_paths`), `MinMaxMulticenter` (`vertex_weights`), `MinimumCapacitatedSpanningTree` (`root`), `MinimumEdgeCostFlow` (`prices`, `required_flow`), `MinimumSumMulticenter` (`vertex_weights`), `PartitionIntoCliques` (`num_cliques`), `PartitionIntoForests` (`num_forests`), `PartitionIntoPerfectMatchings` (`num_matchings`)

**Misc:** `Betweenness` (`num_elements`, `triples`), `BoyceCoddNormalFormViolation` (`functional_deps`, `target_subset`), `CapacityAssignment` (`cost`, `delay`), `Clustering` (`distances`, `num_clusters`), `ConjunctiveBooleanQuery` (`conjuncts`), `ConjunctiveQueryFoldability` (`num_distinguished`, `num_undistinguished`, `relation_arities`, `query1_conjuncts`, `query2_conjuncts`), `CyclicOrdering` (`num_elements`, `triples`), `DynamicStorageAllocation` (`items`, `memory_size`), `FeasibleRegisterAssignment` (`num_registers`), `MinimumAxiomSet` (`num_sentences`), `MinimumCodeGenerationOneRegister` (`edges`, `num_leaves`), `MinimumRegisterSufficiencyForLoops` (`variables`), `NonLivenessFreePetriNet` (`num_places`, `num_transitions`, `place_to_transition`, `transition_to_place`), `Numerical3DimensionalMatching` (`sizes_w`, `sizes_x`, `sizes_y`), `NumericalMatchingWithTargetSums` (`sizes_x`, `sizes_y`, `targets`), `OpenShopScheduling` (`num_machines`, `processing_times`), `StackerCrane` (`edges`), `StaffScheduling` (`shifts_per_schedule`)

**Set:** `SetBasis` (`collection`), `ThreeDimensionalMatching` (`triples`), `ThreeMatroidIntersection` (`ground_set_size`)

### Implementation Note

Many of these "mismatches" are fields that already have CLI flags under a *different* name (e.g., `vertex_weights` → `--weights`, `max_components` → `--k`). The Phase 1 rename aligns them. Some fields use generic flags like `--bound` that map to multiple field names — Phase 1 splits these into specific flags matching each field name. Fields for ILP and CircuitSAT are excluded since those problems already reject CLI creation.
