---
name: Problem
about: Propose a new problem type
title: "[Model] FiniteFunctionGeneration(*)"
labels: model
assignees: ''
---

## Motivation

FINITE FUNCTION GENERATION (*) (P317) from Garey & Johnson, A12 MS5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS5

**Mathematical definition:**

INSTANCE: Finite set A, a collection F of functions f: A→A, and a specified function h: A→A.
QUESTION: Can h be generated from the functions in F by composition?

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

INSTANCE: Finite set A, a collection F of functions f: A→A, and a specified function h: A→A.
QUESTION: Can h be generated from the functions in F by composition?
Reference: [Kozen, 1977d]. Transformation from FINITE STATE AUTOMATA INTERSECTION.
Comment: PSPACE-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
