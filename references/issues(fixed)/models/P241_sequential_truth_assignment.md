---
name: Problem
about: Propose a new problem type
title: "[Model] SequentialTruthAssignment"
labels: model
assignees: ''
---

## Motivation

SEQUENTIAL TRUTH ASSIGNMENT (P241) from Garey & Johnson, A8 GP4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP4

**Mathematical definition:**

INSTANCE: A sequence U = <u1,u2,...,un> of variables and a collection C of clauses over U (as in an instance of SATISFIABILITY).
QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate assigning truth values to the variables in U, with player 1 assigning a value to u2i-1 and player 2 assigning a value to u2i on their ith turns. Player 1 wins if and only if the resulting truth assignment satisfies all clauses in C.

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

INSTANCE: A sequence U = <u1,u2,...,un> of variables and a collection C of clauses over U (as in an instance of SATISFIABILITY).
QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate assigning truth values to the variables in U, with player 1 assigning a value to u2i-1 and player 2 assigning a value to u2i on their ith turns. Player 1 wins if and only if the resulting truth assignment satisfies all clauses in C.

Reference: [Stockmeyer and Meyer, 1973]. Transformation from QBF.
Comment: PSPACE-complete, even if each clause in C has only three literals. Solvable in polynomial time if no clause has more than two literals [Schaefer, 1978b].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
