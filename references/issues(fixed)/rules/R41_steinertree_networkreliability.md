---
name: Rule
about: Propose a new reduction rule
title: "[Rule] STEINER TREE IN GRAPHS to NETWORK RELIABILITY"
labels: rule
assignees: ''
---

**Source:** STEINER TREE IN GRAPHS
**Target:** NETWORK RELIABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND20, p.211

## GJ Source Entry

> [ND20] NETWORK RELIABILITY (*)
> INSTANCE: Graph G=(V,E), subset V'⊆V, a rational "failure probability" p(e), 0≤p(e)≤1, for each e∈E, a positive rational number q≤1.
> QUESTION: Assuming edge failures are independent of one another, is the probability q or greater that each pair of vertices in V' is joined by at least one path containing no failed edge?
> Reference: [Rosenthal, 1974]. Transformation from STEINER TREE IN GRAPHS.
> Comment: Not known to be in NP. Remains NP-hard even if |V'|=2 [Valiant, 1977b]. The related problem in which we want two disjoint paths between each pair of vertices in V' is NP-hard even if V'=V [Ball, 1977b]. If G is directed and we ask for a directed path between each ordered pair of vertices in V', the one-path problem is NP-hard for both |V'|=2 [Valiant, 1977b] and V'=V [Ball, 1977a]. Many of the underlying subgraph enumeration problems are #P-complete (see [Valiant, 1977b]).

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

- **[Rosenthal, 1974]**: [`Rosenthal1974`] A. Rosenthal (1974). "Computing Reliability of Complex Systems". University of California.
- **[Valiant, 1977b]**: [`Valiant1977b`] Leslie G. Valiant (1977). "The complexity of enumeration and reliability problems". Computer Science Dept., University of Edinburgh.
- **[Ball, 1977b]**: [`Ball1977b`] M. O. Ball (1977). "".
- **[Ball, 1977a]**: [`Ball1977a`] M. O. Ball (1977). "Network Reliability and Analysis: Algorithms and Complexity". Cornell University.