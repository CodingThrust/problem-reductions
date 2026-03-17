---
name: Problem
about: Propose a new problem type
title: "[Model] StrongInequivalenceOfIanovSchemes"
labels: model
assignees: ''
---

## Motivation

STRONG INEQUIVALENCE OF IANOV SCHEMES (P308) from Garey & Johnson, A11 PO16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO16

**Mathematical definition:**

INSTANCE: Finite sets F and P of function and predicate symbols, single variable x, and two Ianov schemes over F,P, and x, each a sequence I1,I2,...,Im of instructions of the form "x ← f(x)," "if p(x) then goto Ij else goto Ik," and "halt," where f ∈ F and p ∈ P.
QUESTION: Are the two given Ianov schemes not strongly equivalent, i.e., is there a domain set D, an interpretation of each f ∈ F as a function f: D→D, an interpretation of each p ∈ P as a function p: D→{T,F}, and an initial value x0 ∈ D for x, such that either both schemes halt with different final values for x or one halts and the other doesn't?

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

INSTANCE: Finite sets F and P of function and predicate symbols, single variable x, and two Ianov schemes over F,P, and x, each a sequence I1,I2,...,Im of instructions of the form "x ← f(x)," "if p(x) then goto Ij else goto Ik," and "halt," where f ∈ F and p ∈ P.
QUESTION: Are the two given Ianov schemes not strongly equivalent, i.e., is there a domain set D, an interpretation of each f ∈ F as a function f: D→D, an interpretation of each p ∈ P as a function p: D→{T,F}, and an initial value x0 ∈ D for x, such that either both schemes halt with different final values for x or one halts and the other doesn't?
Reference: [Constable, Hunt, and Sahni, 1974], [Rutledge, 1964]. Transformation from 3SAT. Membership in NP follows from the second reference.
Comment: Remains NP-complete even if neither program contains any loops and P2 is the trivial program that leaves the value of x unchanged. The strong inequivalence problem for Ianov schemes with two variables is undecidable, even if |F| = |P| = 1 [Luckham, Park, and Paterson, 1970]. See references, [Hunt, 1978], and [Hunt and Szymanski, 1976b] for analogous results for other properties, such as "weak equivalence," "divergence," "halting," etc. Strong equivalence can be tested in polynomial time for Ianov schemes that are "strongly free," i.e., in which at least one function application occurs between every two successive predicate tests [Constable, Hunt, and Sahni, 1974]. Strong equivalence is open for "free" Ianov schemes.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
