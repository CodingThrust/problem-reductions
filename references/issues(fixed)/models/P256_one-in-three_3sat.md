---
name: Problem
about: Propose a new problem type
title: "[Model] OneInThree3sat"
labels: model
assignees: ''
---

## Motivation

ONE-IN-THREE 3SAT (P256) from Garey & Johnson, A9 LO4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO4

**Mathematical definition:**

INSTANCE: Set U of variables, collection C of clauses over U such that each clause c∈C has |c|=3.
QUESTION: Is there a truth assignment for U such that each clause in C has exactly one true literal?

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

INSTANCE: Set U of variables, collection C of clauses over U such that each clause c∈C has |c|=3.
QUESTION: Is there a truth assignment for U such that each clause in C has exactly one true literal?
Reference: [Schaefer, 1978b]. Transformation from 3SAT.
Comment: Remains NP-complete even if no c∈C contains a negated literal.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
