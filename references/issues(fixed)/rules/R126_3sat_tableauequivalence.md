---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Tableau Equivalence"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Tableau Equivalence
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.234

## GJ Source Entry

> [SR32] TABLEAU EQUIVALENCE
> INSTANCE: A set A of attribute names, a collection F of ordered pairs of subsets of A, a set X of distinguished variables, a set Y of undistinguished variables, a set C_a of constants for each a E A, and two "tableaux" T_1 and T_2 over X, Y, and the C_a. (A tableau is essentially a matrix with a column for each attribute and entries from X, Y, the C_a, along with a blank symbol. For details and an interpretation in terms of relational expressions, see reference.)
> QUESTION: Are T_1 and T_2 "weakly equivalent," i.e., do they represent identical relations under "universal interpretations"?
> Reference: [Aho, Sagiv, and Ullman, 1978]. Transformation from 3SAT.
> Comment: Remains NP-complete even if the tableaux come from "expressions" that have no "select" operations, or if the tableaux come from expressions that have select operations but F is empty, or if F is empty, the tableaux contain no constants, and the tableaux do not necessarily come from expressions at all. Problem is solvable in polynomial time for "simple" tableaux. The same results hold also for "strong equivalence," where the two tableaux must represent identical relations under all interpretations. The problem of tableau "containment," however, is NP-complete even for simple tableaux and for still further restricted tableaux [Sagiv and Yannakakis, 1978].

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

- **[Aho, Sagiv, and Ullman, 1978]**: [`Aho1978`] A. V. Aho and Y. Sagiv and J. D. Ullman (1978). "Equivalences among relational expressions".
- **[Sagiv and Yannakakis, 1978]**: [`Sagiv1978`] Y. Sagiv and M. Yannakakis (1978). "Equivalence among relational expressions with the union and difference operations". Dept. of Electrical Engineering and Computer Science, Princeton University.