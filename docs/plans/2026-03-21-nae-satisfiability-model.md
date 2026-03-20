# NAESatisfiability Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `NAESatisfiability` formula model, wire it into the registry/CLI/example-db/paper flows, and cover the issue’s worked example and NAE-specific semantics with tests.

**Architecture:** Implement `NAESatisfiability` as a sibling of `Satisfiability`, reusing `CNFClause` but giving the model its own clause-evaluation logic: every clause must contain at least one true literal and at least one false literal. Keep the registration path fully registry-backed so the CLI, exports, and paper all discover the new model through the normal model/example mechanisms.

**Tech Stack:** Rust workspace, serde, inventory registry, `declare_variants!`, cargo tests, Typst paper, GitHub issue `#143`.

---

**Issue:** `#143` `[Model] NAESatisfiability`
**Implementation skill:** repo-local `add-model`
**Execution mode:** batch the paper work separately after the Rust implementation and example-db wiring are stable.

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `NAESatisfiability` |
| 2 | Mathematical definition | Given a CNF formula, find an assignment such that every clause has at least one true literal and at least one false literal |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | None |
| 5 | Struct fields | `num_vars: usize`, `clauses: Vec<CNFClause>` |
| 6 | Configuration space | `vec![2; num_vars]` |
| 7 | Feasibility check | Convert config to Boolean assignment; each clause must contain both truth values among its literal occurrences |
| 8 | Objective function | `bool` |
| 9 | Best known exact algorithm | brute-force, issue gives `2^num_vars`; for `declare_variants!` use the repo’s accepted getter name (`2^num_variables` if `num_vars` is rejected at compile time) |
| 10 | Solving strategy | existing `BruteForce` solver |
| 11 | Category | `src/models/formula/` |
| 12 | Expected outcome | issue example assignment `[0, 0, 0, 1, 1]` is satisfying; complement `[1, 1, 1, 0, 0]` is also satisfying; total satisfying assignments = `10` |

## Associated Rule Check

- Existing cross-reference: GitHub issue `#382` `[Rule] NOT-ALL-EQUAL 3SAT to SET SPLITTING`
- Planned follow-ons in the issue body: `Satisfiability -> NAESatisfiability`, `NAESatisfiability -> MaxCut`
- Result: safe to proceed; this model is not an orphan.

## Design Notes

- Reuse [`src/models/formula/sat.rs`](/Users/jinguomini/rcode/problem-reductions/.worktrees/issue-143/src/models/formula/sat.rs) for the struct/registry/test pattern, but do not reuse `CNFClause::is_satisfied()` for model semantics.
- Add model-local helpers such as `count_nae_satisfied`, `is_nae_satisfying`, and a literal-evaluation helper that works on literal occurrences, not deduplicated variables.
- Enforce the issue schema invariant that each clause has at least two literals in the constructor. Empty formulas remain valid; empty or unit clauses inside a formula should be rejected at construction time.
- No new CLI flags should be necessary: existing `--num-vars` and `--clauses` are enough.
- CLI alias resolution is registry-backed in the current codebase, so prefer defining aliases in `ProblemSchemaEntry` over adding manual alias tables unless implementation proves otherwise.

## Batch Plan

- **Batch 1:** model implementation, registration, CLI create support, example-db, and Rust tests
- **Batch 2:** paper entry, paper-example alignment, final verification

## Batch 1

### Task 1: Add failing model tests first

**Files:**
- Create: `src/unit_tests/models/formula/nae_satisfiability.rs`

**Step 1: Write the failing tests**

Add tests that lock down the issue semantics before implementation:
- `test_nae_satisfiability_creation`
- `test_nae_clause_requires_true_and_false_literals`
- `test_nae_satisfying_example_from_issue`
- `test_nae_complement_symmetry_for_issue_example`
- `test_nae_solver_counts_ten_solutions_for_issue_example`
- `test_nae_empty_formula_is_trivially_satisfying`
- `test_nae_constructor_rejects_short_clauses`
- `test_nae_get_clause_and_num_literals`
- `test_nae_serialization_round_trip`

Use the exact issue example with 5 variables and 5 clauses. The solver-count test should assert that `BruteForce::find_all_satisfying()` returns `10` satisfying assignments and contains both the issue assignment and its complement.

**Step 2: Run the targeted test file and confirm it fails**

Run:
```bash
cargo test nae_satisfiability --lib
```

Expected: compile failure because the model module does not exist yet.

### Task 2: Implement the new model

**Files:**
- Create: `src/models/formula/nae_satisfiability.rs`
- Modify: `src/models/formula/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Implement the model file**

Mirror the structure of `sat.rs`:
- `inventory::submit!` with `ProblemSchemaEntry`
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- fields `num_vars`, `clauses`
- getters `num_vars()`, `num_clauses()`, `num_literals()`, `clauses()`, `get_clause()`
- `Problem` impl with `Metric = bool`, `dims()`, `evaluate()`, `variant()`
- `impl SatisfactionProblem`
- `declare_variants!` entry for the default variant
- `#[cfg(feature = "example-db")]` canonical example spec
- `#[cfg(test)]` path link to the new test file

