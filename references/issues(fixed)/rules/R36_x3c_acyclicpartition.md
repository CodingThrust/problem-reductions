---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to ACYCLIC PARTITION"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** ACYCLIC PARTITION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND15, p.209

## GJ Source Entry

> [ND15] ACYCLIC PARTITION
> INSTANCE: Directed graph G=(V,A), weight w(v)∈Z^+ for each v∈V, cost c(a)∈Z^+ for each a∈A, positive integers B and K.
> QUESTION: Is there a partition of V into disjoint sets V_1,V_2,...,V_m such that the directed graph G'=(V',A'), where V'={V_1,V_2,...,V_m}, and (V_i,V_j)∈A' if and only if (v_i,v_j)∈A for some v_i∈V_i and some v_j∈V_j, is acyclic, such that the sum of the weights of the vertices in each V_i does not exceed B, and such that the sum of the costs of all those arcs having their endpoints in different sets does not exceed K?
> Reference: [Garey and Johnson, ——]. Transformation from X3C.
> Comment: Remains NP-complete even if all v∈V have w(v)=1 and all a∈A have c(a)=1. Can be solved in polynomial time if G contains a Hamiltonian path (a property that can be verified in polynomial time for acyclic digraphs) [Kernighan, 1971]. If G is a tree the general problem is NP-complete in the ordinary sense, but can be solved in pseudo-polynomial time [Lukes, 1974]. The tree problem can be solved in polynomial time if all edge weights are equal (see [Hadlock, 1974]) or if all vertex weights are equal [Garey and Johnson, ——].

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
- **[Kernighan, 1971]**: [`Kernighan1971`] Brian W. Kernighan (1971). "Optimal sequential partitions of graphs". *Journal of the Association for Computing Machinery* 18, pp. 34–40.
- **[Lukes, 1974]**: [`Lukes1974`] J. A. Lukes (1974). "Efficient algorithm for the partitioning of trees". *IBM Journal of Research and Development* 18, pp. 217–224.
- **[Hadlock, 1974]**: [`Hadlock1974`] F. O. Hadlock (1974). "Minimum spanning forests of bounded trees". In: *Proceedings of the 5th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 449–460. Utilitas Mathematica Publishing.