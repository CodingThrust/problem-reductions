---
name: Problem
about: Propose a new problem type
title: "[Model] HittingSet"
labels: model
assignees: ''
---

## Motivation

HITTING SET (P135) from Garey & Johnson, A3 SP8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP8

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |S|.
QUESTION: Is there a subset S' ⊆ S with |S'| ≤ K such that S' contains at least one element from each subset in C?

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

INSTANCE: Collection C of subsets of a finite set S, positive integer K ≤ |S|.
QUESTION: Is there a subset S' ⊆ S with |S'| ≤ K such that S' contains at least one element from each subset in C?
Reference: [Karp, 1972]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if |c| ≤ 2 for all c ∈ C.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
