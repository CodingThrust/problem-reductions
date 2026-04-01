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

**MANDATORY (Check 9 from #974).** Add a builder in `src/example_db/rule_builders.rs` using the YES test vector instance. Register in `build_rule_examples()`. Follow existing patterns in the file.

## Step 4b: Add Example-DB Lookup Test

**MANDATORY (Check 10 from #974).** Verify the new example is discoverable in `src/unit_tests/example_db.rs`. Add the rule to the existing exhaustive lookup test, or add a standalone test.

## Step 5: Write Paper Entry

**MANDATORY (Check 11 from #974).** The Typst proof already exists from `/verify-reduction`. Integrate it into `docs/paper/reductions.typ` using the paper's macros.

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

### 5e. Build

```bash
make paper  # Must compile without errors or new completeness warnings
```

## Step 6: Regenerate Exports and Verify

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures    # Slow — needs fixtures for paper example data
make test clippy
```

## Step 7: Clean Up Verification Artifacts

**MANDATORY.** Verification artifacts must NOT be committed into the library. Remove them after the Rust implementation is complete:

```bash
git rm -f docs/paper/verify-reductions/*<source>*<target>*
```

The Typst proof content lives on in the paper entry. The Python scripts were scaffolding — the Rust tests are the permanent verification.

## Step 8: Commit and Create PR

```bash
git add src/rules/<source>_<target>.rs \
       src/unit_tests/rules/<source>_<target>.rs \
       src/rules/mod.rs \
       src/example_db/rule_builders.rs \
       src/unit_tests/example_db.rs \
       docs/paper/reductions.typ
git commit -m "feat: add <Source> → <Target> reduction (#<ISSUE>)

Implemented via /verify-reduction → /add-reduction pipeline.
Verification: N checks (constructor) + M checks (adversary), 0 failures."
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
