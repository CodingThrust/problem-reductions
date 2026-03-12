# Arxiv Paper Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Write a full research paper (~10-12 pages) on skill-based agentic coding for NP-hard problem reductions, targeting an ICSE/ASE-class venue.

**Architecture:** Typst document at `docs/paper/arxiv/paper.typ` with CeTZ figures, bibliography from survey, and data gathered from git history and the reduction graph. The existing `docs/paper/lib.typ` provides graph drawing utilities.

**Tech Stack:** Typst, CeTZ (`@preview/cetz:0.4.2`), ctheorems (`@preview/ctheorems:1.1.3`), fletcher (`@preview/fletcher:0.5.8`), BibTeX

**Spec:** `docs/superpowers/specs/2026-03-12-arxiv-paper-design.md`

**Compile command** (used throughout): `typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf`
First compilation may download Typst packages — this is expected.

---

## File Structure

| File | Purpose |
|------|---------|
| `docs/paper/arxiv/paper.typ` | Main paper document |
| `docs/paper/arxiv/references.bib` | Bibliography (merged from survey + existing paper refs) |
| `docs/paper/arxiv/images/reduction-graph.typ` | Figure 1: Reduction graph diagram |
| `docs/paper/arxiv/images/architecture.typ` | Figure 2: System architecture diagram |
| `docs/paper/arxiv/images/pipeline.typ` | Figure 3: Card-based pipeline diagram |
| `docs/paper/arxiv/images/verification-pyramid.typ` | Figure 4: Verification stack pyramid |
| `docs/paper/arxiv/data/graph-metrics.json` | Reduction graph metrics (from Task 2) |
| `docs/paper/arxiv/data/git-mining-results.json` | Git history mining results (from Task 11) |
| `docs/paper/arxiv/scripts/mine-git-history.py` | Git history mining script |

---

## Chunk 1: Paper Scaffolding + Data Gathering

### Task 1: Set up paper.typ scaffolding

**Files:**
- Create: `docs/paper/arxiv/paper.typ`
- Create: `docs/paper/arxiv/references.bib`

- [ ] **Step 1: Create bibliography file**

Copy the survey bibliography:

```bash
cp .claude/survey/agentic-coding-reductions/references.bib docs/paper/arxiv/references.bib
```

Then append the following entries from `docs/paper/references.bib` (read that file and copy these exact `@` entries by key): `karp1972`, `cook1971`, `garey1979`, `glover2019`, `lucas2014`, `barahona1982`. These are foundational references not in the survey bib.

- [ ] **Step 2: Write paper.typ header and imports**

Create `docs/paper/arxiv/paper.typ` with:
- Imports: `@preview/cetz:0.4.2`, `@preview/fletcher:0.5.8`, `@preview/ctheorems:1.1.3`
- Page setup: A4, margins `(x: 2cm, y: 2.5cm)`
- Font: New Computer Modern, 10pt
- Two-column body via `#show: columns.with(2)` (after abstract)
- Numbered headings: `#set heading(numbering: "1.")`
- Bibliography: `#bibliography("references.bib", style: "ieee")`

Reference `docs/paper/reductions.typ` for the exact Typst conventions used in the existing paper.

- [ ] **Step 3: Write title, authors, and abstract**

Title: "Skill-Based Agentic Coding for Mathematical Software: A Case Study in NP-Hard Problem Reductions"

Authors: (use placeholder affiliations for now)

Abstract (~150 words) covering:
- Problem: agents fail at long-horizon math coding tasks (70-80% on SWE-Bench Verified, ~20% on long-horizon)
- Insight: decompose into human-creative + agent-managed/executed via skill-based pipeline
- Method: 13 skills + 7-layer verification stack
- Result: 24 problem types, 40 implemented reductions, 52 graph edges
- Contribution: methodology + verification stack + open-source artifact

- [ ] **Step 4: Write section heading stubs**

Add empty section headings (S1 through S8) matching the spec outline:
1. Introduction
2. Why Reductions? The Goldilocks Domain
3. System Architecture
4. Skill-Based Task Decomposition
5. Multi-Layered Verification
6. Evaluation
7. Related Work
8. Discussion & Conclusion

- [ ] **Step 5: Verify scaffolding compiles**

Run: `typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf`
Expected: PDF with title, abstract, and empty section headings. No errors.

