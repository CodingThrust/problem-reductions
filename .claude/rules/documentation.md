---
paths:
  - "docs/paper/**/*.typ"
---

# Documentation Requirements

**Reference:** search `docs/paper/reductions.typ` for `MinimumVertexCover` `MaximumIndependentSet` to see a complete problem-def + reduction-rule example.

## Adding a Problem Definition

```typst
#problem-def("ProblemName")[
  Mathematical definition...
]
```

Also add to the `display-name` dictionary:
```typst
"ProblemName": [Problem Name],
```

## Adding a Reduction Theorem

```typst
#reduction-rule("Source", "Target",
  example: true,
  example-caption: [caption text],
)[
  Rule statement...
][
  Proof sketch...
]
```

Every directed reduction in the graph needs its own `reduction-rule` entry. The paper auto-checks completeness against `reduction_graph.json`.
