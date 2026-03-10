# Zero-to-Infinity v2 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix 5 issues in the zero-to-infinity skill found during manual testing, and create two new standalone sub-skills for issue filing.

**Architecture:** Update SKILL.md with cascading survey, inventory-first dedup, rules-over-models prioritization, 10-20 candidate limit. Create add-issue-model and add-issue-rule as standalone skills that handle template-compliant issue creation.

**Tech Stack:** Claude Code skills (Markdown), GitHub CLI

---

### Task 1: Create add-issue-model skill

**Files:**
- Create: `.claude/skills/add-issue-model/SKILL.md`

**Step 1: Write the skill file**

```markdown
---
name: add-issue-model
description: Use when filing a GitHub issue for a new problem model, ensuring all 11 checklist items from add-model are complete with citations
---

# Add Issue — Model

File a well-formed `[Model]` GitHub issue that passes the `issue-to-pr` validation. This skill ensures all 11 checklist items are complete, cited, and verified against the repo.

## Input

The caller (zero-to-infinity or user) provides:
- Problem name
- Brief description / definition sketch
- Reference URLs (if available)

## Step 1: Verify Non-Existence

Before anything else, confirm the model doesn't already exist:

```bash
# Check implemented models (look for matching filename)
ls src/models/*/ | grep -i "<problem_name_lowercase>"

# Check open issues
gh issue list --state open --limit 200 --json title,number | grep -i "<problem_name>"

# Check closed issues
gh issue list --state closed --limit 200 --json title,number | grep -i "<problem_name>"
```

**If found:** STOP. Report to caller that this model already exists (with issue number or file path).

## Step 2: Research and Fill Checklist

Use `WebSearch` and `WebFetch` to fill all 11 items from the [add-model](../add-model/SKILL.md) Step 0 checklist:

| # | Item | How to fill |
|---|------|-------------|
| 1 | **Problem name** | Use optimization prefix convention: `Maximum*`, `Minimum*`, or no prefix. Check CLAUDE.md "Problem Names" |
| 2 | **Mathematical definition** | Formal definition from textbook/paper. Must include input, output, and objective |
| 3 | **Problem type** | Optimization (maximize/minimize) or Satisfaction (decision). Determines trait impl |
| 4 | **Type parameters** | Usually `G: Graph, W: WeightElement` for graph problems, or none |
| 5 | **Struct fields** | What the struct holds (graph, weights, parameters) |
| 6 | **Configuration space** | What `dims()` returns — e.g., `vec![2; n]` for binary selection over n items |
| 7 | **Feasibility check** | How to determine if a configuration is valid |
| 8 | **Objective function** | How to compute the metric from a valid configuration |
| 9 | **Best known exact algorithm** | Complexity with concrete numbers, author, year, citation URL |
| 10 | **Solving strategy** | BruteForce, ILP reduction, or custom solver |
| 11 | **Category** | `graph/`, `formula/`, `set/`, `algebraic/`, or `misc/` |

**Citation rule:** Every complexity claim and algorithm reference MUST include a URL (paper, Wikipedia, lecture notes).

## Step 3: Verify Algorithm Correctness

For item 9 (best known exact algorithm):
- Cross-check the complexity claim against at least 2 independent sources
- Ensure the complexity uses concrete numeric values (e.g., `1.1996^n`), not symbolic constants
- Verify the variable in the complexity expression maps to a natural size getter (e.g., `n = |V|` → `num_vertices`)

## Step 4: Draft and File Issue

Draft the issue body with all 11 items clearly formatted:

```bash
gh issue create --repo CodingThrust/problem-reductions \
  --title "[Model] ProblemName" \
  --body "$(cat <<'ISSUE_EOF'
## Problem Definition

**1. Problem name:** `ProblemName`

**2. Mathematical definition:** ...

**3. Problem type:** Optimization (Maximize) / Satisfaction

**4. Type parameters:** `G: Graph, W: WeightElement` / None

**5. Struct fields:**
- `field: Type` — description

**6. Configuration space:** `dims() = vec![2; n]`

**7. Feasibility check:** ...

**8. Objective function:** ...

**9. Best known exact algorithm:** O(...) by Author (Year). [Reference](url)

**10. Solving strategy:** BruteForce / ILP reduction

**11. Category:** `graph/` / `formula/` / `set/` / `algebraic/` / `misc/`

## References
- [Source 1](url1)
- [Source 2](url2)
ISSUE_EOF
)"
```

Report the created issue number and URL.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Missing complexity citation | Every algorithm claim needs author + year + URL |
| Symbolic constants in complexity | Use concrete numbers: `1.1996^n` not `(2-epsilon)^n` |
| Wrong optimization prefix | Check CLAUDE.md "Problem Names" for conventions |
| Not checking repo first | Always run Step 1 before researching |
```

**Step 2: Verify file was created correctly**

Read: `.claude/skills/add-issue-model/SKILL.md`
Expected: File exists with correct YAML frontmatter

**Step 3: Commit**

```bash
git add .claude/skills/add-issue-model/SKILL.md
git commit -m "feat: add add-issue-model skill for filing model issues"
```

---

### Task 2: Create add-issue-rule skill

**Files:**
- Create: `.claude/skills/add-issue-rule/SKILL.md`

**Step 1: Write the skill file**

```markdown
---
name: add-issue-rule
description: Use when filing a GitHub issue for a new reduction rule, ensuring all 9 checklist items from add-rule are complete with citations and worked examples
---

# Add Issue — Rule

File a well-formed `[Rule]` GitHub issue that passes the `issue-to-pr` validation. This skill ensures all 9 checklist items are complete, with citations, a worked example, and a correctness argument.

## Input

The caller (zero-to-infinity or user) provides:
- Source problem name
- Target problem name
- Reference URLs (if available)

## Step 1: Verify Non-Existence

Before anything else, confirm the rule doesn't already exist:

```bash
# Check implemented rules (filename pattern: source_target.rs)
ls src/rules/ | grep -i "<source_lowercase>.*<target_lowercase>"

# Check open issues
gh issue list --state open --limit 200 --json title,number | grep -i "<source>.*<target>"

# Check closed issues
gh issue list --state closed --limit 200 --json title,number | grep -i "<source>.*<target>"
```

**If found:** STOP. Report to caller that this rule already exists.

**Also verify both source and target models exist:**
```bash
ls src/models/*/ | grep -i "<source_lowercase>"
ls src/models/*/ | grep -i "<target_lowercase>"
```

If source or target model doesn't exist, report which model(s) are missing. The caller should file model issues first.

## Step 2: Research and Fill Checklist

Use `WebSearch` and `WebFetch` to fill all 9 items from the [add-rule](../add-rule/SKILL.md) Step 0 checklist:

| # | Item | How to fill |
|---|------|-------------|
| 1 | **Source problem** | Full type with generics: `ProblemName<SimpleGraph, i32>` |
| 2 | **Target problem** | Full type with generics |
| 3 | **Reduction algorithm** | Step-by-step: how to transform source instance to target instance |
| 4 | **Solution extraction** | How to map target solution back to source solution |
| 5 | **Correctness argument** | Why the reduction preserves optimality/satisfiability |
| 6 | **Size overhead** | Expressions for target size in terms of source size getters |
| 7 | **Concrete example** | Small worked instance, tutorial style, step-by-step |
| 8 | **Solving strategy** | How to solve the target (BruteForce, existing solver) |
| 9 | **Reference** | Paper/textbook citation with URL |

**Citation rule:** Every claim MUST include a URL.

## Step 3: Verify Example Correctness

For item 7 (concrete example):
- Walk through the reduction step-by-step on paper
- Show: source instance → reduction → target instance → solve target → extract source solution
- Verify the extracted solution is valid and optimal for the source
- The example must be small enough to verify by hand (3-5 vertices/variables)

## Step 4: Verify Nontriviality

The rule must be **nontrivial** (per issue #127 standards):
- NOT a simple identity mapping or type cast
- NOT a trivial embedding (just copying data)
- NOT a weight type conversion (i32 → f64)
- MUST involve meaningful structural transformation

If the rule is trivial, STOP and report to caller.

## Step 5: Draft and File Issue

```bash
gh issue create --repo CodingThrust/problem-reductions \
  --title "[Rule] Source to Target" \
  --body "$(cat <<'ISSUE_EOF'
## Reduction Definition

**1. Source problem:** `SourceProblem<SimpleGraph, i32>`

**2. Target problem:** `TargetProblem<...>`

**3. Reduction algorithm:**
- Step 1: ...
- Step 2: ...

**4. Solution extraction:** ...

**5. Correctness argument:** ...

**6. Size overhead:**
```
field1 = "expression1"
field2 = "expression2"
```

**7. Concrete example:**
Source: ...
→ Reduction: ...
→ Target: ...
→ Solve: ...
→ Extract: ...

**8. Solving strategy:** BruteForce / existing solver

**9. Reference:**
- [Source](url)

## References
- [Source 1](url1)
ISSUE_EOF
)"
```

Report the created issue number and URL.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Filing trivial reductions | Check nontriviality in Step 4 |
| Missing model dependency | Verify both source and target exist in Step 1 |
| Example too complex | Keep to 3-5 vertices/variables, verifiable by hand |
| Missing correctness argument | Must explain WHY, not just HOW |
| Wrong overhead expressions | Must reference getter methods that exist on source type |
```

**Step 2: Verify file was created correctly**

Read: `.claude/skills/add-issue-rule/SKILL.md`
Expected: File exists with correct YAML frontmatter

**Step 3: Commit**

```bash
git add .claude/skills/add-issue-rule/SKILL.md
git commit -m "feat: add add-issue-rule skill for filing rule issues"
```

---

### Task 3: Rewrite zero-to-infinity SKILL.md with all 5 fixes

**Files:**
- Modify: `.claude/skills/zero-to-infinity/SKILL.md`

**Step 1: Rewrite the entire skill file**

Replace the full contents of `.claude/skills/zero-to-infinity/SKILL.md` with the following (this incorporates all 5 fixes):

```markdown
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
```

**Step 2: Verify the rewritten file**

Read: `.claude/skills/zero-to-infinity/SKILL.md`
Expected: Contains "Cascading Elimination", "Phase 1: Build Exclusion Set", "Filing Priority Order", "10–20 candidates", "add-issue-model", "add-issue-rule"

**Step 3: Commit**

```bash
git add .claude/skills/zero-to-infinity/SKILL.md
git commit -m "fix: zero-to-infinity v2 — cascading survey, inventory-first dedup, rules-over-models, sub-skills"
```

---

### Task 4: Register new skills in CLAUDE.md

**Files:**
- Modify: `.claude/CLAUDE.md` (line ~16, after zero-to-infinity entry)

**Step 1: Add two new skill entries**

After the existing `zero-to-infinity` line, add:

```markdown
- [add-issue-model](skills/add-issue-model/SKILL.md) -- File a well-formed `[Model]` GitHub issue with all 11 checklist items, citations, and repo verification.
- [add-issue-rule](skills/add-issue-rule/SKILL.md) -- File a well-formed `[Rule]` GitHub issue with all 9 checklist items, worked example, correctness argument, and nontriviality check.
```

**Step 2: Verify**

Read: `.claude/CLAUDE.md` lines 1-20
Expected: Both new skills appear in the Skills list

**Step 3: Commit**

```bash
git add .claude/CLAUDE.md
git commit -m "docs: register add-issue-model and add-issue-rule skills in CLAUDE.md"
```

---

### Task 5: Push and update PR

**Step 1: Push all commits**

```bash
git push
```

**Step 2: Verify PR is updated**

```bash
gh pr view --web
```

Expected: PR shows 4 new commits with all skill files.
