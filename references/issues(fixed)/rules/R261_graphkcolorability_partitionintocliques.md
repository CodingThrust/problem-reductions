---
name: Rule
about: Propose a new reduction rule
title: "[Rule] GRAPH K-COLORABILITY to PARTITION INTO CLIQUES"
labels: rule
assignees: ''
---

**Source:** GRAPH K-COLORABILITY
**Target:** PARTITION INTO CLIQUES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT15

## GJ Source Entry

> [GT15] PARTITION INTO CLIQUES
> INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION: Can the vertices of G be partitioned into k ≤ K disjoint sets V_1, V_2, . . . , V_k such that, for 1 ≤ i ≤ k, the subgraph induced by V_i is a complete graph?
> Reference: [Karp, 1972] (there called CLIQUE COVER). Transformation from GRAPH K-COLORABILITY.
> Comment: Remains NP-complete for edge graphs [Arjomandi, 1977], for graphs containing no complete subgraphs on 4 vertices (see construction for PARTITION INTO TRIANGLES in Chapter 3), and for all fixed K ≥ 3. Solvable in polynomial time for K ≤ 2, for graphs containing no complete subgraphs on 3 vertices (by matching), for circular arc graphs (given their representations as families of arcs) [Gavril, 1974a], for chordal graphs [Gavril, 1972], and for comparability graphs [Golumbic, 1977].

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
- **[Arjomandi, 1977]**: [`Arjomandi1977`] E. Arjomandi (1977). "".
- **[Gavril, 1974a]**: [`Gavril1974a`] F. Gavril (1974). "Algorithms on circular-arc graphs". *Networks* 4, pp. 357–369.
- **[Gavril, 1972]**: [`Gavril1972`] F. Gavril (1972). "Algorithms for minimum coloring, maximum clique, minimum covering by cliques, and maximum independent set of a chordal graph". *SIAM Journal on Computing* 1, pp. 180–187.
- **[Golumbic, 1977]**: [`Golumbic1977`] M. C. Golumbic (1977). "The complexity of comparability graph recognition and coloring". *Computing* 18, pp. 199–208.