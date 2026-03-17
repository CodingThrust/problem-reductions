---
name: Problem
about: Propose a new problem type
title: "[Model] TransitiveSubgraph"
labels: model
assignees: ''
---

## Motivation

TRANSITIVE SUBGRAPH (P40) from Garey & Johnson, A1.2 GT29. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT29

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≥ K such that G' = (V,A') is transitive, i.e., for all pairs u,v ∈ V, if there exists a w ∈ V for which (u,w),(w,v) ∈ A', then (u,v) ∈ A'?

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

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≥ K such that G' = (V,A') is transitive, i.e., for all pairs u,v ∈ V, if there exists a w ∈ V for which (u,w),(w,v) ∈ A', then (u,v) ∈ A'?
Reference: [Yannakakis, 1978b] Transformation from BIPARTITE SUBGRAPH with no triangles.
Comment: The variant in which G is undirected and we ask for a subgraph that is a "comparability graph," i.e., can be made into a transitive digraph by directing each of its edges in one of the two possible directions, is also NP-complete, even if G has no vertex with degree exceeding 3. For both problems, the variant in which we require the subgraph to be connected is also NP-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
