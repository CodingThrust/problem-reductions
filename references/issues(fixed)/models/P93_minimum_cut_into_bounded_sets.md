---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumCutIntoBoundedSets"
labels: model
assignees: ''
---

## Motivation

MINIMUM CUT INTO BOUNDED SETS (P93) from Garey & Johnson, A2 ND17. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND17

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(e) ∈ Z+ for each e ∈ E, specified vertices s,t ∈ V, positive integer B ≤ |V|, positive integer K.
QUESTION: Is there a partition of V into disjoint sets V1 and V2 such that s ∈ V1, t ∈ V2, |V1| ≤ B, |V2| ≤ B, and such that the sum of the weights of the edges from E that have one endpoint in V1 and one endpoint in V2 is no more than K?

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

INSTANCE: Graph G = (V,E), weight w(e) ∈ Z+ for each e ∈ E, specified vertices s,t ∈ V, positive integer B ≤ |V|, positive integer K.
QUESTION: Is there a partition of V into disjoint sets V1 and V2 such that s ∈ V1, t ∈ V2, |V1| ≤ B, |V2| ≤ B, and such that the sum of the weights of the edges from E that have one endpoint in V1 and one endpoint in V2 is no more than K?

Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from SIMPLE MAX CUT.
Comment: Remains NP-complete for B = |V|/2 and w(e) = 1 for all e ∈ E. Can be solved in polynomial time for B = |V| by standard network flow techniques.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
