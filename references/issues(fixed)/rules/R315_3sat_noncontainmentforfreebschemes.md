---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NON-CONTAINMENT FOR FREE B-SCHEMES"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** NON-CONTAINMENT FOR FREE B-SCHEMES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO18

## GJ Source Entry

> [PO18]  NON-CONTAINMENT FOR FREE B-SCHEMES
> INSTANCE:  Two free B-schemes S1 and S2, where a free B-scheme is a rooted, directed acyclic graph G = (V,A), all of whose vertices have out-degree 0 (leaves) or 2 (tests), with the two arcs leaving a test vertex labeled L and R respectively, together with a set B of Boolean variable symbols and a label l(v) ∈ B for each test vertex, such that no two test vertices on the same directed path get the same label, and a set F of function symbols along with a label l(v) ∈ F ∪ {Ω} for each leaf in V.
> QUESTION:  Is S1 not "contained" in S2, i.e., is there an assignment t: B1 ∪ B2→{L,R} such that if the paths from the roots of G1 and G2 to leaf vertices determined by always leaving a test vertex v by the arc labeled t(l(v)) terminate at leaves labeled f1 and f2 respectively, then f1 ≠ f2 and f1 ≠ Ω?
> Reference:  [Fortune, Hopcroft, and Schmidt, 1977]. Transformation from 3SAT.
> Comment:  The "strong inequivalence" problem for free B-schemes (same as above, only all that we now require is that f1 ≠ f2) is open, but can be solved in polynomial time if one of S1 and S2 is an "ordered" B-scheme.  The open version is Turing equivalent to the strong inequivalence problem for free Ianov schemes (see STRONG INEQUIVALENCE OF IANOV SCHEMES).

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

- **[Fortune, Hopcroft, and Schmidt, 1977]**: [`Fortune1977`] S. Fortune and J. E. Hopcroft and E. M. Schmidt (1977). "The complexity of equivalence and containment for free single variable program schemes". Dept. of Computer Science, Cornell University.