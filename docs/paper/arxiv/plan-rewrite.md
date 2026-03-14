# Paper Rewrite Plan

**Spec:** `docs/paper/arxiv/paper-redesign-spec.md`
**Target:** `docs/paper/arxiv/paper.tex`

## Steps

### Step 1: Rewrite Abstract
Reframe around bridge problem concept. New arc: bridge problems exist (too large for humans) → agents can build them (verification constrains correctness) → NP-hard reductions as first example → evidence (27 types, 45 rules, 9 weeks vs 4 years).

### Step 2: Rewrite Section 1 (Introduction)
- Keep familiar opening (airlines, chips, logistics)
- Introduce reduction graph idea
- **New:** Introduce "bridge problem" claim — this software is too large for humans
- Preview 3 barriers
- Mention verification solves the correctness concern
- Reference Fig 1 (Scaling Wall — to be created later)
- Contributions list

### Step 3: Write Section 2 (Bridge Problems) — NEW
- Formal definition of bridge problems
- Three barriers with evidence:
  - Convention drift
  - Effort exhaustion (merged with testing frequency)
  - Knowledge discontinuity
- Verification constrains agent output (funnel concept)
- Reference Fig 2 (Verification Funnel — to be created later)
- Other candidate domains

### Step 4: Rewrite Section 3 (Case Study: Reduction Graph)
- Move existing graph description content here
- Keep: what is a reduction, graph structure, emergent compositionality
- **New:** Frame as case study illustrating bridge problem concept
- Reference Fig 3 (Reduction Graph — to be redesigned with solver-reachability coloring)
- Note: figure redesign is a separate step

### Step 5: Rewrite Section 4 (Methodology)
- Largely keep existing methodology content
- Reframe skills as "how agents break through the 3 barriers"
- Keep pipeline figure (Fig 4)
- Keep verification stack description

### Step 6: Rewrite Section 5 (Evidence)
- Reference Fig 5 (Development Timeline — to be created later)
- Development metrics
- Quality gate analysis
- **New:** Barrier-by-barrier evidence structure
- Julia predecessor comparison

### Step 7: Rewrite Section 6 (Discussion)
- Keep limitations
- Keep "why human experts remain essential" (shortened)
- Move "Scale Beyond Human Capacity" content — already covered in Sec 2
- Move "Barrier-Free Community Contribution" — fold into Sec 2
- Future work

### Step 8: Clean up appendices
- Move topology issues figure to appendix
- Keep architecture + verification pyramid in appendix
- Remove three roles figure reference

### Step 9: Create Fig 1 (Scaling Wall)
New Typst/CeTZ figure in `figures/scaling-wall.typ`

### Step 10: Create Fig 2 (Verification Funnel)
New Typst/CeTZ figure in `figures/verification-funnel.typ`

### Step 11: Redesign Fig 3 (Reduction Graph with solver coloring)
Modify `figures/reduction-graph.typ` — add color-coded edges by solver reachability

### Step 12: Create Fig 5 (Development Timeline)
New Typst/CeTZ-plot or Python-generated figure in `figures/timeline.typ`

### Step 13: Rewrite abstract (final pass)
After all sections are stable, do a final pass on the abstract to ensure it matches.

## Dependencies
- Steps 1-8 (text) can proceed before figures
- Steps 9-12 (figures) are independent of each other
- Step 13 depends on all prior steps

## Notes
- Keep total under 12 pages (conference format)
- Use existing writing-guidelines.md principles
- Fix reviewer issues: timeline consistency, author count, LLM model identification
