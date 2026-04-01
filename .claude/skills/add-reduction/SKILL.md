---
name: add-reduction
description: Add a new reduction rule using verified artifacts from /verify-reduction — reads Python reduce() as pseudocode, test vectors JSON for Rust tests, overhead from JSON
---

# Add Reduction (from Verified Artifacts)

Complete pipeline for adding a reduction rule when `/verify-reduction` has produced verified artifacts. Translates Python `reduce()` to Rust, writes tests from test vectors, adds example-db entry, writes paper entry, then cleans up verification artifacts.

**When to use:** After `/verify-reduction` PR exists. Use `/add-rule` when no verification artifacts exist.

## Step 0: Locate and Read Verified Artifacts

```bash
ls docs/paper/verify-reductions/verify_<source>_<target>.py
ls docs/paper/verify-reductions/test_vectors_<source>_<target>.json
ls docs/paper/verify-reductions/<source>_<target>.typ
```

If any are missing, run `/verify-reduction` first.

**Read these three artifacts:**
1. **Python `reduce()` function** — the verified spec. Translate the algorithm to Rust, not the syntax.
2. **Test vectors JSON** — YES/NO instances with exact values, overhead expressions, verified claims.
3. **Typst proof** — Construction section for doc comments, proof for paper entry.

## Step 1: Implement the Reduction

Create `src/rules/<source>_<target>.rs`. Follow the pattern in `src/rules/satisfiability_naesatisfiability.rs`.

**Translation guide from Python to Rust:**

