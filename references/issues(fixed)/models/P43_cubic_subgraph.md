---
name: Problem
about: Propose a new problem type
title: "[Model] CubicSubgraph"
labels: model
assignees: ''
---

## Motivation

CUBIC SUBGRAPH (P43) from Garey & Johnson, A1.2 GT32. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT32

**Mathematical definition:**

INSTANCE: Graph G = (V,E).
QUESTION: Is there a nonempty subset E' ⊆ E such that in the graph G' = (V,E') every vertex has either degree 3 or degree 0?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: Graph G = (V,E).
QUESTION: Is there a nonempty subset E' ⊆ E such that in the graph G' = (V,E') every vertex has either degree 3 or degree 0?
Reference: [Chvátal, 1976]. Transformation from GRAPH 3-COLORABILITY.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
