---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to GEOMETRIC STEINER TREE"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
specialization_note: 'X3C is a specialization of Set Covering. Implement general version first.'
---

# [Rule] X3C → GEOMETRIC STEINER TREE

**Status:** SKIP_SPECIALIZATION

X3C (Exact Cover by 3-Sets) is a known specialization of Set Covering (each set has exactly 3 elements, and an exact cover is required). This reduction should be implemented after the general version is available in the codebase.

## Specialization Details

- **Specialized problem:** X3C (Exact Cover by 3-Sets)
- **General version:** Set Covering
- **Restriction:** Each set has exactly 3 elements; an exact cover (every element covered exactly once) is required

## Original Reference

**Reference:** Garey & Johnson, *Computers and Intractability*, ND13, p.209

> [ND13] GEOMETRIC STEINER TREE
> INSTANCE: Set P⊆Z×Z of points in the plane, positive integer K.
> QUESTION: Is there a finite set Q⊆Z×Z such that there is a spanning tree of total weight K or less for the vertex set P∪Q, where the weight of an edge {(x_1,y_1),(x_2,y_2)} is the discretized Euclidean length ⌊((x_1−x_2)^2+(y_1−y_2)^2)^(1/2)⌋?
> Reference: [Garey, Graham, and Johnson, 1977]. Transformation from X3C.
> Comment: NP-complete in the strong sense. Remains so if the distance measure is replaced by the L_1 "rectilinear" metric [Garey and Johnson, 1977a] or the L_∞ metric, which is equivalent to L_1 under a 45° rotation.

## References

- **[Garey, Graham, and Johnson, 1977]**: [`Garey1977b`] M. R. Garey and R. L. Graham and D. S. Johnson (1977). "The complexity of computing {Steiner} minimal trees". *SIAM Journal on Applied Mathematics* 32, pp. 835–859.
- **[Garey and Johnson, 1977a]**: [`Garey1977c`] M. R. Garey and D. S. Johnson (1977). "The rectilinear {Steiner} tree problem is {NP}-complete". *SIAM Journal on Applied Mathematics* 32, pp. 826–834.
- **[Aho, Garey, and Hwang, 1977]**: [`Aho1977a`] A. V. Aho and M. R. Garey and F. K. Hwang (1977). "Rectilinear {Steiner} trees: efficient special case algorithms". *Networks* 7, pp. 37–58.
