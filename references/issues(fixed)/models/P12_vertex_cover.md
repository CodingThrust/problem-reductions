---
name: Problem
about: Propose a new problem type
title: "[Model] VertexCover"
labels: model
assignees: ''
---

## Motivation

VERTEX COVER (P12) from Garey & Johnson, A1.1 GT1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT1

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a vertex cover of size K or less for G, i.e., a subset V' ⊆ V with |V'| ≤ K such that for each edge {u,v} ∈ E at least one of u and v belongs to V'?

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
QUESTION: Is there a vertex cover of size K or less for G, i.e., a subset V' ⊆ V with |V'| ≤ K such that for each edge {u,v} ∈ E at least one of u and v belongs to V'?
Reference: [Karp, 1972]. Transformation from 3SAT (see Chapter 3).
Comment: Equivalent complexity to INDEPENDENT SET with respect to restrictions on G. Variation in which the subgraph induced by V' is required to be connected is also NP-complete, even for planar graphs with no vertex degree exceeding 4 [Garey and Johnson, 1977a]. Easily solved in polynomial time if V' is required to be both a vertex cover and an independent set for G. The related EDGE COVER problem, in which one wants the smallest set E' ⊆ E such that every v ∈ V belongs to at least one e ∈ E', can be solved in polynomial time by graph matching (e.g., see [Lawler, 1976a]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
