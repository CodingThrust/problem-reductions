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

**HARD GATE (Check 9 from #974).** Add a `canonical_rule_example_specs()` function in the reduction rule file, and wire it into `src/rules/mod.rs` where other specs are collected. Read `src/rules/mod.rs` first — search for `canonical_rule_example_specs` to see how existing rules register their examples.

The builder function must:
1. Construct the source instance from the YES test vector
2. Reduce it to the target
3. Return a `RuleExampleSpec` with build closure

**Verification:**
```bash
# The rule file must define canonical_rule_example_specs
grep "canonical_rule_example_specs" src/rules/<source>_<target>.rs
# mod.rs must wire it in
grep "<source>_<target>::canonical_rule_example_specs" src/rules/mod.rs
# Both must show results. If either is empty, go back and do it.
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

## Step 8: Parent-Side Verification and Commit

**Do NOT trust the subagent's self-report.** After the implementation subagent reports DONE, the parent (you) MUST run these checks independently:

### 8a. File gate (parent runs this, not the subagent)

```bash
echo "=== Parent-side file gate ==="
REQUIRED_PATTERNS=(
  "<source>_<target>.rs"    # rule file
  "mod.rs"                  # module registration
  "example_db.rs"           # lookup test
  "reductions.typ"          # paper entry
)
CHANGED=$(git diff --name-only HEAD)
for pat in "${REQUIRED_PATTERNS[@]}"; do
  echo "$CHANGED" | grep -q "$pat" && echo "  ✓ $pat" || echo "  ✗ MISSING: $pat — send subagent back"
done

# Canonical example wired?
grep -q "canonical_rule_example_specs" src/rules/<source>_<target>.rs && echo "  ✓ canonical_rule_example_specs in rule file" || echo "  ✗ MISSING canonical_rule_example_specs"
grep -q "<source>_<target>::canonical_rule_example_specs" src/rules/mod.rs && echo "  ✓ wired in mod.rs" || echo "  ✗ MISSING mod.rs wiring"

# Artifacts cleaned?
ls docs/paper/verify-reductions/*<source>*<target>* 2>/dev/null && echo "  ✗ ARTIFACTS NOT CLEANED" || echo "  ✓ Artifacts cleaned"
```

**If ANY line shows ✗, send the subagent back to fix it.** Do NOT proceed.

### 8b. CI-equivalent checks (parent runs)

```bash
cargo clippy -- -D warnings 2>&1 | tail -3
cargo test 2>&1 | tail -3
make paper 2>&1 | tail -3
```

### 8c. Commit and push

Only after ALL gates pass:

```bash
git add -A
git commit -m "feat: add <Source> → <Target> reduction (#<ISSUE>)"
git push -u origin "<branch>"
gh pr create --title "feat: add <Source> → <Target> reduction (#<ISSUE>)" --body "..."
```

## Pre-Commit Hook

Install this hook to mechanically block commits missing required files. The hook runs automatically — subagents cannot bypass it.

**File:** `.claude/hooks/add-reduction-precommit.sh`

```bash
#!/bin/bash
# Pre-commit hook for add-reduction: blocks commits missing required files.
# Install: copy to .git/hooks/pre-commit or add to settings.json hooks.

STAGED=$(git diff --cached --name-only)

# Only run for reduction rule commits
if ! echo "$STAGED" | grep -q "src/rules/.*\.rs$"; then
  exit 0
fi

# Extract the reduction name from staged rule files
RULE_FILE=$(echo "$STAGED" | grep "^src/rules/" | grep -v mod.rs | grep -v traits.rs | head -1)
if [ -z "$RULE_FILE" ]; then
  exit 0
fi

ERRORS=0

# Check example_db.rs is staged
if ! echo "$STAGED" | grep -q "example_db.rs"; then
  echo "BLOCKED: src/unit_tests/example_db.rs not staged (Check 10 from #974)"
  ERRORS=$((ERRORS + 1))
fi

# Check reductions.typ is staged
if ! echo "$STAGED" | grep -q "reductions.typ"; then
  echo "BLOCKED: docs/paper/reductions.typ not staged (Check 11 from #974)"
  ERRORS=$((ERRORS + 1))
fi

# Check mod.rs is staged
if ! echo "$STAGED" | grep -q "mod.rs"; then
  echo "BLOCKED: src/rules/mod.rs not staged"
  ERRORS=$((ERRORS + 1))
fi

# Check no verification artifacts are staged
if echo "$STAGED" | grep -q "verify-reductions/"; then
  echo "BLOCKED: verification artifacts still staged — run git rm first"
  ERRORS=$((ERRORS + 1))
fi

if [ $ERRORS -gt 0 ]; then
  echo ""
  echo "Fix the above and re-commit. See /add-reduction skill for details."
  exit 1
fi
```

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Re-deriving algorithm from issue instead of Python `reduce()` | Python function is the verified spec — translate it |
| Overhead expressions don't match test vectors JSON | Copy verbatim from `overhead` field |
| Skipping infeasible (NO) test case | NO instance is in test vectors — always include |
| Missing `canonical_rule_example_specs()` in rule file + `mod.rs` wiring | MANDATORY — Check 9 from #974 |
| Missing example-db lookup test in `example_db.rs` | MANDATORY — Check 10 from #974 |
| Missing paper `reduction-rule` entry | MANDATORY — Check 11 from #974 |
| Leaving verification artifacts in repo | MANDATORY cleanup — Step 7 |
| Not regenerating fixtures after example-db | `make regenerate-fixtures` required for paper |
| Running `cargo clippy` without `-D warnings` | CI uses `-D warnings` — local clippy without it misses lint failures |
| New reduction dominates existing direct reduction | Check `test_find_dominated_rules_returns_known_set` — add new pair to known set in `analysis.rs` |
| Skipping full `cargo test` before commit | Dominated-rules test only runs in full suite, not filtered tests |
