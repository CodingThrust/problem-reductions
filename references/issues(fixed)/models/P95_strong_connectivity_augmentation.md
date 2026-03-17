---
name: Problem
about: Propose a new problem type
title: "[Model] StrongConnectivityAugmentation"
labels: model
assignees: ''
---

## Motivation

STRONG CONNECTIVITY AUGMENTATION (P95) from Garey & Johnson, A2 ND19. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND19

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), weight w(u,v) ∈ Z+ for each ordered pair (u,v) ∈ V×V, positive integer B.
QUESTION: Is there a set A' of ordered pairs of vertices from V such that ∑a ∈ A' w(a) ≤ B and such that the graph G' = (V,A∪A') is strongly connected?

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

INSTANCE: Directed graph G = (V,A), weight w(u,v) ∈ Z+ for each ordered pair (u,v) ∈ V×V, positive integer B.
QUESTION: Is there a set A' of ordered pairs of vertices from V such that ∑a ∈ A' w(a) ≤ B and such that the graph G' = (V,A∪A') is strongly connected?

Reference: [Eswaran and Tarjan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete if all weights are either 1 or 2 and A is empty. Can be solved in polynomial time if all weights are equal.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
