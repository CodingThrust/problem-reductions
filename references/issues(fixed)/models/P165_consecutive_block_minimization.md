---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveBlockMinimization"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE BLOCK MINIMIZATION (P165) from Garey & Johnson, A4 SR17. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR17

**Mathematical definition:**

INSTANCE: An m×n matrix A of 0's and 1's and a positive integer K.
QUESTION: Is there a permutation of the columns of A that results in a matrix B having at most K blocks of consecutive 1's, i.e., having at most K entries bij such that bij = 1 and either bi,j+1 = 0 or j = n?

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
QUESTION: Is there a permutation of the columns of A that results in a matrix B having at most K blocks of consecutive 1's, i.e., having at most K entries bij such that bij = 1 and either bi,j+1 = 0 or j = n?
Reference: [Kou, 1977]. Transformation from HAMILTONIAN PATH.
Comment: Remains NP-complete if "j = n" is replaced by "j = n and bi1 = 0" [Booth, 1975]. If K equals the number of rows of A that are not all 0, then these problems are equivalent to testing A for the consecutive ones property or the circular ones property, respectively, and can be solved in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
