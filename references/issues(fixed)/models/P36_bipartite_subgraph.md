---
name: Problem
about: Propose a new problem type
title: "[Model] BipartiteSubgraph"
labels: model
assignees: ''
---

## Motivation

BIPARTITE SUBGRAPH (P36) from Garey & Johnson, A1.2 GT25. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT25

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that G' = (V,E') is bipartite?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that G' = (V,E') is bipartite?
Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from MAXIMUM 2-SATISFIABILITY.
Comment: Remains NP-complete for graphs with no vertex degree exceeding 3 and no triangles and/or if we require that the subgraph be connected [Yannakakis, 1978b]. Solvable in polynomial time if G is planar [Hadlock, 1975], [Orlova and Dorfman, 1972], or if K = |E|.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
