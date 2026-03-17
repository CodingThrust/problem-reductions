---
name: Problem
about: Propose a new problem type
title: "[Model] N×nCheckers"
labels: model
assignees: ''
---

## Motivation

N×N CHECKERS (P247) from Garey & Johnson, A8 GP10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP10

**Mathematical definition:**

INSTANCE: Positive integer N, a partition of the black squares of an N×N Checkerboard into those that are empty, those that are occupied by "Black kings," and those that are occupied by "Red kings," and the identity of the player (Red or Black) whose turn it is.
QUESTION: Does Black have a forced win from the given position in a game of Checkers played according to the standard rules, modified only to take into account the expanded board and number of pieces?

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

INSTANCE: Positive integer N, a partition of the black squares of an N×N Checkerboard into those that are empty, those that are occupied by "Black kings," and those that are occupied by "Red kings," and the identity of the player (Red or Black) whose turn it is.
QUESTION: Does Black have a forced win from the given position in a game of Checkers played according to the standard rules, modified only to take into account the expanded board and number of pieces?

Reference: [Fraenkel, Garey, Johnson, Schaefer, and Yesha, 1978]. Transformation from PLANAR GEOGRAPHY.
Comment: PSPACE-hard, and PSPACE-complete for certain drawing rules. The related problem in which we ask whether Black can jump all of Red's pieces in one turn is solvable in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
