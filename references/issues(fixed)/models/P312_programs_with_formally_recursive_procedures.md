---
name: Problem
about: Propose a new problem type
title: "[Model] ProgramsWithFormallyRecursiveProcedures"
labels: model
assignees: ''
---

## Motivation

PROGRAMS WITH FORMALLY RECURSIVE PROCEDURES (P312) from Garey & Johnson, A11 PO20. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO20

**Mathematical definition:**

INSTANCE: Finite set A of procedure identifiers, ALGOL-like program P involving procedure declarations and procedure calls for procedures in A (see reference for details).
QUESTION: Is any of the procedures in A "formally recursive" in program P (in the sense of [Langmaack, 1973])?

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

INSTANCE: Finite set A of procedure identifiers, ALGOL-like program P involving procedure declarations and procedure calls for procedures in A (see reference for details).
QUESTION: Is any of the procedures in A "formally recursive" in program P (in the sense of [Langmaack, 1973])?
Reference: [Winklmann, 1977]. Transformation from 3SAT.
Comment: See reference for related results concerning deciding whether P has the "formal most-recent property," "formal parameter correctness," the "formal macro-property," and others.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
