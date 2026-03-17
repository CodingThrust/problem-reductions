---
name: Problem
about: Propose a new problem type
title: "[Model] BalancedCompleteBipartiteSubgraph"
labels: model
assignees: ''
---

## Motivation

BALANCED COMPLETE BIPARTITE SUBGRAPH (P35) from Garey & Johnson, A1.2 GT24. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT24

**Mathematical definition:**

INSTANCE: Bipartite graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Are there two disjoint subsets V1, V2 ⊆ V such that |V1| = |V2| = K and such that u ∈ V1, v ∈ V2 implies that {u,v} ∈ E?

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

INSTANCE: Bipartite graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Are there two disjoint subsets V1, V2 ⊆ V such that |V1| = |V2| = K and such that u ∈ V1, v ∈ V2 implies that {u,v} ∈ E?
Reference: [Garey and Johnson, ——]. Transformation from CLIQUE.
Comment: The related problem in which the requirement "|V1| = |V2| = K" is replaced by "|V1|+|V2| = K" is solvable in polynomial time for bipartite graphs (because of the connection between matchings and independent sets in such graphs, e.g., see [Harary, 1969]), but is NP-complete for general graphs [Yannakakis, 1978b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
