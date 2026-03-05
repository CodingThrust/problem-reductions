# check-issue Skill Design

## Purpose
Manual quality gate for `[Rule]` and `[Model]` GitHub issues. Invoked via `/check-issue <number>`. Runs 4 checks (adapted per issue type) and posts a structured report as a GitHub comment. Adds failure labels but does NOT close issues.

## Rule Checks

### 1. Usefulness (label: `Useless`)
- Parse Source/Target from issue body
- `pred path <source> <target> --json` to check existing reductions
- Direct reduction exists → compare proposed overhead vs existing (`pred show <target> --json` → `reduces_from[]`); fail if not strictly lower
- Multi-hop only → pass with note
- No path → pass
- Check Motivation field is substantive

### 2. Non-trivial (label: `Trivial`)
- Flag: pure variable substitution, subtype coercion, same-problem identity
- Algorithm must be detailed enough for a programmer to implement

### 3. Correctness (label: `Wrong`)
- Cross-check references against `references.bib` first
- External references: arxiv MCP → Semantic Scholar MCP → WebSearch (fallback chain)
- Verify paper exists, claim matches, cross-reference with other sources
- If better algorithm found → recommend in comment

### 4. Well-written (label: `PoorWritten`)
- Information completeness (all template sections filled)
- Algorithm is complete step-by-step procedure
- Symbol/notation consistency (defined before use, match between sections)
- Code metric names match actual getter methods
- Example quality (non-trivial, brute-force solvable, fully worked)

## Model Checks

### 1. Usefulness (label: `Useless`)
- Check if problem already exists via `pred show <name> --json`
- Motivation must be concrete with use cases
- Must identify at least one solver method
- Should mention planned reduction rules

### 2. Non-trivial (label: `Trivial`)
- Not isomorphic to existing problem under different name
- Not a trivial variant (graph/weight restriction) of existing model
- Must have genuinely different feasibility constraints or objective

### 3. Correctness (label: `Wrong`)
- Definition mathematically well-formed
- Complexity claims verified against literature
- If better algorithm found → recommend in comment
- Same fallback chain as rules

### 4. Well-written (label: `PoorWritten`)
- All template sections present (Motivation, Definition, Variables, Schema, Complexity, How to solve, Example)
- Definition precise and implementable
- Symbol/notation consistency across sections
- Naming conventions followed (Maximum/Minimum prefix)
- Example with known optimal solution

## Output
Structured GitHub comment with pass/fail/warn table, per-check details, and recommendations. Labels added only for failed checks. Never closes issues.
