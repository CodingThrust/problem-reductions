---
name: Problem
about: Propose a new problem type
title: "[Model] ConjunctiveBooleanQuery"
labels: model
assignees: ''
---

## Motivation

CONJUNCTIVE BOOLEAN QUERY (P179) from Garey & Johnson, A4 SR31. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR31

**Mathematical definition:**

INSTANCE: Finite domain set D, a collection R = {R1,R2,...,Rm} of relations, where each Ri consists of a set of di-tuples with entries from D, and a conjunctive Boolean query Q over R and D, where such a query Q is of the form
(∃y1,y2,...,yl)(A1 ∧ A2 ∧ · · · ∧ Ar)
with each Ai of the form Rj(u1,u2,...,udj) where each u ∈ {y1,y2,...,yl} ∪ D.
QUESTION: Is Q, when interpreted as a statement about R and D, true?

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

INSTANCE: Finite domain set D, a collection R = {R1,R2,...,Rm} of relations, where each Ri consists of a set of di-tuples with entries from D, and a conjunctive Boolean query Q over R and D, where such a query Q is of the form
(∃y1,y2,...,yl)(A1 ∧ A2 ∧ · · · ∧ Ar)
with each Ai of the form Rj(u1,u2,...,udj) where each u ∈ {y1,y2,...,yl} ∪ D.
QUESTION: Is Q, when interpreted as a statement about R and D, true?
Reference: [Chandra and Merlin, 1977]. Transformation from CLIQUE.
Comment: If we are allowed to replace the conjunctive query Q by an arbitrary first-order sentence involving the predicates in R, then the problem becomes PSPACE-complete, even for D = {0,1}.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
