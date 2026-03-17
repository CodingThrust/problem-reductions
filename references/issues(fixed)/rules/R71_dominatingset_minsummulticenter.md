---
name: Rule
about: Propose a new reduction rule
title: "[Rule] DOMINATING SET to MIN-SUM MULTICENTER"
labels: rule
assignees: ''
---

**Source:** DOMINATING SET
**Target:** MIN-SUM MULTICENTER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND51, p.220

## GJ Source Entry

> [ND51] MIN-SUM MULTICENTER
> INSTANCE: Graph G=(V,E), weight w(v)∈Z_0^+ for each v∈V, length l(e)∈Z_0^+ for each e∈E, positive integer K≤|V|, positive rational number B.
> QUESTION: Is there a set P of K "points on G" such that if d(v) is the length of the shortest path from v to the closest point in P, then Σ_{v∈V} d(v)·w(v)≤B?
> Reference: [Kariv and Hakimi, 1976b]. Transformation from DOMINATING SET.
> Comment: Also known as the "p-median" problem. It can be shown that there is no loss of generality in restricting P to being a subset of V. Remains NP-complete if w(v)=1 for all v∈V and l(e)=1 for all e∈E. Solvable in polynomial time for any fixed K and for arbitrary K if G is a tree.

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

- **[Kariv and Hakimi, 1976b]**: [`Kariv1976b`] Oded Kariv and S. Louis Hakimi (1976). "An algorithmic approach to network location problems -- {Part 2}: the p-medians".