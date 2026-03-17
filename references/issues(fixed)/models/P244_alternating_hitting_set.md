---
name: Problem
about: Propose a new problem type
title: "[Model] AlternatingHittingSet"
labels: model
assignees: ''
---

## Motivation

ALTERNATING HITTING SET (P244) from Garey & Johnson, A8 GP7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP7

**Mathematical definition:**

INSTANCE: A collection C of subsets of a basic set B.
QUESTION: Does player 1 have a forced win in the following game played on C and B? Players alternate choosing a new element of B until, for each c ∈ C, some member of c has been chosen. The player whose choice causes this to happen loses.

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

INSTANCE: A collection C of subsets of a basic set B.
QUESTION: Does player 1 have a forced win in the following game played on C and B? Players alternate choosing a new element of B until, for each c ∈ C, some member of c has been chosen. The player whose choice causes this to happen loses.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete even if no set in C contains more than two elements, a subcase of the original HITTING SET problem that can be solved in polynomial time. If the roles of winner and loser are reversed, the problem is PSPACE-complete even if no set in C contains more than three elements.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
