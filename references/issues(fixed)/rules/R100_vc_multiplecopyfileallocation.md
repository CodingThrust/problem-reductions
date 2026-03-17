---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Multiple Copy File Allocation"
labels: rule
assignees: ''
---

**Source:** Vertex Cover
**Target:** Multiple Copy File Allocation
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.1, p.227

## GJ Source Entry

> [SR6] MULTIPLE COPY FILE ALLOCATION
> INSTANCE: Graph G = (V,E), for each v E V a usage u(v) E Z+ and a storage cost s(v) E Z+, and a positive integer K.
> QUESTION: Is there a subset V' ⊆ V such that, if for each v E V we let d(v) denote the number of edges in the shortest path in G from v to a member of V', we have
>
> sum_{v E V'} s(v) + sum_{v E V} d(v)*u(v) <= K ?
>
> Reference: [Van Sickle and Chandy, 1977]. Transformation from VERTEX COVER.
> Comment: NP-complete in the strong sense, even if all v E V have the same value of u(v) and the same value of s(v).

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

- **[Van Sickle and Chandy, 1977]**: [`van Sickle and Chandy1977`] Larry van Sickle and K. Mani Chandy (1977). "The complexity of computer network design problems".