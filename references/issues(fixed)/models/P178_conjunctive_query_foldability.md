---
name: Problem
about: Propose a new problem type
title: "[Model] ConjunctiveQueryFoldability"
labels: model
assignees: ''
---

## Motivation

CONJUNCTIVE QUERY FOLDABILITY (P178) from Garey & Johnson, A4 SR30. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR30

**Mathematical definition:**

INSTANCE: Finite domain set D, a collection R = {R1,R2,...,Rm} of relations, where each Ri consists of a set of di-tuples with entries from D, a set X of distinguished variables, a set Y of undistinguished variables, and two "queries" Q1 and Q2 over X,Y,D, and R, where a query Q has the form
(x1,x2,...,xk)(∃y1,y2,...,yl)(A1 ∧ A2 ∧ · · · ∧ Ar)
for some k,l, and r, with X' = {x1,x2,...,xk} ⊆ X, Y' = {y1,y2,...,yl} ⊆ Y, and each Ai of the form Rj(u1,u2,...,udj) with each u ∈ D ∪ X' ∪ Y' (see reference for interpretation of such expressions in terms of data bases).
QUESTION: Is there a function σ: Y → X ∪ Y ∪ D such that, if for each y ∈ Y the symbol σ(y) is substituted for every occurrence of y in Q1, then the result is query Q2?

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

INSTANCE: Finite domain set D, a collection R = {R1,R2,...,Rm} of relations, where each Ri consists of a set of di-tuples with entries from D, a set X of distinguished variables, a set Y of undistinguished variables, and two "queries" Q1 and Q2 over X,Y,D, and R, where a query Q has the form
(x1,x2,...,xk)(∃y1,y2,...,yl)(A1 ∧ A2 ∧ · · · ∧ Ar)
for some k,l, and r, with X' = {x1,x2,...,xk} ⊆ X, Y' = {y1,y2,...,yl} ⊆ Y, and each Ai of the form Rj(u1,u2,...,udj) with each u ∈ D ∪ X' ∪ Y' (see reference for interpretation of such expressions in terms of data bases).
QUESTION: Is there a function σ: Y → X ∪ Y ∪ D such that, if for each y ∈ Y the symbol σ(y) is substituted for every occurrence of y in Q1, then the result is query Q2?
Reference: [Chandra and Merlin, 1977]. Transformation from GRAPH 3-COLORABILITY.
Comment: The isomorphism problem for conjunctive queries (with two queries being isomorphic if they are the same up to one-to-one renaming of the variables, reordering of conjuncts, and reordering within quantifications) is polynomially equivalent to graph isomorphism.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
