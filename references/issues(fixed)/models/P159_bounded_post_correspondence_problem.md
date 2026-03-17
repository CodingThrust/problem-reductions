---
name: Problem
about: Propose a new problem type
title: "[Model] BoundedPostCorrespondenceProblem"
labels: model
assignees: ''
---

## Motivation

BOUNDED POST CORRESPONDENCE PROBLEM (P159) from Garey & Johnson, A4 SR11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR11

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, two sequences a = (a1,a2,...,an) and b = (b1,b2,...,bn) of strings from Σ*, and a positive integer K ≤ n.
QUESTION: Is there a sequence i1,i2,...,ik of k ≤ K (not necessarily distinct) positive integers, each between 1 and n, such that the two strings ai1 ai2 ··· aik and bi1 bi2 ··· bik are identical?

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

INSTANCE: Finite alphabet Σ, two sequences a = (a1,a2,...,an) and b = (b1,b2,...,bn) of strings from Σ*, and a positive integer K ≤ n.
QUESTION: Is there a sequence i1,i2,...,ik of k ≤ K (not necessarily distinct) positive integers, each between 1 and n, such that the two strings ai1 ai2 ··· aik and bi1 bi2 ··· bik are identical?
Reference: [Constable, Hunt, and Sahni, 1974]. Generic transformation.
Comment: Problem is undecidable if no upper bound is placed on k, e.g., see [Hopcroft and Ullman, 1969].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
