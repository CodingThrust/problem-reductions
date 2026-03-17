---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-DIMENSIONAL MATCHING to MINIMUM BROADCAST TIME"
labels: rule
assignees: ''
---

**Source:** 3-DIMENSIONAL MATCHING
**Target:** MINIMUM BROADCAST TIME
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND49, p.219

## GJ Source Entry

> [ND49] MINIMUM BROADCAST TIME
> INSTANCE: Graph G=(V,E), subset V_0⊆V, and a positive integer K.
> QUESTION: Can a message be "broadcast" from the base set V_0 to all other vertices in time K, i.e., is there a sequence V_0,E_1,V_1,E_2,…,E_K,V_K such that each V_i⊆V, each E_i⊆E, V_K=V, and, for 1≤i≤K, (1) each edge in E_i has exactly one endpoint in V_{i−1}, (2) no two edges in E_i share a common endpoint, and (3) V_i=V_{i−1}∪{v: {u,v}∈E_i}?
> Reference: [Garey and Johnson, ——]. Transformation from 3DM. For more on this problem, see [Farley, Hedetniemi, Mitchell, and Proskurowski, 1977].
> Comment: Remains NP-complete for any fixed K≥4, but is solvable in polynomial time by matching if K=1. The special case where |V_0|=1 remains NP-complete, but is solvable in polynomial time for trees [Cockayne, Hedetniemi, and Slater, 1978].

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
- **[Farley, Hedetniemi, Mitchell, and Proskurowski, 1977]**: [`Farley1977`] A. Farley and S. Hedetniemi and S. Mitchell and A. Proskurowski (1977). "Minimum broadcast graphs". Dept. of Computer Science, University of Oregon.
- **[Cockayne, Hedetniemi, and Slater, 1978]**: [`Cockayne1978`] E. J. Cockayne and S. T. Hedetniemi and P. J. Slater (1978). "".