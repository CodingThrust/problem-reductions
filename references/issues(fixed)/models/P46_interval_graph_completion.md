---
name: Problem
about: Propose a new problem type
title: "[Model] IntervalGraphCompletion"
labels: model
assignees: ''
---

## Motivation

INTERVAL GRAPH COMPLETION (P46) from Garey & Johnson, A1.2 GT35. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT35

**Mathematical definition:**

INSTANCE: Graph G = (V,E), non-negative integer K.
QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') is an interval graph?

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

INSTANCE: Graph G = (V,E), non-negative integer K.
QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') is an interval graph?
Reference: [Garey, Gavril, and Johnson, 1977]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
Comment: Remains NP-complete when G is restricted to be an edge graph. Solvable in polynomial time for K = 0 [Fulkerson and Gross, 1965],[Booth and Lueker, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
