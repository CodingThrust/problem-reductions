---
name: Problem
about: Propose a new problem type
title: "[Model] MetricDimension"
labels: model
assignees: ''
---

## Motivation

METRIC DIMENSION (P72) from Garey & Johnson, A1.5 GT61. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT61

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a metric basis for G of cardinality K or less, i.e., a subset V' ⊆ V with |V'| ≤ K such that for each pair u,v ∈ V there is a w ∈ V' such that the length of the shortest path from u to w is different from the length of the shortest path from v to w?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a metric basis for G of cardinality K or less, i.e., a subset V' ⊆ V with |V'| ≤ K such that for each pair u,v ∈ V there is a w ∈ V' such that the length of the shortest path from u to w is different from the length of the shortest path from v to w?

Reference: [Garey and Johnson, ——]. Transformation from 3DM. The definition of metric dimension appears in [Harary and Melter, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
