---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to GEOMETRIC TRAVELING SALESMAN"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** GEOMETRIC TRAVELING SALESMAN
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND23, p.212

## GJ Source Entry

> [ND23] GEOMETRIC TRAVELING SALESMAN
> INSTANCE: Set P⊆Z×Z of points in the plane, positive integer B.
> QUESTION: Is there a tour of length B or less for the TRAVELING SALESMAN instance with C=P and d((x_1,y_1),(x_2,y_2)) equal to the discretized Euclidean distance ⌊((x_1−x_2)^2+(y_1−y_2)^2)^(1/2)⌋?
> Reference: [Papadimitriou, 1977] [Garey, Graham, and Johnson, 1976]. Transformation from X3C.
> Comment: NP-complete in the strong sense. Remains NP-complete in the strong sense if the distance measure is replaced by the L_1 "rectilinear" metric [Garey, Graham, and Johnson, 1976] or the L_∞ metric, which is equivalent to L_1 under a 45° rotation. Problem remains NP-hard in the strong sense if the (nondiscretized) Euclidean metric is used, but is not known to be in NP [Garey, Graham, and Johnson, 1976].

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

- **[Papadimitriou, 1977]**: [`Papadimitriou1977`] Christos H. Papadimitriou (1977). "The {Euclidean} traveling salesman problem is {NP}-complete". *Theoretical Computer Science* 4, pp. 237–244.
- **[Garey, Graham, and Johnson, 1976]**: [`Garey1976a`] M. R. Garey and R. L. Graham and D. S. Johnson (1976). "Some {NP}-complete geometric problems". In: *Proceedings of the 8th Annual ACM Symposium on Theory of Computing*, pp. 10–22. Association for Computing Machinery.