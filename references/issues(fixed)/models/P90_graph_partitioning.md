---
name: Problem
about: Propose a new problem type
title: "[Model] GraphPartitioning"
labels: model
assignees: ''
---

## Motivation

GRAPH PARTITIONING (P90) from Garey & Johnson, A2 ND14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND14

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weights w(v) ∈ Z+ for each v ∈ V and l(e) ∈ Z+ for each e ∈ E, positive integers K and J.
QUESTION: Is there a partition of V into disjoint sets V1,V2,···,Vm such that ∑v ∈ Vi w(v) ≤ K for 1 ≤ i ≤ m and such that if E' ⊆ E is the set of edges that have their two endpoints in two different sets Vi, then ∑e ∈ E' l(e) ≤ J?

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

INSTANCE: Graph G = (V,E), weights w(v) ∈ Z+ for each v ∈ V and l(e) ∈ Z+ for each e ∈ E, positive integers K and J.
QUESTION: Is there a partition of V into disjoint sets V1,V2,···,Vm such that ∑v ∈ Vi w(v) ≤ K for 1 ≤ i ≤ m and such that if E' ⊆ E is the set of edges that have their two endpoints in two different sets Vi, then ∑e ∈ E' l(e) ≤ J?

Reference: [Hyafil and Rivest, 1973]. Transformation from PARTITION INTO TRIANGLES.
Comment: Remains NP-complete for fixed K ≥ 3 even if all vertex and edge weights are 1. Can be solved in polynomial time for K = 2 by matching.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
