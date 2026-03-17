---
name: Problem
about: Propose a new problem type
title: "[Model] ExpectedRetrievalCost"
labels: model
assignees: ''
---

## Motivation

EXPECTED RETRIEVAL COST (P152) from Garey & Johnson, A4 SR4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR4

**Mathematical definition:**

INSTANCE: Set R of records, rational probability p(r) ∈ [0,1] for each r ∈ R, with ∑r ∈ R p(r) = 1, number m of sectors, and a positive integer K.
QUESTION: Is there a partition of R into disjoint subsets R1,R2,...,Rm such that, if p(Ri) = ∑r ∈ Ri p(r) and the "latency cost" d(i,j) is defined to be j − i − 1 if 1 ≤ i < j ≤ m and to be m − i + j − 1 if 1 ≤ j ≤ i ≤ m, then the sum over all ordered pairs i,j, 1 ≤ i,j ≤ m, of p(Ri)·p(Rj)·d(i,j) is at most K?

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

INSTANCE: Set R of records, rational probability p(r) ∈ [0,1] for each r ∈ R, with ∑r ∈ R p(r) = 1, number m of sectors, and a positive integer K.
QUESTION: Is there a partition of R into disjoint subsets R1,R2,...,Rm such that, if p(Ri) = ∑r ∈ Ri p(r) and the "latency cost" d(i,j) is defined to be j − i − 1 if 1 ≤ i < j ≤ m and to be m − i + j − 1 if 1 ≤ j ≤ i ≤ m, then the sum over all ordered pairs i,j, 1 ≤ i,j ≤ m, of p(Ri)·p(Rj)·d(i,j) is at most K?
Reference: [Cody and Coffman, 1976]. Transformation from PARTITION, 3-PARTITION.
Comment: NP-complete in the strong sense. NP-complete and solvable in pseudo-polynomial time for each fixed m ≥ 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
