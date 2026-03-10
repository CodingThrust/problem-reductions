# Zero-to-Infinity Skill Design

**Issue:** https://github.com/CodingThrust/problem-reductions/issues/161
**Date:** 2026-03-04

## Overview

A skill that discovers high-impact problems and reduction rules, ranks them by user priorities, and files them as GitHub issues — bridging the gap between "what should we add next?" and the existing `issue-to-pr` / `meta-power` pipeline.

## Pipeline

```
Step 1: Survey      → User ranks impact dimensions (single prompt)
Step 2: Discover    → Web search + reduction graph gap analysis (parallel)
Step 3: Rank        → Score candidates against user weights, present table
Step 4: Select      → User picks which candidates to file
Step 5: File        → Create [Model]/[Rule] GitHub issues
Step 6: Implement   → Optionally invoke meta-power
```

## Impact Dimensions

| # | Dimension | Description | Scoring signal |
|---|-----------|-------------|----------------|
| 0 | Academic Publications | Papers in JACM, SICOMP, top venues | Paper count from web search |
| 1 | Industrial Application | Real-world use (search, navigation, scheduling) | Application domain count |
| 2 | Cross-Field Application | Physics, chemistry, biology relevance | Scientific domain count |
| 3 | Top-Scientists Interest | Karp's 21, Garey & Johnson, Aaronson | Named in canonical lists |
| 4 | Graph Connectivity | Bridges disconnected reduction graph components | Structural gap score |
| 5 | Pedagogical Value | Clean, illustrative reductions for teaching | Subjective assessment |

Extensible: the user can add custom dimensions during the survey step.

### Scoring

User ranks dimensions 1-N in a single prompt. Weight for rank k (out of N) = N - k + 1. Each candidate gets a score per dimension (0-5), multiplied by the weight, then summed.

## Discovery Channels

### Channel A: Web Search

Search queries (run via parallel subagents):
- "classical NP-complete problems Karp's 21"
- "NP-hard problems {top user dimension}" (e.g., "NP-hard problems industrial applications")
- "polynomial reductions from {existing_problem}"
- "important reductions in computational complexity"

For each candidate found, gather: formal name, definition sketch, known reductions, complexity class, references.

### Channel B: Reduction Graph Gap Analysis

Read `reduction_graph.json` and identify:
- Problems with no outgoing reductions (dead ends)
- Natural reductions missing between related problems
- Disconnected components that could be bridged
- Well-known reductions from literature that aren't implemented

## Deduplication

Before presenting candidates, filter out:
- Already-implemented models (check `src/models/`)
- Already-implemented rules (check `src/rules/`)
- Open issues (check `gh issue list`)
- Recently closed issues (check `gh issue list --state closed`)

## Ranking & Selection

Present a ranked table with up to 10 candidates:

```
| # | Type  | Name              | Score | Top Dimensions Hit       |
|---|-------|-------------------|-------|--------------------------|
| 1 | Model | SubsetSum         | 23    | Academic(5), Industry(4) |
| 2 | Rule  | 3SAT → MaxCut     | 21    | Academic(5), TopSci(4)   |
...
```

User multi-selects which to file.

## Issue Filing

For each selected candidate, generate a GitHub issue:
- `[Model]` issues: populate all 11 items from `add-model` Step 0 checklist
- `[Rule]` issues: populate all 9 items from `add-rule` Step 0 checklist

Show draft to user for confirmation before filing via `gh issue create`.

## Optional Implementation

After filing, ask user whether to invoke `meta-power` to implement the filed issues automatically.

## Conventions

- No duplicate issues (deduplication check is mandatory)
- Issue content follows existing `[Model]`/`[Rule]` template conventions
- The skill does NOT implement code directly — it only creates issues
- All web search results are cited with URLs in the issue body
