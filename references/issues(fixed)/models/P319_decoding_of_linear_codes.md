---
name: Problem
about: Propose a new problem type
title: "[Model] DecodingOfLinearCodes"
labels: model
assignees: ''
---

## Motivation

DECODING OF LINEAR CODES (P319) from Garey & Johnson, A12 MS7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS7

**Mathematical definition:**

INSTANCE: An n×m matrix A = (aij) of 0's and 1's, a vector ȳ = (y1,y2,...,ym) of 0's and 1's, and a positive integer K.
QUESTION: Is there a 0-1 vector x̄ = (x1,x2,...,xn) with no more than K 1's such that, for 1 ≤ j ≤ m, ∑i=1n xi·aij ≡ yj (mod 2)?

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

INSTANCE: An n×m matrix A = (aij) of 0's and 1's, a vector ȳ = (y1,y2,...,ym) of 0's and 1's, and a positive integer K.
QUESTION: Is there a 0-1 vector x̄ = (x1,x2,...,xn) with no more than K 1's such that, for 1 ≤ j ≤ m, ∑i=1n xi·aij ≡ yj (mod 2)?
Reference: [Berlekamp, McEliece, and van Tilborg, 1978]. Transformation from 3DM.
Comment: If ȳ is the all zero vector, and hence we are asking for a "codeword" of Hamming weight K or less, the problem is open. The variant in which we ask for an x̄ with exactly K 1's is NP-complete, even for fixed ȳ = (0,0,...,0).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
