---
name: verify-reduction
description: End-to-end verification of a reduction rule — generates Typst proof, constructor Python script (>=5000 checks), and adversary Python script (>=5000 independent checks), iterating until all checks pass. Creates worktree + PR.
---

# Verify Reduction

Pre-verification gate for reduction rules. Produces Typst proof + dual Python verification scripts + test vectors JSON, iterating until all checks pass. Creates worktree + PR. Downstream: `/add-reduction` consumes the artifacts to implement Rust code.

```
Issue → /verify-reduction (Typst + Python) → /add-reduction (Rust) → /review-pipeline
```

## Invocation

```
/verify-reduction 868
/verify-reduction SubsetSum Partition
```

## Step 0: Parse Input and Create Worktree

```bash
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
ISSUE=<number>
ISSUE_JSON=$(gh issue view "$ISSUE" --json title,body,number)
REPO_ROOT=$(pwd)
WORKTREE_JSON=$(python3 scripts/pipeline_worktree.py enter --name "verify-$ISSUE" --format json)
WORKTREE_DIR=$(printf '%s\n' "$WORKTREE_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['worktree_dir'])")
cd "$WORKTREE_DIR" && git checkout -b "issue/$ISSUE-verify-<source>-<target>"
```

## Step 1: Read Issue, Study Models, Type Check

```bash
gh issue view "$ISSUE" --json title,body
pred show <Source> --json
pred show <Target> --json
```

### Type compatibility gate — MANDATORY

Check source/target `Value` types before any work:

```bash
grep "type Value = " src/models/*/<source_file>.rs src/models/*/<target_file>.rs
```

**Compatible:** `Or`→`Or`, `Min`→`Min`, `Max`→`Max`, `Or`→`Min`/`Max`.
**Incompatible:** `Min`/`Max`→`Or` (needs threshold K that the source type lacks).

If incompatible, STOP and comment on the issue explaining the type mismatch and options (add decision variant, use `ReduceToAggregate`, or choose different pair). Do NOT proceed.

### If compatible

Extract: construction algorithm, correctness argument, overhead formulas, worked example, reference. Use WebSearch if the issue is incomplete.

## Step 2: Write Typst Proof

Create standalone `docs/paper/verify-reductions/<source>_<target>.typ`.

**Mandatory structure:** Construction (numbered steps, symbols defined before use) → Correctness (independent ⟹ and ⟸) → Solution extraction → Overhead table → YES example (≥3 variables, fully worked) → NO example (fully worked, explain WHY infeasible).

**Hard rules:** Zero "clearly"/"obviously"/"straightforward". Zero scratch work. Two examples minimum with ≥3 variables.

Compile: `python3 -c "import typst; typst.compile('<file>.typ', output='<file>.pdf', root='.')"`

## Step 3: Write Constructor Python Script

Create `docs/paper/verify-reductions/verify_<source>_<target>.py` with ALL 7 mandatory sections:

1. **Symbolic checks** (sympy) — overhead formulas, key algebraic identities
2. **Exhaustive forward + backward** — n ≤ 5 minimum, all instances or ≥300 sampled
3. **Solution extraction** — extract source solution from every feasible target witness
4. **Overhead formula** — compare actual target size against formula
5. **Structural properties** — well-formedness, no degenerate cases, gadget structure
6. **YES example** — reproduce exact Typst numbers
7. **NO example** — reproduce exact Typst numbers, verify both sides infeasible

**Minimum:** 5,000 checks regardless of reduction type. 10,000 for identity/algebraic.

## Step 4: Run and Iterate

Run the script. Fix failures. Re-run until 0 failures.

**Check count audit:** Print totals for each category. If any is below minimum, enhance and re-run.

**Gap analysis:** List every Typst claim and its corresponding test. Add tests for untested claims.

**Export test vectors** (`test_vectors_<source>_<target>.json`) with YES/NO instances, overhead expressions, and structured `claim()` tags. Cross-check that key values from the JSON appear in the Typst file text.

## Step 5: Adversary Verification

Dispatch a subagent that reads ONLY the Typst proof (not the constructor script) and independently implements + tests the reduction. Requirements: own `reduce()`, own `extract_solution()`, `hypothesis` PBT (≥2 strategies), ≥5,000 checks, reproduce both Typst examples.

**Typed adversary prompt:** Include reduction-type focus instructions:
- **Identity:** exhaustive enumeration n ≤ 6, edge-case configs
- **Algebraic:** case boundary conditions (e.g., Σ = 2T ± 1), per-case extraction
- **Gadget:** widget structure invariants, traversal patterns, interior isolation

After adversary runs, **cross-compare** constructor and adversary `reduce()` outputs on shared instances. Use the verdict table:

| Constructor | Adversary | Cross-compare | Verdict |
|-------------|-----------|---------------|---------|
| Pass | Pass | Agree | **Verified** → proceed |
| Pass | Pass | Disagree | **Suspect** → investigate structural differences |
| Any fail | — | — | Fix and re-run from the failing step |
| Both fail | — | — | **Proof bug** → return to Step 2 |

## Step 6: Self-Review Checklist

Every item must be YES: Typst compiles with all sections, zero hand-waving, constructor ≥5K checks with 0 failures across all 7 sections, adversary ≥5K checks with 0 failures + hypothesis, cross-comparison 0 disagreements, test vectors JSON exported with Typst auto-matching verified.

## Step 7: Commit, Create PR, Clean Up

```bash
git add docs/paper/verify-reductions/*<source>*<target>*
git add -f docs/paper/verify-reductions/<source>_<target>.pdf
git commit -m "docs: /verify-reduction #<ISSUE> — <Source> → <Target> VERIFIED"
git push -u origin "<branch>"
gh pr create --title "docs: verify reduction #<ISSUE> — <Source> → <Target>" --body "..."
gh issue comment "$ISSUE" --body "verify-reduction report: VERIFIED (PR #<N>)..."
cd "$REPO_ROOT" && python3 scripts/pipeline_worktree.py cleanup --worktree "$WORKTREE_DIR"
```
