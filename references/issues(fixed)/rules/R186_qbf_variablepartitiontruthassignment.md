---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to VARIABLE PARTITION TRUTH ASSIGNMENT"
labels: rule
assignees: ''
---

**Source:** QBF
**Target:** VARIABLE PARTITION TRUTH ASSIGNMENT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.255

## GJ Source Entry

> [GP5] VARIABLE PARTITION TRUTH ASSIGNMENT (*)
> INSTANCE: A set U of variables and a collection C of clauses over U.
> QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate choosing a variable from U until all variables have been chosen. Player 1 wins if and only if a satisfying truth assignment for C is obtained by setting "true" all variables chosen by player 1 and setting "false" all variables chosen by player 2.
> Reference: [Schaefer, 1978a]. Transformation from QBF.
> Comment: PSPACE-complete, even if each clause consists only of un-negated literals (i.e., contains no literals of the form ū for u E U). Analogous results for several other games played on logical expressions can be found in the reference.

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