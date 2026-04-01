# Design: Adversarial Multi-Agent Reduction Verification

**Date:** 2026-04-01
**Goal:** Strengthen `/verify-reduction` with an adversarial second agent that independently verifies the reduction, catching systematic errors that the single-agent approach misses.

## Problem Statement

The current verification flow has a fundamental weakness: the same AI agent writes both the mathematical proof AND the verification script. If the agent systematically misunderstands the reduction (e.g., wrong variable mapping, wrong extraction direction), it will write a wrong proof and a wrong script that agree with each other.

Evidence from PR #975:
- Python caught 4 bugs; Typst proofs caught 0; Lean caught 0
- All bugs were construction errors or formula mistakes
- No bug was caused by "proof is right but script is wrong" — they were always "both are wrong in the same way"

The adversarial approach addresses this by having two independent agents verify the same reduction.

## Architecture

```
/verify-reduction <issue>
    │
    ├── Steps 0-2: Parse input, create worktree, read issue, write Typst proof
    │
    ├── Step 3: Constructor Agent
    │   └── Writes verify_<source>_<target>.py (7 sections, ≥5000 checks)
    │
    ├── Step 4: Run constructor script, iterate until 0 failures
    │
    ├── Step 4b: Adversary Agent (dispatched as subagent with worktree isolation)
    │   ├── Input: theorem statement + construction steps ONLY
    │   │   (stripped of constructor's script, internal reasoning, examples)
    │   ├── Writes: adversary_<source>_<target>.py
    │   │   ├── Independent reduce() implementation
    │   │   ├── Independent extract_solution()
    │   │   ├── Property-based testing (hypothesis, n up to 50)
    │   │   ├── Targeted counterexample search
    │   │   └── ≥5000 independent checks
    │   └── Output: pass/fail + counterexamples
    │
    ├── Step 4c: Cross-Comparison
    │   ├── constructor.reduce(x) vs adversary.reduce(x) for 1000 instances
    │   ├── constructor.check(adversary.reduce(x)) vs adversary.check(constructor.reduce(x))
    │   └── Disagreement → diagnose (real bug vs script bug)
    │
    ├── Steps 5-8: Self-review, report, commit, PR
    │
    └── Final PR includes BOTH scripts + cross-comparison results
```

## Adversary Agent Design

### What the adversary receives

A stripped prompt containing ONLY:
- The theorem statement (1-3 sentences)
- The construction steps (numbered)
- The extraction procedure
- The overhead formula

The adversary does NOT receive:
- The constructor's Python script
- The Typst proof's correctness argument
- The worked examples
- Any internal reasoning from Step 3

### What the adversary produces

`adversary_<source>_<target>.py` with:

1. **Independent `reduce()` function** — implemented from the construction steps, without seeing the constructor's implementation
2. **Independent `extract_solution()` function** — same
3. **Exhaustive testing** — forward + backward for n ≤ 5 (matching constructor's minimum)
4. **Property-based testing** — `hypothesis` with `@given(st.integers(min_value=2, max_value=50))` for random instances
5. **Targeted counterexample search**:
   - All-True / all-False assignments
   - Single-variable instances
   - Maximum and minimum clause/subset sizes
   - Instances where source is barely feasible (one satisfying assignment)
   - Instances where source is barely infeasible (off by one element)
6. **Diagnostic output** — any counterexample prints the full instance, expected result, actual result

### Cross-comparison

After both scripts pass independently, the orchestrator runs:

```python
# Import both implementations
from verify_source_target import reduce as constructor_reduce, check as constructor_check
from adversary_source_target import reduce as adversary_reduce, check as adversary_check

# 1. Do they produce the same target instance?
for instance in random_instances(1000):
    t1 = constructor_reduce(instance)
    t2 = adversary_reduce(instance)
    assert t1 == t2, f"Reductions disagree on {instance}"

# 2. Cross-check feasibility
for instance in random_instances(1000):
    assert constructor_check(adversary_reduce(instance)) == adversary_check(constructor_reduce(instance))
```

## Verdict Criteria

| Constructor | Adversary | Cross-check | Verdict |
|-------------|-----------|-------------|---------|
| 0 fail | 0 fail | Agreement | **VERIFIED** |
| 0 fail | Counterexample found | — | Real bug (constructor has blind spot) |
| Fail | — | — | Bug found before adversary needed |
| 0 fail | 0 fail | Disagreement | One implementation is wrong — diagnose |

## Why Lean is Dropped

Lean's contribution to this project:
- **Caught bugs:** 0 out of 4
- **Useful proofs:** `G ⊔ Gᶜ = ⊤` (1 genuinely meaningful theorem)
- **Trivial proofs:** `n + m = n + m`, `14m + (2m-n) + 2nK = 16m-n+2nK` (add no confidence)
- **Infrastructure gap:** No `ReduceTo`, `NAESatisfiability`, `SetSplitting`, or reduction framework in Mathlib
- **ROI:** Negative. Every Lean lemma requires duplicating problem definitions that already exist in Rust.

The adversarial Python approach provides stronger evidence:
- Two independent implementations agreeing > one implementation + arithmetic Lean proofs
- Property-based testing catches scale-dependent bugs that Lean can't
- No Mathlib infrastructure needed

Lean remains **optional** (nice for credibility) but is removed from the required quality gates.

## Changes to verify-reduction Skill

| Step | Before | After |
|------|--------|-------|
| Step 3 | Constructor writes script | Unchanged |
| Step 4 | Iterate until 0 failures | Unchanged |
| **Step 4b** | — | Adversary agent writes independent script |
| **Step 4c** | — | Cross-comparison |
| Step 5 | Lean required | Lean optional |
| Quality gates | 1 script ≥5000 checks | 2 independent scripts ≥5000 each + cross-comparison |
| Deliverables | 3 files (Typst + Python + Lean) | 3 files (Typst + constructor Python + adversary Python) |

## Cost

- Compute: ~2x (two agents instead of one)
- Time: ~1.5x (adversary runs in parallel via subagent)
- Complexity: moderate (orchestrator manages two outputs)

Worth it: a wrong reduction that passes review is far more expensive than 2x compute.