- [ ] **Step 6: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ docs/paper/arxiv/references.bib
git commit -m "docs(arxiv): paper scaffolding with bibliography and abstract"
```

---

### Task 2: Data gathering — reduction graph metrics

**Files:**
- Create: `docs/paper/arxiv/data/graph-metrics.json`

**Note:** The file `docs/src/reductions/reduction_graph.json` has a corrupted header (partial JSON + log message before line 10). Regenerate it first or parse from the second valid copy starting after `JSON content:`.

- [ ] **Step 1: Regenerate the reduction graph JSON**

```bash
make rust-export
```

This regenerates `docs/src/reductions/reduction_graph.json` with clean content. Verify it starts with valid JSON:

```bash
python3 -c "import json; json.load(open('docs/src/reductions/reduction_graph.json'))"
```

If the file is still corrupted after `make rust-export`, extract the valid portion:

```bash
python3 -c "
content = open('docs/src/reductions/reduction_graph.json').read()
idx = content.find('JSON content:\n')
if idx >= 0:
    clean = content[idx+len('JSON content:\n'):]
    open('docs/src/reductions/reduction_graph.json', 'w').write(clean)
    print('Fixed corrupted JSON')
else:
    print('JSON is clean')
"
```

- [ ] **Step 2: Count nodes, edges, and types**

```bash
python3 -c "
import json
data = json.load(open('docs/src/reductions/reduction_graph.json'))
nodes = data['nodes']
edges = data['edges']
names = sorted(set(n['name'] for n in nodes))
print(f'Unique problem types: {len(names)}')
print(f'Variant nodes: {len(nodes)}')
print(f'Total directed edges: {len(edges)}')
print(f'Types: {names}')
"
```

Expected: ~24 types, ~42 variant nodes, ~52 edges.

Count implemented ReduceTo impls (the "40 reductions" number):

```bash
grep -c 'impl.*ReduceTo' src/rules/*_*.rs | awk -F: '{s+=$2} END {print "Total ReduceTo impls:", s}'
```

Expected: ~40. Inferred variant edges = total edges - ReduceTo impls.

- [ ] **Step 3: Compute hub node degrees**

```bash
python3 -c "
import json
from collections import Counter
data = json.load(open('docs/src/reductions/reduction_graph.json'))
in_deg = Counter()
out_deg = Counter()
for e in data['edges']:
    src_name = next(n['name'] for n in data['nodes'] if n == e.get('source') or (n['name'] == e['source'].get('name', '') if isinstance(e['source'], dict) else False))
    # Simpler: just use source/target indices
for e in data['edges']:
    in_deg[e['target']['name']] += 1
    out_deg[e['source']['name']] += 1
print('Top in-degree (reduce TO this):')
for name, cnt in in_deg.most_common(5):
    print(f'  {name}: {cnt}')
print('Top out-degree (reduce FROM this):')
for name, cnt in out_deg.most_common(5):
    print(f'  {name}: {cnt}')
"
```

Record QUBO and ILP in-degrees, MIS and SAT out-degrees for S2.

- [ ] **Step 4: Count LOC per reduction (excluding casts files)**

```bash
for f in src/rules/*_*.rs; do
    case "$f" in *_casts.rs) continue;; esac
    echo "$(wc -l < "$f") $f"
done | sort -n
```

Record min, max, median for the "~50-200 LOC" claim.

- [ ] **Step 5: Save metrics to data file**

```bash
mkdir -p docs/paper/arxiv/data
```

Write a JSON file at `docs/paper/arxiv/data/graph-metrics.json` containing:
```json
{
  "unique_types": 24,
  "variant_nodes": 42,
  "total_edges": 52,
  "reduceto_impls": 40,
  "inferred_edges": 12,
  "hub_in_degree": {"QUBO": N, "ILP": N},
  "hub_out_degree": {"MIS": N, "SAT": N},
  "loc_per_reduction": {"min": N, "max": N, "median": N}
}
```

Fill in actual numbers from Steps 2-4.

- [ ] **Step 6: Commit**

```bash
git add -f docs/paper/arxiv/data/graph-metrics.json
git commit -m "docs(arxiv): gather reduction graph metrics"
```

---

## Chunk 2: Figures

**Conventions for all figure files:**
- Use `#set page(width: auto, height: auto, margin: 5pt)` for standalone compilation.
- To use `docs/paper/lib.typ` primitives, import with relative path: `#import "../../lib.typ"` (from `docs/paper/arxiv/images/`).
- Each file must export a public function (e.g., `#let reduction-graph() = { ... }`) for import into `paper.typ`.
- Verify standalone: `typst compile docs/paper/arxiv/images/<file>.typ` — expected: PDF output, no errors.
- Import into paper: `#import "images/<file>.typ": <function-name>`

### Task 3: Figure 1 — Reduction graph

**Files:**
- Create: `docs/paper/arxiv/images/reduction-graph.typ`
- Modify: `docs/paper/arxiv/paper.typ`

- [ ] **Step 1: Define node positions by category**

Create `docs/paper/arxiv/images/reduction-graph.typ`. Read the graph data from `docs/paper/arxiv/data/graph-metrics.json` and the full graph from `docs/src/reductions/reduction_graph.json`.

Use a column-based layout by category:
- Column 1 (blue): graph problems (MIS, MaxClique, MaxCut, MinVC, MinDS, MaxMatching, MaximalIS, KColoring, TSP, SpinGlass, BicliqueCover)
- Column 2 (green): formula problems (SAT, k-SAT, CircuitSAT)
- Column 3 (orange): set problems (MinSetCovering, MaxSetPacking)
- Column 4 (purple): algebraic problems (QUBO, ILP, CVP, BMF)
- Column 5 (gray): misc problems (BinPacking, PaintShop, Factoring, Knapsack)

Place QUBO and ILP centrally as hub nodes (larger circles).

For base problem types only (not all 42 variants — use the 24 unique names). Add a note in the caption about variant nodes.

Import graph drawing utilities: `#import "../../lib.typ"` for `g-node`, `g-edge` if helpful, or use raw CeTZ.

- [ ] **Step 2: Draw edges from the graph data**

Add directed edges (arrows) between nodes based on the reduction graph edges. Use `mark: (end: "straight")` for arrow heads. Group edges by category with consistent styling.

- [ ] **Step 3: Add legend and caption**

Add a color legend for the 5 categories. Define the exported function: `#let reduction-graph() = { ... }`.

- [ ] **Step 4: Verify figure compiles standalone**

Run: `typst compile docs/paper/arxiv/images/reduction-graph.typ`
Expected: PDF of the reduction graph, no errors.

- [ ] **Step 5: Import into paper.typ in S2**

Add to `paper.typ`:
```typst
#import "images/reduction-graph.typ": reduction-graph
```

In S2, place:
```typst
#figure(
  reduction-graph(),
  caption: [The reduction graph: 24 problem types connected by 52 directed edges (40 implemented reductions + 12 inferred variant edges). Hub nodes QUBO and ILP are highlighted.]
) <fig:reduction-graph>
```

- [ ] **Step 6: Commit**

```bash
git add -f docs/paper/arxiv/images/reduction-graph.typ docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): add Figure 1 — reduction graph"
```

---

### Task 4: Figure 3 — Pipeline diagram

**Files:**
- Create: `docs/paper/arxiv/images/pipeline.typ`
- Modify: `docs/paper/arxiv/paper.typ`

- [ ] **Step 1: Create pipeline diagram**

Use Fletcher (`@preview/fletcher:0.5.8`) for a flowchart showing the two-stage card-based pipeline.

Structure:
```
Contributor ──→ [Issue] ──→ Backlog
                              │ Maintainer moves card
                              ▼
                           [Ready]
                              │ project-pipeline (agent)
                              ▼
                        [In Progress]
                              │ issue-to-pr → check-issue → add-model/add-rule → review
                              ▼
                      [review-agentic]
                              │ review-pipeline (agent)
                              │ fix Copilot comments → agentic tests → fix CI
                              ▼
                        [In Review]
                              │ Maintainer merges
                              ▼
                           [Done]
```

Color-code: human decisions in warm color (orange/gold), agent actions in cool color (blue/teal). Board columns as rounded rectangles.

Export as: `#let pipeline-diagram() = { ... }`

- [ ] **Step 2: Verify figure compiles standalone**

Run: `typst compile docs/paper/arxiv/images/pipeline.typ`
Expected: PDF of pipeline flowchart, no errors.

- [ ] **Step 3: Import into paper.typ in S4**

Add `#import "images/pipeline.typ": pipeline-diagram` and place:
```typst
#figure(
  pipeline-diagram(),
  caption: [Two-stage card-based pipeline. Human decisions (orange) are limited to Backlog→Ready and In Review→Done. Agent manages everything in between.]
) <fig:pipeline>
```

- [ ] **Step 4: Commit**

```bash
git add -f docs/paper/arxiv/images/pipeline.typ docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): add Figure 3 — card-based pipeline diagram"
```

---

### Task 5: Figure 4 — Verification pyramid

**Files:**
- Create: `docs/paper/arxiv/images/verification-pyramid.typ`
- Modify: `docs/paper/arxiv/paper.typ`

- [ ] **Step 1: Create verification pyramid figure**

Draw a layered pyramid/stack using CeTZ with 7 layers, widest at bottom:

```
Layer 7: Documentation (proof sketch)             ← catches: logical errors
Layer 6: Agentic review (parallel subagents)       ← catches: convention violations
Layer 5: Materialized fixtures (JSON ground truth) ← catches: test gaming
Layer 4: Overhead validation (symbolic exprs)      ← catches: formula errors
Layer 3: Closed-loop tests (round-trip)            ← catches: mapping errors
Layer 2: Unit tests (eval, serialization)          ← catches: evaluation errors
Layer 1: Type system (Rust compiler)               ← catches: API misuse
```

Each layer labeled with mechanism (left) and error class caught (right). Color gradient from automated (bottom, blue) to human-readable (top, gold).

Export as: `#let verification-pyramid() = { ... }`

- [ ] **Step 2: Verify figure compiles standalone**

Run: `typst compile docs/paper/arxiv/images/verification-pyramid.typ`
Expected: PDF of pyramid, no errors.

- [ ] **Step 3: Import into paper.typ in S5**

Add `#import "images/verification-pyramid.typ": verification-pyramid` and place:
```typst
#figure(
  verification-pyramid(),
  caption: [Seven-layer verification stack. Lower layers (blue) are fully automated; upper layers (gold) involve human-readable arguments.]
) <fig:verification>
```

- [ ] **Step 4: Commit**

```bash
git add -f docs/paper/arxiv/images/verification-pyramid.typ docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): add Figure 4 — verification pyramid"
```

---

### Task 6: Figure 2 — System architecture

**Files:**
- Create: `docs/paper/arxiv/images/architecture.typ`
- Modify: `docs/paper/arxiv/paper.typ`

- [ ] **Step 1: Create architecture diagram**

Use Fletcher or CeTZ to show the key traits and compile-time validation:

```
┌─────────────────────────────────────┐
│           Problem trait              │
│  NAME, Metric, dims(), evaluate()   │
├──────────────┬──────────────────────┤
│ Optimization │    Satisfaction      │
│ SolutionSize │    bool              │
│ direction()  │                      │
└──────┬───────┴──────────────────────┘
       │ ReduceTo<T>
       ▼
┌─────────────────────────────────────┐
│        ReductionResult<T>           │
│  target_problem() + extract_solution│
└──────┬──────────────────────────────┘
       │ #[reduction(overhead = {...})]
       ▼
┌─────────────────────────────────────┐
│      Compile-time validation        │
│  • Variable names → getter methods  │
│  • Expr AST: symbolic overhead      │
│  • declare_variants! → registry     │
└─────────────────────────────────────┘
```

Keep compact. Focus on the verification-enabling aspects.

Export as: `#let architecture-diagram() = { ... }`

- [ ] **Step 2: Verify figure compiles standalone**

Run: `typst compile docs/paper/arxiv/images/architecture.typ`
Expected: PDF of architecture diagram, no errors.

- [ ] **Step 3: Import into paper.typ in S3**

Add `#import "images/architecture.typ": architecture-diagram` and place:
```typst
#figure(
  architecture-diagram(),
  caption: [System architecture: the trait hierarchy and compile-time validation enforce round-trip testing capability by construction.]
) <fig:architecture>
```

- [ ] **Step 4: Commit**

```bash
git add -f docs/paper/arxiv/images/architecture.typ docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): add Figure 2 — system architecture"
```

---

## Chunk 3: Sections S1-S4

**Convention:** All "Verify compiles" steps use: `typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf`. Expected: no errors. All sections use citation format `@BibKey` (e.g., `@Thai2025SWEEVO`). Before writing any section, first read `paper.typ` to understand the heading style and formatting conventions established in Task 1.

**Page budget reference** (two-column format, ~500 words/page):
- S1: ~1.5 pages (~750 words)
- S2: ~1 page (~500 words)
- S3: ~1.5 pages (~750 words)
- S4: ~2 pages (~1000 words)

### Task 7: Write S1 — Introduction

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`

- [ ] **Step 1: Write introduction body (~750 words)**

First read `paper.typ` to understand the heading format. Then write S1 within the existing `= Introduction` stub. Structure:

1. Opening paragraph: agents hit 70-80% on SWE-Bench but ~20% on long-horizon → cite `@Thai2025SWEEVO`, `@Deng2025SWEBenchPro`
2. Our thesis: bottleneck is decomposition, not capability
3. "Review is harder than generation" for mathematical code → cite `@Roychoudhury2025AgenticAI`
4. Three roles paragraph: contributors (creative issues), maintainer (board + skills), agents (manage + execute)
5. Contributions list (3 items from spec)
6. Paper organization paragraph

- [ ] **Step 2: Verify compiles**

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): write S1 Introduction"
```

---

### Task 8: Write S2 — Why Reductions?

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`
**Depends on:** Task 2 (graph metrics), Task 3 (Figure 1)

- [ ] **Step 1: Write S2 body (~500 words)**

Read graph metrics from `docs/paper/arxiv/data/graph-metrics.json` for concrete numbers. If not yet available, use: 24 types, 42 variants, 52 edges, 40 implemented, 12 inferred.

Structure:
1. Goldilocks domain paragraph: self-contained (~50-200 LOC), formally specified, automatable round-trip criterion
2. Contrast with SWE-Bench: homogeneous tasks enable comparison
3. Hardware solvers paragraph: Rydberg atoms for MIS (cite `@lucas2014`), D-Wave for QUBO/Ising (cite `@glover2019`) → the graph as compilation layer
4. Real-world applications paragraph: SDN→ILP, airline→SetCovering, VLSI→coloring, logistics→TSP
5. Reference `@fig:reduction-graph` (placed by Task 3)

- [ ] **Step 2: Verify compiles**

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): write S2 Why Reductions — Goldilocks domain"
```

---

### Task 9: Write S3 — System Architecture

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`
**Depends on:** Task 6 (Figure 2)

- [ ] **Step 1: Write S3 body (~750 words)**

Use the trait hierarchy from CLAUDE.md's Architecture section for reference. Do NOT read source files — the CLAUDE.md summary has sufficient detail. Full trait code belongs in supplementary material.

Structure:
1. Problem trait: `evaluate()` enables brute-force verification of any configuration
2. ReduceTo trait: type system enforces round-trip capability by construction
3. `#[reduction(overhead)]` proc macro: compile-time validation of overhead expressions
4. `declare_variants!`: registry enables automated graph export + completeness checking
5. Design philosophy paragraph: reduce the space of possible agent errors
6. Reference `@fig:architecture` (placed by Task 6)

- [ ] **Step 2: Verify compiles**

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): write S3 System Architecture"
```

---

### Task 10: Write S4 — Skill-Based Task Decomposition

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`
**Depends on:** Task 4 (Figure 3)

- [ ] **Step 1: Write S4.1 — Three Roles (~200 words)**

The roles table from the spec (Contributor/Maintainer/Agent with responsibilities and examples). Brief narrative explaining the human-agent boundary.

- [ ] **Step 2: Read skill files and extract metadata**

Read all 13 skill files (`.claude/skills/*/SKILL.md`). For each, record: name, one-line description, invocation trigger, step count. This data populates Table 1.

- [ ] **Step 3: Write S4.2 — Skills as Agent Functions (~500 words)**

Group the 13 skills into 5 categories (from spec):
- **Orchestration** (4): project-pipeline, review-pipeline, issue-to-pr, meta-power
- **Implementation** (2): add-model, add-rule
- **Quality gate** (4): check-issue, check-rule-redundancy, review-implementation, fix-pr
- **Documentation** (2): write-model-in-paper, write-rule-in-paper
- **Release** (1): release

For each group, write 1-2 sentences explaining the pattern. Create Table 1 with columns: Skill, Category, Trigger, Typical Turns (estimate from step count / 3), Success Rate (use "TBD" — will be filled after Task 11).

- [ ] **Step 4: Write S4.3 — Card-Based Orchestration (~300 words)**

Two-stage pipeline (project-pipeline → review-pipeline). Human touches only Backlog→Ready and In Review→Done. Reference `@fig:pipeline` (placed by Task 4).

- [ ] **Step 5: Verify compiles**

- [ ] **Step 6: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): write S4 Skill-Based Task Decomposition"
```

