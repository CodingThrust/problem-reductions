---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CLIQUE"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** CLIQUE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT19

## GJ Source Entry

> [GT19]  CLIQUE
> INSTANCE:  Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION:  Does G contain a clique of size K or more, i.e., a subset V' ⊆ V with |V'| ≥ K such that every two vertices in V' are joined by an edge in E?
>
> Reference:  [Karp, 1972]. Transformation from VERTEX COVER (see Chapter 3).
> Comment:  Solvable in polynomial time for graphs obeying any fixed degree bound d, for planar graphs, for edge graphs, for chordal graphs [Gavril, 1972], for comparability graphs [Even, Pnueli, and Lempel, 1972], for circle graphs [Gavril, 1973], and for circular arc graphs (given their representation as families of arcs) [Gavril, 1974a]. The variant in which, for a given r, 0 < r < 1, we are asked whether G contains a clique of size r|V| or more is NP-complete for any fixed value of r.

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
- **[Gavril, 1972]**: [`Gavril1972`] F. Gavril (1972). "Algorithms for minimum coloring, maximum clique, minimum covering by cliques, and maximum independent set of a chordal graph". *SIAM Journal on Computing* 1, pp. 180–187.
- **[Even, Pnueli, and Lempel, 1972]**: [`Even1972`] S. Even and A. Pnueli and A. Lempel (1972). "Permutation graphs and transitive graphs". *Journal of the Association for Computing Machinery* 19, pp. 400–410.
- **[Gavril, 1973]**: [`Gavril1973`] F. Gavril (1973). "Algorithms for a maximum clique and a maximum independent set of a circle graph". *Networks* 3, pp. 261–273.
- **[Gavril, 1974a]**: [`Gavril1974a`] F. Gavril (1974). "Algorithms on circular-arc graphs". *Networks* 4, pp. 357–369.