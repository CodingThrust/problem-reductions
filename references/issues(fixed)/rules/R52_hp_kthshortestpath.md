---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to K-th SHORTEST PATH"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN PATH
**Target:** K-th SHORTEST PATH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND31, p.214

## GJ Source Entry

> [ND31] K^th SHORTEST PATH (*)
> INSTANCE: Graph G=(V,E), length l(e)∈Z^+ for each e∈E, specified vertices s,t∈V, positive integers B and K.
> QUESTION: Are there K or more distinct simple paths from s to t in G, each having total length B or less?
> Reference: [Johnson and Kashdan, 1976]. Turing reduction from HAMILTONIAN PATH.
> Comment: Not known to be in NP. Corresponding K^th shortest circuit problem is also NP-hard. Both remain NP-hard if l(e)=1 for all e∈E, as do the corresponding problems for directed graphs. However, all versions can be solved in pseudo-polynomial time (polynomial in |V|, K, and log B) and hence in polynomial time for any fixed value of K. The corresponding enumeration problems are #P-complete.

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