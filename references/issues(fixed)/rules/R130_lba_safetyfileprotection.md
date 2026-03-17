---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Linear Bounded Automaton Acceptance to Safety of File Protection Systems"
labels: rule
assignees: ''
---

**Source:** Linear Bounded Automaton Acceptance
**Target:** Safety of File Protection Systems
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.235

## GJ Source Entry

> [SR36] SAFETY OF FILE PROTECTION SYSTEMS (*)
> INSTANCE: Set R of "rights," set O of objects, set S ⊆ O of subjects, set P(s,o) ⊆ R of rights for each ordered pair s E S and o E O, a finite set C of commands, each having the form "if r_1 E P(X_1,Y_1) and r_2 E P(X_2,Y_2) and ... and r_m E P(X_m,Y_m), then θ_1, θ_2, ..., θ_n" for m,n >= 0 and each θ_i of the form "enter r_i into P(X_j,Y_k)" or "delete r_i from P(K_j,Y_k)," and a specified right r' E R.
> QUESTION: Is there a sequence of commands from C and a way of identifying each r_i, X_i, and Y_k with a particular element of R, S, and O, respectively, such that at some point in the execution of the sequence, the right r' is entered into a set P(s,o) that previously did not contain r' (see reference for details on the execution of such a sequence)?
> Reference: [Harrison, Ruzzo, and Ullman, 1976]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE.
> Comment: PSPACE-complete. Undecidable if operations that create or delete "subjects" and "objects" are allowed, even for certain "fixed" systems in which only the initial values of the P(s,o) are allowed to vary. If no command can contain more than one operation, then the problem is NP-complete in general and solvable in polynomial time for fixed systems.

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

- **[Harrison, Ruzzo, and Ullman, 1976]**: [`Harrison1976`] M. A. Harrison and W. L. Ruzzo and J. D. Ullman (1976). "Protection in operating systems". *Communications of the ACM* 19, pp. 461–471.