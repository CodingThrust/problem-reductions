---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to ALTERNATING HITTING SET"
labels: rule
assignees: ''
---

**Source:** QBF
**Target:** ALTERNATING HITTING SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.255

## GJ Source Entry

> [GP7] ALTERNATING HITTING SET (*)
> INSTANCE: A collection C of subsets of a basic set B.
> QUESTION: Does player 1 have a forced win in the following game played on C and B? Players alternate choosing a new element of B until, for each c E C, some member of c has been chosen. The player whose choice causes this to happen loses.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete even if no set in C contains more than two elements, a subcase of the original HITTING SET problem that can be solved in polynomial time. If the roles of winner and loser are reversed, the problem is PSPACE-complete even if no set in C contains more than three elements.

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