---
name: Rule
about: Propose a new reduction rule
title: "[Rule] generic to CONSTRAINED TRIANGULATION"
labels: rule
assignees: ''
---

**Source:** generic
**Target:** CONSTRAINED TRIANGULATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2.5 ND45

## GJ Source Entry

> [ND45] CONSTRAINED TRIANGULATION
> INSTANCE: Graph G = (V,E), coordinates x(v), y(v) ∈ Z for each v ∈ V.
> QUESTION: Is there a subset E' ⊆ E, such that the set of line segments {[(x(u),y(u)),(x(v),y(v))]: {u,v} ∈ E'} is a triangulation of the set of points {(x(v),y(v)): v ∈ V} in the plane?
> Reference: [Lloyd, 1977].
> Comment: NP-complete in the strong sense.

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

- **[Lloyd, 1977]**: [`Lloyd1977`] Errol L. Lloyd (1977). "On triangulations of a set of points in the plane". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 228–240. IEEE Computer Society.