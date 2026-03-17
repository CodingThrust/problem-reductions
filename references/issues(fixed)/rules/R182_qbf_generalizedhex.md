---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to GENERALIZED HEX"
labels: rule
assignees: ''
---

**Source:** QBF
**Target:** GENERALIZED HEX
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.254

## GJ Source Entry

> [GP1] GENERALIZED HEX (*)
> INSTANCE: Graph G = (V,E) and two specified vertices s, t E V.
> QUESTION: Does player 1 have a forced win in the following game played on G? The players alternate choosing a vertex from V - {s,t}, with those chosen by player 1 being colored "blue" and those chosen by player 2 being colored "red." Play continues until all such vertices have been colored, and player 1 wins if and only if there is a path from s to t in G that passes through only blue vertices.
> Reference: [Even and Tarjan, 1976]. Transformation from QBF.
> Comment: PSPACE-complete. The variant in which players alternate choosing an edge instead of a vertex, known as "the Shannon switching game on edges," can be solved in polynomial time [Bruno and Weinberg, 1970]. If G is a directed graph and player 1 wants a "blue" directed path from s to t, both the vertex selection game and the arc selection game are PSPACE-complete [Even and Tarjan, 1976].

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

- **[Even and Tarjan, 1976]**: [`Even1976b`] S. Even and R. E. Tarjan (1976). "A combinatorial problem which is complete in polynomial space". *Journal of the Association for Computing Machinery* 23, pp. 710–719.
- **[Bruno and Weinberg, 1970]**: [`Bruno1970`] J. Bruno and L. Weinberg (1970). "A constructive graph-theoretic solution of the {Shannon} switching game". *IEEE Transactions on Circuit Theory* CT-17, pp. 74–81.