---
name: Rule
about: Propose a new reduction rule
title: "[Rule] STRONG INEQUIVALENCE OF IANOV SCHEMES to STRONG INEQUIVALENCE FOR MONADIC RECURSION SCHEMES"
labels: rule
assignees: ''
---

**Source:** STRONG INEQUIVALENCE OF IANOV SCHEMES
**Target:** STRONG INEQUIVALENCE FOR MONADIC RECURSION SCHEMES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO17

## GJ Source Entry

> [PO17]  STRONG INEQUIVALENCE FOR MONADIC RECURSION SCHEMES
> INSTANCE:  Finite sets F and P of function and predicate symbols, set G of "defined" function symbols disjoint from F, specified symbol f0 ∈ G, and two linear monadic recursion schemes S1 and S2, each consisting of a defining statement for each f ∈ G of the form "fx = if px then αx else βx" where p ∈ P, α,β ∈ (F∪G)*, and α and β each contain at most one occurrence of a symbol from G.
> QUESTION:  Is there a domain set D, an interpretation of each f ∈ F as a function f: D→D, an interpretation of each p ∈ P as a function P: D→{T,F}, and an initial value x0 ∈ D such that, as defined by the recursion schemes S1 and S2, either the two values for f0(x0) differ or one is defined and the other isn't?
> Reference:  [Constable, Hunt, and Sahni, 1974]. Transformation from STRONG INEQUIVALENCE OF IANOV SCHEMES.  Proof of membership in NP is non-trivial.
> Comment:  Remains NP-complete even if one scheme trivially sets f0(x) = x and the other is "right linear," i.e., each α and β only contains a defined symbol as its rightmost character.  See reference for other NP-completeness and NP-hardness results concerning linear monadic recursion schemes.

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Constable, Hunt, and Sahni, 1974]**: [`Constable1974`] R. L. Constable and H. B. Hunt, III and S. Sahni (1974). "On the computational complexity of scheme equivalence". Cornell University.