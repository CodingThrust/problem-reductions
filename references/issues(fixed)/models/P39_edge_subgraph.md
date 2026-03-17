---
name: Problem
about: Propose a new problem type
title: "[Model] EdgeSubgraph"
labels: model
assignees: ''
---

## Motivation

EDGE-SUBGRAPH (P39) from Garey & Johnson, A1.2 GT28. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT28

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that the subgraph G' = (V,E') is an edge graph, i.e., there exists a graph H = (U,F) such that G' is isomorphic to the graph having vertex set F and edge set consisting of all pairs {e,f} such that the edges e and f share a common endpoint in H?

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
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that the subgraph G' = (V,E') is an edge graph, i.e., there exists a graph H = (U,F) such that G' is isomorphic to the graph having vertex set F and edge set consisting of all pairs {e,f} such that the edges e and f share a common endpoint in H?
Reference: [Yannakakis, 1978b]. Transformation from 3SAT.
Comment: Remains NP-complete even if G has no vertex with degree exceeding 4. If we require that the subgraph be connected, the degree bound for NP-completeness can be reduced to 3. Edge graphs can be recognized in polynomial time, e.g., see [Harary, 1969] (under the term "line graphs").

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
