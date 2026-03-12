# Arxiv Paper Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Write a full research paper (~10-12 pages) on skill-based agentic coding for NP-hard problem reductions, targeting an ICSE/ASE-class venue.

**Architecture:** LaTeX document at `docs/paper/arxiv/paper.tex` using IEEEtran class with figures generated in Typst+CeTZ (compiled to PDF, included via `\includegraphics`), bibliography from survey, and data gathered from git history and the reduction graph.

**Tech Stack:** LaTeX (IEEEtran class), BibTeX, pdflatex, Typst+CeTZ (figures only)

**Spec:** `docs/superpowers/specs/2026-03-12-arxiv-paper-design.md`

**Compile command** (used throughout):
```bash
# Compile Typst figures first
for f in docs/paper/arxiv/figures/*.typ; do typst compile "$f"; done
# Then build LaTeX
cd docs/paper/arxiv && pdflatex paper.tex && bibtex paper && pdflatex paper.tex && pdflatex paper.tex && cd -
```
Or single-pass check (figures already compiled): `cd docs/paper/arxiv && pdflatex -interaction=nonstopmode paper.tex && cd -`

**Review skill:** After writing is complete, use `academic-paper-reviewer` (installed at `.claude/skills/academic-research-skills/academic-paper-reviewer/`) for simulated 5-person peer review.

---

## File Structure

| File | Purpose |
|------|---------|
| `docs/paper/arxiv/paper.tex` | Main paper document (IEEEtran) |
| `docs/paper/arxiv/references.bib` | Bibliography (merged from survey + existing paper refs) |
| `docs/paper/arxiv/figures/reduction-graph.typ` | Figure 1: Reduction graph (Typst+CeTZ → PDF) |
| `docs/paper/arxiv/figures/architecture.typ` | Figure 2: System architecture (Typst+CeTZ → PDF) |
| `docs/paper/arxiv/figures/pipeline.typ` | Figure 3: Card-based pipeline (Typst+CeTZ → PDF) |
| `docs/paper/arxiv/figures/verification-pyramid.typ` | Figure 4: Verification stack pyramid (Typst+CeTZ → PDF) |
| `docs/paper/arxiv/data/graph-metrics.json` | Reduction graph metrics (from Task 2) |
| `docs/paper/arxiv/data/git-mining-results.json` | Git history mining results (from Task 11) |
| `docs/paper/arxiv/scripts/mine-git-history.py` | Git history mining script |

---

## Chunk 1: Paper Scaffolding + Data Gathering

### Task 1: Set up paper.tex scaffolding

**Files:**
- Create: `docs/paper/arxiv/paper.tex`
- Create: `docs/paper/arxiv/references.bib`

- [ ] **Step 1: Create bibliography file**

Copy the survey bibliography:

```bash
cp .claude/survey/agentic-coding-reductions/references.bib docs/paper/arxiv/references.bib
```

Then append the following entries from `docs/paper/references.bib` (read that file and copy these exact `@` entries by key): `karp1972`, `cook1971`, `garey1979`, `glover2019`, `lucas2014`, `barahona1982`. These are foundational references not in the survey bib.

- [ ] **Step 2: Write paper.tex with IEEEtran class**

Create `docs/paper/arxiv/paper.tex` with:

```latex
\documentclass[conference]{IEEEtran}
\usepackage{cite}
\usepackage{amsmath,amssymb,amsfonts}
\usepackage{graphicx}
\usepackage{textcomp}
\usepackage{xcolor}
\usepackage{booktabs}
\usepackage{listings}
\usepackage{hyperref}
\usepackage{cleveref}

\begin{document}

\title{Skill-Based Agentic Coding for Mathematical Software:\\
A Case Study in NP-Hard Problem Reductions}

\author{...}  % placeholder

\maketitle

\begin{abstract}
...
\end{abstract}

\section{Introduction}\label{sec:intro}
\section{Why Reductions? The Goldilocks Domain}\label{sec:domain}
\section{System Architecture}\label{sec:architecture}
\section{Skill-Based Task Decomposition}\label{sec:skills}
\section{Multi-Layered Verification}\label{sec:verification}
\section{Evaluation}\label{sec:evaluation}
\section{Related Work}\label{sec:related}
\section{Discussion \& Conclusion}\label{sec:conclusion}

\bibliographystyle{IEEEtran}
\bibliography{references}

\end{document}
```

