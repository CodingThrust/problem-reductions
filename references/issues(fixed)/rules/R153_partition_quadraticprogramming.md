---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition to Quadratic Programming"
labels: rule
assignees: ''
---

**Source:** Partition
**Target:** Quadratic Programming
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.245

## GJ Source Entry

> [MP2] QUADRATIC PROGRAMMING (*)
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of rational numbers and b is a rational number, two m-tuples c-bar and d-bar of rational numbers, and a rational number B.
> QUESTION: Is there an m-tuple y-bar of rational numbers such that x-bar·y-bar <= b for all (x-bar, b) E X and such that sum_{i=1}^{m} (c_i y_i^2 + d_i y_i) >= B, where c_i, y_i, and d_i denote the i^th components of c-bar, y-bar, and d-bar respectively?
> Reference: [Sahni, 1974]. Transformation from PARTITION.
> Comment: Not known to be in NP, unless the c_i's are all non-negative [Klee, 1978]. If the constraints are quadratic and the objective function is linear (the reverse of the situation above), then the problem is also NP-hard [Sahni, 1974]. If we add to this last problem the requirement that all entries of y-bar be integers, then the problem becomes undecidable [Jeroslow, 1973].

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

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262–279.
- **[Klee, 1978]**: [`Klee1978`] Victor Klee (1978). "Private communication".
- **[Jeroslow, 1973]**: [`Jeroslow1973`] Robert G. Jeroslow (1973). "There cannot be any algorithm for integer programming with quadratic constraints". *Operations Research* 21, pp. 221–224.