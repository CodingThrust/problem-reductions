---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PLANAR GEOGRAPHY to NxN CHECKERS"
labels: rule
assignees: ''
---

**Source:** PLANAR GEOGRAPHY
**Target:** NxN CHECKERS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.256

## GJ Source Entry

> [GP10] N×N CHECKERS (*)
> INSTANCE: Positive integer N, a partition of the black squares of an N×N Checkerboard into those that are empty, those that are occupied by "Black kings," and those that are occupied by "Red kings," and the identity of the player (Red or Black) whose turn it is.
> QUESTION: Does Black have a forced win from the given position in a game of Checkers played according to the standard rules, modified only to take into account the expanded board and number of pieces?
> Reference: [Fraenkel, Garey, Johnson, Schaefer, and Yesha, 1978]. Transformation from PLANAR GEOGRAPHY.
> Comment: PSPACE-hard, and PSPACE-complete for certain drawing rules. The related problem in which we ask whether Black can jump all of Red's pieces in one turn is solvable in polynomial time.

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

- **[Fraenkel, Garey, Johnson, Schaefer, and Yesha, 1978]**: [`Fraenkel1978`] A. S. Fraenkel and M. R. Garey and D. S. Johnson and T. Schaefer and Y. Yesha (1978). "The complexity of {Checkers} on an {N}$\times${N} board --- {Preliminary} report". In: *Proceedings of the 19th Annual Symposium on Foundations of Computer Science*, pp. 55–64. IEEE Computer Society.