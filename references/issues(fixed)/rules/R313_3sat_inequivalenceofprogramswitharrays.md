---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to INEQUIVALENCE OF PROGRAMS WITH ARRAYS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** INEQUIVALENCE OF PROGRAMS WITH ARRAYS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO11

## GJ Source Entry

> [PO11]  INEQUIVALENCE OF PROGRAMS WITH ARRAYS
> INSTANCE:  Finite sets X, Θ, and R of variables, operators, and array variables, two programs P1 and P2 made up of "operate" (x0 ← θx1x2 ··· xr), "update" (α[xi] ← xj), and "select" (xi ← α[xj]) commands, where each xi ∈ X, θ ∈ Θ, r is the "arity" of θ, and α ∈ R, a finite value set V, and an interpretation of each operator θ ∈ Θ as a specific function from Vr to V.
> QUESTION:  Is there an initial assignment of a value from V to each variable in X such that the two programs yield different final values for some variable in X (see reference for details on the execution of such programs)?
> Reference:  [Downey and Sethi, 1976]. Transformation from 3SAT.
> Comment:  Remains NP-complete even if there are no operate commands and only one array variable.  Solvable in polynomial time if there are no update commands, or no select commands, or no array variables.

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