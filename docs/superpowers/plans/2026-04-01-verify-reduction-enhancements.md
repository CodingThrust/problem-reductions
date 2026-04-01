# Verify-Reduction Enhancements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enhance `/verify-reduction` with auto-matching, test vector export, and typed adversary prompts. Create new `/add-reduction` skill that consumes verified artifacts. Keep original `/add-rule` and `/review-pipeline` unchanged.

**Architecture:** Three file changes — update verify-reduction SKILL.md (new Step 4.5, updated Step 5/8), create add-reduction SKILL.md (fork of add-rule that reads Python reduce() + test vectors JSON), register add-reduction in CLAUDE.md.

**Tech Stack:** Markdown skill definitions only — no code changes.

---

### Task 1: Update verify-reduction SKILL.md — Add Step 4.5 (Auto-Matching + Test Vectors Export)

**Files:**
- Modify: `.claude/skills/verify-reduction/SKILL.md`

The new Step 4.5 goes between the current Step 4 (Run and Iterate) and Step 5 (Adversary Verification). It adds three things: (a) constructor exports test vectors JSON, (b) Typst ↔ JSON cross-check, (c) structured claim tags replace manual gap analysis.

- [ ] **Step 1: Insert new Step 4.5 after the current "Iteration 3: Gap analysis" block**

In `.claude/skills/verify-reduction/SKILL.md`, find the line `If any claim has no test, add one. If it's untestable, document WHY.` (end of Step 4, Iteration 3). After it, insert:

```markdown
### Iteration 4: Export test vectors and validate Typst matching — MANDATORY

After all checks pass and gap analysis is complete, the constructor script must export a test vectors JSON file for downstream consumption:

**File:** `docs/paper/verify-reductions/test_vectors_<source>_<target>.json`

```json
{
  "source": "<Source>",
  "target": "<Target>",
  "issue": <ISSUE_NUMBER>,
  "yes_instance": {
    "input": { ... },
    "output": { ... },
    "source_feasible": true,
    "target_feasible": true,
    "source_solution": [ ... ],
    "extracted_solution": [ ... ]
  },
  "no_instance": {
    "input": { ... },
    "output": { ... },
    "source_feasible": false,
    "target_feasible": false
  },
  "overhead": {
    "field_name": "expression using source getters"
  },
  "claims": [
    {"tag": "overhead_universe_size", "formula": "2*n", "verified": true},
    {"tag": "forward_correctness", "description": "NAE-sat implies valid splitting", "verified": true}
  ]
}
```

Add this export at the end of `main()` in the constructor script:

```python
import json

test_vectors = {
    "source": "<Source>",
    "target": "<Target>",
    "issue": <ISSUE>,
    "yes_instance": { ... },   # from Section 6
    "no_instance": { ... },    # from Section 7
    "overhead": { ... },       # from Section 1/4
    "claims": claims_list,     # accumulated claim() calls
}

with open("docs/paper/verify-reductions/test_vectors_<source>_<target>.json", "w") as f:
    json.dump(test_vectors, f, indent=2)
print(f"Test vectors exported to test_vectors_<source>_<target>.json")
```

**Typst ↔ JSON cross-check:**

After exporting, load both the test vectors JSON and the Typst file. For each key numerical value in the JSON (input sizes, target values, output sizes, feasibility verdicts), check that it appears as a substring in the Typst YES/NO example sections. This is a substring search on the raw Typst text, not a full parser:

```python
typst_text = open("<typst_file>").read()
for val in [str(v) for v in yes_instance["input"].values() if isinstance(v, (int, list))]:
    assert str(val) in typst_text, f"Typst missing YES value: {val}"
for val in [str(v) for v in no_instance["input"].values() if isinstance(v, (int, list))]:
    assert str(val) in typst_text, f"Typst missing NO value: {val}"
```

If any value is missing, the Typst proof and Python script are out of sync — fix before proceeding.

**Structured claims (best-effort replacement for manual gap analysis):**

Instead of the manual CLAIM/TESTED BY table, accumulate claims programmatically:

```python
claims_list = []

def claim(tag, formula_or_desc, verified=True):
    claims_list.append({"tag": tag, "formula": formula_or_desc, "verified": verified})
```

Call `claim()` throughout the constructor script wherever a Typst proof claim is verified. The self-review step (Step 6) checks that all claims have `verified: true` and that the claim count is reasonable (at least 5 claims for any non-trivial reduction).
```

- [ ] **Step 2: Verify the edit is well-placed**

