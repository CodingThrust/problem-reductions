---
paths:
  - "docs/paper/**/*.typ"
---

# Documentation Requirements

The technical paper (`docs/paper/reductions.typ`) must include:

1. **Problem Definitions** — using `problem-def` wrapper
2. **Reduction Theorems** — using `reduction-rule` function
3. **Reduction Examples** — JSON data from `make examples`, rendered automatically

## Adding a Problem Definition

```typst
#problem-def("MaximumIndependentSet")[
  Given $G = (V, E)$ with vertex weights $w: V -> RR$, find ...
]
```

This auto-generates:
- A label `<def:MaximumIndependentSet>` for cross-references
- The problem's schema (fields from JSON export)
- The list of available reductions (from `reduction_graph.json` edges)

Also add an entry to the `display-name` dictionary:
```typst
"MaximumIndependentSet": [Maximum Independent Set],
```

## Adding a Reduction Theorem

```typst
#reduction-rule("MaximumIndependentSet", "QUBO",
  example: true,
  example-caption: [IS on path $P_4$ to QUBO],
)[
  Rule statement...
][
  Proof sketch...
]
```

Parameters:
- `source`, `target` — problem names (positional)
- `example: bool` — if `true`, loads `examples/<source>_to_<target>.json` and `.result.json`
- `example-caption: content` — caption for the example box
- `extra: content` — additional content inside the example box
- `theorem-body`, `proof-body` — the rule statement and proof (positional)

This auto-generates:
- A theorem label `<thm:MaximumIndependentSet-to-QUBO>`
- References to source/target problem definitions (if they exist)
- Registration in `covered-rules` state for completeness checking
- Overhead from `reduction_graph.json` edge data

Every directed reduction in the graph needs its own `reduction-rule` entry.

## Completeness Warnings

The paper auto-checks completeness:
- After Problem Definitions: warns if JSON graph nodes are missing from `display-name`
- After Reductions section: warns if JSON graph edges are missing from `covered-rules`
