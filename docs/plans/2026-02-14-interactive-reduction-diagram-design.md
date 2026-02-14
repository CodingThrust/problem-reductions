# Interactive Reduction Diagram Design

**Date:** 2026-02-14
**Location:** mdBook documentation site (docs/src/)
**Stack:** Cytoscape.js + ELK.js (CDN-loaded)

## Goal

An interactive network diagram showing all reduction relationships between NP-hard problems. Users can explore the problem landscape, discover reduction paths, and navigate to documentation pages — all from a single visual interface.

## Architecture

### Data Source

The existing `docs/src/reductions/reduction_graph.json` (auto-generated from `#[reduction]` macros via `cargo run --example export_graph`) provides nodes (39 problem variants) and edges (44 reductions + natural casts).

### Files

```
docs/src/reduction-graph.md          — mdBook page with embedded container div
docs/src/static/reduction-graph.js   — main diagram logic (preprocessing, layout, interactions)
docs/src/static/reduction-graph.css  — diagram styles (node colors, tooltips, controls)
book.toml                            — updated: additional-js/css entries
docs/src/SUMMARY.md                  — updated: new page under User Guide
```

### Libraries (CDN)

- `cytoscape.js` (~112 KB gzipped) — graph rendering + interaction
- `elkjs` (~435 KB gzipped) — stress layout algorithm
- `cytoscape-elk` (~2 KB) — adapter

## Node Design

### Collapsed Node (default)

Every unique problem name renders as one collapsed node. This is the initial view (~20 nodes).

```
┌──────────────────────┐
│  Max Independent Set │
│  ●●●●●               │
└──────────────────────┘
```

- Rounded rectangle, colored by category:
  - graph → blue
  - set → green
  - optimization → orange
  - satisfiability → purple
  - specialized → gray
- Small dots along the bottom preview variant count
- Hover tooltip lists variant names

### Expanded Node (after click)

```
┌─ Maximum Independent Set ────┐
│                              │
│  ● base (default)            │
│  ● SimpleGraph, i32      ●──│──→ edges anchor here
│  ● GridGraph, i32            │
│  ● Triangular, i32           │
│  ● UnitDiskGraph, i32        │
│                              │
└──────────────────────────────┘
```

- Parent container expands; child nodes (variant pills) appear inside
- Each variant is a small labeled pill with a port dot on the side where edges connect
- Natural cast edges (e.g., Triangular → SimpleGraph) shown as dashed arrows inside the container
- Click a variant dot → filter: only that variant's edges are shown; all other edges fade to 10% opacity
- Click parent header → collapse back

### Single-Variant Nodes

Problems with only one variant (e.g., Satisfiability, CircuitSAT, Factoring) render as simple nodes — no expand/collapse, no variant dots.

## Edge Design

| Type | Style | Example |
|------|-------|---------|
| Reduction | Solid arrow, dark stroke | SAT → MIS |
| Natural cast | Dashed arrow, light gray | Triangular → SimpleGraph (internal) |

### Collapsed Mode

Multiple variant-level edges between the same two problems collapse to a single edge with a count label (e.g., "×3").

### Expanded Mode

When a node is expanded, its edges "split" — each edge anchors to the specific variant dot that is its actual source/target in the JSON.

### Hover Tooltip

Shows overhead formula: `num_vars = num_vertices, num_constraints = num_edges`

## Layout

**Algorithm:** ELK.js stress layout (`elk.algorithm: 'stress'`)

- Stress minimization positions nodes so geometric distances approximate graph-theoretic distances
- Produces natural, balanced layouts without strict hierarchical layering
- Compound nodes (expanded parents) handled by ELK's hierarchical container support
- Re-layout with smooth animation on expand/collapse
- Users can drag nodes to adjust positions after layout settles

## Interactions

| Action | Result |
|--------|--------|
| **Click node** | Expand/collapse to show/hide variant dots |
| **Click variant dot** | Filter edges: only that variant's connections shown, others fade |
| **Click background** | Reset all filters, collapse all expanded nodes |
| **Double-click node** | Open problem doc page (via `doc_path` field in JSON) |
| **Double-click edge** | Open reduction doc page (via `doc_path` field in JSON) |
| **Hover node** | Tooltip: variant list + reduction count |
| **Hover edge** | Tooltip: overhead formula |
| **Path finder** | Two dropdowns: "From" → "To". Highlights shortest reduction path. Shows path cost. |
| **Search bar** | Type to filter — matching nodes highlighted, non-matching fade |
| **Zoom/pan** | Scroll to zoom, drag to pan |

## Data Preprocessing (client-side)

1. Load `reduction_graph.json`
2. Group nodes by `name` → create compound parent nodes
3. For groups with >1 variant → create child nodes inside parent
4. For single-variant groups → create simple (non-compound) nodes
5. Map edges to connect to child nodes (variant-level targets)
6. Compute collapsed-mode edge summary (merge parallel edges between same parents)
7. Build adjacency list for client-side path finding (BFS/Dijkstra)

## Path Finder

Two dropdown menus at the top of the page:
- "From: [problem name]" — lists all problem names
- "To: [problem name]" — lists all problem names

Clicking "Find Path" runs client-side Dijkstra on the name-level graph. The result:
- Highlights path edges in a distinct color (e.g., gold)
- Fades non-path nodes/edges
- Shows path summary below: `SAT → 3-SAT → MIS → QUBO` with total overhead

## Category Color Scheme

| Category | Light Mode | Dark Mode |
|----------|-----------|-----------|
| graph | `#3b82f6` (blue) | `#60a5fa` |
| set | `#22c55e` (green) | `#4ade80` |
| optimization | `#f97316` (orange) | `#fb923c` |
| satisfiability | `#a855f7` (purple) | `#c084fc` |
| specialized | `#6b7280` (gray) | `#9ca3af` |

Dark/light mode follows mdBook's theme toggle.

## Non-Goals

- Server-side rendering — everything runs client-side
- Editing the graph — read-only visualization
- Animated edge flow — static arrows are sufficient
- Mobile-optimized layout — desktop-first, basic mobile support via zoom/pan
