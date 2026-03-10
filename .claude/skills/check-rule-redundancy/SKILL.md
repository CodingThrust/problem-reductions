---
name: check-rule-redundancy
description: Use when checking if a reduction rule (source-target pair) is redundant — i.e., dominated by a composite path through other rules in the reduction graph
---

# Check Rule Redundancy

Given a source-target pair, determines whether a direct reduction rule is redundant by comparing its overhead against all composite paths through the reduction graph.

## Invocation

```
/check-rule-redundancy <source> <target>
```

Examples:
```
/check-rule-redundancy MIS ILP
/check-rule-redundancy MaximumIndependentSet QUBO
/check-rule-redundancy Factoring ILP
```

## Process

### Step 1: Resolve Problem Names

Use `pred show` to validate and resolve aliases:

```bash
pred show <source> --json 2>/dev/null
pred show <target> --json 2>/dev/null
```

If either fails, try common aliases (MIS = MaximumIndependentSet, MVC = MinimumVertexCover, SAT = Satisfiability, etc.). Report the resolved names.

### Step 2: Check if Rule Already Exists

```bash
pred show <source> --json
```

Check the output's `reductions` array for a direct edge to `<target>`.

- **Direct edge exists**: Report "Direct rule `<source> -> <target>` already exists" and proceed to redundancy analysis (Step 3).
- **No direct edge**: Report "No direct rule from `<source> -> <target>` exists yet." Then check if any path exists:
  ```bash
  pred path <source> <target> --json
  ```
  - **Path exists**: Report the cheapest existing path and its overhead. This is the baseline the proposed new rule must beat to be non-redundant.
  - **No path exists**: Report "No path exists — a new rule would be novel (not redundant)." Stop here.

### Step 3: Find All Paths

```bash
pred path <source> <target> --all --json
```

This returns all paths between source and target. The output includes overhead composition for each path.

### Step 4: Compare Overheads

For each composite path (length > 1 step):

1. Extract the **overall overhead** from the path JSON
2. Extract the **direct rule's overhead** from the single-step path
3. Compare field by field:
   - Parse overhead expressions (e.g., `num_vars = n`, `num_constraints = n + m`)
   - For polynomial expressions: compare degree — lower degree means the composite is better
   - For equal-degree polynomials: compare leading coefficients
   - For non-polynomial (exp, log): report as "Unknown — manual review needed"

**Dominance definition:** A composite path **dominates** the direct rule if, on every common overhead field, the composite's expression has equal or smaller asymptotic growth.

### Step 5: Report Results

Output a structured report:

```markdown
## Redundancy Check: <Source> -> <Target>

### Direct Rule
- Overhead: [field = expr, ...]
- Variants: [source variant] -> [target variant]

### Composite Paths Found: N

| # | Path | Steps | Overhead | Comparison |
|---|------|-------|----------|------------|
| 1 | A -> B -> C | 2 | field = expr | Dominates / Worse / Unknown |
| 2 | A -> D -> E -> C | 3 | field = expr | Dominates / Worse / Unknown |

### Verdict

- **Redundant**: At least one composite path dominates the direct rule
- **Not Redundant**: No composite path dominates the direct rule
- **Inconclusive**: Some paths have Unknown comparison (non-polynomial overhead)

### Recommendation

If redundant:
> The direct rule `<source> -> <target>` is dominated by the composite path `[path]`.
> Consider removing it unless it provides value for:
> - Simpler solution extraction (fewer intermediate steps)
> - Educational/documentation clarity
> - Better numerical behavior in practice

If not redundant:
> The direct rule `<source> -> <target>` is not dominated by any composite path.
> It provides overhead that cannot be achieved through existing reductions.
```

## Notes

- "Equal overhead" does not necessarily mean the rule should be removed — direct rules have practical advantages (simpler extraction, fewer steps)
- The analysis uses asymptotic comparison (big-O), so constant factors are ignored
- This means the check can produce false alarms, especially when overhead metadata keeps only leading terms or when a long composite path is asymptotically comparable but practically much worse
- Example false-alarm pattern: `Factoring -> ILP` may be flagged by `Factoring -> CircuitSAT -> ILP<bool> -> ILP<i32>` even though the direct arithmetic ILP is still a meaningful canonical rule
- Treat "dominated" as "potentially redundant, requires manual review" unless the composite path is also clearly preferable structurally
- When overhead expressions involve variables from different problems (e.g., `num_vertices` vs `num_clauses`), comparison may not be meaningful — report as Unknown
- Use the `src/rules/analysis.rs` utility as the ground truth for what the codebase considers dominated. This skill provides a quick CLI-based check for individual rules
