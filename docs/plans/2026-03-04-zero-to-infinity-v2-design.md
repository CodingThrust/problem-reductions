# Zero-to-Infinity v2 Design — 5 Fixes

**Date:** 2026-03-04
**Issue:** https://github.com/CodingThrust/problem-reductions/issues/161

## Context

After manual testing of the zero-to-infinity skill, 5 issues were identified. This design addresses all of them.

## Fix 1: Survey — Cascading Elimination

**Problem:** All 6 dimensions shown for every rank selection, causing repeated options.

**Solution:** Each round removes previously selected options:

```
Round 1 (6 options): "Which is your #1 priority?"
  → User picks "Cross-Field Application"
Round 2 (5 options): "Which is your #2 priority?" (Cross-Field removed)
  → User picks "Industrial Application"
Round 3 (4 options): "#3?" (Cross-Field + Industrial removed)
  ...continue until 2 remain
Round N-1 (2 options): final pick, last one auto-assigned to bottom rank
```

The skill must explicitly instruct Claude to track selected dimensions and exclude them from subsequent AskUserQuestion calls.

## Fix 2: Inventory-First Deduplication

**Problem:** Deduplication was a sub-step of discovery, happening too late.

**Solution:** Restructure Step 2 into 3 ordered phases:

### Phase 1: Build Exclusion Set (FIRST, before any search)

```bash
# Implemented models
ls src/models/*/

# Implemented rules
ls src/rules/

# Open issues
gh issue list --state open --limit 200 --json title,number

# Closed issues
gh issue list --state closed --limit 200 --json title,number
```

Build a named set of all known problems, rules, and issue titles.

### Phase 2: Discover (parallel, with exclusion set)

Both web search and graph gap analysis receive the exclusion set upfront and filter during discovery, not after.

### Phase 3: Final Deduplication

Merge results from both channels, remove any remaining duplicates.

## Fix 3: Rules Over Models, Nontrivial Only

**Problem:** No prioritization of rules vs models. Trivial reductions could appear.

**Solution:**

### Filing Priority Order
1. **Rules between existing models** — highest value, both endpoints already implemented
2. **Models needed by high-value rules** — file model first, then rule
3. **Standalone models** — lowest priority (no immediate rule connection)

### Nontrivial Filter

Exclude candidate rules that are:
- Identity mappings or trivial embeddings
- Simple type/weight casts (i32 → f64)
- Variant promotions (SimpleGraph → HyperGraph)

Reference: issue #127's standard for non-trivial cross-domain reductions.

### Presentation

The ranked table groups candidates:
```
--- Rules (models exist) ---
1. Rule: 3SAT → MaxCut          Score: 21
2. Rule: MaxClique ↔ MaxIS      Score: 17
--- Models + Rules (both needed) ---
3. Model: Partition              Score: 22
4. Rule: Partition → BinPacking  Score: 21
--- Models (standalone) ---
5. Model: VehicleRouting         Score: 14
```

## Fix 4: Candidate Limit 10–20

**Problem:** Hard limit of 10 was too restrictive.

**Solution:** Present 10–20 candidates. Default target: ~15. Hard cap: 20 (to avoid overwhelming the user and taking too long to file). If discovery returns fewer than 10 quality candidates, present all.

## Fix 5: Sub-Skills for Issue Filing

**Problem:** Issue filing was inline in zero-to-infinity with no reusable structure.

**Solution:** Create two new standalone skills:

### `.claude/skills/add-issue-model/SKILL.md`

**Input:** Problem name, brief description, references (from zero-to-infinity candidate data)

**Process:**
1. Web search to fill all 11 items from add-model Step 0 checklist
2. Double-check the model doesn't already exist in repo (`src/models/`, open issues)
3. Enforce: citation for every complexity claim, concrete example, algorithm with reference
4. Draft issue body, show to user for confirmation
5. File via `gh issue create --title "[Model] ProblemName" --body ...`

### `.claude/skills/add-issue-rule/SKILL.md`

**Input:** Source problem, target problem, references (from zero-to-infinity candidate data)

**Process:**
1. Web search to fill all 9 items from add-rule Step 0 checklist
2. Double-check the rule doesn't already exist in repo (`src/rules/`, open issues)
3. Enforce: citation, worked step-by-step example, correctness proof sketch
4. Draft issue body, show to user for confirmation
5. File via `gh issue create --title "[Rule] Source to Target" --body ...`

### Integration with zero-to-infinity

Step 5 dispatches parallel subagents, each running `add-issue-model` or `add-issue-rule` for its assigned candidate. The parent skill collects results and reports filed issue URLs.

## Files Changed

1. **Modified:** `.claude/skills/zero-to-infinity/SKILL.md` — all 5 fixes
2. **New:** `.claude/skills/add-issue-model/SKILL.md` — model issue filing sub-skill
3. **New:** `.claude/skills/add-issue-rule/SKILL.md` — rule issue filing sub-skill
4. **Modified:** `.claude/CLAUDE.md` — register 2 new skills
