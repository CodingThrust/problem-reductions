---
name: Problem
about: Propose a new problem type
title: "[Model] RegularExpressionSubstitution"
labels: model
assignees: ''
---

## Motivation

REGULAR EXPRESSION SUBSTITUTION (P172) from Garey & Johnson, A4 SR24. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR24

**Mathematical definition:**

INSTANCE: Two finite alphabets X = {x1,x2,...,xn} and Y = {y1,y2,...,ym}, a regular expression R over X ∪ Y, regular expressions R1,R2,...,Rn over Y, and a string w ∈ Y*.
QUESTION: Is there a string z in the language determined by R and for each i, 1 ≤ i ≤ n, a string wi in the language determined by Ri such that, if each string wi is substituted for every occurrence of the symbol xi in z, then the resulting string is identical to w?

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

INSTANCE: Two finite alphabets X = {x1,x2,...,xn} and Y = {y1,y2,...,ym}, a regular expression R over X ∪ Y, regular expressions R1,R2,...,Rn over Y, and a string w ∈ Y*.
QUESTION: Is there a string z in the language determined by R and for each i, 1 ≤ i ≤ n, a string wi in the language determined by Ri such that, if each string wi is substituted for every occurrence of the symbol xi in z, then the resulting string is identical to w?
Reference: [Aho and Ullman, 1977]. Transformation from X3C.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
