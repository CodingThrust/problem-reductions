# Plan: Add StringToStringCorrection Model

**Issue:** #439 [Model] StringToStringCorrection
**Skill:** add-model
**Date:** 2026-03-16

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `StringToStringCorrection` |
| 2 | Mathematical definition | Given finite alphabet Sigma, two strings x,y in Sigma*, and positive integer K. Can y be derived from x by K or fewer operations of single symbol deletion or adjacent symbol interchange? |
| 3 | Problem type | Satisfaction (bool) |
| 4 | Type parameters | None |
| 5 | Struct fields | `alphabet_size: usize`, `source: Vec<usize>`, `target: Vec<usize>`, `bound_k: usize` |
| 6 | Configuration space | `vec![2 * source.len() + 1; bound_k]` — K operation slots, each in {0..2*source_length} where 0..source_length = delete at position i, source_length..2*source_length = swap positions i and i+1, 2*source_length = no-op |
| 7 | Feasibility check | Apply operations left-to-right to mutable copy of source; skip no-ops; return false if any delete/swap index is out of bounds for current intermediate string |
| 8 | Objective function | `bool` — true iff result equals target after all operations |
| 9 | Best known algorithm | Brute-force: O((2*source_length+1)^bound_k) — Wagner (1975) proved NP-completeness |
| 10 | Solving strategy | BruteForce only |
| 11 | Category | `misc/` |
| 12 | Expected outcome | source=[0,1,2,3,1,0], target=[0,1,3,2,1], bound_k=2, alphabet_size=4. Solution: swap(2,3) then delete(5) → answer YES. Minimum cost is exactly 2. |

**Associated rules:** #453 [Rule] SET COVERING to STRING-TO-STRING CORRECTION

## Batch 1: Implementation (Steps 1-5.5)

### Step 1: Create model file

Create `src/models/misc/string_to_string_correction.rs` following `ShortestCommonSupersequence` as reference.

**Schema entry:**
```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "StringToStringCorrection",
        display_name: "String-to-String Correction",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Derive target string from source using at most K deletions and adjacent swaps",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the finite alphabet" },
            FieldInfo { name: "source", type_name: "Vec<usize>", description: "Source string (symbol indices)" },
            FieldInfo { name: "target", type_name: "Vec<usize>", description: "Target string (symbol indices)" },
            FieldInfo { name: "bound_k", type_name: "usize", description: "Maximum number of operations allowed" },
        ],
    }
}
```

**Struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringToStringCorrection {
    alphabet_size: usize,
    source: Vec<usize>,
    target: Vec<usize>,
    bound_k: usize,
}
```

**Constructor:** Validate that `alphabet_size > 0` when source or target is non-empty, and all symbols in source/target are < alphabet_size.

**Getters:** `alphabet_size()`, `source()`, `target()`, `bound_k()`, `source_length()`, `target_length()`.

**Problem trait:**
- `NAME = "StringToStringCorrection"`
- `type Metric = bool`
- `variant() -> crate::variant_params![]`
- `dims() -> vec![2 * self.source.len() + 1; self.bound_k]`
- `evaluate()`: Apply operations left-to-right to a mutable Vec copy of source. For each operation slot:
  - value < current_len → delete at that position
  - value >= current_len && value < current_len + (current_len - 1) → swap adjacent at position (value - current_len) and (value - current_len + 1)
  - value == 2 * source.len() (the no-op sentinel)
  - Any other value → return false (invalid operation for current string state)
  - After all ops, return result == target

  **IMPORTANT encoding note:** The config space is fixed at `2 * source.len() + 1` per slot. But as the string gets shorter from deletions, valid operation indices change. Values that were valid initially may become out-of-bounds after deletions. The evaluate function must handle this dynamically: check each operation against the *current* intermediate string length, not the original source length.

**SatisfactionProblem impl:** marker trait, no methods.

**declare_variants!:**
```rust
crate::declare_variants! {
    default sat StringToStringCorrection => "(2 * source_length + 1) ^ bound_k",
}
```

### Step 2: Register the model

1. `src/models/misc/mod.rs` — add `pub(crate) mod string_to_string_correction;` and `pub use string_to_string_correction::StringToStringCorrection;`
2. `src/models/mod.rs` — add `StringToStringCorrection` to the `misc` re-export line
3. Update module doc comment in `misc/mod.rs`

### Step 3: Register for CLI discovery

In `problemreductions-cli/src/problem_name.rs` — no short alias needed (no well-established abbreviation).

### Step 4: Add CLI creation support

In `problemreductions-cli/src/commands/create.rs`:
- Add a match arm for `"StringToStringCorrection"` that parses `--source-string`, `--target-string`, `--bound`, `--alphabet-size`
- Source and target strings are comma-separated symbol indices (e.g., "0,1,2,3,1,0")

In `problemreductions-cli/src/cli.rs`:
- Add `--source-string` and `--target-string` flags to `CreateArgs`
- Add entry to "Flags by problem type" help table
- Update `all_data_flags_empty()`

Add example hint in `example_hint()`.

### Step 5: Add canonical model example

In the model file, add `canonical_model_example_specs()` function (feature-gated with `example-db`):
- Use the issue's example: alphabet_size=4, source=[0,1,2,3,1,0], target=[0,1,3,2,1], bound_k=2
- Sample config: the known solution (swap at position 2, then delete at position 5)

Register in `src/models/misc/mod.rs` `canonical_model_example_specs()`.

### Step 6: Write unit tests

Create `src/unit_tests/models/misc/string_to_string_correction.rs`:

- `test_string_to_string_correction_creation` — construct instance, verify dims
- `test_string_to_string_correction_evaluation` — verify evaluate() on the known solution and invalid configs
- `test_string_to_string_correction_serialization` — round-trip serde
- `test_string_to_string_correction_solver` — BruteForce finds satisfying solution
- `test_string_to_string_correction_paper_example` — verify paper example instance, evaluate expected solution, check all satisfying solutions count
- `test_string_to_string_correction_invalid_operations` — test out-of-bounds operations return false
- `test_string_to_string_correction_unsatisfiable` — test case where no solution exists within bound

Link test file via `#[cfg(test)] #[path = "..."] mod tests;`.

### Step 7: Add trait_consistency entry

In `src/unit_tests/trait_consistency.rs`:
- Add `check_problem_trait(...)` call with a small StringToStringCorrection instance
- No `test_direction` entry needed (satisfaction problem)

## Batch 2: Paper Documentation (Step 6 from add-model)

### Step 8: Write paper entry

In `docs/paper/reductions.typ`:

1. Add display name: `"StringToStringCorrection": [String-to-String Correction]`

2. Write `problem-def("StringToStringCorrection")`:
   - **Definition:** Given finite alphabet $Sigma$, source string $x in Sigma^*$, target string $y in Sigma^*$, and positive integer $K$, determine whether $y$ can be derived from $x$ by a sequence of at most $K$ operations, where each operation is either a single-symbol deletion or an adjacent-symbol interchange.
   - **Background:** Classical NP-complete problem from Garey & Johnson (A4 SR20). Wagner (1975) proved NP-completeness via transformation from Set Covering. The standard edit distance (insert, delete, change) is solvable in O(|x|*|y|) time by Wagner-Fischer (1974), but restricting to only deletions and adjacent swaps makes the problem NP-complete.
   - **Example:** Use the issue example with a visualization showing the source string, the two operations (swap + delete), and the resulting target string.

3. Run `make paper` to verify compilation.
