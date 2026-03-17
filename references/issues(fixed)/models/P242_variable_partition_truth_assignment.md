---
name: Problem
about: Propose a new problem type
title: "[Model] VariablePartitionTruthAssignment"
labels: model
assignees: ''
---

## Motivation

VARIABLE PARTITION TRUTH ASSIGNMENT (P242) from Garey & Johnson, A8 GP5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP5

**Mathematical definition:**

INSTANCE: A set U of variables and a collection C of clauses over U.
QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate choosing a variable from U until all variables have been chosen. Player 1 wins if and only if a satisfying truth assignment for C is obtained by setting "true" all variables chosen by player 1 and setting "false" all variables chosen by player 2.

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

INSTANCE: A set U of variables and a collection C of clauses over U.
QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate choosing a variable from U until all variables have been chosen. Player 1 wins if and only if a satisfying truth assignment for C is obtained by setting "true" all variables chosen by player 1 and setting "false" all variables chosen by player 2.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete, even if each clause consists only of un-negated literals (i.e., contains no literals of the form ū for u ∈ U). Analogous results for several other games played on logical expressions can be found in the reference.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
