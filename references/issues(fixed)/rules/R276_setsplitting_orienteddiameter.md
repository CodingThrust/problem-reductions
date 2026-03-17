---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SET SPLITTING to ORIENTED DIAMETER"
labels: rule
assignees: ''
---

**Source:** SET SPLITTING
**Target:** ORIENTED DIAMETER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT64

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION: Can the edges of G be directed in such a way that the resulting directed graph is strongly connected and has diameter no more than K?
>
> Reference: [Chvátal and Thomassen, 1978]. Transformation from SET SPLITTING.
> Comment: The variation in which "diameter" is replaced by "radius" is also NP-complete. Both problems remain NP-complete for K = 2.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Chvátal and Thomassen, 1978]**: [`Chvatal1978a`] V. Chv{\'a}tal and G. Thomassen (1978). "Distances in orientations of graphs". *Journal of Combinatorial Theory, Series B* 24, pp. 61–75.