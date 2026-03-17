---
name: Problem
about: Propose a new problem type
title: "[Model] AlgebraicEquationsOverGf[2]"
labels: model
assignees: ''
---

## Motivation

ALGEBRAIC EQUATIONS OVER GF[2] (P228) from Garey & Johnson, A7 AN9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN9

**Mathematical definition:**

INSTANCE: Polynomials P_i(x_1, x_2, . . . , x_n), 1 ≤ i ≤ m, over GF[2], i.e., each polynomial is a sum of terms, where each term is either the integer 1 or a product of distinct x_i.
QUESTION: Do there exist u_1, u_2, . . . , u_n ∈ {0,1} such that, for 1 ≤ i ≤ m, P_i(u_1, u_2, . . . , u_n) = 0, where arithmetic operations are as defined in GF[2], with 1+1 = 0 and 1·1 = 1?

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

INSTANCE: Polynomials P_i(x_1, x_2, . . . , x_n), 1 ≤ i ≤ m, over GF[2], i.e., each polynomial is a sum of terms, where each term is either the integer 1 or a product of distinct x_i.
QUESTION: Do there exist u_1, u_2, . . . , u_n ∈ {0,1} such that, for 1 ≤ i ≤ m, P_i(u_1, u_2, . . . , u_n) = 0, where arithmetic operations are as defined in GF[2], with 1+1 = 0 and 1·1 = 1?

Reference: [Fraenkel and Yesha, 1977]. Transformation from X3C.
Comment: Remains NP-complete even if none of the polynomials has a term involving more than two variables [Valiant, 1977c]. Easily solved in polynomial time if no term involves more than one variable or if there is just one polynomial. Variant in which the u_j are allowed to range over the algebraic closure of GF[2] is NP-hard, even if no term involves more than two variables [Fraenkel and Yesha, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