---

## Chunk 4: Sections S5-S6

### Task 11: Git history mining script

**Files:**
- Create: `docs/paper/arxiv/scripts/mine-git-history.py`
- Create: `docs/paper/arxiv/data/git-mining-results.json`

- [ ] **Step 1: Create directories**

```bash
mkdir -p docs/paper/arxiv/scripts docs/paper/arxiv/data
```

- [ ] **Step 2: Write PR listing and field extraction**

Write `docs/paper/arxiv/scripts/mine-git-history.py` — Part 1: list all merged PRs with `[Rule]` or `[Model]` in the title.

```bash
gh pr list --repo CodingThrust/ProblemReductions --state merged --limit 999 --json number,title,author,createdAt,mergedAt,labels,headRefName
```

For each PR, extract: number, title, author login, created date, merged date, whether title contains `[Rule]` or `[Model]`.

Author classification: if `author.login` contains `[bot]` or is `github-actions`, classify as "agent"; otherwise "human".

- [ ] **Step 3: Add phase classification and CI status**

Add to the script:

**Phase boundaries** (based on when key skills were introduced — determine by running):
```bash
git log --all --oneline --diff-filter=A -- '.claude/skills/add-rule/SKILL.md' | tail -1
git log --all --oneline --diff-filter=A -- '.claude/skills/project-pipeline/SKILL.md' | tail -1
```