- [ ] **Step 3: Write abstract (~150 words)**

Fill in the abstract covering:
- Problem: agents fail at long-horizon math coding tasks (70-80% on SWE-Bench Verified, ~20% on long-horizon)
- Insight: decompose into human-creative + agent-managed/executed via skill-based pipeline
- Method: 13 skills + 7-layer verification stack
- Result: 24 problem types, 40 implemented reductions, 52 graph edges
- Contribution: methodology + verification stack + open-source artifact

- [ ] **Step 4: Create figures directory**

```bash
mkdir -p docs/paper/arxiv/figures
```

- [ ] **Step 5: Verify scaffolding compiles**

```bash
cd docs/paper/arxiv && pdflatex -interaction=nonstopmode paper.tex && cd -
```

Expected: PDF with title, abstract, and empty section headings. BibTeX warnings about missing refs are expected at this stage.

- [ ] **Step 6: Remove old paper.typ**

```bash
rm -f docs/paper/arxiv/paper.typ
```

- [ ] **Step 7: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex docs/paper/arxiv/references.bib
git rm -f docs/paper/arxiv/paper.typ 2>/dev/null; true
git commit -m "docs(arxiv): LaTeX paper scaffolding with IEEEtran and bibliography"
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
python3 << 'PYEOF'
import json
from collections import Counter
data = json.load(open('docs/src/reductions/reduction_graph.json'))
in_deg = Counter()
out_deg = Counter()
for e in data['edges']:
    # edges use node dicts with 'name' field
    src = e['source']['name'] if isinstance(e['source'], dict) else data['nodes'][e['source']]['name']
    tgt = e['target']['name'] if isinstance(e['target'], dict) else data['nodes'][e['target']]['name']
    in_deg[tgt] += 1
    out_deg[src] += 1
print('Top in-degree (reduce TO this):')
for name, cnt in in_deg.most_common(5):
    print(f'  {name}: {cnt}')
print('Top out-degree (reduce FROM this):')
for name, cnt in out_deg.most_common(5):
    print(f'  {name}: {cnt}')
PYEOF
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

**Conventions for all figure files (Typst+CeTZ → PDF hybrid):**
- Each figure is a standalone `.typ` file in `docs/paper/arxiv/figures/`.
- Figures use `#set page(width: auto, height: auto, margin: 5pt)` for tight bounding box.
- Import CeTZ: `#import "@preview/cetz:0.4.2": canvas, draw`.
- Import the project graph library when useful: `#import "../../../lib.typ": g-node, g-edge, graph-colors`.
- Color scheme: graph=`rgb("#4e79a7")` (blue), formula=`rgb("#59a14f")` (green), set=`rgb("#e15759")` (orange-red), algebraic=`rgb("#b07aa1")` (purple), misc=`rgb("#999")` (gray). Human=`rgb("#f28e2b")` (orange), Agent=`rgb("#4e79a7")` (blue).
- Arrow style: `mark: (end: "straight")` for directed edges.
- Compile each figure to PDF: `typst compile docs/paper/arxiv/figures/filename.typ`.
- Include in LaTeX via `\includegraphics{figures/filename.pdf}`.
- Test figures by compiling individually before full paper build.
- Do NOT commit generated `.pdf` files — they are build artifacts.

### Task 3: Figure 1 — Reduction graph

**Files:**
- Create: `docs/paper/arxiv/figures/reduction-graph.typ`
- Modify: `docs/paper/arxiv/paper.tex`

- [ ] **Step 1: Create reduction graph figure**

Create `docs/paper/arxiv/figures/reduction-graph.typ`. Read the graph data from `docs/src/reductions/reduction_graph.json` for edge connectivity.

```typst
#import "@preview/cetz:0.4.2": canvas, draw
#set page(width: auto, height: auto, margin: 5pt)
#set text(size: 7pt)

// Category colors
#let cat-graph = rgb("#4e79a7")
#let cat-formula = rgb("#59a14f")
#let cat-set = rgb("#e15759")
#let cat-algebraic = rgb("#b07aa1")
#let cat-misc = rgb("#999")

#canvas(length: 1cm, {
  import draw: *

  // Node positions by category (column-based layout)
  // Column 1: graph problems, Column 2: formula, etc.
  // Place QUBO and ILP centrally as hub nodes (larger radius)

  // ... define positions for all 24 unique problem types ...
  // ... draw directed edges from graph JSON ...
  // ... add legend box ...
})
```

