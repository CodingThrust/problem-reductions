---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to GEOMETRIC CAPACITATED SPANNING TREE"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** GEOMETRIC CAPACITATED SPANNING TREE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND6, p.207

## GJ Source Entry

> [ND6] GEOMETRIC CAPACITATED SPANNING TREE
> INSTANCE: Set P ⊆ Z×Z of points in the plane, specified point p_0∈P, requirement r(p)∈Z_0^+ for each p∈P−p_0, capacity c∈Z^+, bound B∈Z^+.
> QUESTION: Is there a spanning tree T=(P,E') for the complete graph G=(P,E) such that ∑_{e∈E'} d(e)≤B, where d((x_1,y_1),(x_2,y_2)) is the discretized Euclidean distance [((x_1−x_2)^2+(y_1−y_2)^2)^½], and such that for each e∈E', if U(e) is the set of vertices whose paths to p_0 pass through e, then ∑_{u∈U(e)} r(u)≤c?
> Reference: [Papadimitriou, 1976c]. Transformation from X3C.
> Comment: Remains NP-complete even if all requirements are equal.

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

- **[Papadimitriou, 1976c]**: [`Papadimitriou1976c`] Christos H. Papadimitriou (1976). "The complexity of the capacitated tree problem". Center for Research in Computing Technology, Harvard University.