Define phases:
- Phase 1 (manual): PRs before add-model/add-rule skills existed
- Phase 2 (basic skills): PRs after implementation skills but before pipeline skills
- Phase 3 (full pipeline): PRs after project-pipeline/review-pipeline skills existed

For CI status on first push, use:
```bash
gh api repos/CodingThrust/ProblemReductions/pulls/{number}/commits --jq '.[0].sha'
```
Then check that SHA's status. This is optional — skip if the API calls are too slow.

- [ ] **Step 4: Run script and save results**

```bash
python3 docs/paper/arxiv/scripts/mine-git-history.py > docs/paper/arxiv/data/git-mining-results.json
```

Expected output schema:
```json
{
  "summary": {
    "total_prs": N,
    "rule_prs": N,
    "model_prs": N,
    "agent_authored": N,
    "human_authored": N
  },
  "by_phase": [
    {"phase": 1, "label": "manual", "count": N, "agent_count": N},
    {"phase": 2, "label": "basic_skills", "count": N, "agent_count": N},
    {"phase": 3, "label": "full_pipeline", "count": N, "agent_count": N}
  ],
  "prs": [
    {"number": 42, "title": "...", "is_agent": false, "phase": 1, "type": "Rule"}
  ]
}
```

- [ ] **Step 5: Commit**

