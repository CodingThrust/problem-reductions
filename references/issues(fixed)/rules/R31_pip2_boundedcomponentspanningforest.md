---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION INTO PATHS OF LENGTH 2 to BOUNDED COMPONENT SPANNING FOREST"
labels: rule
assignees: ''
---

**Source:** PARTITION INTO PATHS OF LENGTH 2
**Target:** BOUNDED COMPONENT SPANNING FOREST
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND10, p.208

## GJ Source Entry

> [ND10] BOUNDED COMPONENT SPANNING FOREST
> INSTANCE: Graph G=(V,E), weight w(v)∈Z_0^+ for each v∈V, positive integers K≤|V| and B.
> QUESTION: Can the vertices in V be partitioned into k≤K disjoint sets V_1,V_2,...,V_k such that, for 1≤i≤k, the subgraph of G induced by V_i is connected and the sum of the weights of the vertices in V_i does not exceed B?
> Reference: [Hadlock, 1974]. Transformation from PARTITION INTO PATHS OF LENGTH 2.
> Comment: Remains NP-complete even if all weights equal 1 and B is any fixed integer larger than 2 [Garey and Johnson, ——]. Can be solved in polynomial time if G is a tree or if all weights equal 1 and B=2 [Hadlock, 1974].

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

- **[Hadlock, 1974]**: [`Hadlock1974`] F. O. Hadlock (1974). "Minimum spanning forests of bounded trees". In: *Proceedings of the 5th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 449–460. Utilitas Mathematica Publishing.
- **[Garey and Johnson, ——]**: *(not found in bibliography)*