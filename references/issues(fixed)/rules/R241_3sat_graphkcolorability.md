---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to GRAPH K-COLORABILITY"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** GRAPH K-COLORABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT4

## GJ Source Entry

> [GT4]  GRAPH K-COLORABILITY  (CHROMATIC NUMBER)
> INSTANCE:  Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION:  Is G K-colorable, i.e., does there exist a function f: V→{1,2, . . . ,K} such that f(u) ≠ f(v) whenever {u,v} ∈ E?
>
> Reference:  [Karp, 1972]. Transformation from 3SAT.
> Comment:  Solvable in polynomial time for K = 2, but remains NP-complete for all fixed K ≥ 3 and, for K = 3, for planar graphs having no vertex degree exceeding 4 [Garey, Johnson, and Stockmeyer, 1976]. Also remains NP-complete for K = 3 if G is an intersection graph for straight line segments in the plane [Ehrlich, Even, and Tarjan, 1976]. For arbitrary K, the problem is NP-complete for circle graphs and circular arc graphs (even given their representation as families of arcs), although for circular arc graphs the problem is solvable in polynomial time for any fixed K (given their representation) [Garey, Johnson, Miller, and Papadimitriou, 1978]. The general problem can be solved in polynomial time for comparability graphs [Even, Pnueli, and Lempel, 1972], for chordal graphs [Gavril, 1972], for (3,1) graphs [Walsh and Burkhard, 1977], and for graphs having no vertex degree exceeding 3 [Brooks, 1941].

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
- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
- **[Ehrlich, Even, and Tarjan, 1976]**: [`Ehrlich1976`] G. Ehrlich and S. Even and R. E. Tarjan (1976). "Intersection graphs of curves in the plane". *Journal of Combinatorial Theory Series B* 21, pp. 8–20.
- **[Garey, Johnson, Miller, and Papadimitriou, 1978]**: [`Garey1978c`] M. R. Garey and D. S. Johnson and G. L. Miller and C. H. Papadimitriou (1978). "Unpublished results".
- **[Even, Pnueli, and Lempel, 1972]**: [`Even1972`] S. Even and A. Pnueli and A. Lempel (1972). "Permutation graphs and transitive graphs". *Journal of the Association for Computing Machinery* 19, pp. 400–410.
- **[Gavril, 1972]**: [`Gavril1972`] F. Gavril (1972). "Algorithms for minimum coloring, maximum clique, minimum covering by cliques, and maximum independent set of a chordal graph". *SIAM Journal on Computing* 1, pp. 180–187.
- **[Walsh and Burkhard, 1977]**: [`Walsh and Burkhard1977`] Aidan M. Walsh and Walter A. Burkhard (1977). "Efficient algorithms for (3,1) graphs". *Information Sciences* 13, pp. 1–10.
- **[Brooks, 1941]**: [`Brooks1941`] R. L. Brooks (1941). "On coloring the nodes of a network". *Proceedings of the Cambridge Philosophical Society* 37, pp. 194–197.