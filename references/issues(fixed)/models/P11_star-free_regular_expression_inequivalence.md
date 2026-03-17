---
name: Problem
about: Propose a new problem type
title: "[Model] StarFreeRegularExpressionInequivalence"
labels: model
assignees: ''
---

## Motivation

STAR-FREE REGULAR EXPRESSION INEQUIVALENCE (P11) from Garey & Johnson, Chapter 3, Section 3.3, p.76. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.3, p.76

**Mathematical definition:**

INSTANCE: Two star-free regular expressions E_1 and E_2 over a finite alphabet Σ, where such expressions are defined by (1) any single symbol σ ∈ Σ is a star-free regular expression, and (2) if e_1 and e_2 are star-free regular expressions, then the strings e_1 e_2 and (e_1 ∨ e_2) are star-free regular expressions.
QUESTION: Do E_1 and E_2 represent different languages over Σ, where the language represented by σ ∈ Σ is {σ}, and, if e_1 and e_2 represent the languages L_1 and L_2 respectively, then e_1 e_2 represents the language {xy: x ∈ L_1 and y ∈ L_2} and (e_1 ∨ e_2) represents the language L_1 ∪ L_2 ?

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

INSTANCE: Two star-free regular expressions E_1 and E_2 over a finite alphabet Σ, where such expressions are defined by (1) any single symbol σ ∈ Σ is a star-free regular expression, and (2) if e_1 and e_2 are star-free regular expressions, then the strings e_1 e_2 and (e_1 ∨ e_2) are star-free regular expressions.
QUESTION: Do E_1 and E_2 represent different languages over Σ, where the language represented by σ ∈ Σ is {σ}, and, if e_1 and e_2 represent the languages L_1 and L_2 respectively, then e_1 e_2 represents the language {xy: x ∈ L_1 and y ∈ L_2} and (e_1 ∨ e_2) represents the language L_1 ∪ L_2 ?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
