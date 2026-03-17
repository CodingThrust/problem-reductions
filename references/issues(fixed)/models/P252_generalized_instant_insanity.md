---
name: Problem
about: Propose a new problem type
title: "[Model] GeneralizedInstantInsanity"
labels: model
assignees: ''
---

## Motivation

GENERALIZED INSTANT INSANITY (P252) from Garey & Johnson, A8 GP15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP15

**Mathematical definition:**

INSTANCE: Finite set C of "colors" and a set Q of cubes, with |Q| = |C| and with each side of each cube in Q having some assigned color from C.
QUESTION: Can the cubes in Q be stacked in one vertical column such that each of the colors in C appears exactly once on each of the four sides of the column?

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

INSTANCE: Finite set C of "colors" and a set Q of cubes, with |Q| = |C| and with each side of each cube in Q having some assigned color from C.
QUESTION: Can the cubes in Q be stacked in one vertical column such that each of the colors in C appears exactly once on each of the four sides of the column?

Reference: [Robertson and Munro, 1978]. Transformation from EXACT COVER.
Comment: The associated two-person game, in which players alternate placing a new cube on the stack, with player 1 trying to construct a stack as specified above and player 2 trying to prevent this, is PSPACE-complete with respect to whether the first player has a forced win. INSTANT INSANITY is a trade name of Parker Brothers, Inc.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