| Python | Rust |
|--------|------|
| `reduce(n, clauses)` → `(universe_size, subsets)` | `fn reduce_to(&self) -> Self::Result` |
| `extract_assignment(n, config)` | `fn extract_solution(&self, target_sol: &[usize]) -> Vec<usize>` |
| Helper functions (e.g., `literal_to_element`) | Private functions in the same file |
| Python list of ints | `Vec<usize>`, `Vec<CNFClause>`, etc. (match the target problem's API) |

**Required structure:**

```rust
/// Result struct — holds target + any state needed for extraction.
#[derive(Debug, Clone)]
pub struct ReductionXToY { target: TargetType, /* mapping state */ }

impl ReductionResult for ReductionXToY {
    type Source = SourceType;
    type Target = TargetType;
    fn target_problem(&self) -> &Self::Target { &self.target }
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Translate Python extract_assignment() logic
    }
}

#[reduction(overhead = {
    // Copy verbatim from test_vectors JSON "overhead" field
    field_name = "expression",
})]
impl ReduceTo<TargetType> for SourceType {
    type Result = ReductionXToY;
    fn reduce_to(&self) -> Self::Result {
        // Translate Python reduce() logic
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/<source>_<target>.rs"]
mod tests;
```

## Step 2: Register in mod.rs

Add to `src/rules/mod.rs`:
```rust
pub(crate) mod <source>_<target>;
```

## Step 3: Write Unit Tests from Test Vectors

Create `src/unit_tests/rules/<source>_<target>.rs`.

**From test vectors JSON, generate at minimum:**

1. **Closed-loop test** (`test_<source>_to_<target>_closed_loop`) — construct source from `yes_instance.input`, reduce, verify target matches `yes_instance.output`, solve target with `BruteForce`, extract solutions, verify each is valid for source.

2. **Infeasible test** (`test_<source>_to_<target>_infeasible`) — construct source from `no_instance.input`, reduce, verify target is also infeasible (no witnesses).

3. **Structural test** — verify target dimensions match overhead formula, check well-formedness.

Reference: `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`

## Step 4: Add Canonical Example to example_db

**HARD GATE (Check 9 from #974).** You MUST modify `src/example_db/rule_builders.rs` — not the rule file, not anywhere else. Read the file first, follow existing patterns, add a builder function using the YES test vector instance, register in `build_rule_examples()`.

**Verification — run this and confirm the file was modified:**
```bash
git diff --name-only | grep "rule_builders.rs"
# MUST show: src/example_db/rule_builders.rs
# If it does NOT appear, you skipped this step. Go back and do it.
```

## Step 4b: Add Example-DB Lookup Test

**HARD GATE (Check 10 from #974).** You MUST modify `src/unit_tests/example_db.rs`. Read the file first, add the new rule to the existing exhaustive lookup test or add a standalone test.

**Verification:**
```bash
git diff --name-only | grep "example_db.rs"
# MUST show: src/unit_tests/example_db.rs
# If it does NOT appear, you skipped this step. Go back and do it.
```

## Step 5: Write Paper Entry

**HARD GATE (Check 11 from #974).** You MUST modify `docs/paper/reductions.typ`. The Typst proof already exists from `/verify-reduction` — reformat it into the paper's macros. Do NOT skip this step.

### 5a. Load example data

```typst
#let src_tgt = load-example("Source", "Target")
#let src_tgt_sol = src_tgt.solutions.at(0)
```

### 5b. Write reduction-rule entry

```typst
#reduction-rule("Source", "Target",
  example: true,
  example-caption: [Description ($n = ...$, $|E| = ...$)],
)[
  // Theorem body: complexity + construction summary + overhead hint
][
  // Proof body: _Construction._ ... _Correctness._ ... _Solution extraction._ ...
  // Adapt from the verified Typst proof — reformat, don't rewrite
]
```

### 5c. Write worked example (extra block)

Step-by-step walkthrough with concrete numbers from JSON data. Must include:
- `pred-commands()` block at top (create/reduce/solve/evaluate)
- Source instance display (graph visualization if applicable)
- Construction walkthrough with intermediate values
- Solution verified end-to-end
- Witness multiplicity note

**Gold-standard reference:** Search for `reduction-rule("KColoring", "QUBO"` in `reductions.typ`.

### 5d. Register display name (if new problem in paper)

Add to `display-name` dictionary if the problem doesn't have an entry yet.

### 5e. Build and verify

```bash
make paper  # Must compile without errors or new completeness warnings
```

**Verification — confirm the paper file was modified:**
```bash
git diff --name-only | grep "reductions.typ"
# MUST show: docs/paper/reductions.typ
# If it does NOT appear, you skipped Step 5. Go back and do it.
```

## Step 6: Regenerate Exports, CI Checks, and Dominated-Rules Gate

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures    # Slow — needs fixtures for paper example data
```

### CI-equivalent checks — MANDATORY

These must pass before committing. CI will fail if you skip them.

```bash
# Clippy with -D warnings (CI uses this exact flag — local clippy without -D is insufficient)
cargo clippy -- -D warnings 2>&1 | tail -5

# Full test suite (catches dominated-rules test failures)
cargo test 2>&1 | tail -5
```

### Dominated-rules gate — CHECK THIS

Adding a new reduction can create paths that **dominate** existing direct reductions. The test `test_find_dominated_rules_returns_known_set` in `src/unit_tests/rules/analysis.rs` has a hardcoded set of known dominated pairs. If your new reduction creates a shorter path to a target that already has a direct reduction, this test will fail.

```bash
cargo test test_find_dominated_rules_returns_known_set 2>&1 | tail -10
```

If it fails with "Unexpected dominated rule: X -> Y (dominated by X -> ... -> Y)", add the new pair to the known set in `src/unit_tests/rules/analysis.rs`.

## Step 7: Clean Up Verification Artifacts

**MANDATORY.** Verification artifacts must NOT be committed into the library. Remove them after the Rust implementation is complete:

```bash
git rm -f docs/paper/verify-reductions/*<source>*<target>*
```

The Typst proof content lives on in the paper entry. The Python scripts were scaffolding — the Rust tests are the permanent verification.

## Step 8: Pre-Commit Gate and Create PR

**Before committing, run this checklist. ALL must pass:**

```bash
# Gate 1: Required files modified
echo "=== Pre-commit file gate ==="
for f in \
  "src/rules/<source>_<target>.rs" \
  "src/unit_tests/rules/<source>_<target>.rs" \
  "src/rules/mod.rs" \
  "src/example_db/rule_builders.rs" \
  "src/unit_tests/example_db.rs" \
  "docs/paper/reductions.typ"; do
  git diff --name-only HEAD | grep -q "$(basename $f)" && echo "  ✓ $f" || echo "  ✗ MISSING: $f"
done

# Gate 2: No verification artifacts remaining
ls docs/paper/verify-reductions/*<source>*<target>* 2>/dev/null && echo "  ✗ ARTIFACTS NOT CLEANED" || echo "  ✓ Artifacts cleaned"

# Gate 3: Clippy with -D warnings (CI uses this — local clippy alone is insufficient)
cargo clippy -- -D warnings 2>&1 | tail -3

# Gate 4: Full test suite (catches dominated-rules failures)
cargo test 2>&1 | tail -3

# Gate 5: Dominated-rules test specifically
cargo test test_find_dominated_rules_returns_known_set 2>&1 | tail -3

# Gate 6: Paper compiles
make paper 2>&1 | tail -3
```

**If ANY file shows ✗ MISSING, STOP and go back to the skipped step.** Do NOT commit with missing files.

```bash
git add src/rules/<source>_<target>.rs \
       src/unit_tests/rules/<source>_<target>.rs \
       src/rules/mod.rs \
       src/example_db/rule_builders.rs \
       src/unit_tests/example_db.rs \
       docs/paper/reductions.typ
git commit -m "feat: add <Source> → <Target> reduction (#<ISSUE>)"
git push -u origin "<branch>"
gh pr create --title "feat: add <Source> → <Target> reduction (#<ISSUE>)" --body "..."
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Re-deriving algorithm from issue instead of Python `reduce()` | Python function is the verified spec — translate it |
| Overhead expressions don't match test vectors JSON | Copy verbatim from `overhead` field |
| Skipping infeasible (NO) test case | NO instance is in test vectors — always include |
| Missing canonical example in `rule_builders.rs` | MANDATORY — Check 9 from #974 |
| Missing example-db lookup test | MANDATORY — Check 10 from #974 |
| Missing paper `reduction-rule` entry | MANDATORY — Check 11 from #974 |
| Leaving verification artifacts in repo | MANDATORY cleanup — Step 7 |
| Not regenerating fixtures after example-db | `make regenerate-fixtures` required for paper |
| Running `cargo clippy` without `-D warnings` | CI uses `-D warnings` — local clippy without it misses lint failures |
| New reduction dominates existing direct reduction | Check `test_find_dominated_rules_returns_known_set` — add new pair to known set in `analysis.rs` |
| Skipping full `cargo test` before commit | Dominated-rules test only runs in full suite, not filtered tests |
