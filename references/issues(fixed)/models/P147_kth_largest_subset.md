---
name: Problem
about: Propose a new problem type
title: "[Model] K^thLargestSubset"
labels: model
assignees: ''
---

## Motivation

K^th LARGEST SUBSET (P147) from Garey & Johnson, A3 SP20. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP20

**Mathematical definition:**

INSTANCE: Finite set A, size s(a) ∈ Z^+ for each a ∈ A, positive integers K and B.
QUESTION: Are there K or more distinct subsets A' ⊆ A for which the sum of the sizes of the elements in A' does not exceed B?

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

INSTANCE: Finite set A, size s(a) ∈ Z^+ for each a ∈ A, positive integers K and B.
QUESTION: Are there K or more distinct subsets A' ⊆ A for which the sum of the sizes of the elements in A' does not exceed B?
Reference: [Johnson and Kashdan, 1976]. Transformation from SUBSET SUM.
Comment: Not known to be in NP. Solvable in pseudo-polynomial time (polynomial in K, |A|, and log Σs(a)) [Lawler, 1972]. The corresponding enumeration problem is #P-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
