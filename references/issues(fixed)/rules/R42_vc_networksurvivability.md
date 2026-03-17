---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to NETWORK SURVIVABILITY"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** NETWORK SURVIVABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND21, p.211

## GJ Source Entry

> [ND21] NETWORK SURVIVABILITY (*)
> INSTANCE: Graph G=(V,E), a rational "failure probability" p(x), 0≤p(x)≤1, for each x∈V∪E, a positive rational number q≤1.
> QUESTION: Assuming all edge and vertex failures are independent of one another, is the probability q or greater that for all {u,v}∈E at least one of u, v, or {u,v} will fail?
> Reference: [Rosenthal, 1974]. Transformation from VERTEX COVER.
> Comment: Not known to be in NP.

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