---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to FAULT DETECTION WITH TEST POINTS"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** FAULT DETECTION WITH TEST POINTS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS19

## GJ Source Entry

> [MS19]  FAULT DETECTION WITH TEST POINTS
> INSTANCE:  Directed acyclic graph G = (V,A) having exactly one vertex s ∈ V with in-degree 0 and exactly one vertex t ∈ V with out-degree 0, and a positive integer K.
> QUESTION:  Can all "single faults" in G be located by attaching K or fewer "test points" to arcs in A, i.e., is there a subset A' ⊆ A with |A'| ≤ K such that the test set
> T = ({s} ∪ {u1: (u1,u2) ∈ A'}) × ({t} ∪ {u2: (u1,u2) ∈ A'})
> has the property that, for each pair v,v' ∈ V−{s,t}, there is some (u1,u2) ∈ T such that v is on a directed path from u1 to u2 but v' is on no such path?
> Reference:  [Ibaraki, Kameda, and Toida, 1977]. Transformation from X3C.
> Comment:  Variants in which we are asked to locate all single faults by using K or fewer "test connections" or "blocking gates" are also NP-complete, as are the problems of finding a test set T with |T| ≤ K in the presence of a fixed set of "test points," "test connections," or "blocking gates." See reference for more details.

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

- **[Ibaraki, Kameda, and Toida, 1977]**: [`Ibaraki1977`] Toshihide Ibaraki and T. Kameda and Shmuel Toida (1977). "{NP}-complete diagnosis problems on systems graphs".