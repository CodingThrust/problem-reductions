---
name: Rule
about: Propose a new reduction rule
title: "[Rule] EXACT COVER to GENERALIZED INSTANT INSANITY"
labels: rule
assignees: ''
---

**Source:** EXACT COVER
**Target:** GENERALIZED INSTANT INSANITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.258

## GJ Source Entry

> [GP15] GENERALIZED INSTANT INSANITY
> INSTANCE: Finite set C of "colors" and a set Q of cubes, with |Q| = |C| and with each side of each cube in Q having some assigned color from C.
> QUESTION: Can the cubes in Q be stacked in one vertical column such that each of the colors in C appears exactly once on each of the four sides of the column?
> Reference: [Robertson and Munro, 1978]. Transformation from EXACT COVER.
> Comment: The associated two-person game, in which players alternate placing a new cube on the stack, with player 1 trying to construct a stack as specified above and player 2 trying to prevent this, is PSPACE-complete with respect to whether the first player has a forced win. INSTANT INSANITY is a trade name of Parker Brothers, Inc.

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

- **[Robertson and Munro, 1978]**: [`Robertson1978b`] E. Robertson and I. Munro (1978). "{NP}-completeness, puzzles, and games". *Utilitas Mathematica* 13, pp. 99–116.