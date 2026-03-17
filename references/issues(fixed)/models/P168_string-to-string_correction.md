---
name: Problem
about: Propose a new problem type
title: "[Model] StringToStringCorrection"
labels: model
assignees: ''
---

## Motivation

STRING-TO-STRING CORRECTION (P168) from Garey & Johnson, A4 SR20. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR20

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, two strings x,y ∈ Σ*, and a positive integer K.
QUESTION: Is there a way to derive the string y from the string x by a sequence of K or fewer operations of single symbol deletion or adjacent symbol interchange?

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

INSTANCE: Finite alphabet Σ, two strings x,y ∈ Σ*, and a positive integer K.
QUESTION: Is there a way to derive the string y from the string x by a sequence of K or fewer operations of single symbol deletion or adjacent symbol interchange?
Reference: [Wagner, 1975]. Transformation from SET COVERING.
Comment: Solvable in polynomial time if the operation set is expanded to include the operations of changing a single character and of inserting a single character, even if interchanges are not allowed (e.g., see [Wagner and Fischer, 1974]), or if the only operation is adjacent symbol interchange [Wagner, 1975]. See reference for related results for cases in which different operations can have different costs.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
