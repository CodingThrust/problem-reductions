---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to DEGREE-BOUNDED CONNECTED SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN PATH
**Target:** DEGREE-BOUNDED CONNECTED SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT26

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), non-negative integer d ≤ |V|, positive integer K ≤ |E|.
> QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that the subgraph G' = (V,E') is connected and has no vertex with degree exceeding d?
>
> Reference: [Yannakakis, 1978b]. Transformation from HAMILTONIAN PATH.
> Comment: Remains NP-complete for any fixed d ≥ 2. Solvable in polynomial time if G' is not required to be connected (by matching techniques, see [Edmonds and Johnson, 1970]). The corresponding induced subgraph problem, where we ask for a subset V' ⊆ V with |V'| ≥ K such that the subgraph of G induced by V' has no vertex with degree exceeding d, is NP-complete for any fixed d ≥ 0 [Lewis, 1976] and for any fixed d ≥ 2 if we require that G' be connected [Yannakakis, 1978b].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.
- **[Edmonds and Johnson, 1970]**: [`Edmonds1970`] J. Edmonds and E. L. Johnson (1970). "Matching: a well-solved class of integer linear programs". In: *Combinatorial Structures and their Applications*. Gordon and Breach.
- **[Lewis, 1976]**: [`Lewis1976`] J. M. Lewis (1976). "".