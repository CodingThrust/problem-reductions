# Plan: Add Minesweeper Model (#135)

## Overview
Add the Minesweeper Consistency problem as a `SatisfactionProblem`. This is a decision problem: given a partially revealed Minesweeper grid, determine if there exists a valid mine assignment for unrevealed cells satisfying all revealed cell constraints.

**Reference:** Kaye, R. (2000). "Minesweeper is NP-complete." The Mathematical Intelligencer, 22(2), 9–15.

## Information Summary

| Property | Value |
|----------|-------|
| **Struct name** | `Minesweeper` |
| **Category** | `misc/` (unique grid-based input) |
| **Problem type** | Satisfaction (`Metric = bool`) |
| **Type parameters** | None |
| **Variables** | k binary variables (one per unrevealed cell) |
| **dims()** | `vec![2; k]` where k = unrevealed.len() |
| **evaluate()** | Check all revealed cell constraints are satisfied |
| **Complexity** | `O(2^num_unrevealed)` brute-force |
| **Getter** | `num_unrevealed()` → number of unrevealed cells |

## Steps

### Step 1: Implement the Model (`src/models/misc/minesweeper.rs`)

**Struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minesweeper {
    rows: usize,
    cols: usize,
    revealed: Vec<(usize, usize, u8)>,  // (row, col, mine_count)
    unrevealed: Vec<(usize, usize)>,     // (row, col)
}
```

**Constructor (`new`):**
- Validate: rows, cols > 0
- Validate: all revealed positions in bounds, counts 0..=8
- Validate: all unrevealed positions in bounds
- Validate: no overlap between revealed and unrevealed positions
- Store fields

**Accessors:**
- `rows()`, `cols()`, `revealed()`, `unrevealed()`
- `num_unrevealed()` — for complexity expression

**`inventory::submit!` for ProblemSchemaEntry:**
```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "Minesweeper",
        description: "Minesweeper Consistency: determine if a valid mine assignment exists",
        fields: &[
            SchemaField { name: "rows", type_name: "usize", description: "Number of rows" },
            SchemaField { name: "cols", type_name: "usize", description: "Number of columns" },
            SchemaField { name: "revealed", type_name: "Vec<(usize,usize,u8)>", description: "Revealed cells (row,col,count)" },
            SchemaField { name: "unrevealed", type_name: "Vec<(usize,usize)>", description: "Unrevealed cell positions" },
        ],
    }
}
```

**Problem trait impl:**
```rust
impl Problem for Minesweeper {
    const NAME: &'static str = "Minesweeper";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.unrevealed.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        // For each revealed cell (r, c, count):
        //   Count how many of its 8 neighbors are unrevealed cells with config[i] == 1
        //   If count doesn't match, return false
        // Return true if all constraints satisfied
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for Minesweeper {}
```

**evaluate() algorithm:**
1. Build a HashMap mapping (row, col) → index into unrevealed list
2. For each revealed cell (r, c, count):
   - Count neighbors: iterate over 8 directions (dr, dc) ∈ {-1,0,1}²\{(0,0)}
   - For each neighbor (r+dr, c+dc) in bounds:
     - If it's in the unrevealed map at index i, add config[i] to sum
   - If sum != count, return false
3. Return true

**Variant complexity:**
```rust
crate::declare_variants! {
    Minesweeper => "2^num_unrevealed",
}
```

### Step 2: Register the Model

**`src/models/misc/mod.rs`:** Add `mod minesweeper;` and `pub use minesweeper::Minesweeper;`

**`src/models/mod.rs`:** Add `Minesweeper` to the `pub use misc::{...}` line.

### Step 3: Register in CLI

**`problemreductions-cli/src/dispatch.rs`:**
- In `load_problem()`: add `"Minesweeper" => deser_sat::<Minesweeper>(data)`
- In `serialize_any_problem()`: add `"Minesweeper" => try_ser::<Minesweeper>(any)`

**`problemreductions-cli/src/problem_name.rs`:**
- Add alias: `"minesweeper" => "Minesweeper".to_string()`

### Step 4: Add CLI Creation Support

**`problemreductions-cli/src/cli.rs`:** Add new CLI flags:
```rust
/// Revealed cells for Minesweeper (semicolon-separated "row,col,count", e.g., "1,1,1;0,0,2")
#[arg(long)]
pub revealed: Option<String>,
```
Also add `rows` and `cols` as new `Option<usize>` fields.

Update `all_data_flags_empty()` to check `args.revealed.is_none() && args.rows.is_none() && args.cols.is_none()`.

**`problemreductions-cli/src/commands/create.rs`:**
- Add `"Minesweeper"` match arm that parses `--rows`, `--cols`, `--revealed`
- Unrevealed cells are computed automatically: all grid cells not in the revealed set
- Add example string in `example_for()`

### Step 5: Write Unit Tests (`src/unit_tests/models/misc/minesweeper.rs`)

Tests to write:
1. `test_minesweeper_creation` — construct 3×3 instance, verify dimensions
2. `test_minesweeper_evaluate_satisfiable` — Instance 1 from issue (3×3, center=1, YES)
3. `test_minesweeper_evaluate_unsatisfiable` — Instance 2 from issue (contradictory, NO)
4. `test_minesweeper_classic_pattern` — Instance 3 from issue (classic pattern, YES)
5. `test_minesweeper_serialization` — round-trip serde
6. `test_minesweeper_solver` — BruteForce finds satisfying assignment for Instance 1
7. `test_minesweeper_variant` — verify variant() returns empty

Link from model file:
```rust
#[cfg(test)]
#[path = "../../unit_tests/models/misc/minesweeper.rs"]
mod tests;
```

Also update `src/unit_tests/models/misc/mod.rs` to include the new test module.

### Step 6: Write Example (`examples/minesweeper_consistency.rs`)

Use Instance 3 from the issue (the classic pattern). Create the Minesweeper instance, evaluate a valid and invalid config, use BruteForce to find a satisfying assignment. Output JSON.

Must have `pub fn run()` and `fn main() { run() }`.

Register in `Cargo.toml` under `[[example]]`.

### Step 7: Verify

```bash
make fmt clippy test
```

All must pass. Check that the new model appears in `pred list` output and `pred create Minesweeper` shows help.

## File Manifest

| File | Action |
|------|--------|
| `src/models/misc/minesweeper.rs` | CREATE — model implementation |
| `src/models/misc/mod.rs` | EDIT — register module |
| `src/models/mod.rs` | EDIT — add re-export |
| `src/unit_tests/models/misc/minesweeper.rs` | CREATE — unit tests |
| `src/unit_tests/models/misc/mod.rs` | EDIT — register test module |
| `problemreductions-cli/src/dispatch.rs` | EDIT — add load/serialize arms |
| `problemreductions-cli/src/problem_name.rs` | EDIT — add alias |
| `problemreductions-cli/src/cli.rs` | EDIT — add --rows, --cols, --revealed flags |
| `problemreductions-cli/src/commands/create.rs` | EDIT — add Minesweeper creation |
| `examples/minesweeper_consistency.rs` | CREATE — example program |
| `Cargo.toml` | EDIT — register example |

## Dependencies
- Independent tasks: Steps 1-2 (model), Step 5 (tests skeleton)
- Sequential: Step 3-4 (CLI) depends on Step 2
- Step 6 (example) depends on Steps 1-2
- Step 7 (verify) depends on all prior steps
