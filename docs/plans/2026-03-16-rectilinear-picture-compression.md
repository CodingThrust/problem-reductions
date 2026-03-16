# Plan: Add RectilinearPictureCompression Model

Fixes #443

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `RectilinearPictureCompression` |
| 2 | Mathematical definition | Given an m×n binary matrix M and positive integer K, is there a collection of K or fewer axis-aligned all-1 rectangles that covers precisely the 1-entries of M? |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | None |
| 5 | Struct fields | `matrix: Vec<Vec<bool>>`, `bound_k: usize` |
| 6 | Configuration space | `vec![2; R]` where R = number of maximal all-1 rectangles in the matrix |
| 7 | Feasibility check | Union of selected rectangles covers exactly all 1-entries, and count ≤ K |
| 8 | Objective function | N/A (satisfaction) — returns `true` iff feasible |
| 9 | Best known exact algorithm | Brute-force: O(2^(num_rows * num_cols)) — no significantly better algorithm known (Masek, 1978) |
| 10 | Solving strategy | BruteForce works (enumerate subsets of maximal all-1 rectangles) |
| 11 | Category | `misc` (binary matrix input — unique structure) |
| 12 | Expected outcome | Instance 1 (6×6, K=3): YES — 3 rectangles cover all 1-entries. Instance 2 (same, K=2): NO |

Associated rule: #458 [Rule] 3SAT to Rectilinear Picture Compression (not an orphan).

## Batch 1: Implementation (Steps 1–5.5)

All steps in this batch are independent and can be parallelized within a single subagent.

### Step 1: Category & file
- Category: `misc/`
- File: `src/models/misc/rectilinear_picture_compression.rs`

### Step 1.5: Size getters
- `num_rows()` → `self.matrix.len()`
- `num_cols()` → `self.matrix.first().map_or(0, |r| r.len())`
- `bound_k()` → `self.bound_k`

### Step 2: Implement the model

Create `src/models/misc/rectilinear_picture_compression.rs`:

**Struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectilinearPictureCompression {
    matrix: Vec<Vec<bool>>,
    bound_k: usize,
}
```

**Constructor & getters:**
- `new(matrix, bound_k)` — validate all rows have equal length
- `matrix()`, `bound_k()`, `num_rows()`, `num_cols()`

**Core helper — `maximal_rectangles(&self) -> Vec<(usize, usize, usize, usize)>`:**

Enumerate all maximal all-1 sub-rectangles `(r1, r2, c1, c2)` where `r1..=r2` are row bounds and `c1..=c2` are column bounds. Algorithm:
1. Enumerate all all-1 rectangles by iterating over starting positions `(r1, c1)` and extending rows while narrowing the column range
2. Collect into a set
3. Filter to keep only maximal ones (no proper superset in the set)
4. Sort lexicographically for deterministic ordering

Since candidates are all-1 by construction, they can never cover a 0-entry. This simplifies evaluate().

**Problem trait:**
```rust
impl Problem for RectilinearPictureCompression {
    const NAME: &'static str = "RectilinearPictureCompression";
    type Metric = bool;
    fn variant() -> Vec<(&'static str, &'static str)> { crate::variant_params![] }
    fn dims(&self) -> Vec<usize> { vec![2; self.maximal_rectangles().len()] }
    fn evaluate(&self, config: &[usize]) -> bool {
        let candidates = self.maximal_rectangles();
        if config.len() != candidates.len() { return false; }
        // Collect selected rectangles (config[i] == 1)
        // Check: count of selected ≤ bound_k
        // Check: union of selected covers ALL 1-entries in matrix
        // (No need to check 0-coverage — candidates are all-1 by construction)
    }
}
impl SatisfactionProblem for RectilinearPictureCompression {}
```

**ProblemSchemaEntry:**
```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "RectilinearPictureCompression",
        display_name: "Rectilinear Picture Compression",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Can a binary matrix be covered by K or fewer axis-aligned all-1 rectangles?",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<bool>>", description: "Binary matrix" },
            FieldInfo { name: "bound_k", type_name: "usize", description: "Maximum number of rectangles" },
        ],
    }
}
```

### Step 2.5: Variant complexity

```rust
crate::declare_variants! {
    default sat RectilinearPictureCompression => "2^(num_rows * num_cols)",
}
```

Conservative bound: number of maximal all-1 rectangles R ≤ num_rows * num_cols in worst case.

### Step 3: Register the model

1. `src/models/misc/mod.rs` — add `pub(crate) mod rectilinear_picture_compression;` and `pub use rectilinear_picture_compression::RectilinearPictureCompression;`. Also add to `canonical_model_example_specs()` chain.
2. `src/models/mod.rs` — add `RectilinearPictureCompression` to the misc re-export line.

### Step 4: CLI discovery

`problemreductions-cli/src/problem_name.rs`:
- Add `"rectilinearpicturecompression" => "RectilinearPictureCompression"` to `resolve_alias()`
- No short alias (none established in literature)

### Step 4.5: CLI creation support

`problemreductions-cli/src/commands/create.rs`:
- Add match arm for `"RectilinearPictureCompression"` using existing `parse_bool_matrix(args)` and `args.k` (reuse `--k` flag for bound_k)
- Add to `example_command_line()`:
  ```
  "RectilinearPictureCompression" => "--matrix '1,1,0;0,1,1' --k 2",
  ```

`problemreductions-cli/src/cli.rs`:
- Add `RectilinearPictureCompression` to the help table under "Flags by problem type":
  ```
  RectilinearPictureCompression  --matrix, --k
  ```

### Step 4.6: Canonical model example

In the model file, add (feature-gated under `example-db`):
```rust
#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "rectilinear_picture_compression",
        build: || {
            // 4x4 matrix with two disjoint all-1 blocks (small enough for brute force)
            let matrix = vec![
                vec![true, true, false, false],
                vec![true, true, false, false],
                vec![false, false, true, true],
                vec![false, false, true, true],
            ];
            let problem = RectilinearPictureCompression::new(matrix, 2);
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![],  // samples filled by brute force
            )
        },
    }]
}
```

### Step 5: Unit tests

Create `src/unit_tests/models/misc/rectilinear_picture_compression.rs`.

Link from model file: `#[cfg(test)] #[path = "../../unit_tests/models/misc/rectilinear_picture_compression.rs"] mod tests;`

