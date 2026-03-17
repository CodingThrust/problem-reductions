---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PARTITION INTO HAMILTONIAN SUBGRAPHS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** PARTITION INTO HAMILTONIAN SUBGRAPHS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT13

## GJ Source Entry

> [GT13] PARTITION INTO HAMILTONIAN SUBGRAPHS
> INSTANCE: Directed graph G = (V,A).
> QUESTION: Can the vertices of G be partitioned into disjoint sets V_1, V_2, . . . , V_k, for some k, such that each V_i contains at least three vertices and induces a subgraph of G that contains a Hamiltonian circuit?
> Reference: [Valiant, 1977a]. Transformation from 3SAT. (See also [Herrmann, 1973]).
> Comment: Solvable in polynomial time by matching techniques if each V_i need only contain at least 2 vertices [Edmonds and Johnson, 1970]. The analogous problem for undirected graphs can be similarly solved, even with the requirement that |V_i| ≥ 3. However, it becomes NP-complete if we require that |V_i| ≥ 6 [Papadimitriou, 1978d] or if the instance includes an upper bound K on k.

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

- **[Valiant, 1977a]**: [`Valiant1977a`] Leslie G. Valiant (1977). "The complexity of computing the permanent". Computer Science Department, University of Edinburgh.
- **[Herrmann, 1973]**: [`Herrmann1973`] P. P. Herrmann (1973). "On reducibility among combinatorial problems". Project MAC, Massachusetts Institute of Technology.
- **[Edmonds and Johnson, 1970]**: [`Edmonds1970`] J. Edmonds and E. L. Johnson (1970). "Matching: a well-solved class of integer linear programs". In: *Combinatorial Structures and their Applications*. Gordon and Breach.
- **[Papadimitriou, 1978d]**: [`Papadimitriou1978d`] Christos H. Papadimitriou (1978). "".