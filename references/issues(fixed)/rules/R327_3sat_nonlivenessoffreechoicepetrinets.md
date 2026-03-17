---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NON-LIVENESS OF FREE CHOICE PETRI NETS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** NON-LIVENESS OF FREE CHOICE PETRI NETS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS3

## GJ Source Entry

> [MS3]  NON-LIVENESS OF FREE CHOICE PETRI NETS
> INSTANCE:  Petri net P = (n,M0,T), where n ∈ Z+, M0 is an n-tuple of non-negative integers, and T is a set of transitions <a,b> in which both a and b are n-tuples of 0's and 1's, such that P has the "free choice" property, i.e., for each <a,b> ∈ T, either a contains exactly one 1 or in every other transition <c,d> ∈ T, c has a 0 in every position where a has a 1.
> QUESTION:  Is P not "live," i.e., is there a transition t ∈ T and a sequence σ of transitions from T such that, for every sequence τ of transitions from T, the sequence στt is not "fireable" at M0, where <a1,b1> <a2,b2> ··· <am,bm> is fireable at M0 if and only if the sequence M0,M1,...,M2m in which M2i+1 = M2i−ai and M2i+2 = M2i+1 + bi, 0 ≤ i < m, contains no vector with a negative component?
> Reference:  [Jones, Landweber, and Lien, 1977]. Transformation from 3SAT. Proof of membership in NP is nontrivial and is based on a result of [Hack, 1972].

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
- **[Hack, 1972]**: [`Hack1972`] M. Hack (1972). "Analysis of production schemata by {Petri} nets". Project MAC, Massachusetts Institute of Technology.