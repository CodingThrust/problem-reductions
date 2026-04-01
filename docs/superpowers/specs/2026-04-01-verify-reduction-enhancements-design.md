# Verify-Reduction Skill Enhancements

**Date:** 2026-04-01
**Status:** Draft
**Context:** Lessons from running `/verify-reduction` on 3 reductions (#841 NAE→SS, #973 SS→Partition, #198 VC→HC) and comparison with PR #975's verification approach.

## Problem

The verify-reduction skill (PR #979) produces high-quality mathematical verification (Typst proofs + dual Python scripts), but:

1. **Typst ↔ Python matching is manual.** The human writes examples in both places and hopes they agree. No automated cross-check.
2. **Gap analysis is manual.** The skill asks the agent to list every Typst claim and map it to a test — this is done as a text block, not verified programmatically.
3. **Verification is disconnected from Rust implementation.** The Python scripts verify the math, but when `/add-rule` implements the Rust code, it re-reads the issue from scratch. Verified artifacts aren't consumed downstream.
4. **No compositional testing.** The strongest verification — testing via alternative `pred` reduction paths — only becomes possible after the Rust implementation exists. The current skill doesn't set this up.

## Design

### Pipeline Integration

The verify-reduction skill is a **pre-verification gate** in a larger pipeline:

```
Issue → /verify-reduction → /add-rule → /review-pipeline
         (Typst + Python)   (Rust impl)   (agentic test)
```

The Python `reduce()` function is the verified spec. `/add-rule` translates it to Rust. `/review-pipeline`'s agentic test (`pred reduce`, `pred solve`) confirms the Rust matches.

### Enhancement 1: Typst ↔ Python Auto-Matching

#### Example-level (mandatory)

The constructor script exports test vectors to a JSON sidecar file:

```
docs/paper/verify-reductions/test_vectors_<source>_<target>.json
```

Contents:
```json
{
  "source": "SubsetSum",
  "target": "Partition",
  "yes_instance": {
    "input": {"sizes": [3, 5, 7, 1, 4], "target": 8},
    "output": {"sizes": [3, 5, 7, 1, 4, 4]},
    "source_feasible": true,
    "target_feasible": true,
    "source_solution": [0, 1, 1, 0, 0],
    "extracted_solution": [0, 1, 1, 0, 0]
  },
  "no_instance": {
    "input": {"sizes": [3, 7, 11], "target": 5},
    "output": {"sizes": [3, 7, 11, 11]},
    "source_feasible": false,
    "target_feasible": false
  },
  "overhead": {
    "num_elements": "num_elements + 1"
  }
}
```

A new validation step (Step 4.5) loads both the test vectors JSON and the Typst file, then checks that key numerical values from the JSON (input sizes, targets, output sizes, feasibility verdicts) appear in the Typst YES/NO example sections. The check is substring-based on the rendered numbers — not a full Typst parser. This catches divergence between the proof and the scripts (e.g., Typst says `S = {3, 5, 7}` but Python tests `S = {3, 5, 8}`).

The same JSON is consumed downstream:
- `/add-rule` reads it to generate Rust `#[test]` closed-loop cases
- `/review-pipeline`'s agentic test uses it for `pred reduce`/`pred solve` verification

#### Claim-level (best-effort)

The constructor script emits structured claim tags:

```python
claim("overhead_universe_size", "2*n", verified=True)
claim("forward_case2", "T + d = Sigma - T", verified=True)
claim("extraction_opposite_side", "sigma < 2T => opposite side from padding", verified=True)
```

Implementation: a `claims` list accumulated during `main()`, exported alongside the test vectors. The self-review step checks: every claim keyword should plausibly map to a Typst proof section. Claims without `verified=True` get flagged. This replaces the manual gap analysis table.

### Enhancement 2: Adversary Tailoring by Reduction Type

The adversary prompt is currently generic. Tailor it by reduction type:

| Type | Adversary Focus | Rationale |
|------|----------------|-----------|
| Identity (same graph, different objective) | Exhaustive enumeration, edge-case configs | Bugs hide in subtle objective differences |
| Algebraic (padding, complement, case split) | Symbolic case boundaries, off-by-one in padding | SS→Partition had 3 cases; boundary conditions are error-prone |
| Gadget (widget construction) | Widget structure, traversal patterns, connectivity | VC→HC: the 3 traversal patterns and cross-edge positions are the core invariant |

The skill detects type from the issue description (gadget keywords: "widget", "component", "gadget"; algebraic keywords: "padding", "complement", "case"; identity: everything else) and adjusts the adversary prompt template.

### Enhancement 3: Downstream Integration with add-rule

When `/add-rule` is invoked for a reduction that has verified artifacts:

1. **Check for existing verification:** Look for `docs/paper/verify-reductions/verify_<source>_<target>.py` and `test_vectors_<source>_<target>.json`.
2. **Read Python `reduce()` as pseudocode:** The verified Python function is the spec for the Rust `reduce_to()` implementation. The agent translates it directly.
3. **Read overhead from test vectors JSON:** The `overhead` field maps directly to `#[reduction(overhead = {...})]` expressions.
4. **Generate Rust tests from test vectors:** The YES/NO instances become closed-loop test cases: construct source → reduce → solve target → extract → verify.
5. **Read Typst proof for documentation:** The proof's Construction section becomes the doc comment on the Rust impl.

This eliminates re-derivation from the issue. The verified artifacts are the single source of truth.

### Enhancement 4: Compositional Verification in review-pipeline

After the Rust implementation exists, review-pipeline's agentic test gains a compositional check:

1. Load test vectors from `test_vectors_<source>_<target>.json`
2. For each test vector, run `pred solve <source-instance>` via the new path
3. If an alternative path exists (e.g., both `NAE→SS→ILP` and `NAE→ILP`), run via both and compare
4. Disagreement = bug in either the new reduction or the test vector

This is the strongest verification because it tests the actual Rust code through independent code paths. It happens naturally in review-pipeline's existing agentic test step — we just need the test vectors file to drive it.

## Changes to SKILL.md

### New Step 4.5: Auto-Matching Validation

After the constructor passes (Step 4), before the adversary (Step 5):

1. Constructor exports `test_vectors_<source>_<target>.json` with YES/NO instances, overhead, and claims
2. Parse the Typst proof for numerical values in example sections
3. Verify the Typst examples match the JSON test vectors
4. Verify all `claim()` tags have `verified=True`
5. Flag any untested claims

### Updated Step 5: Typed Adversary Prompt

The adversary prompt template includes a reduction-type section:

- **Identity reductions:** "Focus on exhaustive enumeration of all source instances for n ≤ 6. Test every possible configuration."
- **Algebraic reductions:** "Focus on case boundary conditions. Test instances where the case selection changes (e.g., Σ = 2T exactly, Σ = 2T ± 1). Verify extraction logic for each case independently."
- **Gadget reductions:** "Focus on widget structure invariants. Verify each traversal pattern independently. Test that interior vertices have no external edges. Check connectivity after removing widgets."

### Updated Step 8: Downstream Artifacts

The commit includes the test vectors JSON alongside the Typst and Python files:

```bash
git add docs/paper/verify-reductions/test_vectors_<source>_<target>.json
```

The PR description notes: "Test vectors available for `/add-rule` to consume when implementing the Rust reduction."

### Updated Integration Section

```
- **Before `/add-rule`**: `/verify-reduction` produces Typst proof + Python scripts + test vectors JSON
- **During `/add-rule`**: reads Python `reduce()` as pseudocode, overhead from JSON, generates Rust tests from test vectors
- **During `/review-pipeline`**: agentic test runs `pred reduce`/`pred solve` on test vector instances, compositional check via alternative paths if available
```

## Non-Goals

- **No Rust code generation.** verify-reduction stays in Typst + Python. Rust translation is add-rule's job.
- **No manifest format.** The Python script + test vectors JSON are the artifacts. No intermediate spec language.
- **No Lean changes.** Lean remains optional and orthogonal to this enhancement.

## Files Changed

1. `.claude/skills/verify-reduction/SKILL.md` — new Step 4.5, updated Step 5 and Step 8
2. `.claude/skills/add-rule/SKILL.md` — add "check for existing verification artifacts" to Step 1
3. `.claude/skills/review-pipeline/SKILL.md` — add compositional test vector check to agentic test step (or document as optional enhancement)

## Success Criteria

After implementing these enhancements, running `/verify-reduction` → `/add-rule` → `/review-pipeline` on a new reduction should:

1. Produce a test vectors JSON consumed by both add-rule and review-pipeline
2. Auto-detect Typst ↔ Python example divergence (if introduced deliberately as a test)
3. Generate Rust tests from the same test vectors that the Python scripts verified
4. Catch Rust implementation bugs via `pred reduce`/`pred solve` on those test vectors
