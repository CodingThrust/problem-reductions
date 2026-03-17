---
name: Problem
about: Propose a new problem type
title: "[Model] 3Satisfiability(3sat)"
labels: model
assignees: ''
---

## Motivation

3-SATISFIABILITY (3SAT) (P254) from Garey & Johnson, A9 LO2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO2

**Mathematical definition:**

INSTANCE: Set U of variables, collection C of clauses over U such that each clause c∈C has |c|=3.
QUESTION: Is there a satisfying truth assignment for C?

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
QUESTION: Is there a satisfying truth assignment for C?
Reference: [Cook, 1971a]. Transformation from SATISFIABILITY.
Comment: Remains NP-complete even if each clause contains either only negated variables or only un-negated variables (MONOTONE 3SAT) [Gold, 1974], or if for each u∈U there are at most 5 clauses in C that contain either u or ū.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
