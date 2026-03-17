---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to ACYCLIC PARTITION"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_note: 'X3C is a specialization of Set Covering. Implement general version first.'
---

# [Rule] X3C → ACYCLIC PARTITION

**Status:** SKIP_SPECIALIZATION

X3C (Exact Cover by 3-Sets) is a known specialization of Set Covering (each set has exactly 3 elements, and an exact cover is required). This reduction should be implemented after the general version is available in the codebase.

## Specialization Details

- **Specialized problem:** X3C (Exact Cover by 3-Sets)
- **General version:** Set Covering
- **Restriction:** Each set has exactly 3 elements; an exact cover (every element covered exactly once) is required

## Original Reference

**Reference:** Garey & Johnson, *Computers and Intractability*, ND15, p.209

> [ND15] ACYCLIC PARTITION
> INSTANCE: Directed graph G=(V,A), weight w(v)∈Z^+ for each v∈V, cost c(a)∈Z^+ for each a∈A, positive integers B and K.
> QUESTION: Is there a partition of V into disjoint sets V_1,V_2,...,V_m such that the directed graph G'=(V',A'), where V'={V_1,V_2,...,V_m}, and (V_i,V_j)∈A' if and only if (v_i,v_j)∈A for some v_i∈V_i and some v_j∈V_j, is acyclic, such that the sum of the weights of the vertices in each V_i does not exceed B, and such that the sum of the costs of all those arcs having their endpoints in different sets does not exceed K?
> Reference: [Garey and Johnson, ——]. Transformation from X3C.
> Comment: Remains NP-complete even if all v∈V have w(v)=1 and all a∈A have c(a)=1.

## References

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Kernighan, 1971]**: [`Kernighan1971`] Brian W. Kernighan (1971). "Optimal sequential partitions of graphs". *Journal of the Association for Computing Machinery* 18, pp. 34–40.
- **[Lukes, 1974]**: [`Lukes1974`] J. A. Lukes (1974). "Efficient algorithm for the partitioning of trees". *IBM Journal of Research and Development* 18, pp. 217–224.
- **[Hadlock, 1974]**: [`Hadlock1974`] F. O. Hadlock (1974). "Minimum spanning forests of bounded trees". In: *Proceedings of the 5th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 449–460. Utilitas Mathematica Publishing.
