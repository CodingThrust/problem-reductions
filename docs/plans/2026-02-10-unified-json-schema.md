# Unified JSON Schema for Reduction Examples

**Date**: 2026-02-10
**Status**: Draft
**Scope**: All 30 example JSON files in `docs/paper/examples/`

## Problem

Three inconsistent JSON schemas exist across 30 example files:

1. **Flat schema** (e.g., `sat_to_ksat.json`): `source_num_variables`, `target_num_variables`, `source_solution`, `target_solution` at top level
2. **QUBO rich schema** (e.g., `is_to_qubo.json`): `source_instance`, `qubo`, `optimal_solutions` with domain-specific details
3. **Nested schema** (e.g., `matching_to_setpacking.json`): `source`, `target`, `solution` objects

This forces Typst's `load-example()` to normalize across all three formats at load time.

## Design Decisions (from brainstorming)

1. **Two files per example**: `<name>.json` (reduction structure) and `<name>.result.json` (runtime solutions)
2. **Typed instance objects**: Each problem type has its own natural fields (`num_vertices`/`edges` for graphs, `num_vars`/`matrix` for QUBO, `clauses` for SAT)
3. **No field duplication**: No synthetic `num_variables` when the instance already has `num_vertices` or `num_vars`
4. **Raw configs only**: Solution arrays are `[0,1,0,1]`, no interpretation strings like `"V0=Red"`
5. **Polynomial overhead**: Matches `ReductionOverhead` from `src/rules/registry.rs`
6. **Variant dict**: Matches `reduction_graph.json` node format

## Schema: `<name>.json` (Reduction File)

```json
{
  "source": {
    "problem": "IndependentSet",
    "variant": { "graph": "SimpleGraph", "weight": "Unweighted" },
    "instance": {
      "num_vertices": 4,
      "num_edges": 3,
      "edges": [[0,1], [1,2], [2,3]]
    }
  },
  "target": {
    "problem": "QUBO",
    "variant": { "graph": "SimpleGraph", "weight": "f64" },
    "instance": {
      "num_vars": 4,
      "matrix": [[-1.0, 5.0, 0.0, 0.0], ...]
    }
  },
  "overhead": [
    { "field": "num_vars", "polynomial": [{ "coefficient": 1.0, "variables": [["num_vertices", 1]] }] }
  ]
}
```

### `source` / `target` object

| Field | Type | Description |
|-------|------|-------------|
| `problem` | string | Problem name matching `Problem::NAME` (e.g., `"IndependentSet"`, `"QUBO"`, `"KSatisfiability<3>"`) |
| `variant` | object | Key-value pairs matching `reduction_graph.json` node variant (e.g., `{"graph": "SimpleGraph", "weight": "Unweighted"}`) |
| `instance` | object | Problem-specific fields (see Instance Schemas below) |

### `overhead` array

Directly mirrors `ReductionOverhead::output_size: Vec<(&str, Polynomial)>` from `src/rules/registry.rs`.

Each element maps one output size field to a polynomial of input size variables:

```json
{
  "field": "num_vars",
  "polynomial": [
    { "coefficient": 1.0, "variables": [["num_vertices", 1]] }
  ]
}
```

Each polynomial entry is a monomial (mirrors `Monomial` from `src/polynomial.rs`):

| Field | Type | Description |
|-------|------|-------------|
| `coefficient` | float | Scalar multiplier |
| `variables` | array of `[name, exponent]` | Variable-exponent pairs (empty = constant) |

The polynomial is the sum of all monomials: `Σ (coefficient × Π variable^exponent)`.

**Examples matching actual code declarations:**

| Code (`#[reduction(overhead = ...)]`) | JSON `overhead` |
|---------------------------------------|-----------------|
| `poly!(num_vertices)` | `[{"coefficient": 1.0, "variables": [["num_vertices", 1]]}]` |
| `poly!(7 * num_clauses)` | `[{"coefficient": 7.0, "variables": [["num_clauses", 1]]}]` |
| `poly!(num_bits_first^2)` | `[{"coefficient": 1.0, "variables": [["num_bits_first", 2]]}]` |
| `poly!(3 * num_vars)` | `[{"coefficient": 3.0, "variables": [["num_vars", 1]]}]` |
| `poly!(3)` (constant) | `[{"coefficient": 3.0, "variables": []}]` |

**Multi-field overhead** (e.g., SAT → IS has both `num_vertices` and `num_edges`):

```json
"overhead": [
  { "field": "num_vertices", "polynomial": [{ "coefficient": 7.0, "variables": [["num_clauses", 1]] }] },
  { "field": "num_edges", "polynomial": [{ "coefficient": 21.0, "variables": [["num_clauses", 1]] }] }
]
```

### Instance Schemas (by problem type)

Each problem type uses its natural fields from `problem_size()`. No generic `num_variables` wrapper.

**Graph problems** (IndependentSet, VertexCovering, DominatingSet, MaxCut, Clique, Matching):
```json
{ "num_vertices": 4, "num_edges": 3, "edges": [[0,1], [1,2], [2,3]] }
```
Optional: `"weights": [1, 2, 3, 4]` for weighted variants.

