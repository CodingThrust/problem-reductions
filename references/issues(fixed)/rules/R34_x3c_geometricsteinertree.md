---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to GEOMETRIC STEINER TREE"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** GEOMETRIC STEINER TREE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND13, p.209

## GJ Source Entry

> [ND13] GEOMETRIC STEINER TREE
> INSTANCE: Set P⊆Z×Z of points in the plane, positive integer K.
> QUESTION: Is there a finite set Q⊆Z×Z such that there is a spanning tree of total weight K or less for the vertex set P∪Q, where the weight of an edge {(x_1,y_1),(x_2,y_2)} is the discretized Euclidean length ⌊((x_1−x_2)^2+(y_1−y_2)^2)^(1/2)⌋?
> Reference: [Garey, Graham, and Johnson, 1977]. Transformation from X3C.
> Comment: NP-complete in the strong sense. Remains so if the distance measure is replaced by the L_1 "rectilinear" metric, |x_1−x_2|+|y_1−y_2|, [Garey and Johnson, 1977a] or the L_∞ metric, max{|x_1−x_2|,|y_1−y_2|}, which is equivalent to L_1 under a 45° rotation. Problem remains NP-hard in the strong sense if the (nondiscretized) Euclidean metric ((x_1−x_2)^2+(y_1−y_2)^2)^(1/2) is used, but is not known to be in NP [Garey, Graham, and Johnson, 1977]. Some polynomial time algorithms for special cases of the rectilinear case are presented in [Aho, Garey, and Hwang, 1977].

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

- **[Garey, Graham, and Johnson, 1977]**: [`Garey1977b`] M. R. Garey and R. L. Graham and D. S. Johnson (1977). "The complexity of computing {Steiner} minimal trees". *SIAM Journal on Applied Mathematics* 32, pp. 835–859.
- **[Garey and Johnson, 1977a]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826–834.
- **[Aho, Garey, and Hwang, 1977]**: [`Aho1977a`] A. V. Aho and M. R. Garey and F. K. Hwang (1977). "Rectilinear {Steiner} trees: efficient special case algorithms". *Networks* 7, pp. 37–58.