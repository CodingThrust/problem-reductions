---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION INTO TRIANGLES to GRAPH PARTITIONING"
labels: rule
assignees: ''
---

**Source:** PARTITION INTO TRIANGLES
**Target:** GRAPH PARTITIONING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND14, p.209

## GJ Source Entry

> [ND14] GRAPH PARTITIONING
> INSTANCE: Graph G=(V,E), weights w(v)∈Z^+ for each v∈V and l(e)∈Z^+ for each e∈E, positive integers K and J.
> QUESTION: Is there a partition of V into disjoint sets V_1,V_2,...,V_m such that ∑_{v∈V_i} w(v)≤K for 1≤i≤m and such that if E'⊆E is the set of edges that have their two endpoints in two different sets V_i, then ∑_{e∈E'} l(e)≤J?
> Reference: [Hyafil and Rivest, 1973]. Transformation from PARTITION INTO TRIANGLES.
> Comment: Remains NP-complete for fixed K≥3 even if all vertex and edge weights are 1. Can be solved in polynomial time for K=2 by matching.

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

- **[Hyafil and Rivest, 1973]**: [`Hyafil1973`] Laurent Hyafil and Ronald L. Rivest (1973). "Graph partitioning and constructing optimal decision trees are polynomial complete problems". IRIA-Laboria.