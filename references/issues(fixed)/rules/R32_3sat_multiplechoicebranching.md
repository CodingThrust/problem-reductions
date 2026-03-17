---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MULTIPLE CHOICE BRANCHING"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** MULTIPLE CHOICE BRANCHING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND11, p.208

## GJ Source Entry

> [ND11] MULTIPLE CHOICE BRANCHING
> INSTANCE: Directed graph G=(V,A), a weight w(a)∈Z^+ for each arc a∈A, a partition of A into disjoint sets A_1,A_2,...,A_m, and a positive integer K.
> QUESTION: Is there a subset A'⊆A with ∑_{a∈A'} w(a)≥K such that no two arcs in A' enter the same vertex, A' contains no cycles, and A' contains at most one arc from each of the A_i, 1≤i≤m?
> Reference: [Garey and Johnson, ——]. Transformation from 3SAT.
> Comment: Remains NP-complete even if G is strongly connected and all weights are equal. If all A_i have |A_i|=1, the problem becomes simply that of finding a "maximum weight branching," a 2-matroid intersection problem that can be solved in polynomial time (e.g., see [Tarjan, 1977]). (In a strongly connected graph, a maximum weight branching can be viewed as a maximum weight directed spanning tree.) Similarly, if the graph is symmetric, the problem becomes equivalent to the "multiple choice spanning tree" problem, another 2-matroid intersection problem that can be solved in polynomial time [Suurballe, 1975].

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

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Tarjan, 1977]**: [`Tarjan1977`] Robert E. Tarjan (1977). "Finding optimum branchings". *Networks* 7, pp. 25–35.
- **[Suurballe, 1975]**: [`Suurballe1975`] James W. Suurballe (1975). "Minimal spanning trees subject to disjoint arc set constraints".