Required tests:
- `test_rectilinear_picture_compression_basic` — construct 6×6 instance from issue, verify `num_rows()`, `num_cols()`, `bound_k()`, `dims()` length, NAME, variant()
- `test_rectilinear_picture_compression_evaluation` — verify satisfying config (3 correct rectangles selected) returns true; verify unsatisfying configs (wrong selection, too many) return false
- `test_rectilinear_picture_compression_no_solution` — same matrix with bound_k=2, verify no satisfying config exists
- `test_rectilinear_picture_compression_serialization` — serde JSON roundtrip
- `test_rectilinear_picture_compression_solver` — BruteForce::find_satisfying() returns Some for K=3, None for K=2
- `test_rectilinear_picture_compression_paper_example` — use the paper example instance, verify expected outcome, verify find_all_satisfying() count
- `test_rectilinear_picture_compression_edge_cases` — empty matrix, all-zeros matrix, single cell

### Step 5.5: trait_consistency

`src/unit_tests/trait_consistency.rs`:
- Add `check_problem_trait(RectilinearPictureCompression::new(...))` call in `test_all_problems_implement_trait_correctly`

## Batch 2: Paper Entry (Step 6)

Depends on Batch 1 completion (needs `make paper` exports).

### Step 6: Document in paper

`docs/paper/reductions.typ`:

**6a. Display name:**
```typst
"RectilinearPictureCompression": [Rectilinear Picture Compression],
```

**6b. Problem definition:**
```typst
#problem-def("RectilinearPictureCompression")[
  Given an $m times n$ binary matrix $M$ and a positive integer $K$,
  determine whether there exists a collection of at most $K$
  axis-aligned rectangles that covers precisely the 1-entries of $M$.
  Each rectangle $(a, b, c, d)$ with $a <= b$ and $c <= d$ covers
  entries $M_(i j)$ for $a <= i <= b$ and $c <= j <= d$, and every
  covered entry must satisfy $M_(i j) = 1$.
][
  ... body ...
]
```

**6c. Body:**
- Background: classical NP-complete problem (Garey & Johnson A4 SR25). Applications in image compression, DNA array synthesis, IC manufacturing, ACL minimization. NP-completeness via 3SAT (Masek, 1978).
- Best known algorithms: no algorithm significantly improving on brute-force is known; cite Masek 1978 and Applegate et al. 2007.
- Example: CeTZ diagram showing a small binary matrix (e.g., 4×4 with two blocks) with rectangles highlighted in color. Show the covering solution.
- Evaluation: show that selected rectangles cover all 1-entries.

**6d. Build:** `make paper` — must compile without errors.

## Reference files
- Satisfaction model pattern: `src/models/misc/subset_sum.rs`
- Model tests: `src/unit_tests/models/misc/subset_sum.rs`
- CLI create: `problemreductions-cli/src/commands/create.rs` (see BMF for bool matrix pattern)
- Example specs: `src/example_db/specs.rs` (satisfaction_example helper)
- Trait consistency: `src/unit_tests/trait_consistency.rs`
