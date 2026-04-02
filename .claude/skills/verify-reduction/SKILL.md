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

**Compatible pairs for `ReduceTo` (witness-capable):**
- `Or`→`Or`, `Min`→`Min`, `Max`→`Max` (same type)
- `Or`→`Min`, `Or`→`Max` (feasibility embeds into optimization)

**Incompatible — STOP if any of these:**
- `Min`→`Or` or `Max`→`Or` — optimization source has no threshold K; needs a decision-variant source model (e.g., `VertexCover(Or)` instead of `MinimumVertexCover(Min)`)
- `Max`→`Min` or `Min`→`Max` — opposite optimization directions; needs `ReduceToAggregate` or a decision-variant wrapper
- `Or`→`Sum` or `Min`→`Sum` — Sum is aggregate-only; needs `ReduceToAggregate`
- Any pair involving `And` or `Sum` on the target side

If incompatible, STOP and comment on the issue explaining the type mismatch and options (add decision variant, use `ReduceToAggregate`, or choose different pair). Do NOT proceed with the mathematical verification — the reduction may be correct classically but cannot be implemented as `ReduceTo` in this codebase without a type-compatible source/target pair.

### If compatible

Extract: construction algorithm, correctness argument, overhead formulas, worked example, reference. Use WebSearch if the issue is incomplete.

## Step 2: Write Typst Proof

Create standalone `docs/paper/verify-reductions/<source>_<target>.typ`.

**Mandatory structure:**

```typst
== Source $arrow.r$ Target <sec:source-target>
#theorem[...] <thm:source-target>
#proof[
  _Construction._ (numbered steps, every symbol defined before first use)
  _Correctness._
  ($arrow.r.double$) ... (genuinely independent, NOT "the converse is similar")
  ($arrow.l.double$) ...
  _Solution extraction._ ...
]
*Overhead.* (table with target metric → formula)
*Feasible example.* (YES instance, ≥3 variables, fully worked with numbers)
*Infeasible example.* (NO instance, fully worked — show WHY no solution exists)
```

**Hard rules:**
- Zero instances of "clearly", "obviously", "it is easy to see", "straightforward"
- Zero scratch work ("Wait", "Hmm", "Actually", "Let me try")
- Two examples minimum, both with ≥3 variables/vertices
- Every symbol defined before first use

Compile: `python3 -c "import typst; typst.compile('<file>.typ', output='<file>.pdf', root='.')"`

## Step 3: Write Constructor Python Script

Create `docs/paper/verify-reductions/verify_<source>_<target>.py` with ALL 7 mandatory sections:

| Section | What to verify | Notes |
|---------|---------------|-------|
| 1. Symbolic (sympy) | Overhead formulas symbolically for general n | "The overhead is trivial" is NOT an excuse to skip |
| 2. Exhaustive forward+backward | Source feasible ⟺ target feasible | n ≤ 5 minimum. ALL instances or ≥300 sampled per (n,m) |
| 3. Solution extraction | Extract source solution from every feasible target witness | Most commonly skipped section. DO NOT SKIP |
| 4. Overhead formula | Build target, measure actual size, compare against formula | Catches off-by-one in construction |
| 5. Structural properties | Target well-formed, no degenerate cases | Gadget reductions: girth, connectivity, widget structure |
| 6. YES example | Reproduce exact Typst feasible example numbers | Every value must match |
| 7. NO example | Reproduce exact Typst infeasible example, verify both sides infeasible | Must verify WHY infeasible |

### Minimum check counts — STRICTLY ENFORCED

| Type | Minimum checks | Minimum n |
|------|---------------|-----------|
| Identity (same graph, different objective) | 10,000 | n ≤ 6 |
| Algebraic (padding, complement, case split) | 10,000 | n ≤ 5 |
| Gadget (widget, cycle construction) | 5,000 | n ≤ 5 |

Every reduction gets at least 5,000 checks regardless of perceived simplicity.

## Step 4: Run and Iterate

```bash
python3 docs/paper/verify-reductions/verify_<source>_<target>.py
```

### Iteration 1: Fix failures

Run the script. Fix any failures. Re-run until 0 failures.

### Iteration 2: Check count audit

Print and fill this table honestly:

```
CHECK COUNT AUDIT:
  Total checks:          ___ (minimum: 5,000)
  Forward direction:     ___ instances (minimum: all n ≤ 5)
  Backward direction:    ___ instances (minimum: all n ≤ 5)
  Solution extraction:   ___ feasible instances tested
  Overhead formula:      ___ instances compared
  Symbolic (sympy):      ___ identities verified
  YES example:           verified? [yes/no]
  NO example:            verified? [yes/no]
  Structural properties: ___ checks
```

If ANY line is below minimum, enhance the script and re-run. Do NOT proceed.

### Iteration 3: Gap analysis

List EVERY claim in the Typst proof and whether it's tested:

```
CLAIM                                    TESTED BY
"Universe has 2n elements"               Section 4: overhead ✓
"Complementarity forces consistency"     Section 3: extraction ✓
"Forward: NAE-sat → valid splitting"     Section 2: exhaustive ✓
...
```

If any claim has no test, add one. If untestable, document WHY.

### Iteration 4: Export test vectors and validate Typst matching

