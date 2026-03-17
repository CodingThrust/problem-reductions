---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SUBSET SUM to K-th LARGEST SUBSET"
labels: rule
assignees: ''
---

**Source:** SUBSET SUM
**Target:** K-th LARGEST SUBSET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP20, p.225

## GJ Source Entry

> [SP20] K^th LARGEST SUBSET (*)
> INSTANCE: Finite set A, size s(a)∈Z^+ for each a∈A, positive integers K and B.
> QUESTION: Are there K or more distinct subsets A'⊆A for which the sum of the sizes of the elements in A' does not exceed B?
> Reference: [Johnson and Kashdan, 1976]. Transformation from SUBSET SUM.
> Comment: Not known to be in NP. Solvable in pseudo-polynomial time (polynomial in K, |A|, and log Σ s(a)) [Lawler, 1972]. The corresponding enumeration problem is #P-complete.

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

- **[Johnson and Kashdan, 1976]**: [`Johnson1976a`] David B. Johnson and S. D. Kashdan (1976). "Lower bounds for selection in $X+Y$ and other multisets". Computer Science Department, Pennsylvania State University.
- **[Lawler, 1972]**: [`Lawler1972`] Eugene L. Lawler (1972). "A procedure for computing the {$K$} best solutions to discrete optimization problems and its application to the shortest path problem". *Management Science* 18, pp. 401–405.