```bash
git add -f docs/paper/arxiv/scripts/ docs/paper/arxiv/data/git-mining-results.json
git commit -m "docs(arxiv): git history mining script and results"
```

---

### Task 12: Write S5 — Multi-Layered Verification

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`
**Depends on:** Task 5 (Figure 4)

- [ ] **Step 1: Write S5.1 — The Verification Stack (~500 words)**

Write the 7-layer table from the spec. Use these concrete error examples for each layer (constructed from the domain):

| Layer | Mechanism | Example Error Caught |
|-------|-----------|---------------------|
| 1. Type system | Rust compiler | Agent returns `bool` instead of `SolutionSize<i32>` from `evaluate()` |
| 2. Unit tests | `test_*_basic` | Agent evaluates MaxCut objective with wrong sign (sum vs difference) |
| 3. Closed-loop tests | `test_*_to_*_closed_loop` | SAT→MIS reduction maps clause variables to wrong vertex indices |
| 4. Overhead validation | Symbolic expr vs sizes | Agent writes `num_edges = num_clauses` instead of `3 * num_clauses` |
| 5. Materialized fixtures | JSON ground truth | Agent changes expected QUBO matrix values to make failing test pass |
| 6. Agentic review | Parallel subagents | Missing `declare_variants!` macro, wrong file naming convention |
| 7. Documentation | Proof sketch | Reduction proof assumes graph is connected but problem allows disconnected |

Reference `@fig:verification` (placed by Task 5).

- [ ] **Step 2: Write S5.2 — Why Layers? (~250 words)**

The "lazy agent" problem: agents take the shortest path to close an issue (e.g., changing expected test values instead of fixing bugs). Materialized test data (Layer 5) prevents this. No single layer is sufficient. Cross-reference Table 2 in S6.

- [ ] **Step 3: Verify compiles**

Run: `typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf`

- [ ] **Step 4: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): write S5 Multi-Layered Verification"
```