Export `docs/paper/verify-reductions/test_vectors_<source>_<target>.json` with:

```json
{
  "source": "<Source>", "target": "<Target>", "issue": <N>,
  "yes_instance": { "input": {...}, "output": {...}, "source_feasible": true, "target_feasible": true, "source_solution": [...], "extracted_solution": [...] },
  "no_instance": { "input": {...}, "output": {...}, "source_feasible": false, "target_feasible": false },
  "overhead": { "field": "expression" },
  "claims": [ {"tag": "...", "formula": "...", "verified": true} ]
}
```

**Typst ↔ JSON cross-check:** Verify key numerical values from the JSON appear in the Typst example sections (substring search). If any value is missing, the proof and script are out of sync — fix before proceeding.

## Step 5: Adversary Verification

Dispatch a subagent that reads ONLY the Typst proof (not the constructor script) and independently implements + tests the reduction.

**Adversary requirements:**
- Own `reduce()` function from scratch
- Own `extract_solution()` function
- Own `is_feasible_source()` and `is_feasible_target()` validators
- Exhaustive forward + backward for n ≤ 5
- `hypothesis` property-based testing (≥2 strategies)
- Reproduce both Typst examples (YES and NO)
- ≥5,000 total checks
- Must NOT import from the constructor script

**Typed adversary focus** (include in prompt):
- **Identity reductions:** exhaustive enumeration n ≤ 6, edge-case configs (all-zero, all-one, alternating)
- **Algebraic reductions:** case boundary conditions (e.g., Σ = 2T exactly, Σ = 2T ± 1), per-case extraction
- **Gadget reductions:** widget structure invariants, traversal patterns, interior vertex isolation

### Cross-comparison

After both scripts pass, compare `reduce()` outputs on shared instances. Both must produce structurally identical targets and agree on feasibility for all tested instances.

### Verdict table

| Constructor | Adversary | Cross-compare | Verdict | Action |
|-------------|-----------|---------------|---------|--------|
| Pass | Pass | Agree | **Verified** | Proceed to Step 6 |
| Pass | Pass | Disagree | **Suspect** | Investigate — may be isomorphic or latent bug |
| Pass | Fail | — | **Adversary bug** | Fix adversary or clarify Typst spec |
| Fail | Pass | — | **Constructor bug** | Fix constructor, re-run from Step 4 |
| Fail | Fail | — | **Proof bug** | Re-examine Typst proof, return to Step 2 |

## Step 6: Self-Review Checklist

Every item must be YES. If any is NO, go back and fix.

### Typst proof
- [ ] Compiles without errors
- [ ] Construction with numbered steps, symbols defined before use
- [ ] Correctness with independent ⟹ and ⟸ paragraphs
- [ ] Solution extraction section present
- [ ] Overhead table with formulas
- [ ] YES example (≥3 variables, fully worked)
- [ ] NO example (fully worked, explains WHY infeasible)
- [ ] Zero hand-waving language
- [ ] Zero scratch work

### Constructor Python
- [ ] 0 failures, ≥5,000 total checks
- [ ] All 7 sections present and non-empty
- [ ] Exhaustive n ≤ 5
- [ ] Extraction tested for every feasible instance
- [ ] Gap analysis: every Typst claim has a test

### Adversary Python
- [ ] 0 failures, ≥5,000 total checks
- [ ] Independent implementation (no imports from constructor)
- [ ] `hypothesis` PBT with ≥2 strategies
- [ ] Reproduces both Typst examples

### Cross-consistency
- [ ] Cross-comparison: 0 disagreements, 0 feasibility mismatches
- [ ] Test vectors JSON exported with Typst auto-matching verified
- [ ] All claims have `verified: true`

## Common Mistakes

| Mistake | Consequence |
|---------|-------------|
| Proceeding past type gate with incompatible types | Wasted work — math may be correct but `ReduceTo` impl is impossible. Common: `Min→Or` (MVC→HamCircuit), `Max→Min` (MaxCut→OLA) |
| Adversary imports from constructor script | Rejected — must be independent |
| No `hypothesis` PBT in adversary | Rejected |
| Section 1 (symbolic) empty | Rejected — "overhead is trivial" is not an excuse |
| Only YES example, no NO example | Rejected |
| n ≤ 3 or n ≤ 4 "because it's simple" | Rejected — minimum n ≤ 5 |
| No gap analysis | Rejected — perform before proceeding |
| Example has < 3 variables | Rejected — too degenerate |
| Either script has < 5,000 checks | Rejected — enhance testing |
| Extraction (Section 3) not tested | Rejected — most commonly skipped |
| Cross-comparison skipped | Rejected |
| Disagreements dismissed without investigation | Rejected |

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

## Integration

### Pipeline: Issue → verify-reduction → add-reduction → review-pipeline

`/verify-reduction` is a **pre-verification gate**. The Python `reduce()` is the verified spec. `/add-reduction` translates it to Rust. `/review-pipeline`'s agentic test confirms the Rust matches.

### Standalone usage

- After `write-rule-in-paper`: invoke to verify paper entry
- During `review-structural`: check verification script exists and passes
- Before `issue-to-pr --execute`: pre-validate the algorithm
