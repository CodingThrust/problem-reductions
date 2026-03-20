# Plan: Add QuantifiedBooleanFormulas Model

**Issue:** #571 — [Model] QuantifiedBooleanFormulas(qbf)(*)
**Skill:** add-model

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `QuantifiedBooleanFormulas` |
| 2 | Mathematical definition | Given a fully quantified Boolean formula F=(Q_1 u_1)...(Q_n u_n)E where each Q_i is ∀ or ∃ and E is a CNF formula, determine whether F is true |
| 3 | Problem type | Satisfaction (Metric = bool) |
| 4 | Type parameters | None |
| 5 | Struct fields | `num_vars: usize`, `quantifiers: Vec<Quantifier>`, `clauses: Vec<CNFClause>` |
| 6 | Configuration space | `vec![2; num_vars]` — each variable is 0 or 1 |
| 7 | Feasibility check | A config represents a full assignment; evaluate returns true iff the formula is true under that assignment (ignoring quantifier semantics in evaluate — quantifier semantics are captured by the brute-force solver's game-tree search) |
| 8 | Objective function | `bool` — satisfied or not under the given assignment |
| 9 | Best known exact algorithm | O(2^n) brute-force game-tree evaluation (Stockmeyer & Meyer, 1973); complexity string: `"2^num_vars"` |
| 10 | Solving strategy | BruteForce works — but needs special handling: `find_satisfying` must find a *witnessing assignment* for the existential variables such that for all universal variable assignments, E is satisfied. The `evaluate()` method just checks if a single full assignment satisfies the CNF matrix E (standard SAT-like evaluation). |
| 11 | Category | `formula` |

## Design Decisions

### evaluate() Semantics
Following the check-issue comment's analysis, `evaluate()` will treat the config as a full assignment and check whether the CNF matrix E is satisfied. This is consistent with how the `Problem` trait works (a single config → metric). The quantifier semantics are implicit: a QBF is TRUE iff there exists an assignment to existential variables such that for ALL universal variable assignments, E evaluates to true. The brute-force solver enumerates all 2^n assignments and returns any satisfying one.

### Quantifier Enum
Define a `Quantifier` enum with `Exists` and `ForAll` variants, serializable with serde.

### Reusing CNFClause
Reuse the existing `CNFClause` type from `sat.rs` (1-indexed signed integers).

## Steps

### Step 1: Implement the model (`src/models/formula/qbf.rs`)

1. Define `Quantifier` enum: `{ Exists, ForAll }` with `Debug, Clone, PartialEq, Eq, Serialize, Deserialize`
2. Define `QuantifiedBooleanFormulas` struct with fields: `num_vars`, `quantifiers`, `clauses`
3. Add `inventory::submit!` for `ProblemSchemaEntry`
4. Constructor: `new(num_vars, quantifiers, clauses)` with assertion that `quantifiers.len() == num_vars`
5. Getter methods: `num_vars()`, `num_clauses()`, `quantifiers()`, `clauses()`
6. Implement `Problem` trait:
   - `NAME = "QuantifiedBooleanFormulas"`
   - `Metric = bool`
   - `dims() = vec![2; num_vars]`
   - `evaluate(config)` — convert to bool assignment, check if all clauses are satisfied (same as SAT)
   - `variant() = variant_params![]`
7. Implement `SatisfactionProblem` (marker trait)
8. Add `declare_variants!` with complexity `"2^num_vars"`
9. Add `is_true(&self) -> bool` method that implements proper QBF game-tree evaluation (recursive minimax)
10. Link test file: `#[cfg(test)] #[path = "../../unit_tests/models/formula/qbf.rs"] mod tests;`

### Step 2: Register the model

1. `src/models/formula/mod.rs` — add `pub(crate) mod qbf;` and `pub use qbf::{QuantifiedBooleanFormulas, Quantifier};`
2. `src/models/mod.rs` — add `QuantifiedBooleanFormulas, Quantifier` to the formula re-export line
3. `src/lib.rs` prelude — add `QuantifiedBooleanFormulas` to the formula prelude exports

### Step 3: Register in CLI

1. `problemreductions-cli/src/dispatch.rs`:
   - Add import for `QuantifiedBooleanFormulas`
   - Add `"QuantifiedBooleanFormulas" => deser_sat::<QuantifiedBooleanFormulas>(data)` in `load_problem()`
   - Add `"QuantifiedBooleanFormulas" => try_ser::<QuantifiedBooleanFormulas>(any)` in `serialize_any_problem()`
2. `problemreductions-cli/src/problem_name.rs`:
   - Add `"qbf" | "quantifiedbooleanformulas" => "QuantifiedBooleanFormulas".to_string()` in `resolve_alias()`
   - Add `("QBF", "QuantifiedBooleanFormulas")` to `ALIASES` array

### Step 4: Add CLI creation support

1. `problemreductions-cli/src/commands/create.rs`:
   - Add `"QuantifiedBooleanFormulas"` match arm: parse `--num-vars`, `--clauses`, and a new `--quantifiers` flag
   - Add to `example_for()`: `"QuantifiedBooleanFormulas" => "--num-vars 3 --clauses \"1,2;-1,3\" --quantifiers \"E,A,E\""`
2. `problemreductions-cli/src/cli.rs`:
   - Add `--quantifiers` flag to `CreateArgs`: `pub quantifiers: Option<String>`
   - Update `all_data_flags_empty()` to include `args.quantifiers.is_none()`
   - Add QBF to "Flags by problem type" table

### Step 5: Write unit tests (`src/unit_tests/models/formula/qbf.rs`)

1. `test_quantifier_creation` — verify Quantifier enum
2. `test_qbf_creation` — construct instance, verify dimensions
3. `test_qbf_evaluate` — verify evaluate() on valid/invalid assignments
4. `test_qbf_is_true` — verify game-tree evaluation for known true/false instances
5. `test_qbf_solver` — verify brute-force solver finds satisfying assignments
6. `test_qbf_serialization` — round-trip serde test
7. `test_qbf_trivial` — empty formula, all-exists (reduces to SAT)

### Step 6: Document in paper

Add problem-def entry in `docs/paper/reductions.typ`:
- Add display name: `"QuantifiedBooleanFormulas": [Quantified Boolean Formulas (QBF)]`
- Add `#problem-def("QuantifiedBooleanFormulas")[...]` with formal definition and background

### Step 7: Verify

```bash
make test clippy fmt-check
```
