---
name: add-issue-model
description: Use when filing a GitHub issue for a new problem model, ensuring all template sections are complete with citations
---

# Add Issue — Model

File a `[Model]` GitHub issue on CodingThrust/problem-reductions using the upstream "Problem" issue template. Ensures all sections are complete, cited, and verified against the repo.

## Input

The caller (zero-to-infinity or user) provides:
- Problem name
- Brief description / definition sketch
- Reference URLs (if available)

## Step 1: Verify Non-Existence

Before anything else, confirm the model doesn't already exist:

```bash
# Check implemented models
ls src/models/*/ | grep -i "<problem_name_lowercase>"

# Check open issues
gh issue list --state open --limit 200 --json title,number | grep -i "<problem_name>"

# Check closed issues
gh issue list --state closed --limit 200 --json title,number | grep -i "<problem_name>"
```

**If found:** STOP. Report to caller that this model already exists (with issue number or file path).

## Step 2: Research and Fill Template Sections

Use `WebSearch` and `WebFetch` to fill all sections from the upstream template (`.github/ISSUE_TEMPLATE/problem.md`):

| Section | What to fill | Guidance |
|---------|-------------|----------|
| **Motivation** | One sentence: why include this problem? | E.g. "Widely used in network design and has known reductions to QUBO." |
| **Definition — Name** | Use `Maximum*`/`Minimum*` prefix for optimization. Check CLAUDE.md "Problem Names" | E.g. `MaximumIndependentSet` |
| **Definition — Reference** | URL or citation for the formal definition | Must be a real, accessible URL |
| **Definition — Formal** | Input, feasibility constraints, and objective. Define ALL symbols before using them. Use LaTeX math (`$...$` inline, `$$...$$` display) | E.g. "Given $G=(V,E)$ where $V$ is vertex set and $E$ is edge set, find $S \subseteq V$ such that..." |
| **Variables — Count** | Number of variables in configuration vector | E.g. $n = |V|$ (one variable per vertex) |
| **Variables — Domain** | Per-variable domain | E.g. binary $\{0,1\}$ or $\{0,\ldots,K-1\}$ for $K$ colors |
| **Variables — Meaning** | What each variable represents | E.g. $x_i = 1$ if vertex $i \in S$ |
| **Schema — Type name** | Rust struct name | Must match the Definition Name |
| **Schema — Variants** | Graph topology variants, weighted/unweighted | E.g. `SimpleGraph, GridGraph; weighted or unweighted` |
| **Schema — Fields table** | `\| Field \| Type \| Description \|` for each struct field | Connect fields to symbols defined in Definition |
| **Complexity** | Best known exact algorithm with concrete numbers | E.g. $O(1.1996^n)$ by Xiao & Nagamochi (2017). **No symbolic constants.** |
| **Complexity — References** | URL for complexity results | Must be citable |
| **Extra Remark** | Optional: historical context, applications, relationships | Can be brief or empty |
| **How to solve** | Check applicable boxes | BruteForce / ILP reduction / Other |
| **Example Instance** | Small but non-trivial instance with known optimal solution | Must be large enough to exercise constraints (avoid trivial cases). Will appear in paper. |

**Citation rule:** Every complexity claim and reference MUST include a URL.

**Formatting rule:** All mathematical expressions MUST use GitHub LaTeX rendering: `$...$` for inline math (e.g., $G=(V,E)$, $x_i$, $O(1.1996^n)$) and `$$...$$` for display equations. Never use plain text for math.

## Step 3: Verify Algorithm Correctness

For the Complexity section:
- Cross-check the complexity claim against at least 2 independent sources
- Ensure the complexity uses concrete numeric values (e.g., $1.1996^n$), not symbolic constants
- Verify the variable in the complexity expression maps to a natural size getter (e.g., $n = |V|$ → `num_vertices`)

## Step 4: Draft and File Issue

Draft the issue body matching the upstream template format exactly:

```bash
gh issue create --repo CodingThrust/problem-reductions \
  --title "[Model] ProblemName" \
  --label "model" \
  --body "$(cat <<'ISSUE_EOF'
## Motivation

<one sentence>

## Definition

**Name:** ProblemName
**Reference:** [citation](url)

<formal definition with all symbols defined, using LaTeX: $G=(V,E)$, $S \subseteq V$, etc.>

## Variables

- **Count:** $n = |V|$ (one variable per vertex)
- **Per-variable domain:** binary $\{0,1\}$
- **Meaning:** $x_i = 1$ if vertex $i$ is selected

## Schema (data type)

**Type name:** ProblemName
**Variants:** graph topology (SimpleGraph, ...), weighted or unweighted

| Field | Type | Description |
|-------|------|-------------|
| graph | SimpleGraph | the graph $G=(V,E)$ |
| weights | Vec<W> | vertex weights $w_i$ (weighted variant only) |

## Complexity

- **Best known exact algorithm:** $O(1.1996^n)$ by Author (Year), where $n = |V|$
- **References:** [paper](url)

## Extra Remark

<optional notes>

## How to solve

- [x] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing the integer programming, through #issue-number.
- [ ] Other, refer to ...

## Example Instance

<small but non-trivial instance with known optimal solution, for testing and the paper>
ISSUE_EOF
)"
```

Report the created issue number and URL.

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Using custom format instead of template | Must match `.github/ISSUE_TEMPLATE/problem.md` sections exactly |
| Missing complexity citation | Every algorithm claim needs author + year + URL |
| Symbolic constants in complexity | Use concrete numbers: $1.1996^n$ not $(2-\epsilon)^n$ |
| Plain text math | Use LaTeX: `$G=(V,E)$` not `G=(V,E)` |
| Undefined symbols in definition | Define ALL symbols (G, V, E, S, etc.) before using them |
| Trivial example instance | Use non-trivial instance (e.g., Petersen graph, not triangle) |
| Not checking repo first | Always run Step 1 before researching |
| Missing label | Use `--label "model"` to match template metadata |
