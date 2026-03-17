---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to GENERALIZED GEOGRAPHY"
labels: rule
assignees: ''
---

**Source:** QBF
**Target:** GENERALIZED GEOGRAPHY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.254

## GJ Source Entry

> [GP2] GENERALIZED GEOGRAPHY (*)
> INSTANCE: Directed graph G = (V,A) and a specified vertex v_0 E V.
> QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a new arc from A. The first arc chosen must have its tail at v_0 and each subsequently chosen arc must have its tail at the vertex that was the head of the previous arc. The first player unable to choose such a new arc loses.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete, even if G is bipartite, planar, and has no in- or out-degree exceeding 2 and no degree exceeding 3 (PLANAR GEOGRAPHY) [Lichtenstein and Sipser, 1978]. This game is a generalization of the "Geography" game in which players alternate choosing countries, each name beginning with the same letter that ends the previous country's name.

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
- **[Lichtenstein and Sipser, 1978]**: [`Lichtenstein1978`] David Lichtenstein and Michael Sipser (1978). "{GO} is {Pspace} hard". In: *Proceedings of the 19th Annual Symposium on Foundations of Computer Science*, pp. 48–54. IEEE Computer Society.