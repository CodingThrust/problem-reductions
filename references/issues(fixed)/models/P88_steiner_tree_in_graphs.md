---
name: Problem
about: Propose a new problem type
title: "[Model] SteinerTreeInGraphs"
labels: model
assignees: ''
---

## Motivation

STEINER TREE IN GRAPHS (P88) from Garey & Johnson, A2 ND12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND12

**Mathematical definition:**

INSTANCE: Graph G = (V,E), a weight w(e) ∈ Z0+ for each e ∈ E, a subset R ⊆ V, and a positive integer bound B.
QUESTION: Is there a subtree of G that includes all the vertices of R and such that the sum of the weights of the edges in the subtree is no more than B?

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

INSTANCE: Graph G = (V,E), a weight w(e) ∈ Z0+ for each e ∈ E, a subset R ⊆ V, and a positive integer bound B.
QUESTION: Is there a subtree of G that includes all the vertices of R and such that the sum of the weights of the edges in the subtree is no more than B?

Reference: [Karp, 1972]. Transformation from EXACT COVER BY 3-SETS.
Comment: Remains NP-complete if all edge weights are equal, even if G is a bipartite graph having no edges joining two vertices in R or two vertices in V−R [Berlekamp, 1976] or G is planar [Garey and Johnson, 1977a].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
