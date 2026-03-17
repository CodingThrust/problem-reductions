---
name: Problem
about: Propose a new problem type
title: "[Model] OrientedDiameter"
labels: model
assignees: ''
---

## Motivation

ORIENTED DIAMETER (P75) from Garey & Johnson, A1.5 GT64. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT64

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Can the edges of G be directed in such a way that the resulting directed graph is strongly connected and has diameter no more than K?

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
QUESTION: Can the edges of G be directed in such a way that the resulting directed graph is strongly connected and has diameter no more than K?

Reference: [Chvátal and Thomassen, 1978]. Transformation from SET SPLITTING.
Comment: The variation in which "diameter" is replaced by "radius" is also NP-complete. Both problems remain NP-complete for K = 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
