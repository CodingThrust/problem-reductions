---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PLANAR 3SAT to GEOMETRIC CONNECTED DOMINATING SET"
labels: rule
assignees: ''
---

**Source:** PLANAR 3SAT
**Target:** GEOMETRIC CONNECTED DOMINATING SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND48, p.219

## GJ Source Entry

> [ND48] GEOMETRIC CONNECTED DOMINATING SET
> INSTANCE: Set P⊆Z×Z of points in the plane, positive integers B and K.
> QUESTION: Is there a subset P'⊆P with |P'|≤K such that all points in P−P' are within Euclidean distance B of some point in P', and such that the graph G=(P',E), with an edge between two points in P' if and only if they are within distance B of each other, is connected?
> Reference: [Lichtenstein, 1977]. Transformation from PLANAR 3SAT.
> Comment: Remains NP-complete if the Euclidean metric is replaced by the L_1 rectilinear metric or the L_∞ metric [Garey and Johnson, ——].

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

- **[Lichtenstein, 1977]**: [`Lichtenstein1977`] David Lichtenstein (1977). "Planar satisfiability and its uses". *SIAM Journal on Computing*.
- **[Garey and Johnson, ——]**: *(not found in bibliography)*