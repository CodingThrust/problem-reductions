---
name: Problem
about: Propose a new problem type
title: "[Model] DegreeBoundedConnectedSubgraph"
labels: model
assignees: ''
---

## Motivation

DEGREE-BOUNDED CONNECTED SUBGRAPH (P37) from Garey & Johnson, A1.2 GT26. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT26

**Mathematical definition:**

INSTANCE: Graph G = (V,E), non-negative integer d ≤ |V|, positive integer K ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that the subgraph G' = (V,E') is connected and has no vertex with degree exceeding d?

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

INSTANCE: Graph G = (V,E), non-negative integer d ≤ |V|, positive integer K ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that the subgraph G' = (V,E') is connected and has no vertex with degree exceeding d?
Reference: [Yannakakis, 1978b]. Transformation from HAMILTONIAN PATH.
Comment: Remains NP-complete for any fixed d ≥ 2. Solvable in polynomial time if G' is not required to be connected (by matching techniques, see [Edmonds and Johnson, 1970]). The corresponding induced subgraph problem, where we ask for a subset V' ⊆ V with |V'| ≥ K such that the subgraph of G induced by V' has no vertex with degree exceeding d, is NP-complete for any fixed d ≥ 0 [Lewis, 1976] and for any fixed d ≥ 2 if we require that G' be connected [Yannakakis, 1978b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
