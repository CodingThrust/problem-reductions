---
name: Problem
about: Propose a new problem type
title: "[Model] StrongInequivalenceForMonadicRecursionSchemes"
labels: model
assignees: ''
---

## Motivation

STRONG INEQUIVALENCE FOR MONADIC RECURSION SCHEMES (P309) from Garey & Johnson, A11 PO17. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO17

**Mathematical definition:**

INSTANCE: Finite sets F and P of function and predicate symbols, set G of "defined" function symbols disjoint from F, specified symbol f0 ∈ G, and two linear monadic recursion schemes S1 and S2, each consisting of a defining statement for each f ∈ G of the form "fx = if px then αx else βx" where p ∈ P, α,β ∈ (F ∪ G)*, and α and β each contain at most one occurrence of a symbol from G.
QUESTION: Is there a domain set D, an interpretation of each f ∈ F as a function f: D→D, an interpretation of each p ∈ P as a function P: D→{T,F}, and an initial value x0 ∈ D such that, as defined by the recursion schemes S1 and S2, either the two values for f0(x0) differ or one is defined and the other isn't?

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

INSTANCE: Finite sets F and P of function and predicate symbols, set G of "defined" function symbols disjoint from F, specified symbol f0 ∈ G, and two linear monadic recursion schemes S1 and S2, each consisting of a defining statement for each f ∈ G of the form "fx = if px then αx else βx" where p ∈ P, α,β ∈ (F ∪ G)*, and α and β each contain at most one occurrence of a symbol from G.
QUESTION: Is there a domain set D, an interpretation of each f ∈ F as a function f: D→D, an interpretation of each p ∈ P as a function P: D→{T,F}, and an initial value x0 ∈ D such that, as defined by the recursion schemes S1 and S2, either the two values for f0(x0) differ or one is defined and the other isn't?
Reference: [Constable, Hunt, and Sahni, 1974]. Transformation from STRONG INEQUIVALENCE OF IANOV SCHEMES. Proof of membership in NP is non-trivial.
Comment: Remains NP-complete even if one scheme trivially sets f0(x) = x and the other is "right linear," i.e., each α and β only contains a defined symbol as its rightmost character. See reference for other NP-completeness and NP-hardness results concerning linear monadic recursion schemes.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
