---
name: zero-to-infinity
description: Use when you want to discover and prioritize new problems and reduction rules to add to the codebase, based on user-ranked impact dimensions
---

# Zero to Infinity

Discover high-impact problems and reduction rules, rank them by user priorities, and file them as GitHub issues — feeding the existing `issue-to-pr` / `meta-power` pipeline.

## Overview

This skill bridges "what should we add next?" with the implementation pipeline. It does NOT write code — it creates well-formed `[Model]` and `[Rule]` issues via the `add-issue-model` and `add-issue-rule` sub-skills.

## Step 1: Survey — Rank Impact Dimensions

Rank dimensions using **cascading elimination** — each round removes previously selected options.

**Default dimensions:**

| # | Dimension | Description |
|---|-----------|-------------|
| 0 | Academic Publications | Papers in JACM, SICOMP, and top CS venues studying this problem/reduction |
| 1 | Industrial Application | Real-world use cases (search engines, navigation, scheduling, compilers) |
| 2 | Cross-Field Application | Relevance to physics, chemistry, biology, or other scientific domains |
| 3 | Top-Scientists Interest | Featured in Karp's 21, Garey & Johnson, or by researchers like Aaronson |
| 4 | Graph Connectivity | Bridges disconnected components in the existing reduction graph |
| 5 | Pedagogical Value | Clean, illustrative reductions good for teaching |

### Cascading Elimination Process

Maintain a list of `remaining_dimensions` (initially all 6). For each round:

1. Present `remaining_dimensions` as options via `AskUserQuestion`: "Which is your #K priority?"
2. User selects one → assign it rank K
3. Remove selected dimension from `remaining_dimensions`
4. Repeat until 2 remain → user picks between them, last one auto-assigned to bottom rank

**Example flow:**
```
Round 1 (6 options): "#1 priority?" → User picks "Cross-Field"
Round 2 (5 options): "#2 priority?" → User picks "Industry" (Cross-Field removed)
Round 3 (4 options): "#3 priority?" → User picks "TopSci" (Cross-Field + Industry removed)
Round 4 (3 options): "#4 priority?" → User picks "Academic" (3 removed)
Round 5 (2 options): "#5 priority?" → User picks one, last auto-assigned #6
```

**IMPORTANT:** You MUST track which dimensions have been selected and exclude them from subsequent AskUserQuestion calls. Never show an already-ranked dimension again.

**Scoring weights:** For N dimensions, rank k gets weight N - k + 1.

User may also add custom dimensions during the first round.

## Step 2: Discover — Inventory First, Then Search

### Phase 1: Build Exclusion Set (MANDATORY FIRST STEP)

Before any web search or analysis, build a complete inventory of what already exists:

```bash
# Implemented models
ls src/models/*/

# Implemented rules
ls src/rules/

# Open issues (increase limit to 200)
gh issue list --state open --limit 200 --json title,number

# Closed issues
gh issue list --state closed --limit 200 --json title,number
```

Build a named **exclusion set** containing:
- Every implemented model name (from filenames)
- Every implemented rule (source→target pairs from filenames)
- Every issue title mentioning a problem or rule name

**Pass this exclusion set to both discovery channels.**

### Phase 2: Discover (parallel, with exclusion set)

Run two channels in parallel (use `dispatching-parallel-agents` or concurrent subagents). Both channels receive the exclusion set and must filter results against it during discovery.

#### Channel A: Web Search

Search queries targeting the user's top-ranked dimensions:
- `"classical NP-complete problems Karp's 21 reductions"`
- `"NP-hard problems {top_dimension_keyword}"` (e.g., `"NP-hard problems condensed matter physics"`)
- `"polynomial reductions from {existing_problem}"` for each problem with few outgoing edges
- `"important reductions computational complexity textbook"`

For each candidate, collect:
- Formal problem name
- Brief definition
- Known reductions to/from other problems
- Complexity class and best known algorithms
- Reference URLs

**Filter:** Immediately discard any candidate in the exclusion set.

#### Channel B: Reduction Graph Gap Analysis

```bash
cat docs/data/reduction_graph.json
```

Identify:
- **Dead-end problems**: nodes with no outgoing reductions
- **Missing natural reductions**: pairs of related problems without a direct edge
- **Disconnected components**: subgraphs that could be bridged by a single reduction
- **Well-known textbook reductions** not yet implemented (Garey & Johnson, CLRS, Arora & Barak)

**Filter:** Only suggest gaps where neither the model nor rule is in the exclusion set.

### Phase 3: Final Deduplication

Merge results from both channels. Remove any remaining duplicates (same problem/rule found by both channels).

## Step 3: Rank — Score, Sort, and Prioritize

For each candidate, assign a score (0-5) per dimension:

