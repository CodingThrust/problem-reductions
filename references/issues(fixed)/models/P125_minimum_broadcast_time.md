---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumBroadcastTime"
labels: model
assignees: ''
---

## Motivation

MINIMUM BROADCAST TIME (P125) from Garey & Johnson, A2 ND49. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND49

**Mathematical definition:**

INSTANCE: Graph G = (V,E), subset V_0 ⊆ V, and a positive integer K.
QUESTION: Can a message be "broadcast" from the base set V_0 to all other vertices in time K, i.e., is there a sequence V_0,E_1,V_1,E_2,…,E_K,V_K such that each V_i ⊆ V, each E_i ⊆ E, V_K = V, and, for 1 ≤ i ≤ K, (1) each edge in E_i has exactly one endpoint in V_{i−1}, (2) no two edges in E_i share a common endpoint, and (3) V_i = V_{i−1} ∪ {v: {u,v} ∈ E_i}?

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

INSTANCE: Graph G = (V,E), subset V_0 ⊆ V, and a positive integer K.
QUESTION: Can a message be "broadcast" from the base set V_0 to all other vertices in time K, i.e., is there a sequence V_0,E_1,V_1,E_2,…,E_K,V_K such that each V_i ⊆ V, each E_i ⊆ E, V_K = V, and, for 1 ≤ i ≤ K, (1) each edge in E_i has exactly one endpoint in V_{i−1}, (2) no two edges in E_i share a common endpoint, and (3) V_i = V_{i−1} ∪ {v: {u,v} ∈ E_i}?
Reference: [Garey and Johnson, ——]. Transformation from 3DM. For more on this problem, see [Farley, Hedetniemi, Mitchell, and Proskurowski, 1977].
Comment: Remains NP-complete for any fixed K ≥ 4, but is solvable in polynomial time by matching if K = 1. The special case where |V_0| = 1 remains NP-complete, but is solvable in polynomial time for trees [Cockayne, Hedetniemi, and Slater, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
