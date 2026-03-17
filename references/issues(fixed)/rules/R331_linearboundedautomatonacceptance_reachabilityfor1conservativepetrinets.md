---
name: Rule
about: Propose a new reduction rule
title: "[Rule] LINEAR BOUNDED AUTOMATON ACCEPTANCE to REACHABILITY FOR 1-CONSERVATIVE PETRI NETS"
labels: rule
assignees: ''
---

**Source:** LINEAR BOUNDED AUTOMATON ACCEPTANCE
**Target:** REACHABILITY FOR 1-CONSERVATIVE PETRI NETS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS4

## GJ Source Entry

> [MS4]  REACHABILITY FOR 1-CONSERVATIVE PETRI NETS (*)
> INSTANCE:  Petri net P = (n,M0,T) that is "1-conservative," i.e., for each <a,b> ∈ T, a and b have the same number of 1's, and an n-tuple M of nonnegative integers.
> QUESTION:  Is M reachable from M0 in P, i.e., is there a sequence <a1,b1> <a2,b2> ··· <am,bm> of transitions from T such that the sequence M0,M1,...,M2m obtained as in the preceding problem contains no vector with a negative component and satisfies M2m = M?
> Reference:  [Jones, Landweber, and Lien, 1977]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE.
> Comment:  PSPACE-complete, even if P is also a free choice Petri net. Problem is not known to be decidable for arbitrary Petri nets, but is known to require at least exponential space [Lipton, 1975]. Analogous results hold for the "coverability" problem: Is there an M' having each of its components no smaller than the corresponding component of M such that M' is reachable from M0? The related "K-boundedness" problem (given P and an integer K, is there no vector that exceeds K in every component that is reachable from M0?) is PSPACE-complete for arbitrary Petri nets, as well as for 1-conservative free choice Petri nets. See [Jones, Landweber, and Lien, 1977] and [Hunt, 1977] for additional details and related results.

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

- **[Jones, Landweber, and Lien, 1977]**: [`Jones1977a`] Neil D. Jones and L. H. Landweber and Y. Edmund Lien (1977). "Complexity of some problems in {Petri} nets". *Theoretical Computer Science* 4, pp. 277–299.
- **[Lipton, 1975]**: [`Lipton1975`] Richard J. Lipton (1975). "The reachability problem requires exponential space". Dept. of Computer Science, Yale University.
- **[Hunt, 1977]**: [`Hunt1977a`] Harry B. Hunt III (1977). "A complexity theory of computation structures: preliminary report".