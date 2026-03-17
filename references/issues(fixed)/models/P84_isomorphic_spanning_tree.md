---
name: Problem
about: Propose a new problem type
title: "[Model] IsomorphicSpanningTree"
labels: model
assignees: ''
---

## Motivation

ISOMORPHIC SPANNING TREE (P84) from Garey & Johnson, A2 ND8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND8

**Mathematical definition:**

INSTANCE: Graph G = (V,E), tree T = (VT,ET).
QUESTION: Does G contain a spanning tree isomorphic to T?

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

INSTANCE: Graph G = (V,E), tree T = (VT,ET).
QUESTION: Does G contain a spanning tree isomorphic to T?

Reference: Transformation from HAMILTONIAN PATH.
Comment: Remains NP-complete even if (a) T is a path, (b) T is a full binary tree [Papadimitriou and Yannakakis, 1978], or if (c) T is a 3-star (that is, VT = {v0} ∪ {ui,vi,wi: 1 ≤ i ≤ n}, ET = {{v0,ui},{ui,vi},{vi,wi}: 1 ≤ i ≤ n}) [Garey and Johnson, ——]. Solvable in polynomial time by graph matching if G is a 2-star. For a classification of the complexity of this problem for other types of trees, see [Papadimitriou and Yannakakis, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
