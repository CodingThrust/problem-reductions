---
name: Problem
about: Propose a new problem type
title: "[Model] NonDivisibilityOfAProductPolynomial"
labels: model
assignees: ''
---

## Motivation

NON-DIVISIBILITY OF A PRODUCT POLYNOMIAL (P225) from Garey & Johnson, A7 AN6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN6

**Mathematical definition:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0, and an integer N.
QUESTION: Is ∏_{i=1}^{m} (Σ_{j=1}^{k} a_i[j]·z^{b_i[j]}) not divisible by z^N - 1?

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

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0, and an integer N.
QUESTION: Is ∏_{i=1}^{m} (Σ_{j=1}^{k} a_i[j]·z^{b_i[j]}) not divisible by z^N - 1?

Reference: [Plaisted, 1977a], [Plaisted, 1977b]. Transformation from 3SAT. Proof of membership in NP is non-trivial and appears in the second reference.
Comment: The related problem in which we are given two sequences <a_1, a_2, . . . , a_m> and <b_1, b_2, . . . , b_n> of positive integers and are asked whether ∏_{i=1}^{m} (z^{a_i} - 1) does not divide ∏_{j=1}^{n} (z^{b_j} - 1) is also NP-complete [Plaisted, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
