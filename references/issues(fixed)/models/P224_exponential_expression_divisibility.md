---
name: Problem
about: Propose a new problem type
title: "[Model] ExponentialExpressionDivisibility"
labels: model
assignees: ''
---

## Motivation

EXPONENTIAL EXPRESSION DIVISIBILITY (P224) from Garey & Johnson, A7 AN5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN5

**Mathematical definition:**

INSTANCE: Sequences a_1, a_2, . . . , a_n and b_1, b_2, . . . , b_m of positive integers, and an integer q.
QUESTION: Does ∏_{i=1}^{n} (q^{a_i} - 1) divide ∏_{j=1}^{m} (q^{b_j} - 1)?

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

INSTANCE: Sequences a_1, a_2, . . . , a_n and b_1, b_2, . . . , b_m of positive integers, and an integer q.
QUESTION: Does ∏_{i=1}^{n} (q^{a_i} - 1) divide ∏_{j=1}^{m} (q^{b_j} - 1)?

Reference: [Plaisted, 1976]. Transformation from 3SAT.
Comment: Not known to be in NP or co-NP, but solvable in pseudo-polynomial time using standard greatest common divisor algorithms. Remains NP-hard for any fixed value of q with |q| > 1, even if the a_i and b_j are restricted to being products of distinct primes.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
