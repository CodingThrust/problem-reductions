---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to INEQUIVALENCE OF PROGRAMS WITH ASSIGNMENTS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** INEQUIVALENCE OF PROGRAMS WITH ASSIGNMENTS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO12

## GJ Source Entry

> [PO12]  INEQUIVALENCE OF PROGRAMS WITH ASSIGNMENTS
> INSTANCE:  Finite set X of variables, two programs P1 and P2, each a sequence of assignments of the form "x0 ← if x1=x2 then x3 else x4" where the xi are in X, and a value set V.
> QUESTION:  Is there an initial assignment of a value from V to each variable in X such that the two programs yield different final values for some variable in X (see reference for details on the execution of such programs)?
> Reference:  [Downey and Sethi, 1976]. Transformation from 3SAT.
> Comment:  Remains NP-complete for V={0,1}.  This problem can be embedded in many inequivalence problems for simple programs, thus rendering them NP-hard [Downey and Sethi, 1976], [van Leeuwen, 1977].

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

- **[Downey and Sethi, 1976]**: [`Downey1976`] P. J. Downey and R. Sethi (1976). "Assignment commands and array structures". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 57–66. IEEE Computer Society.
- **[van Leeuwen, 1977]**: [`van Leeuwen1977`] Jan van Leeuwen (1977). "Inequivalence of program-segments and {NP}-completeness". Computer Science Dept., Pennsylvania State University.