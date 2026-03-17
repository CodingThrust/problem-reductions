---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to HAMILTONIAN PATH"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** HAMILTONIAN PATH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT39

## Reduction Algorithm

> INSTANCE: Graph G = (V,E).
> QUESTION: Does G contain a Hamiltonian path?
>
> Reference: Transformation from VERTEX COVER (see Chapter 3).
>
> Comment: Remains NP-complete under restrictions (1) and (2) for HAMILTONIAN CIRCUIT and is polynomially solvable under the same restrictions as HC. Corresponding DIRECTED HAMILTONIAN PATH problem is also NP-complete, and the comments for DIRECTED HC apply to it as well. The variants in which either the starting point or the ending point or both are specified in the instance are also NP-complete. DIRECTED HAMILTONIAN PATH can be solved in polynomial time for acyclic digraphs, e.g., see [Lawler, 1976a].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.