---

### Task 13: Write S6 — Evaluation

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`
**Depends on:** Task 11 (git mining data)

- [ ] **Step 1: Write S6.1 — Ablation setup (~400 words)**

Describe the experimental DESIGN (actual results are `[TBD: ablation not yet run]` placeholders):
- Setup: select 5-10 reductions of varying complexity
- Two configurations: skill-based (full pipeline) vs no-skill baseline (raw agent + CLAUDE.md only)
- Metrics: first-attempt CI pass rate, review rounds, final correctness, convention adherence
- Framing: "controlled illustration" (n=5-10), not statistically powered experiment

- [ ] **Step 2: Write S6.2 — Git History Mining results (~500 words)**

Read data from `docs/paper/arxiv/data/git-mining-results.json`. If not yet available, use `[TBD: data]` placeholders.

Write up agent vs human implementation counts, success rates stratified by phase.

Create Table 2 (error taxonomy × verification layer matrix):

| Error Category | Layer | Example | Count |
|---------------|-------|---------|-------|
| Type errors | 1 (type system) | Wrong return type | [TBD] |
| Mapping errors | 3 (closed-loop) | Wrong vertex index | [TBD] |
| Formula errors | 4 (overhead) | Linear vs quadratic | [TBD] |
| Test gaming | 5 (fixtures) | Changed expected value | [TBD] |
| Convention violations | 6 (review) | Missing macro | [TBD] |
| Logical errors | 7 (documentation) | Invalid proof | [TBD] |

- [ ] **Step 3: Write S6.3 — Case Studies (~600 words)**

Three reductions spanning the complexity spectrum. For each, find the actual PR by searching:

```bash
gh pr list --repo CodingThrust/ProblemReductions --state merged --limit 999 --search "MinimumVertexCover MaximumIndependentSet" --json number,title
gh pr list --repo CodingThrust/ProblemReductions --state merged --limit 999 --search "Satisfiability MaximumIndependentSet" --json number,title
gh pr list --repo CodingThrust/ProblemReductions --state merged --limit 999 --search "Factoring CircuitSAT" --json number,title
```

If PRs are found, reference them and analyze the pipeline trace (skills activated, human decisions, errors caught). If not found, describe the expected pipeline trace based on the skill definitions.

**Case 1 — Simple (MVC→MIS):** complement relationship, ~30 LOC, smooth pipeline.
**Case 2 — Complex (SAT→MIS):** clause-variable gadget, quadratic blowup, agent mistakes in edge counts.
**Case 3 — Composition (Factoring→CircuitSAT→ILP):** two independent reductions that compose in the graph. Analyze each separately, then show graph-level composition.

- [ ] **Step 4: Verify compiles**

Run: `typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf`

- [ ] **Step 5: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): write S6 Evaluation"
```

