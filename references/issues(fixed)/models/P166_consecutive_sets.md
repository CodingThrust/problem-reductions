---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveSets"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE SETS (P166) from Garey & Johnson, A4 SR18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR18

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, collection C = {Σ1,Σ2,...,Σn} of subsets of Σ, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that, for each i, the elements of Σi occur in a consecutive block of |Σi| symbols of W?

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

INSTANCE: Finite alphabet Σ, collection C = {Σ1,Σ2,...,Σn} of subsets of Σ, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that, for each i, the elements of Σi occur in a consecutive block of |Σi| symbols of W?
Reference: [Kou, 1977]. Transformation from HAMILTONIAN PATH.
Comment: The variant in which we ask only that the elements of each Σi occur in a consecutive block of |Σi| symbols of the string ww (i.e., we allow blocks that circulate from the end of w back to its beginning) is also NP-complete [Booth, 1975]. If K is the number of distinct symbols in the Σi, then these problems are equivalent to determining whether a matrix has the consecutive ones property or the circular ones property and are solvable in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
