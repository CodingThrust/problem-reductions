---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumDisjunctiveNormalForm"
labels: model
assignees: ''
---

## Motivation

MINIMUM DISJUNCTIVE NORMAL FORM (P261) from Garey & Johnson, A9 LO9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO9

**Mathematical definition:**

INSTANCE: Set U={u_1,u_2,...,u_n} of variables, set A⊆{T,F}^n of "truth assignments," and a positive integer K.
QUESTION: Is there a disjunctive normal form expression E over U, having no more than K disjuncts, such that E is true for precisely those truth assignments in A, and no others?

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

INSTANCE: Set U={u_1,u_2,...,u_n} of variables, set A⊆{T,F}^n of "truth assignments," and a positive integer K.
QUESTION: Is there a disjunctive normal form expression E over U, having no more than K disjuncts, such that E is true for precisely those truth assignments in A, and no others?
Reference: [Gimpel, 1965]. Transformation from MINIMUM COVER.
Comment: Variant in which the instance contains a complete truth table, i.e., disjoint sets A and B⊆{T,F}^n such that A∪B={T,F}^n, and E must be true for all truth assignments in A and false for all those in B, is also NP-complete, despite the possibly much larger instance size [Masek, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