Read the file and confirm Step 4.5 appears between Step 4 and Step 5 with correct markdown heading level.

- [ ] **Step 3: Commit**

```bash
git add .claude/skills/verify-reduction/SKILL.md
git commit -m "feat: add Step 4.5 to verify-reduction — test vectors export + Typst auto-matching"
```

---

### Task 2: Update verify-reduction SKILL.md — Typed Adversary Prompt (Step 5)

**Files:**
- Modify: `.claude/skills/verify-reduction/SKILL.md`

- [ ] **Step 1: Add reduction-type detection and tailored instructions to the adversary prompt**

In `.claude/skills/verify-reduction/SKILL.md`, find the adversary prompt template in Step 5a (the block starting `You are an adversary verifier`). Before the `## Your task` section of the adversary prompt, insert:

````markdown
## Reduction type

Detect the reduction type from the Typst proof and tailor your testing focus:

- **Identity reduction** (same graph/structure, different objective — keywords: "complement", "same graph", "negation"): Focus on exhaustive enumeration of all source instances for n ≤ 6. Test every possible configuration. Edge-case configs (all-zero, all-one, alternating) are highest priority.

- **Algebraic reduction** (padding, case split, formula transformation — keywords: "padding", "case", "if Σ", "d ="): Focus on case boundary conditions. Test instances where the case selection changes (e.g., Σ = 2T exactly, Σ = 2T ± 1). Verify extraction logic for each case independently. Include at least one hypothesis strategy targeting boundary values.

- **Gadget reduction** (widget/component construction — keywords: "widget", "component", "gadget", "cover-testing"): Focus on widget structure invariants. Verify each traversal/usage pattern independently. Test that interior vertices/elements have no external connections. Check structural properties (connectivity, edge counts, degree sequences) across all small instances.
````

- [ ] **Step 2: Commit**

```bash
git add .claude/skills/verify-reduction/SKILL.md
git commit -m "feat: add typed adversary prompts to verify-reduction (identity/algebraic/gadget)"
```

---

### Task 3: Update verify-reduction SKILL.md — Downstream Artifacts (Step 8) and Integration

**Files:**
- Modify: `.claude/skills/verify-reduction/SKILL.md`

- [ ] **Step 1: Update Step 8a commit to include test vectors JSON**

Find the `git add` block in Step 8a. Change it to:

```bash
git add docs/paper/<typst-file>.typ \
       docs/paper/verify-reductions/verify_*.py \
       docs/paper/verify-reductions/adversary_*.py \
       docs/paper/verify-reductions/test_vectors_*.json
git add -f docs/paper/<typst-file>.pdf
```

- [ ] **Step 2: Update the Integration section at the bottom of the file**

Replace the current Integration section with:

```markdown
## Integration

### Pipeline: Issue → verify-reduction → add-reduction → review-pipeline

`/verify-reduction` is a **pre-verification gate**. The Python `reduce()` function is the verified spec. `/add-reduction` translates it to Rust. `/review-pipeline`'s agentic test confirms the Rust matches.

- **Before `/add-reduction`**: `/verify-reduction` produces Typst proof + Python scripts + test vectors JSON
- **During `/add-reduction`**: reads Python `reduce()` as pseudocode, overhead from test vectors JSON, generates Rust tests from test vectors
- **During `/review-pipeline`**: agentic test runs `pred reduce`/`pred solve` on test vector instances; compositional check via alternative paths if available

### Standalone usage

- **After `write-rule-in-paper`**: invoke to verify paper entry
- **During `review-structural`**: check verification script exists and passes
- **Before `issue-to-pr --execute`**: pre-validate the algorithm
```

- [ ] **Step 3: Add test vectors JSON to the Quality Gates checklist**

In the Quality Gates section, add after the cross-consistency items:

```markdown
- [ ] Test vectors JSON exported with YES/NO instances, overhead, and claims
- [ ] Typst ↔ JSON auto-matching passed (key values present in Typst text)
```

- [ ] **Step 4: Add test vectors JSON to the Self-Review checklist (Step 6)**

In Step 6, add a new subsection after "Cross-consistency":

```markdown
### Test vectors and auto-matching

- [ ] `test_vectors_<source>_<target>.json` exported successfully
- [ ] YES instance in JSON matches Typst feasible example (values present)
- [ ] NO instance in JSON matches Typst infeasible example (values present)
- [ ] All claims have `verified: true`
- [ ] At least 5 claims for non-trivial reductions
```

- [ ] **Step 5: Commit**

