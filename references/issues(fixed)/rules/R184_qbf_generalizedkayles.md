---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to GENERALIZED KAYLES"
labels: rule
assignees: ''
---

**Source:** QBF
**Target:** GENERALIZED KAYLES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.254

## GJ Source Entry

> [GP3] GENERALIZED KAYLES (*)
> INSTANCE: Graph G = (V,E).
> QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a vertex in the graph, removing that vertex and all vertices adjacent to it from the graph. Player 1 wins if and only if player 2 is the first player left with no vertices to choose from.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete. The variant in which G = (V_1 ∪ V_2, E) is bipartite, with each edge involving one vertex from V_1 and one from V_2, and player i can only choose vertices from the set V_i (but still removes all adjacent vertices as before) is also PSPACE-complete. For a description of the game Kayles upon which this generalization is based, see [Conway, 1976].

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

- **[Schaefer, 1978a]**: [`Schaefer1978a`] T. J. Schaefer (1978). "Complexity of some two-person perfect-information games". *Journal of Computer and System Sciences* 16, pp. 185–225.
- **[Conway, 1976]**: [`Conway1976`] J. H. Conway (1976). "On Numbers and Games". Academic Press, New York.