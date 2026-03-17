---
name: Problem
about: Propose a new problem type
title: "[Model] SimultaneousDivisibilityOfLinearPolynomials"
labels: model
assignees: ''
---

## Motivation

SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS (P222) from Garey & Johnson, A7 AN3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN3

**Mathematical definition:**

INSTANCE: Vectors a_i = (a_i[0], . . . , a_i[m]) and b_i = (b_i[0], . . . , b_i[m]), 1 ≤ i ≤ n, with positive integer entries.
QUESTION: Do there exist positive integers x_1, x_2, . . . , x_m such that, for 1 ≤ i ≤ n, a_i[0] + Σ_{j=1}^{m} (a_i[j]·x_j) divides b_i[0] + Σ_{j=1}^{m} (b_i[j]·x_j)?

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

INSTANCE: Vectors a_i = (a_i[0], . . . , a_i[m]) and b_i = (b_i[0], . . . , b_i[m]), 1 ≤ i ≤ n, with positive integer entries.
QUESTION: Do there exist positive integers x_1, x_2, . . . , x_m such that, for 1 ≤ i ≤ n, a_i[0] + Σ_{j=1}^{m} (a_i[j]·x_j) divides b_i[0] + Σ_{j=1}^{m} (b_i[j]·x_j)?

Reference: [Lipshitz, 1977], [Lipshitz, 1978]. Transformation from QUADRATIC DIOPHANTINE EQUATIONS.
Comment: Not known to be in NP, but belongs to NP for any fixed n. NP-complete for any fixed n ≥ 5. General problem is undecidable if the vector entries and the x_j are allowed to range over the ring of "integers" in a real quadratic extension of the rationals. See reference for related decidability and undecidability results.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
