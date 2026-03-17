---
name: Problem
about: Propose a new problem type
title: "[Model] SubsetSum"
labels: model
assignees: ''
---

## Motivation

SUBSET SUM (P140) from Garey & Johnson, A3 SP13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP13

**Mathematical definition:**

INSTANCE: Finite set A, size s(a) ∈ Z^+ for each a ∈ A, positive integer B.
QUESTION: Is there a subset A' ⊆ A such that the sum of the sizes of the elements in A' is exactly B?

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

INSTANCE: Finite set A, size s(a) ∈ Z^+ for each a ∈ A, positive integer B.
QUESTION: Is there a subset A' ⊆ A such that the sum of the sizes of the elements in A' is exactly B?
Reference: [Karp, 1972]. Transformation from PARTITION.
Comment: Solvable in pseudo-polynomial time (see Section 4.2).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
