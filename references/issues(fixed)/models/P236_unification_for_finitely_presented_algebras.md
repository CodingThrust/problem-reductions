---
name: Problem
about: Propose a new problem type
title: "[Model] UnificationForFinitelyPresentedAlgebras"
labels: model
assignees: ''
---

## Motivation

UNIFICATION FOR FINITELY PRESENTED ALGEBRAS (P236) from Garey & Johnson, A7 AN17. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN17

**Mathematical definition:**

INSTANCE: Finite presentation of an algebra A in terms of a set G of generators, a collection O of operators of various finite dimensions, and a collection Γ of defining relations on well-formed formulas over G and O; two well-formed expressions e and f over G, O, and a variable set V (see reference for details).
QUESTION: Is there an assignment to each v ∈ V of a unique "term" I(v) over G and O such that, if I(e) and I(f) denote the expressions obtained by replacing all variables in e and f by their corresponding terms, then I(e) and I(f) represent the same element in A?

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

INSTANCE: Finite presentation of an algebra A in terms of a set G of generators, a collection O of operators of various finite dimensions, and a collection Γ of defining relations on well-formed formulas over G and O; two well-formed expressions e and f over G, O, and a variable set V (see reference for details).
QUESTION: Is there an assignment to each v ∈ V of a unique "term" I(v) over G and O such that, if I(e) and I(f) denote the expressions obtained by replacing all variables in e and f by their corresponding terms, then I(e) and I(f) represent the same element in A?

Reference: [Kozen, 1977a], [Kozen, 1976]. Transformation from 3SAT. Proof of membership in NP is non-trivial and appears in the second reference.
Comment: Remains NP-complete if only one of e and f contains variable symbols, but is solvable in polynomial time if neither contains variable symbols. See [Kozen, 1977b] for quantified versions of this problem that are complete for PSPACE and for the various levels of the polynomial hierarchy.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
