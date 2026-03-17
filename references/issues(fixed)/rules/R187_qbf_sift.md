---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to SIFT"
labels: rule
assignees: ''
---

**Source:** QBF
**Target:** SIFT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.255

## GJ Source Entry

> [GP6] SIFT (*)
> INSTANCE: Two collections A and B of subsets of a finite set X, with A and B having no subsets in common.
> QUESTION: Does player 1 have a forced win in the following game played on A, B, and X? Players alternate choosing an element from X until the set X' of all elements chosen so far either intersects all the subsets in A or intersects all the subsets in B. Player 1 wins if and only if the final set X' of chosen elements intersects all the subsets in B and, if player 1 made the last move, does not intersect all subsets in A.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete.

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

- **[Schaefer, 1978a]**: [`Schaefer1978a`] T. J. Schaefer (1978). "Complexity of some two-person perfect-information games". *Journal of Computer and System Sciences* 16, pp. 185–225.