Use a column-based layout by category:
- Column 1 (blue): graph problems (MIS, MaxClique, MaxCut, MinVC, MinDS, MaxMatching, MaximalIS, KColoring, TSP, SpinGlass, BicliqueCover)
- Column 2 (green): formula problems (SAT, k-SAT, CircuitSAT)
- Column 3 (orange-red): set problems (MinSetCovering, MaxSetPacking)
- Column 4 (purple): algebraic problems (QUBO, ILP, CVP, BMF, Knapsack)
- Column 5 (gray): misc problems (BinPacking, PaintShop, Factoring)

Place QUBO and ILP centrally as hub nodes (larger circles, `radius: 0.4` vs `0.2`). Use the 24 unique problem type names (not all 42 variants). Mention variants in caption.

For each node, use `draw.circle(pos, radius: r, fill: cat-color.lighten(70%), stroke: 0.5pt + cat-color, name: id)` and `draw.content(id, text(6pt, abbreviation))`.

For directed edges, use `draw.line(src, tgt, stroke: 0.4pt + luma(100), mark: (end: "straight", scale: 0.4))`. Keep edges thin to avoid clutter with 52 edges.

Add a small legend box in one corner with the 5 category colors.

- [ ] **Step 2: Compile figure to PDF**

```bash
typst compile docs/paper/arxiv/figures/reduction-graph.typ
```

Verify output: `docs/paper/arxiv/figures/reduction-graph.pdf` exists.

- [ ] **Step 3: Include in paper.tex**

In Section 2, add:
```latex
\begin{figure*}[t]
  \centering
  \includegraphics[width=\textwidth]{figures/reduction-graph.pdf}
  \caption{The reduction graph: 24 problem types connected by 52 directed edges (40 implemented reductions + 12 inferred variant edges). Hub nodes QUBO and ILP are highlighted.}
  \label{fig:reduction-graph}
\end{figure*}
```

Use `figure*` for full-width in two-column layout.

- [ ] **Step 4: Verify full paper compiles**

```bash
cd docs/paper/arxiv && pdflatex -interaction=nonstopmode paper.tex && cd -
```

- [ ] **Step 5: Commit**

```bash
git add -f docs/paper/arxiv/figures/reduction-graph.typ docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): add Figure 1 — reduction graph (Typst+CeTZ)"
```

---

### Task 4: Figure 3 — Pipeline diagram

**Files:**
- Create: `docs/paper/arxiv/figures/pipeline.typ`
- Modify: `docs/paper/arxiv/paper.tex`

- [ ] **Step 1: Create pipeline diagram**

Create `docs/paper/arxiv/figures/pipeline.typ` using CeTZ:

```typst
#import "@preview/cetz:0.4.2": canvas, draw
#set page(width: auto, height: auto, margin: 5pt)
#set text(size: 8pt)

#let human-color = rgb("#f28e2b")
#let agent-color = rgb("#4e79a7")

#canvas(length: 1cm, {
  import draw: *

  // Board columns as rounded rectangles, connected vertically
  // Color-code: human decisions in orange, agent actions in blue
  // Layout:
  // Contributor → [Issue] → [Backlog]
  //                            │ Maintainer moves card (orange)
  //                            ▼
  //                         [Ready]
  //                            │ project-pipeline (blue)
  //                            ▼
  //                      [In Progress]
  //                            │ issue-to-pr → check → implement → review (blue)
  //                            ▼
  //                    [review-agentic]
  //                            │ review-pipeline (blue)
  //                            ▼
  //                      [In Review]
  //                            │ Maintainer merges (orange)
  //                            ▼
  //                         [Done]

  // Use rect(..., radius: 4pt) for rounded board columns
  // Use line() with mark: (end: "straight") for arrows
  // Add action labels on edges with draw.content()
})
```

- [ ] **Step 2: Compile figure to PDF**

```bash
typst compile docs/paper/arxiv/figures/pipeline.typ
```

- [ ] **Step 3: Include in paper.tex in S4**

```latex
\begin{figure}[t]
  \centering
  \includegraphics[width=\columnwidth]{figures/pipeline.pdf}
  \caption{Two-stage card-based pipeline. Human decisions (orange) are limited to Backlog$\to$Ready and In Review$\to$Done. Agent manages everything in between.}
  \label{fig:pipeline}
\end{figure}
```

- [ ] **Step 4: Verify compiles**

