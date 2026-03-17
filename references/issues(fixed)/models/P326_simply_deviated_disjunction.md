---
name: Problem
about: Propose a new problem type
title: "[Model] SimplyDeviatedDisjunction"
labels: model
assignees: ''
---

## Motivation

SIMPLY DEVIATED DISJUNCTION (P326) from Garey & Johnson, A12 MS14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS14

**Mathematical definition:**

INSTANCE: Collection M of m-tuples (Mi[1],Mi[2],...,Mi[m]), 1 ≤ i ≤ n, with each Mi[j] being either 0,1, or x.
QUESTION: Is there a partition of {1,2,...,m} into disjoint sets I,J and an assignment f: {1,2,...,m}→{0,1} such that, if Φ is the formula ∨j∈I (M[j]=f(j)) and Ψ is the formula ∨j∈J (M[j]=f(j)), then Φ and Ψ are simply deviated in M, i.e., the number of Mi ∈ M such that Φ and Ψ are both true for Mi times the number of Mi ∈ M such that Φ and Ψ are both false for Mi is larger than the number of Mi ∈ M such that Φ is true and Ψ is false for Mi times the number of Mi ∈ M such that Φ is false and Ψ is true for Mi? (The definition of "simply deviated" is from [Havránek, 1975].)

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

INSTANCE: Collection M of m-tuples (Mi[1],Mi[2],...,Mi[m]), 1 ≤ i ≤ n, with each Mi[j] being either 0,1, or x.
QUESTION: Is there a partition of {1,2,...,m} into disjoint sets I,J and an assignment f: {1,2,...,m}→{0,1} such that, if Φ is the formula ∨j∈I (M[j]=f(j)) and Ψ is the formula ∨j∈J (M[j]=f(j)), then Φ and Ψ are simply deviated in M, i.e., the number of Mi ∈ M such that Φ and Ψ are both true for Mi times the number of Mi ∈ M such that Φ and Ψ are both false for Mi is larger than the number of Mi ∈ M such that Φ is true and Ψ is false for Mi times the number of Mi ∈ M such that Φ is false and Ψ is true for Mi? (The definition of "simply deviated" is from [Havránek, 1975].)
Reference: [Pudlák and Springsteel, 1975]. Transformation from MAX CUT.
Comment: Remains NP-complete even if f(j) = 1 for 1 ≤ j ≤ m. Solvable in polynomial time if each Mi[j] is either 0 or 1. See reference for additional related results.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
