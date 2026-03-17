---
name: Problem
about: Propose a new problem type
title: "[Model] Clustering"
labels: model
assignees: ''
---

## Motivation

CLUSTERING (P321) from Garey & Johnson, A12 MS9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS9

**Mathematical definition:**

INSTANCE: Finite set X, a distance d(x,y) ∈ Z0+ for each pair x,y ∈ X, and two positive integers K and B.
QUESTION: Is there a partition of X into disjoint sets X1,X2,...,Xk such that, for 1 ≤ i ≤ k and all pairs x,y ∈ Xi, d(x,y) ≤ B?

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

INSTANCE: Finite set X, a distance d(x,y) ∈ Z0+ for each pair x,y ∈ X, and two positive integers K and B.
QUESTION: Is there a partition of X into disjoint sets X1,X2,...,Xk such that, for 1 ≤ i ≤ k and all pairs x,y ∈ Xi, d(x,y) ≤ B?
Reference: [Brucker, 1978]. Transformation from GRAPH 3-COLORABILITY.
Comment: Remains NP-complete even for fixed K = 3 and all distances in {0,1}. Solvable in polynomial time for K = 2. Variants in which we ask that the sum, over all Xi, of max{d(x,y): x,y ∈ Xi} or of ∑x,y∈Xi d(x,y) be at most B, are similarly NP-complete (with the last one NP-complete even for K = 2).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
