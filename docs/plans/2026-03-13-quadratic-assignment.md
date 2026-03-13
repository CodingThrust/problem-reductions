# Plan: Add QuadraticAssignment Model (#300)

## Summary

Add the Quadratic Assignment Problem (QAP) to the codebase as an optimization model in `src/models/algebraic/`. QAP is a classical NP-hard problem (Sahni & Gonzalez, 1976) for facility-location assignment: given cost matrix C (flows between facilities) and distance matrix D (distances between locations), find an injection f minimizing Σ_{i,j} c_{ij} · d_{f(i),f(j)}.

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `QuadraticAssignment` |
| 2 | Mathematical definition | Given n×n cost matrix C and m×m distance matrix D, find injection f:{1..n}→{1..m} minimizing Σ c_{ij}·d_{f(i),f(j)} |
| 3 | Problem type | Optimization (Minimize) |
| 4 | Type parameters | None (no graph type; uses i64 matrices directly) |
| 5 | Struct fields | `cost_matrix: Vec<Vec<i64>>`, `distance_matrix: Vec<Vec<i64>>` |
| 6 | Configuration space | `vec![m; n]` — each facility i chooses a location in {0..m-1} |
| 7 | Feasibility check | Assignment must be injective (no two facilities share a location) |
| 8 | Objective function | Σ_{i≠j} cost_matrix[i][j] · distance_matrix[config[i]][config[j]] |
| 9 | Best known exact | Brute force: O(n! · n²). Branch-and-bound with Gilmore-Lawler bounds (Anstreicher 2003). Complexity string: `"num_facilities ^ num_facilities"` (n^n ≥ n!) |
| 10 | Solving strategy | BruteForce enumeration; ILP linearization possible but not required for this PR |
| 11 | Category | `algebraic` (two matrices as input) |

## Steps

### Step 1: Create model file `src/models/algebraic/quadratic_assignment.rs`

**File:** `src/models/algebraic/quadratic_assignment.rs`

Implement:
1. `inventory::submit!` with `ProblemSchemaEntry` — fields: `cost_matrix` (Vec<Vec<i64>>), `distance_matrix` (Vec<Vec<i64>>)
2. Struct `QuadraticAssignment` with fields `cost_matrix: Vec<Vec<i64>>`, `distance_matrix: Vec<Vec<i64>>`
3. Constructor `new(cost_matrix, distance_matrix)` — validate: cost_matrix is n×n, distance_matrix is m×m, n ≤ m
4. Getters: `cost_matrix()`, `distance_matrix()`, `num_facilities()` (= n), `num_locations()` (= m)
5. `Problem` trait impl:
   - `NAME = "QuadraticAssignment"`
   - `Metric = SolutionSize<i64>`
   - `dims()` → `vec![m; n]` (each of n facilities chooses from m locations)
   - `evaluate()`: check injectivity first (return `Invalid` if duplicates), then compute Σ_{i≠j} C[i][j]·D[f(i)][f(j)]
   - `variant()` → `crate::variant_params![]` (no type parameters)
6. `OptimizationProblem` impl: `direction()` → `Direction::Minimize`, `Value = i64`
7. `declare_variants!`: `QuadraticAssignment => "num_facilities ^ num_facilities"`
8. Test link: `#[cfg(test)] #[path = "../../unit_tests/models/algebraic/quadratic_assignment.rs"] mod tests;`

### Step 2: Register the model

1. **`src/models/algebraic/mod.rs`**: Add `mod quadratic_assignment;` and `pub use quadratic_assignment::QuadraticAssignment;`
2. **`src/models/mod.rs`**: Add `QuadraticAssignment` to the `algebraic` re-export line
3. **`src/lib.rs`**: If QUBO is in prelude, add QuadraticAssignment there too (check first)

### Step 3: Register in CLI

1. **`problemreductions-cli/src/dispatch.rs`**:
   - Add `use problemreductions::models::algebraic::QuadraticAssignment;` import
   - Add match arm in `load_problem()`: `"QuadraticAssignment" => deser_opt::<QuadraticAssignment>(data)`
   - Add match arm in `serialize_any_problem()`: `"QuadraticAssignment" => try_ser::<QuadraticAssignment>(any)`

2. **`problemreductions-cli/src/problem_name.rs`**:
   - Add `"quadraticassignment" | "qap" => "QuadraticAssignment".to_string()` in `resolve_alias()`
   - Add `("QAP", "QuadraticAssignment")` to `ALIASES` array (QAP is a well-established abbreviation)

3. **`problemreductions-cli/src/commands/create.rs`**:
   - Add match arm for `"QuadraticAssignment"` that parses `--matrix` for cost_matrix and a second `--distance-matrix` flag for distance_matrix (both semicolon-separated row format)
   - Add CLI flags if needed (`--distance-matrix`)
   - Update help text in `after_help`

### Step 4: Write unit tests

**File:** `src/unit_tests/models/algebraic/quadratic_assignment.rs`

Also update `src/unit_tests/models/algebraic/mod.rs` if it exists to include the new test module.

Tests:
- `test_quadratic_assignment_creation` — construct n=4 instance, verify num_facilities=4, num_locations=4, dims
- `test_quadratic_assignment_evaluate_valid` — test identity assignment f=(0,1,2,3) on the example instance, verify cost=38
- `test_quadratic_assignment_evaluate_swap` — test f=(0,2,1,3) on example, verify different cost
- `test_quadratic_assignment_evaluate_invalid` — test duplicate assignment like [0,0,1,2], verify SolutionSize::Invalid
- `test_quadratic_assignment_direction` — verify Direction::Minimize
- `test_quadratic_assignment_serialization` — round-trip serde JSON test
- `test_quadratic_assignment_solver` — BruteForce::find_best on the n=4 example, verify optimal cost=38

### Step 5: Write paper entry

Invoke `/write-model-in-paper` to add `QuadraticAssignment` problem-def entry in `docs/paper/reductions.typ`:
- Add to `display-name` dict: `"QuadraticAssignment": [Quadratic Assignment]`
- Formal definition, background (Koopmans-Beckmann 1957, Sahni-Gonzalez 1976)
- Example: the n=4 instance from the issue with CeTZ visualization (facility-location bipartite diagram)

### Step 6: Regenerate exports

```bash
make export-schemas  # Regenerate problem schemas JSON
make fmt             # Format code
```

### Step 7: Verify

```bash
make check  # fmt + clippy + test
```

## Example Instance (for tests)

Cost matrix C (4×4):
```
[[0, 5, 2, 0],
 [5, 0, 0, 3],
 [2, 0, 0, 4],
 [0, 3, 4, 0]]
```

Distance matrix D (4×4):
```
[[0, 1, 2, 3],
 [1, 0, 1, 2],
 [2, 1, 0, 1],
 [3, 2, 1, 0]]
```

Identity assignment f=(0,1,2,3): cost = 38
Optimal: cost = 38 (multiple permutations achieve this due to matrix symmetry)
