---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to HAMILTONIAN COMPLETION"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** HAMILTONIAN COMPLETION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT34

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), non-negative integer K ≤ |V|.
> QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') has a Hamiltonian circuit?
>
> Reference: Transformation from HAMILTONIAN CIRCUIT.
>
> Comment: Remains NP-complete for any fixed K ≥ 0. Corresponding "completion" versions of HAMILTONIAN PATH, DIRECTED HAMILTONIAN PATH, and DIRECTED HAMILTONIAN CIRCUIT are also NP-complete. HAMILTONIAN COMPLETION and HAMILTONIAN PATH COMPLETION can be solved in polynomial time if G is a tree [Boesch, Chen, and McHugh, 1974].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Boesch, Chen, and McHugh, 1974]**: [`Boesch1974`] F. T. Boesch and S. Chen and J. A. M. McHugh (1974). "On covering the points of a graph with point disjoint paths". In: *Graphs and Combinatorics*. Springer.