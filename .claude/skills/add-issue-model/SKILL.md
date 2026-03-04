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