- [ ] **Step 5: Commit**

```bash
git add -f docs/paper/arxiv/figures/pipeline.typ docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): add Figure 3 — card-based pipeline diagram (Typst+CeTZ)"
```

---

### Task 5: Figure 4 — Verification pyramid

**Files:**
- Create: `docs/paper/arxiv/figures/verification-pyramid.typ`
- Modify: `docs/paper/arxiv/paper.tex`

- [ ] **Step 1: Create verification pyramid figure**

Create `docs/paper/arxiv/figures/verification-pyramid.typ` using CeTZ:

```typst
#import "@preview/cetz:0.4.2": canvas, draw
#set page(width: auto, height: auto, margin: 5pt)
#set text(size: 7pt)

#canvas(length: 1cm, {
  import draw: *

  // 7-layer trapezoid/pyramid, widest at bottom
  // Each layer is a filled trapezoid with text on left (mechanism) and right (error class)
  // Color gradient: bottom = blue (automated), top = orange/gold (human-readable)

  // Layer data: (mechanism, error class caught)
  // 1: Type system (Rust compiler)        → API misuse
  // 2: Unit tests (eval, serialization)   → evaluation errors
  // 3: Closed-loop tests (round-trip)     → mapping errors
  // 4: Overhead validation (symbolic)     → formula errors
  // 5: Materialized fixtures (JSON)       → test gaming
  // 6: Agentic review (parallel)          → convention violations
  // 7: Documentation (proof sketch)       → logical errors

  // Draw each layer as a trapezoid using merge-path with line segments
  // Width decreases from bottom to top
  // Use draw.content() for labels on each layer
  // Use color.mix() or manual gradient for blue→gold transition
})
```

- [ ] **Step 2: Compile figure to PDF**

```bash
typst compile docs/paper/arxiv/figures/verification-pyramid.typ
```

- [ ] **Step 3: Include in paper.tex in S5**

```latex
\begin{figure}[t]
  \centering
  \includegraphics[width=\columnwidth]{figures/verification-pyramid.pdf}
  \caption{Seven-layer verification stack. Lower layers (blue) are fully automated; upper layers (gold) involve human-readable arguments.}
  \label{fig:verification}
\end{figure}
```

- [ ] **Step 4: Verify compiles**

- [ ] **Step 5: Commit**

```bash
git add -f docs/paper/arxiv/figures/verification-pyramid.typ docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): add Figure 4 — verification pyramid (Typst+CeTZ)"
```

---

### Task 6: Figure 2 — System architecture

**Files:**
- Create: `docs/paper/arxiv/figures/architecture.typ`
- Modify: `docs/paper/arxiv/paper.tex`

- [ ] **Step 1: Create architecture diagram**

Create `docs/paper/arxiv/figures/architecture.typ` using CeTZ:

```typst
#import "@preview/cetz:0.4.2": canvas, draw
#set page(width: auto, height: auto, margin: 5pt)
#set text(size: 8pt)

#canvas(length: 1cm, {
  import draw: *

  // Three stacked boxes connected by labeled arrows:
  //
  // ┌─────────────────────────────────────┐
  // │           Problem trait              │
  // │  NAME, Metric, dims(), evaluate()   │
  // ├──────────────┬──────────────────────┤
  // │ Optimization │    Satisfaction      │
  // │ SolutionSize │    bool              │
  // └──────┬───────┴──────────────────────┘
  //        │ ReduceTo<T>
  //        ▼
  // ┌─────────────────────────────────────┐
  // │        ReductionResult<T>           │
  // │  target_problem() + extract_solution│
  // └──────┬──────────────────────────────┘
  //        │ #[reduction(overhead = {...})]
  //        ▼
  // ┌─────────────────────────────────────┐
  // │      Compile-time validation        │
  // │  • Variable names → getter methods  │
  // │  • Expr AST: symbolic overhead      │
  // │  • declare_variants! → registry     │
  // └─────────────────────────────────────┘

  // Use rect() with name for each box
  // Use draw.content() for text inside boxes (use raw() for code identifiers)
  // Use line() with mark for connecting arrows
  // Use draw.content() on arrow midpoints for edge labels
})
```

Keep compact. Use `raw()` (backtick syntax) for code identifiers in Typst.

- [ ] **Step 2: Compile figure to PDF**

```bash
typst compile docs/paper/arxiv/figures/architecture.typ
```

- [ ] **Step 3: Include in paper.tex in S3**