```bash
git add .claude/skills/verify-reduction/SKILL.md
git commit -m "feat: update verify-reduction Step 8 + Integration for downstream pipeline"
```

---

### Task 4: Create add-reduction skill

**Files:**
- Create: `.claude/skills/add-reduction/SKILL.md`

This is a fork of add-rule that reads verified artifacts instead of deriving from the issue.

- [ ] **Step 1: Create the skill directory**

```bash
mkdir -p .claude/skills/add-reduction
```

- [ ] **Step 2: Write the SKILL.md**

Create `.claude/skills/add-reduction/SKILL.md` with the following content:

```markdown
---
name: add-reduction
description: Add a new reduction rule using verified artifacts from /verify-reduction — reads Python reduce() as pseudocode, test vectors JSON for Rust tests, overhead from JSON
---

# Add Reduction (from Verified Artifacts)

Step-by-step guide for adding a new reduction rule (A → B) when `/verify-reduction` has already produced verified artifacts (Typst proof, Python scripts, test vectors JSON). This skill consumes those artifacts directly instead of re-deriving from the issue.

**When to use:** After `/verify-reduction` has produced a PR with verified artifacts for a reduction rule issue. Use `/add-rule` instead when no verification artifacts exist.

## Step 0: Locate Verified Artifacts

Check for existing verification artifacts:

```bash
ls docs/paper/verify-reductions/verify_<source>_<target>.py
ls docs/paper/verify-reductions/test_vectors_<source>_<target>.json
ls docs/paper/verify-reductions/<source>_<target>.typ
```

If any are missing, run `/verify-reduction` first.

### Read the artifacts

1. **Python `reduce()` function** — this is the verified spec for the Rust `reduce_to()` implementation. Read it carefully; translate the algorithm, not the syntax.
2. **Test vectors JSON** — contains YES/NO instances with exact input/output values, overhead expressions, and verified claims.
3. **Typst proof** — the Construction section describes the algorithm in mathematical notation. Use for doc comments.

```bash
# Load test vectors
TEST_VECTORS=$(cat docs/paper/verify-reductions/test_vectors_<source>_<target>.json)
```

Extract from test vectors JSON:
- `overhead` → use directly in `#[reduction(overhead = { ... })]`
- `yes_instance.input` / `yes_instance.output` → first closed-loop test case
- `no_instance.input` / `no_instance.output` → infeasible test case
- `claims` → verify each is preserved in the Rust implementation

## Reference Implementations

Same as `/add-rule`:
- **Reduction rule:** `src/rules/minimumvertexcover_maximumindependentset.rs`
- **Reduction tests:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`
- **Paper entry:** search `docs/paper/reductions.typ` for `MinimumVertexCover` `MaximumIndependentSet`
- **Traits:** `src/rules/traits.rs` (`ReduceTo<T>`, `ReduceToAggregate<T>`, `ReductionResult`, `AggregateReductionResult`)

## Step 1: Implement the reduction

Create `src/rules/<source>_<target>.rs`.

**Translation guide:** Map the Python `reduce()` function to Rust:

| Python | Rust |
|--------|------|
| `reduce(n, clauses)` → `(universe_size, subsets)` | `fn reduce_to(&self) -> Self::Result` |
| `extract_assignment(n, config)` | `fn extract_solution(&self, target_sol: &[usize]) -> Vec<usize>` |
| `literal_to_element(lit, n)` | Private helper method |
| Python list of ints | `Vec<usize>`, `Vec<CNFClause>`, etc. (match problem type) |

**Overhead from test vectors JSON:** The `overhead` field maps directly to the `#[reduction]` macro:

```rust
#[reduction(overhead = {
    // Copy expressions verbatim from test_vectors JSON "overhead" field
    field_name = "expression",
})]
```

The rest of the implementation structure follows `/add-rule` Step 1 exactly: ReductionResult struct, trait impl, ReduceTo impl.

## Step 2: Register in mod.rs

Same as `/add-rule` Step 2. Add `mod <source>_<target>;` to `src/rules/mod.rs`.

## Step 3: Write unit tests from test vectors

Create `src/unit_tests/rules/<source>_<target>.rs`.

**Generate tests directly from test vectors JSON:**

The YES instance becomes the primary closed-loop test:

