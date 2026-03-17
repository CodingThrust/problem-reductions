---
name: Rule
about: Propose a new reduction rule
title: "[Rule] DOMINATING SET to MIN-MAX MULTICENTER"
labels: rule
assignees: ''
---

**Source:** DOMINATING SET
**Target:** MIN-MAX MULTICENTER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND50, p.220

## GJ Source Entry

> [ND50] MIN-MAX MULTICENTER
> INSTANCE: Graph G=(V,E), weight w(v)∈Z_0^+ for each v∈V, length l(e)∈Z_0^+ for each e∈E, positive integer K≤|V|, positive rational number B.
> QUESTION: Is there a set P of K "points on G" (where a point on G can be either a vertex in V or a point on an edge e∈E, with e regarded as a line segment of length l(e)) such that if d(v) is the length of the shortest path from v to the closest point in P, then max{d(v)·w(v): v∈V}≤B?
> Reference: [Kariv and Hakimi, 1976a]. Transformation from DOMINATING SET.
> Comment: Also known as the "p-center" problem. Remains NP-complete if w(v)=1 for all v∈V and l(e)=1 for all e∈E. Solvable in polynomial time for any fixed K and for arbitrary K if G is a tree [Kariv and Hakimi, 1976a]. Variant in which we must choose a subset P⊆V is also NP-complete but solvable for fixed K and for trees [Slater, 1976].

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

- **[Kariv and Hakimi, 1976a]**: [`Kariv1976a`] Oded Kariv and S. Louis Hakimi (1976). "An algorithmic approach to network location problems -- {Part I}: the p-centers".
- **[Slater, 1976]**: [`Slater1976`] Peter J. Slater (1976). "{$R$}-domination in graphs". *Journal of the Association for Computing Machinery* 23, pp. 446–450.