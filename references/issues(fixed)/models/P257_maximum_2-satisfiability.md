---
name: Problem
about: Propose a new problem type
title: "[Model] Maximum2Satisfiability"
labels: model
assignees: ''
---

## Motivation

MAXIMUM 2-SATISFIABILITY (P257) from Garey & Johnson, A9 LO5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO5

**Mathematical definition:**

INSTANCE: Set U of variables, collection C of clauses over U such that each clause c∈C has |c|=2, positive integer K≤|C|.
QUESTION: Is there a truth assignment for U that simultaneously satisfies at least K of the clauses in C?

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

INSTANCE: Set U of variables, collection C of clauses over U such that each clause c∈C has |c|=2, positive integer K≤|C|.
QUESTION: Is there a truth assignment for U that simultaneously satisfies at least K of the clauses in C?
Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from 3SAT.
Comment: Solvable in polynomial time if K=|C| (e.g.,see [Even, Itai, and Shamir, 1976]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