Add model-local helpers:
- `config_to_assignment(config: &[usize]) -> Vec<bool>`
- `literal_value(lit: i32, assignment: &[bool]) -> bool`
- `clause_is_nae_satisfied(clause: &CNFClause, assignment: &[bool]) -> bool`
- `is_nae_satisfying(&self, assignment: &[bool]) -> bool`
- `count_nae_satisfied(&self, assignment: &[bool]) -> usize`

`clause_is_nae_satisfied` should evaluate every literal occurrence, then return `true` iff at least one evaluated literal is `true` and at least one is `false`.

**Step 2: Register the model**

Update module exports:
- `src/models/formula/mod.rs`: add module declaration, public re-export, and include its example specs in the formula example chain
- `src/models/mod.rs`: add `NAESatisfiability` to the formula re-export block
- `src/lib.rs`: add `NAESatisfiability` to the public formula exports

**Step 3: Run the new targeted tests**

Run:
```bash
cargo test nae_satisfiability --lib
```

Expected: the new test file passes or exposes only the remaining registration/example-db gaps.

### Task 3: Wire CLI creation and example-db

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `src/models/formula/mod.rs`

**Step 1: Add CLI create support**

In `problemreductions-cli/src/commands/create.rs`:
- add an `example_for("NAESatisfiability")` usage string beside the SAT entries
- add a `create()` match arm for `"NAESatisfiability"` reusing the existing clause parser and `--num-vars`
- import the new model if the file needs it

Do not add new flags unless the existing parser proves insufficient.

**Step 2: Confirm example-db discovery**

Ensure the new model file exposes `canonical_model_example_specs()` and that `src/models/formula/mod.rs` includes it in the aggregated example spec list. `src/example_db/model_builders.rs` should then pick it up automatically through the existing formula chain, so no direct edit is expected there.

**Step 3: Run CLI/example-focused tests**

Run:
```bash
cargo test nae_satisfiability --lib
cargo test create --package problemreductions-cli
```

Expected: the new model can be constructed by the CLI path and the model tests still pass.

### Task 4: Full Rust verification for Batch 1

**Files:**
- No new files; verification only

**Step 1: Run focused formatting/build checks**

Run:
```bash
cargo fmt --all
cargo test nae_satisfiability --lib
cargo test --package problemreductions-cli create
```

**Step 2: Commit Batch 1**

Run:
```bash
git add src/models/formula/nae_satisfiability.rs src/models/formula/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/formula/nae_satisfiability.rs problemreductions-cli/src/commands/create.rs
git commit -m "Add NAESatisfiability model"
```

## Batch 2

### Task 5: Add the paper entry and align the worked example

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add display name**

Insert a display-name entry for `NAESatisfiability` near the other formula problems.

**Step 2: Add `problem-def("NAESatisfiability")`**

Use the SAT and K-SAT entries as the reference pattern. The body should include:
- a short explanation of NAE-SAT and complement symmetry
- the brute-force complexity statement with citation already present in `references.bib`
- the exact 5-variable issue example
- a sentence showing why `[0,0,0,1,1]` satisfies every clause
- a note that `[1,1,1,0,0]` is also satisfying by complement symmetry

Prefer to load the canonical example from example-db rather than duplicating raw data.

### Task 6: Add/finish the paper-example test and run paper verification

**Files:**
- Modify: `src/unit_tests/models/formula/nae_satisfiability.rs`
- Modify: `docs/paper/reductions.typ`

**Step 1: Ensure there is a dedicated paper-example test**

Add `test_nae_satisfiability_paper_example` that:
- constructs the exact issue/paper instance
- asserts the issue solution evaluates to `true`
- asserts the complement evaluates to `true`
- asserts the satisfying-solution count is `10`

**Step 2: Build the paper**

Run:
```bash
make paper
```

Expected: Typst compiles without errors and the new problem definition is included.

### Task 7: Final verification

**Files:**
- No new files; verification only

**Step 1: Run project verification**

Run:
```bash
make test
make clippy
```

If either command surfaces unrelated pre-existing failures, capture them explicitly before proceeding.

**Step 2: Commit Batch 2**

Run:
```bash
git add docs/paper/reductions.typ src/unit_tests/models/formula/nae_satisfiability.rs
git commit -m "Document NAESatisfiability"
```

## Handoff Notes for Execution

- Keep the implementation close to `Satisfiability`; avoid premature abstractions shared between SAT and NAE-SAT in this PR.
- Treat the issue’s 5-variable example as the source of truth for the canonical example, test data, and paper write-up.
- Before final push, verify the plan file itself is deleted from the branch as required by `issue-to-pr`.
