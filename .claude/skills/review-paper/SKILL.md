---
name: review-paper
description: Review the Typst paper (docs/paper/reductions.typ) for quality issues — evaluates 10 entries per session, reports mechanical and critical issues without fixing
---

# Review Paper

Evaluate the quality of problem definitions and reduction rules in `docs/paper/reductions.typ`. Each session reviews **10 entries** (problems or rules), producing a structured report. **Read-only — do not modify any files.**

## Usage

```
/review-paper                  # review next 10 unreviewed problem-defs
/review-paper rules            # review next 10 unreviewed reduction-rules
/review-paper ProblemName      # review a specific problem-def
/review-paper Source Target    # review a specific reduction-rule
```

## Step 0: Determine Scope

Parse the argument:
- No argument or `problems` → review problem-defs
- `rules` → review reduction-rules
- A specific name → review that single entry

To pick which 10 to review, scan `docs/paper/reductions.typ` for all `problem-def(...)` or `reduction-rule(...)` entries. Start from the beginning of the file, skipping any that have been reviewed in a previous session (check memory for `paper-review-progress`). If all have been reviewed, report completion.

## Step 1: Load Gold Standard

Read the reference examples before reviewing:
- **Problem gold standard:** search for `problem-def("MaximumIndependentSet")` in `reductions.typ` — note its structure, depth, and components
- **Rule gold standard:** search for `reduction-rule("MaximumIndependentSet", "MinimumVertexCover"` — note its proof depth and example

## Step 2: Review Each Entry

For each of the 10 entries, read the full entry text and evaluate against the checklists below.

### Problem-Def Checklist

**Mechanical checks** (objective, can be verified by reading):

| Check | Criterion |
|-------|-----------|
| M1. Display name | Entry exists in `display-name` dictionary |
| M2. Formal definition | `def` parameter is present and non-empty |
| M3. Self-contained notation | Every symbol in `def` is defined before first use |
| M4. Background text | Body contains at least 2 sentences of background/motivation |
| M5. Example present | Body contains `*Example.*` or `Example.` |
| M6. Example from fixture | Example data matches `src/example_db/fixtures/examples.json` (not invented) — check by loading the JSON and comparing |
| M7. Figure present | Body contains `#figure(` |
| M8. Pred commands | Body contains `pred-commands(` or `pred create` |
| M9. Algorithm citation | Complexity claims have `@citation` or a footnote explaining absence |
| M10. Evaluation shown | Example shows how the objective/verifier computes the value |

**Critical checks** (require judgment):

| Check | Criterion |
|-------|-----------|
| C1. Definition correctness | Does the formal definition accurately describe the problem? Compare with the Rust implementation (`src/models/`) and literature |
| C2. Background quality | Is the background informative? Does it mention applications, history, special cases, or algorithmic context? |
| C3. Example pedagogy | Is the example small enough to verify by hand? Does it illustrate the key aspects of the problem? |
| C4. Completeness | Are there important aspects of the problem that are missing (e.g., well-known special cases, relationship to other problems)? |

### Reduction-Rule Checklist

**Mechanical checks:**

| Check | Criterion |
|-------|-----------|
| M1. Theorem statement | Rule body describes the construction |
| M2. Proof present | Proof body is non-empty |
| M3. Proof length | Proof is at least 3 sentences (not just "trivial" or a one-liner) |
| M4. Overhead documented | Overhead is auto-generated from JSON (verify edge exists in `reduction_graph.json`) |
| M5. Example present | `example: true` and example renders correctly |
| M6. Example from fixture | Example data matches `src/example_db/fixtures/examples.json` |
| M7. Pred commands | Example section contains `pred-commands(` with create/reduce/evaluate pipeline |
| M8. Both directions | If the reverse rule also exists in the graph, check it has its own entry |

**Critical checks:**

| Check | Criterion |
|-------|-----------|
| C1. Construction correctness | Does the theorem statement accurately describe what `reduce_to()` does? Read `src/rules/<source>_<target>.rs` to verify |
| C2. Proof correctness | Does the proof correctly argue that the reduction preserves solutions? |
| C3. Example clarity | Does the example clearly show source → target → solution extraction? |
| C4. Proof-only flag | If this is a proof-only reduction (not solver-executable), is that stated? |

## Step 3: Generate Report

Present results **one entry at a time** in this format:

```
### [N/10] ProblemName (or Source → Target)

**Mechanical Issues:**
- [PASS] M1. Display name
- [FAIL] M5. Example present — no worked example in body
- [WARN] M9. Algorithm citation — complexity claim "O*(2^n)" has no @citation

**Critical Issues:**
- [FAIL] C2. Background quality — body is only one sentence ("This is NP-hard.")
  with no applications, history, or algorithmic context
- [OK] C1. Definition correctness — matches Rust implementation

**Verdict:** 2 mechanical fails, 1 critical fail — needs improvement
```

After each entry, pause and ask: **"Continue to next entry, or discuss this one?"**

Use these severity levels:
- **PASS** — meets criterion
- **WARN** — minor issue, could be improved but acceptable
- **FAIL** — does not meet criterion, should be fixed

## Step 4: Session Summary

After all 10 entries, print a summary table:

```
## Session Summary

| Entry | Mechanical | Critical | Verdict |
|-------|-----------|----------|---------|
| ProblemA | 9/10 pass | 4/4 pass | Good |
| ProblemB | 7/10 pass | 3/4 pass | Needs work |
| ...   | ...       | ...      | ...     |

Overall: X/10 entries pass all checks.
Top priorities for improvement: [list the 3 worst entries]
```

## Step 5: Save Progress

Save progress to memory so the next session can continue where this one left off. Record which entries have been reviewed and their verdicts.

## Important Rules

1. **Do not modify any files.** This skill is read-only.
2. **Do not invent issues.** Only report problems you can verify by reading the source.
3. **Check the Rust source** for critical checks — don't guess whether the math is right.
4. **Be specific.** "Background is thin" is not useful. "Background is one sentence with no applications or algorithmic context" is useful.
5. **Compare to gold standard.** The MIS entry is the reference — entries don't need to be as long, but they should cover the same structural elements.
