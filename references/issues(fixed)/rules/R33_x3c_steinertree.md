---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to STEINER TREE IN GRAPHS"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** STEINER TREE IN GRAPHS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND12, p.208

## GJ Source Entry

> [ND12] STEINER TREE IN GRAPHS
> INSTANCE: Graph G=(V,E), a weight w(e)∈Z_0^+ for each e∈E, a subset R⊆V, and a positive integer bound B.
> QUESTION: Is there a subtree of G that includes all the vertices of R and such that the sum of the weights of the edges in the subtree is no more than B?
> Reference: [Karp, 1972]. Transformation from EXACT COVER BY 3-SETS.
> Comment: Remains NP-complete if all edge weights are equal, even if G is a bipartite graph having no edges joining two vertices in R or two vertices in V−R [Berlekamp, 1976] or G is planar [Garey and Johnson, 1977a].

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

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Berlekamp, 1976]**: [`Berlekamp1976`] E. R. Berlekamp (1976). "".
- **[Garey and Johnson, 1977a]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826–834.