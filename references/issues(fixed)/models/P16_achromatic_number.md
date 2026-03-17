---
name: Problem
about: Propose a new problem type
title: "[Model] AchromaticNumber"
labels: model
assignees: ''
---

## Motivation

ACHROMATIC NUMBER (P16) from Garey & Johnson, A1.1 GT5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT5

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Does G have achromatic number K or greater, i.e., is there a partition of V into disjoint sets V_1, V_2, . . . , V_k, k ≥ K, such that each V_i is an independent set for G (no two vertices in V_i are joined by an edge in E) and such that, for each pair of distinct sets V_i, V_j, V_i ∪ V_j is not an independent set for G?

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
QUESTION: Does G have achromatic number K or greater, i.e., is there a partition of V into disjoint sets V_1, V_2, . . . , V_k, k ≥ K, such that each V_i is an independent set for G (no two vertices in V_i are joined by an edge in E) and such that, for each pair of distinct sets V_i, V_j, V_i ∪ V_j is not an independent set for G?
Reference: [Yannakakis and Gavril, 1978]. Transformation from MINIMUM MAXIMAL MATCHING.
Comment: Remains NP-complete even if G is the complement of a bipartite graph and hence has no independent set of more than two vertices.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
