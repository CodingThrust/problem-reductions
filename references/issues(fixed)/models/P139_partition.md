---
name: Problem
about: Propose a new problem type
title: "[Model] Partition"
labels: model
assignees: ''
---

## Motivation

PARTITION (P139) from Garey & Johnson, A3 SP12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP12

**Mathematical definition:**

INSTANCE: Finite set A and a size s(a) ∈ Z^+ for each a ∈ A.
QUESTION: Is there a subset A' ⊆ A such that Σ_{a ∈ A'} s(a) = Σ_{a ∈ A−A'} s(a)?

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

INSTANCE: Finite set A and a size s(a) ∈ Z^+ for each a ∈ A.
QUESTION: Is there a subset A' ⊆ A such that Σ_{a ∈ A'} s(a) = Σ_{a ∈ A−A'} s(a)?
Reference: [Karp, 1972]. Transformation from 3DM (see Section 3.1.5).
Comment: Remains NP-complete even if we require that |A'| = |A|/2, or if the elements in A are ordered as a_1,a_2,…,a_{2n} and we require that A' contain exactly one of a_{2i−1},a_{2i} for 1 ≤ i ≤ n. However, all these problems can be solved in pseudo-polynomial time by dynamic programming (see Section 4.2).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
