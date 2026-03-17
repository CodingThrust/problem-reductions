---
name: Problem
about: Propose a new problem type
title: "[Model] ExpectedComponentSum"
labels: model
assignees: ''
---

## Motivation

EXPECTED COMPONENT SUM (P145) from Garey & Johnson, A3 SP18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP18

**Mathematical definition:**

INSTANCE: Collection C of m-dimensional vectors v = (v_1,v_2,…,v_m) with non-negative integer entries, positive integers K and B.
QUESTION: Is there a partition of C into disjoint sets C_1,C_2,…,C_K such that
Σ_{i=1}^{K} max_{1 ≤ j ≤ m} (Σ_{v ∈ C_i} v_j) ≥ B ?

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

INSTANCE: Collection C of m-dimensional vectors v = (v_1,v_2,…,v_m) with non-negative integer entries, positive integers K and B.
QUESTION: Is there a partition of C into disjoint sets C_1,C_2,…,C_K such that
Σ_{i=1}^{K} max_{1 ≤ j ≤ m} (Σ_{v ∈ C_i} v_j) ≥ B ?
Reference: [Garey and Johnson, ——]. Transformation from X3C. The problem is due to [Witsenhausen, 1978] and corresponds to finding a partition that maximizes the expected value of the largest component sum, assuming all sets in the partition are equally likely.
Comment: NP-complete even if all entries are 0's and 1's. Solvable in polynomial time if K is fixed. The variant in which we ask for a partition with K non-empty sets that yields a sum of B or less is NP-complete even if K is fixed at 3 and all entries are 0's and 1's.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
