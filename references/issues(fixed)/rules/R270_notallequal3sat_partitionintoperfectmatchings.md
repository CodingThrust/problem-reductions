---
name: Rule
about: Propose a new reduction rule
title: "[Rule] NOT-ALL-EQUAL 3SAT to PARTITION INTO PERFECT MATCHINGS"
labels: rule
assignees: ''
---

**Source:** NOT-ALL-EQUAL 3SAT
**Target:** PARTITION INTO PERFECT MATCHINGS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT16

## GJ Source Entry

> [GT16] PARTITION INTO PERFECT MATCHINGS
> INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION: Can the vertices of G be partitioned into k ≤ K disjoints sets V_1, V_2, . . . , V_k such that, for 1 ≤ i ≤ k, the subgraph induced by V_i is a perfect matching (consists entirely of vertices with degree one)?
> Reference: [Schaefer, 1978b]. Transformation from NOT-ALL-EQUAL 3SAT.
> Comment: Remains NP-complete for K = 2 and for planar cubic graphs.

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

- **[Schaefer, 1978b]**: [`Schaefer1978b`] T. J. Schaefer (1978). "The complexity of satisfiability problems". In: *Proceedings of the 10th Annual ACM Symposium on Theory of Computing*, pp. 216–226. Association for Computing Machinery.