---

## Chunk 5: Sections S7-S8 + Final Assembly

### Task 14: Write S7 — Related Work

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`

- [ ] **Step 1: Write S7 body (~500 words)**

Four subsections, each 1-2 paragraphs. Use these specific citation keys:

1. **AI coding agents:** `@Yang2024SWEagent`, `@Wang2024OpenHands`, `@Anthropic2025ClaudeCode`, `@Wu2024Devin`, `@Thai2025SWEEVO` (SWE-EVO ~20%), `@Deng2025SWEBenchPro` (SWE-Bench Pro ~45%), `@Xia2025LiveSWEagent` (self-evolution complementary to skills), `@Roychoudhury2025AgenticAI` (agentic SE perspective), `@Anthropic2026AgenticCoding` (developer-AI collaboration survey)

2. **AI-discovered reductions:** `@Novikov2025AlphaEvolve` (NP-hardness gadgets), `@Janicic2025URSA` (SAT-based verification), `@RomeraParedes2023FunSearch`. Our work is complementary: we implement/verify known reductions, not discover new ones.

3. **Formal verification:** `@Bursuc2025VeriCoding`, `@Thakur2025CLEVER`, `@Miranda2025VeriBench`, `@Mukherjee2025CoqPL`, `@Mukherjee2025SynVer`. Our approach: pragmatic multi-layer verification vs end-to-end formal proofs.

4. **Physics-inspired optimization:** `@Schuetz2022PhysicsGNN` (GNN/QUBO for MIS/MaxCut/MinVC at million-variable scale), `@He2024QuantumTSP`. Our graph provides the verified compilation layer connecting problems to these solvers.

For each: position our work as complementary, not competing.

- [ ] **Step 2: Verify compiles**

Run: `typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf`

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): write S7 Related Work"
```

---