| Score | Meaning |
|-------|---------|
| 0 | No relevance |
| 1 | Marginal |
| 2 | Some relevance |
| 3 | Moderate |
| 4 | Strong |
| 5 | Exceptional |

**Total score** = sum of (dimension_score × dimension_weight) for all dimensions.

### Filing Priority Order

After scoring, group candidates by priority:

```
--- Group 1: Rules between existing models (highest value) ---
Rules where BOTH source and target models already exist in the codebase.
These can be implemented immediately without new models.

--- Group 2: Models + Rules (both needed) ---
A model that enables one or more high-value rules. File model first, rules second.
List the model and its dependent rules together.

--- Group 3: Standalone models (lowest priority) ---
Models with no immediate rule connection to existing problems.
```

Within each group, sort by total score descending.

### Nontrivial Filter

**Exclude** candidate rules that are:
- Identity mappings or trivial embeddings
- Simple type/weight casts (i32 → f64)
- Variant promotions (SimpleGraph → HyperGraph)
- Any reduction without meaningful structural transformation

Reference: issue #127's standard for non-trivial cross-domain reductions.

### Present 10–20 Candidates

Present the ranked table with **10–20 candidates** (default target: ~15, hard cap: 20). If discovery returns fewer than 10 quality candidates, present all.

```
| # | Group | Type  | Name              | Score | Top Dimensions Hit         |
|---|-------|-------|-------------------|-------|----------------------------|
|   | **Rules (models exist)** |
| 1 |   1   | Rule  | 3SAT → MaxCut     | 21    | Academic(5), TopSci(4)     |
| 2 |   1   | Rule  | MaxClique ↔ MaxIS | 17    | GraphConn(5), Pedagogical(5)|
|   | **Models + Rules** |
| 3 |   2   | Model | Partition          | 22    | Industry(4), TopSci(4)     |
| 4 |   2   | Rule  | Partition → BinPack| 21    | GraphConn(5), Industry(4)  |
|   | **Standalone models** |
| 5 |   3   | Model | VehicleRouting     | 14    | Industry(5)                |
...
```

Include a 1-line justification for each candidate's top scores.

## Step 4: Select — User Picks Candidates

Use `AskUserQuestion` with `multiSelect: true` to let the user choose which candidates to file as issues.

Present each candidate as an option with its score, group, and type in the label.

**Hint to user:** Filing rules is higher impact than filing standalone models, since rules connect the graph.

## Step 5: File — Dispatch Sub-Skills

For each selected candidate, dispatch a subagent running the appropriate sub-skill:

- **Model candidates:** Invoke `add-issue-model` with the problem name, definition, and references
- **Rule candidates:** Invoke `add-issue-rule` with source, target, and references

**Parallelization:** Use `dispatching-parallel-agents` to file multiple issues concurrently. Each subagent independently:
1. Verifies non-existence (double-check)
2. Researches to fill the full checklist
3. Drafts the issue
4. Files via `gh issue create`
5. Reports the issue URL

**Ordering constraint:** If a model and its dependent rules are both selected, file the model FIRST (sequential), then file rules (can be parallel with each other).

Collect all filed issue URLs and present a summary table.

## Step 6: Implement (Optional)

After all issues are filed, ask:

```
Would you like to invoke meta-power to automatically implement these issues?
```

If yes, invoke the `meta-power` skill. If no, stop — the user can run `/meta-power` later.

## Key Constraints

- **No code writing**: This skill only creates issues. Implementation is delegated to downstream skills.
- **No duplicates**: Inventory check (Phase 1) is mandatory BEFORE any discovery.
- **Template compliance**: Every issue must fully satisfy the `add-model` or `add-rule` checklist. Incomplete issues get rejected by `issue-to-pr`.
- **Citations required**: Every claim about a problem's complexity, applications, or significance must include a reference URL.
- **Nontrivial rules only**: No identity mappings, type casts, or trivial embeddings.
- **User approval gates**: The user approves at two points — candidate selection (Step 4) and optionally at issue draft (via sub-skills).
- **Rules over models**: Prioritize rules between existing models over standalone models.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Repeated survey options | Use cascading elimination — track and exclude selected dimensions |
| Filing issues without inventory check | Always run Phase 1 (exclusion set) BEFORE discovery |
| Presenting trivial rules | Apply nontrivial filter — no identity maps, type casts, or embeddings |
| Filing model when only rule is needed | Check if models already exist; file rules first |
| Too many candidates | Hard cap at 20; default target ~15 |
| Filing without sub-skill | Always dispatch via `add-issue-model` or `add-issue-rule` for template compliance |
| Showing >4 options in one AskUserQuestion | AskUserQuestion supports max 4 options; for candidate selection use multiSelect with up to 4 per call, or present in batches |
