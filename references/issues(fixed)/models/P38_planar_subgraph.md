---
name: Problem
about: Propose a new problem type
title: "[Model] PlanarSubgraph"
labels: model
assignees: ''
---

## Motivation

PLANAR SUBGRAPH (P38) from Garey & Johnson, A1.2 GT27. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT27

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that G' = (V,E') is planar?

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
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that G' = (V,E') is planar?
Reference: [Liu and Geldmacher, 1978]. Transformation from HAMILTONIAN PATH restricted to bipartite graphs.
Comment: Corresponding problem in which G' is the subgraph induced by a set V' of at least K vertices is also NP-complete [Krishnamoorthy and Deo, 1977a], [Yannakakis, 1978b]. The former can be solved in polynomial time when K = |E|, and the latter when K = |V|, since planarity testing can be done in polynomial time (e.g., see [Hopcroft and Tarjan, 1974]). The related problem in which we ask if G contains a connected "outerplanar" subgraph with K or more edges is also NP-complete [Yannakakis, 1978b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
