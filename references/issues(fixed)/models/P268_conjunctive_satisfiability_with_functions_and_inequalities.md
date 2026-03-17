---
name: Problem
about: Propose a new problem type
title: "[Model] ConjunctiveSatisfiabilityWithFunctionsAndInequalities"
labels: model
assignees: ''
---

## Motivation

CONJUNCTIVE SATISFIABILITY WITH FUNCTIONS AND INEQUALITIES (P268) from Garey & Johnson, A9 LO16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO16

**Mathematical definition:**

INSTANCE: Set U of variables, set F of univariate function symbols, and a collection C of "clauses" of the form U*V where * is either "≤," ">," "=," or "≠," and U and V are either "0," "1," "u," "f(0)," "f(1)," or "f(u)," for some f∈F and u∈U.
QUESTION: Is there an assignment of integer values to all the variables u∈U and to all f(u), for u∈U and f∈F, such that all the clauses in C are satisfied under the usual interpretations of ≤, >, =, and ≠?

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

INSTANCE: Set U of variables, set F of univariate function symbols, and a collection C of "clauses" of the form U*V where * is either "≤," ">," "=," or "≠," and U and V are either "0," "1," "u," "f(0)," "f(1)," or "f(u)," for some f∈F and u∈U.
QUESTION: Is there an assignment of integer values to all the variables u∈U and to all f(u), for u∈U and f∈F, such that all the clauses in C are satisfied under the usual interpretations of ≤, >, =, and ≠?
Reference: [Pratt, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete even if = and ≠ are not used. Solvable in polynomial time if ≤ and > are not used [Nelson and Oppen, 1977], or if = and ≠ are not used and no function symbols are allowed [Litvintchouk and Pratt, 1977]. Variant in which W and V are either of the form "u" or "u+c" for some u∈U and c∈Z is NP-complete if all four relations are allowed, but solvable in polynomial time if only ≤ and > or only = and ≠ are allowed [Chan, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
