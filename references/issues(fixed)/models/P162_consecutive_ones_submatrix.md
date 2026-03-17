---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveOnesSubmatrix"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE ONES SUBMATRIX (P162) from Garey & Johnson, A4 SR14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR14

**Mathematical definition:**

INSTANCE: An m×n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there an m×K submatrix B of A that has the "consecutive ones" property, i.e., such that the columns of B can be permuted so that in each row all the 1's occur consecutively?

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

INSTANCE: An m×n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there an m×K submatrix B of A that has the "consecutive ones" property, i.e., such that the columns of B can be permuted so that in each row all the 1's occur consecutively?
Reference: [Booth, 1975]. Transformation from HAMILTONIAN PATH.
Comment: The variant in which we ask instead that B have the "circular ones" property, i.e., that the columns of B can be permuted so that in each row either all the 1's or all the 0's occur consecutively, is also NP-complete. Both problems can be solved in polynomial time if K = n (in which case we are asking if A has the desired property), e.g., see [Fulkerson and Gross, 1965], [Tucker, 1971], and [Booth and Lueker, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
