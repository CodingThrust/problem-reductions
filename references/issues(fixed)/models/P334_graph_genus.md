---
name: Problem
about: Propose a new problem type
title: "[Model] GraphGenus"
labels: model
assignees: ''
---

## Motivation

GRAPH GENUS (P334) from Garey & Johnson, A13 OPEN3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN3

**Mathematical definition:**

INSTANCE: Graph G = (V, E) and a non-negative integer K.
QUESTION: Can G be embedded on a surface of genus K such that no two edges cross one another?

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

INSTANCE: Graph G = (V, E) and a non-negative integer K.
QUESTION: Can G be embedded on a surface of genus K such that no two edges cross one another?

Comment: Solvable in polynomial time for K = 0, i.e., if the question is whether G is planar (e.g., see [Hopcroft and Tarjan, 1974]). A polynomial time algorithm for K = 1 and cubic graphs is announced in [Filotti, 1978], and, in [Reif, 1978b], polynomial time algorithms for arbitrary graphs and any fixed value of K are presented. In addition, for some restricted classes of graphs, such as cliques, cubes, and complete bipartite graphs, simple closed formulas for the genus have been derived (e.g., see [Harary, 1969]). Although the problem for general G and K is open, the closely related GENUS EXTENSION problem (given G, K, and an embedding of a subgraph of G into a surface of genus K, can the embedding be extended to one for all of G?) is NP-complete [Reif, 1978a]. Open problems for other generalizations of planarity include "Does G have crossing number K or less, i.e., can G be embedded in the plane with K or fewer pairs of edges crossing one another?" and "Does G have thickness K or less, i.e., can E be partitioned into K disjoint sets E1, E2, ..., Ek such that each subgraph Gi = (V, Ei) is planar?" Related NP-complete problems include PLANAR SUBGRAPH and PLANAR INDUCED SUBGRAPH (see INDUCED SUBGRAPH WITH PROPERTY Π).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
