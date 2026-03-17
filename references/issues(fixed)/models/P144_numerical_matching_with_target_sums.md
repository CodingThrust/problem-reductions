---
name: Problem
about: Propose a new problem type
title: "[Model] NumericalMatchingWithTargetSums"
labels: model
assignees: ''
---

## Motivation

NUMERICAL MATCHING WITH TARGET SUMS (P144) from Garey & Johnson, A3 SP17. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP17

**Mathematical definition:**

INSTANCE: Disjoint sets X and Y, each containing m elements, a size s(a) ∈ Z^+ for each element a ∈ X ∪ Y, and a target vector <B_1,B_2,…,B_m> with positive integer entries.
QUESTION: Can X ∪ Y be partitioned into m disjoint sets A_1,A_2,…,A_m, each containing exactly one element from each of X and Y, such that, for 1 ≤ i ≤ m, Σ_{a ∈ A_i} s(a) = B_i?

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

INSTANCE: Disjoint sets X and Y, each containing m elements, a size s(a) ∈ Z^+ for each element a ∈ X ∪ Y, and a target vector <B_1,B_2,…,B_m> with positive integer entries.
QUESTION: Can X ∪ Y be partitioned into m disjoint sets A_1,A_2,…,A_m, each containing exactly one element from each of X and Y, such that, for 1 ≤ i ≤ m, Σ_{a ∈ A_i} s(a) = B_i?
Reference: Transformation from NUMERICAL 3-DIMENSIONAL MATCHING.
Comment: NP-complete in the strong sense, but solvable in polynomial time if B_1 = B_2 = ··· = B_m.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
