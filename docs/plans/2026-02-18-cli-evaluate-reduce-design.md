# CLI Evaluate & Reduce Commands — Design

## Goal

Implement `pred evaluate` and `pred reduce` commands that work with JSON problem files.

## JSON Problem Format

Wrapper format with explicit type:

```json
{
  "type": "MaximumIndependentSet",
  "variant": {"graph": "SimpleGraph", "weight": "i32"},
  "data": {
    "graph": {"num_vertices": 4, "edges": [[0,1],[1,2],[2,3]]},
    "weights": [1, 1, 1, 1]
  }
}
```

- `type`: Problem name (aliases like "MIS" accepted)
- `variant`: Optional. Defaults to base variant if omitted.
- `data`: Raw serde JSON of the problem struct

## Approach: Match-based dispatch in CLI

Simple match statement in the CLI crate maps `(name, variant)` to concrete typed operations. No library changes needed.

### `DynProblem` trait (CLI-local)

```rust
trait DynProblem {
    fn evaluate(&self, config: &[usize]) -> String;
    fn serialize_json(&self) -> serde_json::Value;
    fn as_any(&self) -> &dyn Any;
    fn dims(&self) -> Vec<usize>;
}
```

Blanket impl for all `Problem + Serialize + 'static` types.

### Dispatch function (CLI-local)

```rust
fn load_problem(name: &str, variant: &Map, data: Value) -> Result<Box<dyn DynProblem>> {
    match name {
        "MaximumIndependentSet" => match graph_variant(variant) {
            "SimpleGraph" => deser::<MIS<SimpleGraph, i32>>(data),
            "UnitDiskGraph" => deser::<MIS<UnitDiskGraph, i32>>(data),
            ...
        },
        "QUBO" => deser::<QUBO<f64>>(data),
        "Satisfiability" => deser::<Satisfiability>(data),
        ...
    }
}
```

## Commands

### `pred evaluate <input.json> --config 1,0,1,0`

1. Read JSON file
2. Parse type/variant/data
3. `load_problem()` → `Box<dyn DynProblem>`
4. Parse config string → `Vec<usize>`
5. Call `evaluate(config)` → print result

### `pred reduce <input.json> --to QUBO`

1. Read JSON file → source `Box<dyn DynProblem>`
2. Find reduction path from source to target via `ReductionGraph`
3. `graph.reduce_along_path(&path, source.as_any())` → `ReductionChain`
4. Serialize the final target problem via match table
5. Output full reduction bundle (source + target + path) as JSON

Always outputs the full bundle to preserve the information needed for solution extraction.

### `reduce` execution flow

```
source JSON → load_problem() → Box<dyn DynProblem>
    → as_any() → &dyn Any
    → graph.reduce_along_path(&path, &dyn Any) → ReductionChain
    → chain.target_problem_any() → &dyn Any
    → serialize_any_problem(name, variant, &dyn Any) → JSON
    → wrap in ReductionBundle { source, target, path }
```

The target serialization uses the same match table, trying each registered type for the target problem name.

## Library Refactoring

The old `ExecutablePath<S,T>` + `ChainedReduction<S,T>` + `make_executable::<S,T>()` are replaced with a single untyped `ReductionChain` + `reduce_along_path()`. This simplifies the API and gives the CLI untyped access for free.

## Files to Change

### Library (refactoring only)

- `src/rules/graph.rs` — replace `ExecutablePath`/`ChainedReduction`/`make_executable` with `ReductionChain`/`reduce_along_path` (DONE)
- `src/rules/mod.rs` — update re-exports
- `src/rules/traits.rs` — update doc comment
- `src/unit_tests/rules/graph.rs` — migrate callers
- `src/unit_tests/rules/reduction_path_parity.rs` — migrate callers
- `examples/chained_reduction_ksat_to_mis.rs` — migrate callers

### CLI (new features)

- `problemreductions-cli/src/dispatch.rs` — new: `DynProblem` trait, `load_problem()`, variant match table
- `problemreductions-cli/src/commands/evaluate.rs` — new: evaluate command
- `problemreductions-cli/src/commands/reduce.rs` — new: reduce command
- `problemreductions-cli/src/commands/mod.rs` — add modules
- `problemreductions-cli/src/main.rs` — wire up commands
- `problemreductions-cli/tests/cli_tests.rs` — add tests
- `docs/src/cli.md` — document new commands
