---
name: Rule
about: Propose a new reduction rule
title: "[Rule] (generic transformation) to QUANTIFIED BOOLEAN FORMULAS (QBF)"
labels: rule
assignees: ''
---

**Source:** (generic transformation)
**Target:** QUANTIFIED BOOLEAN FORMULAS (QBF)
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.261-262

## GJ Source Entry

> [LO11] QUANTIFIED BOOLEAN FORMULAS (QBF) (*)
> INSTANCE: Set U = {u_1,u_2,...,u_n} of variables, well-formed quantified Boolean formula F = (Q_1 u_1)(Q_2 u_2) ··· (Q_n u_n)E, where E is a Boolean expression and each Q_i is either ∀ or ∃.
> QUESTION: Is F true?
> Reference: [Stockmeyer and Meyer, 1973]. Generic transformation.
> Comment: PSPACE-complete, even if E is in conjunctive normal form with three literals per clause (QUANTIFIED 3SAT), but solvable in polynomial time when there are at most two literals per clause [Schaefer, 1978b]. If F is restricted to at most k alternations of quantifiers (i.e., there are at most k indices i such that Q_i ≠ Q_{i+1}), then the restricted problem is complete for some class in the polynomial hierarchy, depending on k and the allowed values for Q_1 (see Section 7.2).

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
- **[Schaefer, 1978b]**: [`Schaefer1978b`] T. J. Schaefer (1978). "The complexity of satisfiability problems". In: *Proceedings of the 10th Annual ACM Symposium on Theory of Computing*, pp. 216–226. Association for Computing Machinery.