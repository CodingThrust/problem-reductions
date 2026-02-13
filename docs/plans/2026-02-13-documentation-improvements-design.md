# Documentation Improvements Design

## Goal

Improve the "Getting Started" and "Architecture" sections of the mdBook documentation.

- **Getting Started**: For all audiences (researchers, developers, students, contributors)
- **Architecture**: For contributors and developers

## Getting Started

### Current Problems

- Jumps straight into code without explaining what the library does
- No high-level workflow overview
- Missing JSON resource documentation

### Proposed Structure

1. **What This Library Does** (~50 words)
   - One paragraph explaining: "Reduce hard problems to solver-friendly forms"
   - Link to Introduction page for the interactive reduction graph

2. **Installation** (keep existing content)

3. **The Reduction Workflow**
   - Cetz diagram showing: `Problem A → reduce → Problem B → solve → extract → Solution for A`
   - One complete code example walking through each step with comments
   - Brief mention: chaining reductions works the same way
   - Note: automated reduction path optimization for connected problems coming in the future

4. **Solvers** (brief, with links)
   - `BruteForce` — for small instances (<20 variables)
   - `ILPSolver` — for larger instances (requires `ilp` feature)
   - Link to API docs for details

5. **JSON Resources** (new section)
   - `reduction_graph.json` — all problems and reduction edges; useful for tooling, visualization, and research
   - `problem_schemas.json` — field definitions for each problem type
   - Location: `docs/src/reductions/` in the built docs, or generate via `cargo run --example export_graph`

6. **Next Steps**
   - Link to Architecture for internals
   - Link to API Reference for full documentation

## Architecture

### Current Problems

- Outdated trait references (`solution_size()` → `evaluate()`, `ConstraintSatisfactionProblem` removed)
- No visual diagram of module relationships
- Unclear entry points for contributors

### Proposed Structure

1. **Module Overview** (new)
   - Cetz diagram showing module relationships:
     ```
     models/ ←→ rules/ → registry/
                  ↑
              solvers/
     ```
   - One sentence description per module

2. **Trait Hierarchy** (updated)
   - Cetz diagram showing trait relationships
   - Updated to current API:
     - `Problem`: `NAME`, `Metric`, `dims()`, `evaluate()`, `variant()`, `num_variables()`
     - `OptimizationProblem`: `Value`, `direction()`
   - Remove references to `ConstraintSatisfactionProblem`

3. **Problems** (streamlined)
   - Keep graph types table (SimpleGraph, GridGraph, UnitDiskGraph, HyperGraph)
   - Keep variant ID explanation
   - Update code examples to use `evaluate()` instead of `solution_size()`

4. **Reductions** (streamlined)
   - Keep reduce → solve → extract explanation
   - Update `#[reduction]` macro example to current syntax
   - Keep overhead tracking mention

5. **Registry** (keep mostly as-is)
   - JSON schema details are good
   - Keep `reduction_graph.json` and `problem_schemas.json` schema examples

6. **Solvers** (keep mostly as-is)
   - Update trait signature if needed

7. **Contributing** (new section, replaces scattered links)

   Priority order:

   a. **Recommended: Issue-based workflow**
      - Open an issue using [Problem template](link) or [Rule template](link)
      - Fill in all sections (definition, algorithm, size overhead, example instance)
      - AI handles implementation automatically

   b. **Optional: Plan + automated PR**
      - Use `superpowers:brainstorming` to create a detailed plan
      - Create PR with `[action]` prefix in description to trigger automated implementation

   c. **Last resort: Manual implementation**
      - See `adding-models.md` for adding problem types
      - See `adding-reductions.md` for adding reduction rules
      - See `testing.md` for test requirements

## Diagrams

Three separate Cetz diagrams in Typst, output to `docs/src/static/`:

1. **`module-overview.typ`** → `module-overview.svg`
   - Shows relationships between `src/models/`, `src/rules/`, `src/registry/`, `src/solvers/`
   - Arrows showing data flow and dependencies

2. **`trait-hierarchy.typ`** → `trait-hierarchy.svg`
   - `Problem` trait with key methods
   - `OptimizationProblem` extension
   - Type parameters (`Metric`, `Value`)

3. **`reduction-workflow.typ`** → `reduction-workflow.svg`
   - Linear flow: Create Problem → Reduce → Solve Target → Extract Solution
   - Shows the round-trip nature

## Files to Modify

- `docs/src/getting-started.md` — rewrite
- `docs/src/arch.md` — update
- `docs/src/static/module-overview.typ` — new
- `docs/src/static/trait-hierarchy.typ` — new
- `docs/src/static/reduction-workflow.typ` — new

## Out of Scope

- Introduction page (already has reduction graph)
- API reference (auto-generated)
- CLAUDE.md (separate concern)
