---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoCliques"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO CLIQUES (P26) from Garey & Johnson, A1.1 GT15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT15

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Can the vertices of G be partitioned into k ≤ K disjoint sets V_1, V_2, . . . , V_k such that, for 1 ≤ i ≤ k, the subgraph induced by V_i is a complete graph?

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
QUESTION: Can the vertices of G be partitioned into k ≤ K disjoint sets V_1, V_2, . . . , V_k such that, for 1 ≤ i ≤ k, the subgraph induced by V_i is a complete graph?
Reference: [Karp, 1972] (there called CLIQUE COVER). Transformation from GRAPH K-COLORABILITY.
Comment: Remains NP-complete for edge graphs [Arjomandi, 1977], for graphs containing no complete subgraphs on 4 vertices (see construction for PARTITION INTO TRIANGLES in Chapter 3), and for all fixed K ≥ 3. Solvable in polynomial time for K ≤ 2, for graphs containing no complete subgraphs on 3 vertices (by matching), for circular arc graphs (given their representation as families of arcs) [Gavril, 1974a], for chordal graphs [Gavril, 1972], for comparability graphs [Golumbic, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
