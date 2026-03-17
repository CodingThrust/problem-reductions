---
name: Problem
about: Propose a new problem type
title: "[Model] LargestCommonSubgraph"
labels: model
assignees: ''
---

## Motivation

LARGEST COMMON SUBGRAPH (P60) from Garey & Johnson, A1.4 GT49. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT49

**Mathematical definition:**

INSTANCE: Graphs G = (V_1,E_1), H = (V_2,E_2), positive integer K.
QUESTION: Do there exist subsets E_1' ⊆ E_1 and E_2' ⊆ E_2 with |E_1'| = |E_2'| ≥ K such that the two subgraphs G' = (V_1,E_1') and H' = (V_2,E_2') are isomorphic?

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

INSTANCE: Graphs G = (V_1,E_1), H = (V_2,E_2), positive integer K.
QUESTION: Do there exist subsets E_1' ⊆ E_1 and E_2' ⊆ E_2 with |E_1'| = |E_2'| ≥ K such that the two subgraphs G' = (V_1,E_1') and H' = (V_2,E_2') are isomorphic?

Reference: Transformation from CLIQUE.
Comment: Can be solved in polynomial time if both G and H are trees [Edmonds and Matula, 1975].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
