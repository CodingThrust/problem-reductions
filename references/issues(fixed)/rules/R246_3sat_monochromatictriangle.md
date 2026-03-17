---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MONOCHROMATIC TRIANGLE"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** MONOCHROMATIC TRIANGLE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT6

## GJ Source Entry

> [GT6] MONOCHROMATIC TRIANGLE
> INSTANCE: Graph G = (V,E).
> QUESTION: Is there a partition of E into two disjoint sets E_1, E_2 such that neither G_1 = (V,E_1) nor G_2 = (V,E_2) contains a triangle?
> Reference: [Burr, 1976]. Transformation from 3SAT.
> Comment: Variants in which "triangle" is replaced by any larger fixed complete graph are also NP-complete [Burr, 1976]. Variants in which "triangle" is replaced by "k-star" (a single degree k vertex adjacent to k degree one vertices) is solvable in polynomial time [Burr, Erdös, and Lovasz, 1976].

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

- **[Burr, 1976]**: [`Burr1976a`] S. Burr (1976). "".
- **[Burr, Erdös, and Lovasz, 1976]**: [`Burr1976b`] S. Burr and P. Erd{\"o}s and L. Lov{\'a}sz (1976). "On graphs of {Ramsey} type". *Ars Combinatorica* 1, pp. 167–190.