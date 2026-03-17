---
name: Rule
about: Propose a new reduction rule
title: "[Rule] OPTIMAL LINEAR ARRANGEMENT to INTERVAL GRAPH COMPLETION"
labels: rule
assignees: ''
---

**Source:** OPTIMAL LINEAR ARRANGEMENT
**Target:** INTERVAL GRAPH COMPLETION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT35

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), non-negative integer K.
> QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') is an interval graph?
>
> Reference: [Garey, Gavril, and Johnson, 1977]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
>
> Comment: Remains NP-complete when G is restricted to be an edge graph. Solvable in polynomial time for K = 0 [Fulkerson and Gross, 1965],[Booth and Lueker, 1976].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Garey, Gavril, and Johnson, 1977]**: [`Garey1977a`] M. R. Garey and F. Gavril and D. S. Johnson (1977). "Unpublished results".
- **[Fulkerson and Gross, 1965]**: [`Fulkerson1965`] D. R. Fulkerson and D. A. Gross (1965). "Incidence matrices and interval graphs". *Pacific Journal of Mathematics* 15, pp. 835–855.
- **[Booth and Lueker, 1976]**: [`Booth1976`] K. S. Booth and G. S. Lueker (1976). "Testing for the consecutive ones property, interval graphs, and graph planarity using {PQ}-tree algorithms". *Journal of Computer and System Sciences* 13, pp. 335–379.