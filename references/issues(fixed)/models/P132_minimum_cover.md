---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumCover"
labels: model
assignees: ''
---

## Motivation

MINIMUM COVER (P132) from Garey & Johnson, A3 SP5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP5

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |C|.
QUESTION: Does C contain a cover for S of size K or less, i.e., a subset C' ⊆ C with |C'| ≤ K such that every element of S belongs to at least one member of C'?

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

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |C|.
QUESTION: Does C contain a cover for S of size K or less, i.e., a subset C' ⊆ C with |C'| ≤ K such that every element of S belongs to at least one member of C'?
Reference: [Karp, 1972]. Transformation from X3C.
Comment: Remains NP-complete even if all c ∈ C have |c| ≤ 3. Solvable in polynomial time by matching techniques if all c ∈ C have |c| ≤ 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
