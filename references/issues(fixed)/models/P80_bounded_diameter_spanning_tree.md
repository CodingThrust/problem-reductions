---
name: Problem
about: Propose a new problem type
title: "[Model] BoundedDiameterSpanningTree"
labels: model
assignees: ''
---

## Motivation

BOUNDED DIAMETER SPANNING TREE (P80) from Garey & Johnson, A2 ND4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND4

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(e) ∈ Z+ for each e ∈ E, positive integer D ≤ |V|, positive integer B.
QUESTION: Is there a spanning tree T for G such that the sum of the weights of the edges in T does not exceed B and such that T contains no simple path with more than D edges?

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

INSTANCE: Graph G = (V,E), weight w(e) ∈ Z+ for each e ∈ E, positive integer D ≤ |V|, positive integer B.
QUESTION: Is there a spanning tree T for G such that the sum of the weights of the edges in T does not exceed B and such that T contains no simple path with more than D edges?

Reference: [Garey and Johnson, ——]. Transformation from EXACT COVER BY 3-SETS.
Comment: Remains NP-complete for any fixed D ≥ 4, even if all edge weights are either 1 or 2. Can be solved easily in polynomial time if D ≤ 3, or if all edge weights are equal.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
