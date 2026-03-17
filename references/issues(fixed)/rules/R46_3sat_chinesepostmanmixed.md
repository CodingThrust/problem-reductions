---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CHINESE POSTMAN FOR MIXED GRAPHS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** CHINESE POSTMAN FOR MIXED GRAPHS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND25, p.212

## GJ Source Entry

> [ND25] CHINESE POSTMAN FOR MIXED GRAPHS
> INSTANCE: Mixed graph G=(V,A,E), where A is a set of directed edges and E is a set of undirected edges on V, length l(e)∈Z_0^+ for each e∈A∪E, bound B∈Z^+.
> QUESTION: Is there a cycle in G that includes each directed and undirected edge at least once, traversing directed edges only in the specified direction, and that has total length no more than B?
> Reference: [Papadimitriou, 1976b]. Transformation from 3SAT.
> Comment: Remains NP-complete even if all edge lengths are equal, G is planar, and the maximum vertex degree is 3. Can be solved in polynomial time if either A or E is empty (i.e., if G is either a directed or an undirected graph) [Edmonds and Johnson, 1973].

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

- **[Papadimitriou, 1976b]**: [`Papadimitriou1976b`] Christos H. Papadimitriou (1976). "On the complexity of edge traversing". *Journal of the Association for Computing Machinery* 23, pp. 544–554.
- **[Edmonds and Johnson, 1973]**: [`Edmonds1973`] J. Edmonds and E. L. Johnson (1973). "Matching, {Euler} tours, and the {Chinese} postman". *Mathematical Programming* 5, pp. 88–124.