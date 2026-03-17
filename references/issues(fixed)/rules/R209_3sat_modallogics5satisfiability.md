---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MODAL LOGIC S5-SATISFIABILITY"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** MODAL LOGIC S5-SATISFIABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.262

## GJ Source Entry

> [LO13] MODAL LOGIC S5-SATISFIABILITY
> INSTANCE: Well-formed modal formula A over a finite set U of variables, where a modal formula is either a variable u E U or is of the form "(A ∧ B)," "¬A," or "□A," where A and B are modal formulas.
> QUESTION: Is A "S5-satisfiable," i.e., is there a model (W,R,V), where W is a set, R is a reflexive, transitive, and symmetric binary relation on W, and V is a mapping from U×W into {T,F} such that, for some w E W, V(A,w) = T, where V is extended to formulas by V(A ∧ B,w) = T if and only if V(A,w) = V(B,w) = T, V(¬A,w) = T if and only if V(A,w) = F, and V(□A,w) = T if and only if V(A,w') = T for all w' E W satisfying (w,w') E R ?
> Reference: [Ladner, 1977]. Transformation from 3SAT. Nontrivial part is proving membership in NP.

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

- **[Ladner, 1977]**: [`Ladner1977`] Richard E. Ladner (1977). "The computational complexity of provability in systems of modal propositional logic". *SIAM Journal on Computing* 6, pp. 467–480.