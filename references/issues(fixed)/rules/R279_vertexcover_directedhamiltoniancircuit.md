---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to DIRECTED HAMILTONIAN CIRCUIT"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** DIRECTED HAMILTONIAN CIRCUIT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT38

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A).
> QUESTION: Does G contain a directed Hamiltonian circuit?
>
> Reference: [Karp, 1972]. Transformation from VERTEX COVER (see Chapter 3).
>
> Comment: Remains NP-complete if G is planar and has no vertex involved in more than three arcs [Plesnik, 1978]. Solvable in polynomial time if no in-degree (no out-degree) exceeds 1, if G is a tournament [Morrow and Goodman, 1976], or if G is an edge digraph (e.g., see [Liu, 1968]).

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Plesnik, 1978]**: [`Plesnik1978`] J. Plesn{\'i}k (1978). "The {NP}-completeness of the {Hamiltonian} cycle problem in planar digraphs with degree bound two".
- **[Morrow and Goodman, 1976]**: [`Morrow1976`] C. Morrow and S. Goodman (1976). "An efficient algorithm for finding a longest cycle in a tournament". In: *Proceedings of the 7th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 453–462. Utilitas Mathematica Publishing.
- **[Liu, 1968]**: [`Liu1968`] C. L. Liu (1968). "Introduction to Combinatorial Mathematics". McGraw-Hill, New York.