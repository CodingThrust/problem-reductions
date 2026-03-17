---
name: Problem
about: Propose a new problem type
title: "[Model] Sift"
labels: model
assignees: ''
---

## Motivation

SIFT (P243) from Garey & Johnson, A8 GP6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP6

**Mathematical definition:**

INSTANCE: Two collections A and B of subsets of a finite set X, with A and B having no subsets in common.
QUESTION: Does player 1 have a forced win in the following game played on A, B, and X? Players alternate choosing an element from X until the set X' of all elements chosen so far either intersects all the subsets in A or intersects all the subsets in B. Player 1 wins if and only if the final set X' of chosen elements intersects all the subsets in B and, if player 1 made the last move, does not intersect all subsets in A.

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

INSTANCE: Two collections A and B of subsets of a finite set X, with A and B having no subsets in common.
QUESTION: Does player 1 have a forced win in the following game played on A, B, and X? Players alternate choosing an element from X until the set X' of all elements chosen so far either intersects all the subsets in A or intersects all the subsets in B. Player 1 wins if and only if the final set X' of chosen elements intersects all the subsets in B and, if player 1 made the last move, does not intersect all subsets in A.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