```rust
#[test]
fn test_<source>_to_<target>_closed_loop() {
    // Construct source from test_vectors.yes_instance.input
    let source = <SourceType>::try_new(/* fields from JSON */).unwrap();

    // Reduce
    let reduction = ReduceTo::<TargetType>::reduce_to(&source);

    // Verify target matches test_vectors.yes_instance.output
    let target = reduction.target_problem();
    assert_eq!(target.<field>(), /* value from JSON output */);

    // Solve and extract
    let solver = BruteForce;
    for witness in solver.find_all_witnesses(target).unwrap() {
        let extracted = reduction.extract_solution(&witness);
        // Verify extracted solution is valid for source
        let val = source.evaluate(&extracted);
        assert!(val.0); // Or check objective value
    }
}
```

The NO instance becomes the infeasible test:

```rust
#[test]
fn test_<source>_to_<target>_infeasible() {
    // Construct source from test_vectors.no_instance.input
    let source = <SourceType>::try_new(/* fields from JSON */).unwrap();

    // Reduce
    let reduction = ReduceTo::<TargetType>::reduce_to(&source);

    // Verify target is also infeasible
    let solver = BruteForce;
    let witnesses = solver.find_all_witnesses(reduction.target_problem());
    assert!(witnesses.is_none() || witnesses.unwrap().is_empty());
}
```

Add additional structural tests as needed (target size, edge count, etc.) guided by the `claims` field in the test vectors JSON.

## Step 4: Add canonical example to example_db

Same as `/add-rule` Step 4. The YES instance from the test vectors JSON is a good candidate for the canonical example.

## Step 5: Document in paper

The Typst proof already exists from `/verify-reduction`. Integrate it into `docs/paper/reductions.typ` using the `reduction-rule` template. The proof text, worked examples, and overhead table are already written — adapt them to the paper's macros (`reduction-rule`, `problem-def`, etc.).

Follow `/add-rule` Step 5 for the exact format. The heavy writing is already done; this step is reformatting.

## Step 6: Regenerate exports and verify

Same as `/add-rule` Step 6:

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
```

## Solver Rules

Same as `/add-rule`. If the target problem needs ILP, implement alongside.

## CLI Impact

Same as `/add-rule`. No CLI changes needed for witness-preserving reductions.

## File Naming

Same as `/add-rule`:
- Rule file: `src/rules/<sourcelower>_<targetlower>.rs`
- Test file: `src/unit_tests/rules/<sourcelower>_<targetlower>.rs`
- Canonical example: builder function in `src/example_db/rule_builders.rs`

## Common Mistakes

All mistakes from `/add-rule` apply, plus:

| Mistake | Fix |
|---------|-----|
| Re-deriving algorithm from issue instead of reading Python `reduce()` | The Python function is the verified spec — translate it, don't reinvent |
| Ignoring test vectors JSON | Use the YES/NO instances for Rust tests directly |
| Overhead expressions don't match test vectors JSON | Copy verbatim from the `overhead` field |
| Skipping the infeasible (NO) test case | The NO instance is in the test vectors — always include it |
| Not integrating the existing Typst proof into the paper | The proof is already written; reformat, don't rewrite |
```

- [ ] **Step 3: Commit**

```bash
git add .claude/skills/add-reduction/SKILL.md
git commit -m "feat: create add-reduction skill — consumes verified artifacts from verify-reduction"
```

---

### Task 5: Register add-reduction in CLAUDE.md

**Files:**
- Modify: `.claude/CLAUDE.md`

- [ ] **Step 1: Add add-reduction entry to the Skills list**

In `.claude/CLAUDE.md`, find the line:
```
- [add-rule](skills/add-rule/SKILL.md) -- Add a new reduction rule. Can be used standalone (brainstorms with user) or called from `issue-to-pr`.
```

Add immediately after it:
```
- [add-reduction](skills/add-reduction/SKILL.md) -- Add a new reduction rule from verified artifacts (Python reduce() + test vectors JSON from `/verify-reduction`). Use instead of `add-rule` when verification artifacts exist.
```

- [ ] **Step 2: Update the verify-reduction description**

Find the line starting `- [verify-reduction]` and replace it with:

```
- [verify-reduction](skills/verify-reduction/SKILL.md) -- End-to-end verification of a reduction rule: generates Typst proof (with YES+NO examples), Python verification script (7 mandatory sections, ≥5000 checks, exhaustive n≤5), adversary Python script (≥5000 independent checks), and test vectors JSON for downstream consumption by `add-reduction`. Iterates until all checks pass. Creates worktree + PR.
```

- [ ] **Step 3: Commit**

```bash
git add .claude/CLAUDE.md
git commit -m "feat: register add-reduction skill in CLAUDE.md, update verify-reduction description"
```

---

Plan complete and saved to `docs/superpowers/plans/2026-04-01-verify-reduction-enhancements.md`. Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

Which approach?