### Task 15: Write S8 — Discussion & Conclusion

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`

- [ ] **Step 1: Write S8 body (~500 words)**

Four parts from spec, then a concluding subsection:

1. **Generalizability:** Goldilocks property, candidate domains (compiler peephole rules, algebraic identities, protocol verification lemmas)
2. **Limitations:** n=1 threat, skill engineering cost, domain specificity, git mining confounds (addressed by stratification), maintainer requirement
3. **Human value proposition:** repositioned not eliminated, creativity + judgment remains human. Cite `@Anthropic2026AgenticCoding` for the broader trend.
4. **Future directions:** AlphaEvolve integration (cite `@Novikov2025AlphaEvolve`), formal verification (cite `@Bursuc2025VeriCoding`), scaling to 100+ problems

End with a `=== Conclusion` subsection: 2-3 crisp sentences restating the thesis and key result.

- [ ] **Step 2: Verify compiles**

Run: `typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf`

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.typ
git commit -m "docs(arxiv): write S8 Discussion and Conclusion"
```

---

### Task 16: Final assembly and polish

**Files:**
- Modify: `docs/paper/arxiv/paper.typ`

- [ ] **Step 1: Verify all figures are placed correctly**

Check that these figure references exist in the paper text:
- `@fig:reduction-graph` in S2
- `@fig:architecture` in S3
- `@fig:pipeline` in S4
- `@fig:verification` in S5

Search for each label in `paper.typ`. If any is missing, add the reference.

- [ ] **Step 2: Verify all tables are placed correctly**

Check for Table 1 (skills inventory) in S4 and Table 2 (error taxonomy) in S6.

- [ ] **Step 3: Verify all citations resolve**

```bash
typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf 2>&1 | grep -i "warning\|error\|unknown\|not found"
```

Expected: no unresolved citation or label warnings. If any `@key` references are missing from `references.bib`, add them.

- [ ] **Step 4: Check page count**

```bash
typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf && python3 -c "
import subprocess
result = subprocess.run(['pdfinfo', 'docs/paper/arxiv/paper.pdf'], capture_output=True, text=True)
for line in result.stdout.splitlines():
    if 'Pages' in line:
        print(line)
"
```

Expected: 10-12 pages. If over, identify sections to trim. If under, identify sections to expand.

- [ ] **Step 5: Final compile and flag visual review**

Run: `typst compile docs/paper/arxiv/paper.typ docs/paper/arxiv/paper.pdf`

Verify no warnings are emitted. Visual inspection (layout, orphans, figure legibility) requires human review — flag as TODO for the maintainer.

- [ ] **Step 6: Commit final version**

```bash
git add -f docs/paper/arxiv/paper.typ docs/paper/arxiv/images/ docs/paper/arxiv/references.bib
git commit -m "docs(arxiv): final paper assembly and polish"
```

Note: Do NOT commit `paper.pdf` — it is a build artifact.

---

## Execution Notes

### Dependency Graph

```
Task 1 (scaffolding) ──→ Task 2 (metrics) ──→ Task 8 (S2)
                     ──→ Tasks 3-6 (figures, parallel)
                     ──→ Task 11 (git mining)

Task 3 (Fig 1) ──→ Task 8 (S2)
Task 4 (Fig 3) ──→ Task 10 (S4)
Task 5 (Fig 4) ──→ Task 12 (S5)
Task 6 (Fig 2) ──→ Task 9 (S3)

Task 7 (S1): no figure dependency — can run after Task 1
Task 11 (git mining) ──→ Task 13 (S6)
Task 14 (S7): independent — can run after Task 1
Task 15 (S8): independent — can run after Task 1
Task 16 (assembly): must run LAST
```

### Suggested Parallel Batches

1. **Tasks 1-2** (scaffolding + data) — sequential, run first
2. **Tasks 3-6** (all figures) + **Task 7** (S1) + **Task 11** (git mining) — parallel
3. **Tasks 8-10** (S2-S4) + **Tasks 14-15** (S7-S8) — parallel (each depends on its figure from batch 2)
4. **Tasks 12-13** (S5-S6) — parallel (depend on Figure 4 + git mining from batch 2)
5. **Task 16** (assembly) — last

### Open Dependencies

- **S6.1 ablation results** are `[TBD]` placeholders. The ablation experiment is a separate effort outside this plan. The paper will contain placeholder markers until that data is available.
- **Table 1 success rates** are `[TBD]` — will be filled from git mining data (Task 11) if available, otherwise left as placeholders.
