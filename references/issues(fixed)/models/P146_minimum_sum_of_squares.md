---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumSumOfSquares"
labels: model
assignees: ''
---

## Motivation

MINIMUM SUM OF SQUARES (P146) from Garey & Johnson, A3 SP19. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP19

**Mathematical definition:**

INSTANCE: Finite set A, a size s(a) ∈ Z^+ for each a ∈ A, positive integers K ≤ |A| and J.
QUESTION: Can A be partitioned into K disjoint sets A_1,A_2,…,A_K such that
Σ_{i=1}^{K} (Σ_{a ∈ A_i} s(a))^2 ≤ J ?

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

INSTANCE: Finite set A, a size s(a) ∈ Z^+ for each a ∈ A, positive integers K ≤ |A| and J.
QUESTION: Can A be partitioned into K disjoint sets A_1,A_2,…,A_K such that
Σ_{i=1}^{K} (Σ_{a ∈ A_i} s(a))^2 ≤ J ?
Reference: Transformation from PARTITION or 3-PARTITION.
Comment: NP-complete in the strong sense. NP-complete in the ordinary sense and solvable in pseudo-polynomial time for any fixed K. Variants in which the bound K on the number of sets is replaced by a bound B on either the maximum set cardinality or the maximum total set size are also NP-complete in the strong sense [Wong and Yao, 1976]. In all these cases, NP-completeness is preserved if the exponent 2 is replaced by any fixed rational α > 1.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
