---
name: Problem
about: Propose a new problem type
title: "[Model] DominatingSet"
labels: model
assignees: ''
---

## Motivation

DOMINATING SET (P13) from Garey & Johnson, A1.1 GT2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT2

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a dominating set of size K or less for G, i.e., a subset V' ⊆ V with |V'| ≤ K such that for all u ∈ V−V' there is a v ∈ V' for which {u,v} ∈ E?

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
QUESTION: Is there a dominating set of size K or less for G, i.e., a subset V' ⊆ V with |V'| ≤ K such that for all u ∈ V−V' there is a v ∈ V' for which {u,v} ∈ E?
Reference: Transformation from VERTEX COVER.
Comment: Remains NP-complete for planar graphs with maximum vertex degree 3 and planar graphs that are regular of degree 4 [Garey and Johnson, ——]. Variation in which the subgraph induced by V' is required to be connected is also NP-complete, even for planar graphs that are regular of degree 4 [Garey and Johnson, ——]. Also NP-complete if V' is required to be both a dominating set and an independent set. Solvable in polynomial time for trees [Cockayne, Goodman, and Hedetniemi, 1975]. The related EDGE DOMINATING SET problem, where we ask for a set E' ⊆ E of K or fewer edges such that every edge in E shares at least one endpoint with some edge in E', is NP-complete, even for planar or bipartite graphs of maximum degree 3, but can be solved in polynomial time for trees [Yannakakis and Gavril, 1978], [Mitchell and Hedetniemi, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
