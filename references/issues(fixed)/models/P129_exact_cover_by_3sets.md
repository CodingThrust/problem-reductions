---
name: Problem
about: Propose a new problem type
title: "[Model] ExactCoverBy3Sets(x3c)"
labels: model
assignees: ''
---

## Motivation

EXACT COVER BY 3-SETS (X3C) (P129) from Garey & Johnson, A3 SP2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP2

**Mathematical definition:**

INSTANCE: Set X with |X| = 3q and a collection C of 3-element subsets of X.
QUESTION: Does C contain an exact cover for X, i.e., a subcollection C' ⊆ C such that every element of X occurs in exactly one member of C'?

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

INSTANCE: Set X with |X| = 3q and a collection C of 3-element subsets of X.
QUESTION: Does C contain an exact cover for X, i.e., a subcollection C' ⊆ C such that every element of X occurs in exactly one member of C'?
Reference: [Karp, 1972]. Transformation from 3DM.
Comment: Remains NP-complete if no element occurs in more than three subsets, but is solvable in polynomial time if no element occurs in more than two subsets [Garey and Johnson, ——]. Related EXACT COVER BY 2-SETS problem is also solvable in polynomial time by matching techniques.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
