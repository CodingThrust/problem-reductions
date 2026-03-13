# Plan: Add PartiallyOrderedKnapsack Model

Fixes #534

## Overview

Add the PartiallyOrderedKnapsack problem model — a knapsack variant where items are subject to a partial order (precedence constraints). Including an item requires including all its predecessors. Modeled as an optimization problem (maximize total value subject to precedence + capacity constraints), consistent with the existing Knapsack model.

## Design Decisions

- **Optimization, not satisfaction**: Following the check-issue recommendation and consistency with existing `Knapsack`, model as `OptimizationProblem` with `Direction::Maximize`. The capacity constraint is part of feasibility; the objective is total value. No `target_value` field needed.
- **Precedences as edge list**: Store `Vec<(usize, usize)>` where `(a, b)` means item `a` must precede item `b` (a < b in the partial order). The full precedence requirement is the transitive closure.
- **Complexity**: Use `"2^num_items"` as a baseline (naive enumeration). The problem is strongly NP-hard, so no pseudo-polynomial algorithm exists for general partial orders.
- **Category**: `misc/` — unique input structure (items + partial order + capacity), doesn't fit graph/formula/set/algebraic.

## Steps

### Step 1: Implement the model (`src/models/misc/partially_ordered_knapsack.rs`)

- `ProblemSchemaEntry` with fields: `sizes`, `values`, `precedences`, `capacity`
- Struct: `PartiallyOrderedKnapsack { sizes: Vec<i64>, values: Vec<i64>, precedences: Vec<(usize, usize)>, capacity: i64 }`
- Constructor: `new(sizes, values, precedences, capacity)` — assert sizes.len() == values.len(), validate precedence indices
- Getters: `sizes()`, `values()`, `capacity()`, `precedences()`, `num_items()`, `num_precedences()`
- `Problem` impl: `NAME = "PartiallyOrderedKnapsack"`, `Metric = SolutionSize<i64>`, `dims() = vec![2; n]`, `variant() = variant_params![]`
- `evaluate()`: check config length/values, check downward-closure (compute transitive closure, verify for each selected item all predecessors are selected), check capacity, return total value
- `OptimizationProblem` impl: `Value = i64`, `direction() = Maximize`
- `declare_variants!`: `PartiallyOrderedKnapsack => "2^num_items"`
- Test link: `#[cfg(test)] #[path = "../../unit_tests/models/misc/partially_ordered_knapsack.rs"] mod tests;`

### Step 2: Register the model

- `src/models/misc/mod.rs`: add `mod partially_ordered_knapsack;` and `pub use`
- `src/models/mod.rs`: add to `misc` re-export line

### Step 3: Register in CLI

- `problemreductions-cli/src/dispatch.rs`: add `load_problem` and `serialize_any_problem` arms
- `problemreductions-cli/src/problem_name.rs`: add `"partiallyorderedknapsack" | "pok"` alias (POK is not well-established, so only add lowercase identity mapping, no short alias)

### Step 4: Add CLI creation support

- `problemreductions-cli/src/commands/create.rs`: add `"PartiallyOrderedKnapsack"` arm parsing `--sizes`, `--values`, `--capacity`, `--precedences`
- `problemreductions-cli/src/cli.rs`: add `--values` and `--precedences` flags to `CreateArgs`, update `all_data_flags_empty()`, update help table
- Note: `--sizes` and `--capacity` already exist from BinPacking

### Step 5: Write unit tests (`src/unit_tests/models/misc/partially_ordered_knapsack.rs`)

- `test_partially_ordered_knapsack_basic`: construct instance, verify dims, getters
- `test_partially_ordered_knapsack_evaluate_valid`: valid downward-closed set within capacity
- `test_partially_ordered_knapsack_evaluate_precedence_violation`: select item without predecessor
- `test_partially_ordered_knapsack_evaluate_overweight`: valid precedence but over capacity
- `test_partially_ordered_knapsack_evaluate_empty`: empty selection
- `test_partially_ordered_knapsack_brute_force`: solver finds optimal
- `test_partially_ordered_knapsack_serialization`: round-trip serde
- `test_partially_ordered_knapsack_direction`: verify Maximize
- Use example from issue: items a-f with precedences a<c, a<d, b<e, d<f, e<f

### Step 6: Document in paper

Add `problem-def("PartiallyOrderedKnapsack")` and `display-name` entry in `docs/paper/reductions.typ`.

### Step 7: Verify

```bash
make test clippy
```
