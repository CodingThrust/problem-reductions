---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to INDEPENDENT SET"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** INDEPENDENT SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT20

## GJ Source Entry

> [GT20]  INDEPENDENT SET
> INSTANCE:  Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION:  Does G contain an independent set of size K or more, i.e., a subset V' ⊆ V such that |V'| ≥ K and such that no two vertices in V' are joined by an edge in E?
>
> Reference:  Transformation from VERTEX COVER (see Chapter 3).
> Comment:  Remains NP-complete for cubic planar graphs [Garey, Johnson, and Stockmeyer, 1976], [Garey and Johnson, 1977a], [Maier and Storer, 1977], for edge graphs of directed graphs [Gavril, 1977a], for total graphs of bipartite graphs [Yannakakis and Gavril, 1978], and for graphs containing no triangles [Poljak, 1974]. Solvable in polynomial time for bipartite graphs (by matching, e.g., see [Harary, 1969]), for edge graphs (by matching), for graphs with no vertex degree exceeding 2, for chordal graphs [Gavril, 1972], for circle graphs [Gavril, 1973], for circular arc graphs (given their representation as families of arcs) [Gavril, 1974a], for comparability graphs [Golumbic, 1977], and for claw-free graphs [Minty, 1977].

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

- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
- **[Garey and Johnson, 1977a]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826–834.
- **[Maier and Storer, 1977]**: [`Maier1977a`] David Maier and James A. Storer (1977). "A note on the complexity of the superstring problem". Computer Science Laboratory, Princeton University.
- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some {NP}-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91–95. Johns Hopkins University.
- **[Yannakakis and Gavril, 1978]**: [`Yannakakis and Gavril1978`] Mihalis Yannakakis and Fanica Gavril (1978). "Edge dominating sets in graphs".
- **[Poljak, 1974]**: [`Poljak1974`] S. Poljak (1974). "A note on stable sets and colorings of graphs". *Commentationes Mathematicae Universitatis Carolinae* 15, pp. 307–309.
- **[Harary, 1969]**: [`Harary1969`] F. Harary (1969). "Graph Theory". Addison-Wesley, Reading, MA.
- **[Gavril, 1972]**: [`Gavril1972`] F. Gavril (1972). "Algorithms for minimum coloring, maximum clique, minimum covering by cliques, and maximum independent set of a chordal graph". *SIAM Journal on Computing* 1, pp. 180–187.
- **[Gavril, 1973]**: [`Gavril1973`] F. Gavril (1973). "Algorithms for a maximum clique and a maximum independent set of a circle graph". *Networks* 3, pp. 261–273.
- **[Gavril, 1974a]**: [`Gavril1974a`] F. Gavril (1974). "Algorithms on circular-arc graphs". *Networks* 4, pp. 357–369.
- **[Golumbic, 1977]**: [`Golumbic1977`] M. C. Golumbic (1977). "The complexity of comparability graph recognition and coloring". *Computing* 18, pp. 199–208.
- **[Minty, 1977]**: [`Minty1977`] George J. Minty (1977). "On maximal independent sets of vertices in claw-free graphs".