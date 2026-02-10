---
paths:
  - "docs/paper/**/*.typ"
---

# Documentation Requirements

The technical paper (`docs/paper/reductions.typ`) must include:

1. **Problem Definitions** — using `problem-def` wrapper
2. **Reduction Theorems** — using `reduction-rule` function
3. **Reduction Examples** — minimal working example showing reduce → solve → extract

## Adding a Problem Definition

```typst
#problem-def("MaximumIndependentSet", "Maximum Independent Set (MIS)")[
  Mathematical definition...
]
```

This auto-generates:
- A label `<def:MaximumIndependentSet>` for cross-references
- The problem's schema (fields from Rust struct)
- The list of available reductions

Also add an entry to the `display-name` dictionary:
```typst
"MaximumIndependentSet": "MIS",
```

## Adding a Reduction Theorem

```typst
#reduction-rule(
  "MaximumIndependentSet", "QUBO",
  example: "maximumindependentset_to_qubo",
  overhead: (n: 0, m: 1),
)[
  Proof sketch...
]
```

This auto-generates:
- A theorem label `<thm:MaximumIndependentSet-to-QUBO>`
- References to source/target problem definitions (if they exist)
- Registration in `covered-rules` state for completeness checking
- The example code block from `examples/reduction_<example>.rs`

## Completeness Warnings

The paper auto-checks completeness:
- After Problem Definitions: warns if JSON graph nodes are missing from `display-name`
- After Reductions section: warns if JSON graph edges are missing from `covered-rules`
