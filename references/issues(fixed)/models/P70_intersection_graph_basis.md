---
name: Problem
about: Propose a new problem type
title: "[Model] IntersectionGraphBasis"
labels: model
assignees: ''
---

## Motivation

INTERSECTION GRAPH BASIS (P70) from Garey & Johnson, A1.5 GT59. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT59

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is G the intersection graph for a family of sets whose union has cardinality K or less, i.e., is there a K-element set S and for each v ∈ V a subset S[v] ⊆ S such that {u,v} ∈ E if and only if S[u] and S[v] are not disjoint?

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
QUESTION: Is G the intersection graph for a family of sets whose union has cardinality K or less, i.e., is there a K-element set S and for each v ∈ V a subset S[v] ⊆ S such that {u,v} ∈ E if and only if S[u] and S[v] are not disjoint?

Reference: [Kou, Stockmeyer, and Wong, 1978]. Transformation from COVERING BY CLIQUES.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