**KColoring**:
```json
{ "num_vertices": 3, "num_edges": 3, "num_colors": 3, "edges": [[0,1], [1,2], [0,2]] }
```

**SAT / Satisfiability**:
```json
{ "num_vars": 4, "num_clauses": 2, "clauses": [[1, -2, 3], [-1, 4]] }
```

**KSatisfiability<K>**:
```json
{ "num_vars": 8, "num_clauses": 6, "k": 3, "clauses": [[1, -2, 3], ...] }
```

**QUBO**:
```json
{ "num_vars": 4, "matrix": [[-1.0, 5.0, 0.0, 0.0], ...] }
```

**SpinGlass**:
```json
{ "num_spins": 4, "num_interactions": 3, "interactions": [[0,1,1.0], [1,2,-1.0], ...] }
```

**SetPacking / SetCovering**:
```json
{ "num_sets": 4, "num_elements": 3, "sets": [[0], [0,1], [1,2], [2]] }
```
Optional: `"weights": [1, 1, 1, 1]` for weighted variants.

**ILP**:
```json
{ "num_vars": 4, "num_constraints": 2, "objective": [1.0, 2.0, 3.0, 4.0], "constraints": [...] }
```

**CircuitSAT**:
```json
{ "num_gates": 5, "num_assignments": 8, "gates": [...] }
```

**Factoring**:
```json
{ "number": 15, "num_bits_first": 2, "num_bits_second": 2 }
```

## Schema: `<name>.result.json` (Results File)

```json
{
  "solutions": [
    {
      "source_config": [1, 0, 1, 0],
      "target_config": [1, 0, 1, 0]
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `solutions` | array | One or more optimal solution pairs |
| `solutions[].source_config` | array of int | Raw variable assignment for source problem |
| `solutions[].target_config` | array of int | Raw variable assignment for target problem |

No interpretation fields (no `"coloring": ["V0=Red", ...]`, no `"selected_vertices": [1,3]`). Consumers derive meaning from config + instance.

## Implementation Plan

### Step 1: Create `ExampleData` Rust struct

Add a shared serialization module (e.g., `examples/shared/schema.rs` or a helper in `src/`) that all examples import:

```rust
#[derive(Serialize)]
struct ProblemSide {
    problem: String,
    variant: HashMap<String, String>,
    instance: serde_json::Value,
}

#[derive(Serialize)]
struct OverheadEntry {
    field: String,
    polynomial: Vec<MonomialJson>,
}

#[derive(Serialize)]
struct MonomialJson {
    coefficient: f64,
    variables: Vec<(String, u8)>,
}

#[derive(Serialize)]
struct ReductionData {
    source: ProblemSide,
    target: ProblemSide,
    overhead: Vec<OverheadEntry>,
}

#[derive(Serialize)]
struct SolutionPair {
    source_config: Vec<usize>,
    target_config: Vec<usize>,
}

#[derive(Serialize)]
struct ResultData {
    solutions: Vec<SolutionPair>,
}
```

Add helper methods to build `ProblemSide` from any `Problem` impl and `OverheadEntry` from `ReductionOverhead`.

### Step 2: Update all 30 example files

Replace ad-hoc serialization with the shared struct. Each example:
1. Creates source problem
2. Reduces to target
3. Solves target, extracts solutions
4. Builds `ReductionData` + `ResultData`
5. Writes `<name>.json` and `<name>.result.json`

### Step 3: Update Typst `load-example()`

Replace the 3-schema normalization in `reductions.typ` with direct field access. Since all JSON files now share the same schema, `load-example()` becomes trivial:

```typst
#let load-example(name) = json("examples/" + name + ".json")
#let load-results(name) = json("examples/" + name + ".result.json")
```

Update all `reduction-example()` calls and the resource estimation table to use `data.source.instance.num_vertices` etc.

### Step 4: Update integration test assertions

If `tests/suites/examples.rs` checks JSON structure, update to match new schema.

### Step 5: Verify

```bash
make examples   # All 30 examples regenerate both files
make paper      # Typst compiles with new schema
make test       # All tests pass
make clippy     # No warnings
```

## File Impact

| Files | Count | Action |
|-------|-------|--------|
| `examples/shared/schema.rs` (or similar) | 1 | New: shared serialization structs |
| `examples/reduction_*.rs` | 30 | Update: use shared schema |
| `docs/paper/examples/*.json` | 30 | Regenerated: unified schema |
| `docs/paper/examples/*.result.json` | 30 | New: split solution data |
| `docs/paper/reductions.typ` | 1 | Update: simplify `load-example()` |

## Migration Notes

- Old JSON files are fully replaced (not backwards-compatible)
- The `overhead` field is new — sourced from each reduction's `#[reduction(overhead = ...)]` macro
- Polynomial serialization is a 1:1 mapping from `Polynomial { terms: Vec<Monomial> }` in Rust to `[{coefficient, variables}]` in JSON
- Problem names use `Problem::NAME` exactly (e.g., `"KSatisfiability<3>"` not `"3-SAT"`)
- Variant dicts match `reduction_graph.json` nodes