```latex
\begin{figure}[t]
  \centering
  \includegraphics[width=\columnwidth]{figures/architecture.pdf}
  \caption{System architecture: the trait hierarchy and compile-time validation enforce round-trip testing capability by construction.}
  \label{fig:architecture}
\end{figure}
```

- [ ] **Step 4: Verify compiles**

- [ ] **Step 5: Commit**

```bash
git add -f docs/paper/arxiv/figures/architecture.typ docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): add Figure 2 — system architecture (Typst+CeTZ)"
```

---

## Chunk 3: Sections S1-S4

**Convention:** All "Verify compiles" steps use: `cd docs/paper/arxiv && pdflatex -interaction=nonstopmode paper.tex && cd -`. Expected: no fatal errors. Citations use `\cite{BibKey}` (e.g., `\cite{Thai2025SWEEVO}`). Cross-references use `\Cref{fig:...}` or `Fig.~\ref{fig:...}`. Before writing any section, first read `paper.tex` to understand the formatting conventions established in Task 1.

**Page budget reference** (IEEEtran two-column, ~800 words/page):
- S1: ~1.5 pages (~1200 words)
- S2: ~1 page (~800 words)
- S3: ~1.5 pages (~1200 words)
- S4: ~2 pages (~1600 words)

### Task 7: Write S1 — Introduction

**Files:**
- Modify: `docs/paper/arxiv/paper.tex`

- [ ] **Step 1: Write introduction body (~1200 words)**

First read `paper.tex` to understand the document structure. Then fill in `\section{Introduction}`. Structure:

1. Opening paragraph: agents hit 70-80% on SWE-Bench but ~20% on long-horizon → cite `\cite{Thai2025SWEEVO}`, `\cite{Deng2025SWEBenchPro}`
2. Our thesis: bottleneck is decomposition, not capability
3. "Review is harder than generation" for mathematical code → cite `\cite{Roychoudhury2025AgenticAI}`
4. Three roles paragraph: contributors (creative issues), maintainer (board + skills), agents (manage + execute)
5. Contributions list (3 items from spec) — use `\begin{itemize}...\end{itemize}`
6. Paper organization paragraph

- [ ] **Step 2: Verify compiles**

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): write S1 Introduction"
```

---

### Task 8: Write S2 — Why Reductions?

**Files:**
- Modify: `docs/paper/arxiv/paper.tex`
**Depends on:** Task 2 (graph metrics), Task 3 (Figure 1)

- [ ] **Step 1: Write S2 body (~800 words)**

Read graph metrics from `docs/paper/arxiv/data/graph-metrics.json` for concrete numbers. If not yet available, use: 24 types, 42 variants, 52 edges, 40 implemented, 12 inferred.

Structure:
1. Goldilocks domain paragraph: self-contained (~50-200 LOC), formally specified, automatable round-trip criterion
2. Contrast with SWE-Bench: homogeneous tasks enable comparison
3. Hardware solvers paragraph: Rydberg atoms for MIS (cite `\cite{lucas2014}`), D-Wave for QUBO/Ising (cite `\cite{glover2019}`) → the graph as compilation layer
4. Real-world applications paragraph: SDN→ILP, airline→SetCovering, VLSI→coloring, logistics→TSP
5. Reference `Fig.~\ref{fig:reduction-graph}` (placed by Task 3)

- [ ] **Step 2: Verify compiles**

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): write S2 Why Reductions — Goldilocks domain"
```

---

### Task 9: Write S3 — System Architecture

**Files:**
- Modify: `docs/paper/arxiv/paper.tex`
**Depends on:** Task 6 (Figure 2)

- [ ] **Step 1: Write S3 body (~1200 words)**

Use the trait hierarchy from CLAUDE.md's Architecture section for reference. Do NOT read source files — CLAUDE.md has sufficient detail. Full trait code belongs in supplementary material.

Structure:
1. Problem trait: `evaluate()` enables brute-force verification of any configuration
2. ReduceTo trait: type system enforces round-trip capability by construction
3. `#[reduction(overhead)]` proc macro: compile-time validation of overhead expressions
4. `declare_variants!`: registry enables automated graph export + completeness checking
5. Design philosophy paragraph: reduce the space of possible agent errors
6. Reference `Fig.~\ref{fig:architecture}` (placed by Task 6)

Use `\texttt{}` for code identifiers and `\lstinline` for inline code snippets.

