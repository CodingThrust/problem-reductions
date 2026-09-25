---
name: how-to-triage-issue
description: Use when a [Model] or [Rule] GitHub issue needs a quality check before implementation, or when problems found by such a check need fixing.
---

# How to Triage an Issue

Gate `[Model]` / `[Rule]` issues before anyone implements them: check, report once, label, and fix
what can be fixed. Never close issues.

Fetch with `gh issue view N --json title,body,labels,comments`. If a `## Issue Quality Check` comment
already exists, stop and say so unless asked to re-check; a re-check notes what changed since the
previous report. Build `pred` with `make cli` if missing; resolve aliases (MIS, MVC, SAT) with
`pred show <name> --json`.

## Checks

Each check yields Pass / Warn / Fail. Only Fail adds a label. Unsure → Warn, never a guessed Fail.

**1. Usefulness** (label `Useless`)
- Rule: no existing `pred path Src Tgt` → Pass. If a path exists, the new rule must not be dominated
  by a composite path — apply the redundancy heuristic from how-to-verify; dominated → Fail naming
  the dominating path, inconclusive → Warn.
- Model: already in `pred show` → Fail. No concrete planned reduction to/from an existing problem
  (orphan) → Fail; vague ("can connect to others") → Warn. Direct ILP solvability claimed without a
  linked `[Rule] <Name> to ILP` issue → Fail. No solver path at all → Warn.
- Empty or hand-wavy Motivation → Warn.

**2. Non-triviality** (label `Trivial`)
- Rule Fails on: pure relabeling / complement substitution, subtype coercion (e.g. UnitDiskGraph →
  SimpleGraph), same-problem variant identity, or a hand-waved algorithm ("map variables
  accordingly"). A trivial rule that connects otherwise disconnected components of the graph still
  Passes.
- Model Fails if isomorphic to an existing problem (`pred list --json`), a mere graph/weight variant
  of one, or a renaming.

**3. Correctness** (label `Wrong`)
- Check claims against `references.md` (this directory) and `docs/paper/references.bib`, then the
  literature itself (arXiv / Semantic Scholar tools if present, else WebSearch + WebFetch).
- Read the actual construction and quote the theorem you rely on. Paper not found or claim not in it
  → Fail; say "not found", never reconstruct what a paper "probably" says.
- Model: definition well-formed, feasibility vs objective separated, variable domain fits the
  semantics; complexity bound matches the cited paper, polynomial problems not given exponential
  bounds, exponential base correct (1.1996^n for MIS, not 2^n).
- Better algorithms or lower-overhead constructions found along the way are Recommendations, not
  failures.

**4. Completeness** (rules only, label `Incomplete`) — does the construction handle every source
instance?
- Literature: does the theorem say "for any instance" or carry a precondition ("connected",
  "no isolated vertices", "k even")? A hidden restriction the issue ignores → Fail, quoting it.
  Fixes: preprocess to the restricted form, re-target the rule at an existing restricted model, or drop.
- Codebase: read `parameters` and inputs from `pred show <Source> --json`, then hand-trace the issue's
  algorithm on ≥ 2 corner cases other than the worked example (empty/single-vertex/disconnected
  graphs, zero or equal weights, empty or unit clauses, empty/duplicate subsets, zero or singular
  matrices — whatever the model allows). Compare with existing rules from the same source
  (`grep -rl "ReduceTo.*for <Source>" src/rules/`). A break → Fail; works but issue is silent on edge
  inputs → Warn.
- Show the quoted passages and the traces in the report — this is the expensive check.

**5. Writing quality** (label `PoorWritten`)
- All template sections of `.github/ISSUE_TEMPLATE/rule.md` / `problem.md` present and substantive.
- Rule algorithm is an implementable step-by-step procedure with solution extraction; no "similarly
  for the rest".
- Symbols defined before use and consistent across sections; overhead metric names match the
  target's `parameters` in `pred show <Target> --json`.
- Model: precise definition with explicit quantifiers; `Maximum`/`Minimum` prefix for optimization;
  complexity given as a concrete expression with citation; expected outcome given (a satisfying
  solution, or an optimal solution with its value).
- Example: small enough to brute-force, fully worked, exercises the defining structure (a
  "quadratic" model with only linear terms fails), and has ≥ 2 suboptimal feasible solutions besides
  the optimum so a buggy round trip cannot pass by accident.
- Do not fail on implementation data types in the Schema section — contributors state the math, not
  Rust types.

**Value choice for models:** default to objective style (`Max` / `Min`, or `Extremum` when the sense
is runtime data). `Or` only for inherently existential problems (SAT, KColoring). `Sum` / `And` only
for genuine folds over all configurations with no representative witness.

## Report and labels

Post one comment headed `## Issue Quality Check — Rule` or `## Issue Quality Check — Model`: a
summary table (check, result, one-line detail), an overall count, a section per check with evidence,
then Recommendations. Never cite an issue you have not fetched or a path you have not verified.

Labels: add `Useless`, `Trivial`, `Wrong`, `Incomplete`, `PoorWritten` per failed check. Add `Good`
only with zero failures and no warning on Usefulness, Correctness, or Completeness. On re-check,
remove stale failure labels.

## Fixing

Gather context first: for a model, every open `[Rule]` issue mentioning it
(`gh issue list --search "<Name> in:title" --state open`) to see which fields and framing rules rely
on; for a rule, whether source and target exist in `pred show` or as open `[Model]` issues.

- **Mechanical** (undefined or inconsistent symbols, wrong metric names, broken formatting, fields
  derivable from other sections, DOI format): fix directly. Write the new body to a file in the
  scratchpad, `gh issue edit N --body-file <file>`, post a `## Fix-issue changelog` comment listing
  each change, re-check, update labels (remove the fixed failure labels including `Incomplete`; add
  `Good` once the re-check passes).
- **Substantive** (wrong or incomplete reduction, bad or trivial example, missing/unread reference,
  complexity claim, naming, value framing): stop and discuss with the human, one issue at a time.
  Quote the relevant issue text and evidence first, then offer 2–3 concrete options with a
  recommendation (for examples: concrete candidate instances with optimum and suboptimal solutions).
- Preserve the author's text outside flagged sections and keep `<!-- Unverified -->` provenance
  markers; mark newly AI-filled content the same way.
- On rename, update the title (`gh issue edit N --title ...`) and propagate to related issues whose
  titles or bodies use the old name.
