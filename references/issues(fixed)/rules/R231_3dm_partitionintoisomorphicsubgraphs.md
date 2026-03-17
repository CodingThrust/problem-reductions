---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3DM to PARTITION INTO ISOMORPHIC SUBGRAPHS"
labels: rule
assignees: ''
---

**Source:** 3DM
**Target:** PARTITION INTO ISOMORPHIC SUBGRAPHS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT12

## GJ Source Entry

> [GT12] PARTITION INTO ISOMORPHIC SUBGRAPHS
> INSTANCE: Graphs G = (V,E) and H = (V',E') with |V| = q|V'| for some q ∈ Z+.
> QUESTION: Can the vertices of G be partitioned into q disjoint sets V_1, V_2, . . . , V_q such that, for 1 ≤ i ≤ q, the subgraph of G induced by V_i is isomorphic to H?
> Reference: [Kirkpatrick and Hell, 1978]. Transformation from 3DM.
> Comment: Remains NP-complete for any fixed H that contains at least 3 vertices. The analogous problem in which the subgraph induced by V_i need only have the same number of vertices as H and contain a subgraph isomorphic to H is also NP-complete, for any fixed H that contains a connected component of three or more vertices. Both problems can be solved in polynomial time (by matching) for any H not meeting the stated restrictions.

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

- **[Kirkpatrick and Hell, 1978]**: [`Kirkpatrick1978`] David G. Kirkpatrick and Peter Hell (1978). "On the complexity of a generalized matching problem". In: *Proceedings of the 10th Annual ACM Symposium on Theory of Computing*, pp. 240–245. Association for Computing Machinery.