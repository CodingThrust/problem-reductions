---
name: Rule
about: Propose a new reduction rule
title: "[Rule] (generic transformation) to FIRST ORDER THEORY OF EQUALITY"
labels: rule
assignees: ''
---

**Source:** (generic transformation)
**Target:** FIRST ORDER THEORY OF EQUALITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.262

## GJ Source Entry

> [LO12] FIRST ORDER THEORY OF EQUALITY (*)
> INSTANCE: Finite set U = {u_1,u_2,...,u_n} of variables, sentence S over U in the first order theory of equality. (Such sentences can be defined inductively as follows: An "expression" is of the form "u=v" where u,v E U, or of the form "¬E," "(E V F)," "(E ∧ F)," or "(E → F)" where E and F are expressions. A sentence is of the form (Q_1 u_1)(Q_2 u_2) ··· (Q_n u_n)E where E is an expression and each Q_i is either ∀ or ∃.)
> QUESTION: Is S true in all models of the theory?
> Reference: [Stockmeyer and Meyer, 1973]. Generic transformation.
> Comment: PSPACE-complete. The analogous problem for any fixed first order theory that has a model in which some predicate symbol is interpreted as a relation that holds sometimes but not always is PSPACE-hard [Hunt, 1977].

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

- **[Stockmeyer and Meyer, 1973]**: [`Stockmeyer and Meyer1973`] Larry J. Stockmeyer and Albert R. Meyer (1973). "Word problems requiring exponential time". In: *Proc. 5th Ann. ACM Symp. on Theory of Computing*, pp. 1–9. Association for Computing Machinery.
- **[Hunt, 1977]**: [`Hunt1977a`] Harry B. Hunt III (1977). "A complexity theory of computation structures: preliminary report".