- [ ] **Step 2: Verify compiles**

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): write S3 System Architecture"
```

---

### Task 10: Write S4 — Skill-Based Task Decomposition

**Files:**
- Modify: `docs/paper/arxiv/paper.tex`
**Depends on:** Task 4 (Figure 3)

- [ ] **Step 1: Write S4.1 — Three Roles (~300 words)**

The roles table from the spec (Contributor/Maintainer/Agent). Use `\begin{table}...\end{table}` with `booktabs`.

- [ ] **Step 2: Read skill files and extract metadata**

Read all 13 skill files (`.claude/skills/*/SKILL.md`). For each, record: name, one-line description, invocation trigger, step count. This data populates Table 1.

- [ ] **Step 3: Write S4.2 — Skills as Agent Functions (~800 words)**

Group the 13 skills into 5 categories (from spec):
- **Orchestration** (4): project-pipeline, review-pipeline, issue-to-pr, meta-power
- **Implementation** (2): add-model, add-rule
- **Quality gate** (4): check-issue, check-rule-redundancy, review-implementation, fix-pr
- **Documentation** (2): write-model-in-paper, write-rule-in-paper
- **Release** (1): release

Create Table 1 with `booktabs`:
```latex
\begin{table}[t]
\caption{Skills inventory.}\label{tab:skills}
\centering
\begin{tabular}{llcc}
\toprule
Skill & Category & Steps & Success \\
\midrule
...
\bottomrule
\end{tabular}
\end{table}
```

Success Rate column: use "TBD" — filled after Task 11.

- [ ] **Step 4: Write S4.3 — Card-Based Orchestration (~500 words)**

Two-stage pipeline (project-pipeline → review-pipeline). Human touches only Backlog→Ready and In Review→Done. Reference `Fig.~\ref{fig:pipeline}`.

- [ ] **Step 5: Verify compiles**

- [ ] **Step 6: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex
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

- [ ] **Step 3: Add phase classification**

**Phase boundaries** (based on when key skills were introduced — determine by running):
```bash
git log --all --oneline --diff-filter=A -- '.claude/skills/add-rule/SKILL.md' | tail -1
git log --all --oneline --diff-filter=A -- '.claude/skills/project-pipeline/SKILL.md' | tail -1
```

Define phases:
- Phase 1 (manual): PRs before add-model/add-rule skills existed
- Phase 2 (basic skills): PRs after implementation skills but before pipeline skills
- Phase 3 (full pipeline): PRs after project-pipeline/review-pipeline skills existed

- [ ] **Step 4: Run script and save results**

```bash
python3 docs/paper/arxiv/scripts/mine-git-history.py > docs/paper/arxiv/data/git-mining-results.json
```

Expected output schema:
```json
{
  "summary": {"total_prs": N, "rule_prs": N, "model_prs": N, "agent_authored": N, "human_authored": N},
  "by_phase": [{"phase": 1, "label": "manual", "count": N, "agent_count": N}, ...],
  "prs": [{"number": 42, "title": "...", "is_agent": false, "phase": 1, "type": "Rule"}, ...]
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
- Modify: `docs/paper/arxiv/paper.tex`
**Depends on:** Task 5 (Figure 4)

- [ ] **Step 1: Write S5.1 — The Verification Stack (~700 words)**

Write the 7-layer table from the spec using `booktabs`. Use these concrete error examples:

| Layer | Mechanism | Example Error Caught |
|-------|-----------|---------------------|
| 1. Type system | Rust compiler | Agent returns `bool` instead of `SolutionSize<i32>` from `evaluate()` |
| 2. Unit tests | `test_*_basic` | Agent evaluates MaxCut objective with wrong sign |
| 3. Closed-loop tests | `test_*_to_*_closed_loop` | SAT→MIS maps clause variables to wrong vertex indices |
| 4. Overhead validation | Symbolic expr vs sizes | Agent writes `num_edges = num_clauses` instead of `3 * num_clauses` |
| 5. Materialized fixtures | JSON ground truth | Agent changes expected QUBO matrix to make failing test pass |
| 6. Agentic review | Parallel subagents | Missing `declare_variants!`, wrong file naming |
| 7. Documentation | Proof sketch | Proof assumes connected graph but problem allows disconnected |

Reference `Fig.~\ref{fig:verification}`.

- [ ] **Step 2: Write S5.2 — Why Layers? (~400 words)**

The "lazy agent" problem. Materialized test data as defense. No single layer is sufficient. Cross-reference Table 2 in S6.

- [ ] **Step 3: Verify compiles**

```bash
cd docs/paper/arxiv && pdflatex -interaction=nonstopmode paper.tex && cd -
```

- [ ] **Step 4: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): write S5 Multi-Layered Verification"
```

---

### Task 13: Write S6 — Evaluation

**Files:**
- Modify: `docs/paper/arxiv/paper.tex`
**Depends on:** Task 11 (git mining data)

- [ ] **Step 1: Write S6.1 — Ablation setup (~500 words)**

Experimental DESIGN only (results are `[TBD]` placeholders):
- Setup: 5-10 reductions, skill-based vs no-skill baseline
- Metrics: first-attempt CI pass rate, review rounds, correctness, convention adherence
- Framing: "controlled illustration" (n=5-10)

- [ ] **Step 2: Write S6.2 — Git History Mining results (~700 words)**

Read data from `docs/paper/arxiv/data/git-mining-results.json`. If not yet available, use `[TBD]` placeholders.

Create Table 2 (error taxonomy × verification layer):
```latex
\begin{table}[t]
\caption{Error taxonomy by verification layer.}\label{tab:errors}
\centering
\begin{tabular}{llc}
\toprule
Error Category & Layer & Count \\
\midrule
Type errors & 1 (type system) & [TBD] \\
Mapping errors & 3 (closed-loop) & [TBD] \\
...
\bottomrule
\end{tabular}
\end{table}
```

- [ ] **Step 3: Write S6.3 — Case Studies (~800 words)**

Search for actual PRs:
```bash
gh pr list --repo CodingThrust/ProblemReductions --state merged --limit 999 --search "MinimumVertexCover MaximumIndependentSet" --json number,title
gh pr list --repo CodingThrust/ProblemReductions --state merged --limit 999 --search "Satisfiability MaximumIndependentSet" --json number,title
gh pr list --repo CodingThrust/ProblemReductions --state merged --limit 999 --search "Factoring CircuitSAT" --json number,title
```

**Case 1 — Simple (MVC→MIS):** complement relationship, ~30 LOC.
**Case 2 — Complex (SAT→MIS):** clause-variable gadget, quadratic blowup.
**Case 3 — Composition (Factoring→CircuitSAT→ILP):** two independent reductions composing in graph.

- [ ] **Step 4: Verify compiles**

- [ ] **Step 5: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): write S6 Evaluation"
```

---

## Chunk 5: Sections S7-S8 + Review + Final Assembly

### Task 14: Write S7 — Related Work

**Files:**
- Modify: `docs/paper/arxiv/paper.tex`

- [ ] **Step 1: Write S7 body (~800 words)**

Four subsections with specific citation keys:

1. **AI coding agents:** `\cite{Yang2024SWEagent}`, `\cite{Wang2024OpenHands}`, `\cite{Anthropic2025ClaudeCode}`, `\cite{Wu2024Devin}`, `\cite{Thai2025SWEEVO}`, `\cite{Deng2025SWEBenchPro}`, `\cite{Xia2025LiveSWEagent}`, `\cite{Roychoudhury2025AgenticAI}`, `\cite{Anthropic2026AgenticCoding}`

2. **AI-discovered reductions:** `\cite{Novikov2025AlphaEvolve}`, `\cite{Janicic2025URSA}`, `\cite{RomeraParedes2023FunSearch}`

3. **Formal verification:** `\cite{Bursuc2025VeriCoding}`, `\cite{Thakur2025CLEVER}`, `\cite{Miranda2025VeriBench}`, `\cite{Mukherjee2025CoqPL}`, `\cite{Mukherjee2025SynVer}`

4. **Physics-inspired optimization:** `\cite{Schuetz2022PhysicsGNN}`, `\cite{He2024QuantumTSP}`

Position our work as complementary, not competing.

- [ ] **Step 2: Verify compiles**

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): write S7 Related Work"
```

---

### Task 15: Write S8 — Discussion & Conclusion

**Files:**
- Modify: `docs/paper/arxiv/paper.tex`

- [ ] **Step 1: Write S8 body (~800 words)**

Four parts:
1. **Generalizability:** Goldilocks property, candidate domains
2. **Limitations:** n=1, skill engineering cost, domain specificity, confounds, maintainer requirement
3. **Human value proposition:** repositioned not eliminated. Cite `\cite{Anthropic2026AgenticCoding}`.
4. **Future directions:** AlphaEvolve, formal verification, scaling to 100+

End with `\subsection{Conclusion}`: 2-3 crisp sentences.

- [ ] **Step 2: Verify compiles**

- [ ] **Step 3: Commit**

```bash
git add -f docs/paper/arxiv/paper.tex
git commit -m "docs(arxiv): write S8 Discussion and Conclusion"
```

---

### Task 16: Simulated peer review

**Files:** None modified (review only)

- [ ] **Step 1: Run academic-paper-reviewer**

Read `.claude/skills/academic-research-skills/academic-paper-reviewer/SKILL.md` and invoke the review process on `docs/paper/arxiv/paper.tex`. This simulates a 5-person review panel (Editor-in-Chief + 3 domain reviewers + Devil's Advocate) with quality rubrics.

- [ ] **Step 2: Record review findings**

Save the review output to `docs/paper/arxiv/data/peer-review-round1.md`.

- [ ] **Step 3: Address critical review findings**

Fix any issues scored below 65 (Major Revision threshold). Update paper.tex accordingly.

- [ ] **Step 4: Commit fixes**

```bash
git add -f docs/paper/arxiv/paper.tex docs/paper/arxiv/data/peer-review-round1.md
git commit -m "docs(arxiv): address peer review round 1 findings"
```

---

### Task 17: Final assembly and polish

**Files:**
- Modify: `docs/paper/arxiv/paper.tex`

- [ ] **Step 1: Compile all Typst figures**

```bash
for f in docs/paper/arxiv/figures/*.typ; do typst compile "$f"; done
```

Verify all 4 PDFs exist:
```bash
ls docs/paper/arxiv/figures/*.pdf
```

Expected: `reduction-graph.pdf`, `architecture.pdf`, `pipeline.pdf`, `verification-pyramid.pdf`.

- [ ] **Step 2: Verify all figures are placed correctly**

Check that these references exist in the paper text:
- `\ref{fig:reduction-graph}` in S2
- `\ref{fig:architecture}` in S3
- `\ref{fig:pipeline}` in S4
- `\ref{fig:verification}` in S5

- [ ] **Step 3: Verify all tables are placed**

Check for `\ref{tab:skills}` in S4 and `\ref{tab:errors}` in S6.

- [ ] **Step 4: Full compile with bibliography**

```bash
cd docs/paper/arxiv && pdflatex paper.tex && bibtex paper && pdflatex paper.tex && pdflatex paper.tex && cd -
```

Check for unresolved citations: `grep "Citation.*undefined" docs/paper/arxiv/paper.log`
Expected: no undefined citations.

- [ ] **Step 5: Check page count**

```bash
pdfinfo docs/paper/arxiv/paper.pdf | grep Pages
```

Expected: 10-12 pages. If over, trim. If under, expand.

- [ ] **Step 6: Flag visual review for maintainer**

Verify no LaTeX warnings about overfull hboxes (>1pt). Visual inspection (layout, figures, tables) requires human review.

- [ ] **Step 7: Commit final version**

```bash
git add -f docs/paper/arxiv/paper.tex docs/paper/arxiv/figures/*.typ docs/paper/arxiv/references.bib
git commit -m "docs(arxiv): final paper assembly and polish"
```

Note: Do NOT commit `paper.pdf`, `paper.aux`, `paper.bbl`, `paper.blg`, `paper.log`, or `figures/*.pdf` — these are build artifacts. Add them to `.gitignore` if needed.

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
Task 16 (peer review): must run after Tasks 7-15
Task 17 (assembly): must run LAST
```

### Suggested Parallel Batches

1. **Tasks 1-2** (scaffolding + data) — sequential, run first
2. **Tasks 3-6** (all figures) + **Task 7** (S1) + **Task 11** (git mining) — parallel
3. **Tasks 8-10** (S2-S4) + **Tasks 14-15** (S7-S8) — parallel
4. **Tasks 12-13** (S5-S6) — parallel
5. **Task 16** (peer review) — after all sections written
6. **Task 17** (assembly) — last

### Open Dependencies

- **S6.1 ablation results** are `[TBD]` placeholders. The ablation experiment is a separate effort outside this plan.
- **Table 1 success rates** are `[TBD]` — filled from git mining data if available.
- **Peer review** (Task 16) may surface issues requiring additional revision cycles.
