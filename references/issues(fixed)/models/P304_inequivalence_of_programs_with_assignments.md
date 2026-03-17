---
name: Problem
about: Propose a new problem type
title: "[Model] InequivalenceOfProgramsWithAssignments"
labels: model
assignees: ''
---

## Motivation

INEQUIVALENCE OF PROGRAMS WITH ASSIGNMENTS (P304) from Garey & Johnson, A11 PO12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO12

**Mathematical definition:**

INSTANCE: Finite set X of variables, two programs P1 and P2, each a sequence of assignments of the form "x0 ← if x1=x2 then x3 else x4" where the xi are in X, and a value set V.
QUESTION: Is there an initial assignment of a value from V to each variable in X such that the two programs yield different final values for some variable in X (see reference for details on the execution of such programs)?

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

INSTANCE: Finite set X of variables, two programs P1 and P2, each a sequence of assignments of the form "x0 ← if x1=x2 then x3 else x4" where the xi are in X, and a value set V.
QUESTION: Is there an initial assignment of a value from V to each variable in X such that the two programs yield different final values for some variable in X (see reference for details on the execution of such programs)?
Reference: [Downey and Sethi, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete for V = {0,1}. This problem can be embedded in many inequivalence problems for simple programs, thus rendering them NP-hard [Downey and Sethi, 1976], [van Leeuwen, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
