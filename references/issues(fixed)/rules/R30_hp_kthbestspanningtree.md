---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to KTH BEST SPANNING TREE"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN PATH
**Target:** KTH BEST SPANNING TREE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND9, p.208

## GJ Source Entry

> [ND9] K^th BEST SPANNING TREE (*)
> INSTANCE: Graph G=(V,E), weight w(e)∈Z_0^+ for each e∈E, positive integers K and B.
> QUESTION: Are there K distinct spanning trees for G, each having total weight B or less?
> Reference: [Johnson and Kashdan, 1976]. Turing reduction from HAMILTONIAN PATH.
> Comment: Not known to be in NP. Can be solved in pseudo-polynomial time (polynomial in |V|, K, log B, max{log w(e): e∈E}) [Lawler, 1972], and hence in polynomial time for any fixed value of K. The corresponding enumeration problem is #P-complete. However, the unweighted case of the enumeration problem is solvable in polynomial time (e.g., see [Harary and Palmer, 1973]).

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
- **[Harary and Palmer, 1973]**: [`Harary1973`] F. Harary and E. M. Palmer (1973). "Graphical Enumeration". Academic Press, New York.