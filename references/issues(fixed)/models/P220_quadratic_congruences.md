---
name: Problem
about: Propose a new problem type
title: "[Model] QuadraticCongruences"
labels: model
assignees: ''
---

## Motivation

QUADRATIC CONGRUENCES (P220) from Garey & Johnson, A7 AN1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN1

**Mathematical definition:**

INSTANCE: Positive integers a, b, and c.
QUESTION: Is there a positive integer x < c such that x^2 ≡ a (mod b)?

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

INSTANCE: Positive integers a, b, and c.
QUESTION: Is there a positive integer x < c such that x^2 ≡ a (mod b)?

Reference: [Manders and Adleman, 1978]. Transformation from 3SAT.
Comment: Remains NP-complete even if the instance includes a prime factorization of b and solutions to the congruence modulo all prime powers occurring in the factorization. Solvable in polynomial time if c = ∞ (i.e., there is no upper bound on x) and the prime factorization of b is given. Assuming the Extended Riemann Hypothesis, the problem is solvable in polynomial time when b is prime. The general problem is trivially solvable in pseudo-polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
