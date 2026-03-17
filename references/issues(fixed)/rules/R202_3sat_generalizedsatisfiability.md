---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to GENERALIZED SATISFIABILITY"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** GENERALIZED SATISFIABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.1, p.260

## GJ Source Entry

> [LO6] GENERALIZED SATISFIABILITY
> INSTANCE: Positive integers k_1,k_2,...,k_m, sequence S = <R_1,R_2,...,R_m> of subsets R_i ⊆ {T,F}^{k_i}, set U of variables, and, for 1 ≤ i ≤ m, a collection C_i of k_i-tuples of variables from U.
> QUESTION: Is there a truth assignment t: U → {T,F} such that for all i, 1 ≤ i ≤ m, and for all k_i-tuples (u[1],u[2],...,u[k_i]) in C_i, we have
>
> (t(u[1]),t(u[2]),...,t(u[k_i])) E R_i?
>
> Reference: [Schaefer, 1978b]. Transformation from 3SAT.
> Comment: For any fixed sequence S, the problem is NP-complete unless one of the following six alternatives holds, in which case the problem with that S is solvable in polynomial time:
> (1) Each R_i contains {T}^{k_i},
> (2) each R_i contains {F}^{k_i},
> (3) each R_i is logically "equivalent" to some conjunctive normal form expression having at most one negated literal per clause,
> (4) each R_i is logically "equivalent" to some conjunctive normal form expression having at most one un-negated literal per clause,
> (5) each R_i is logically "equivalent" to some conjunctive normal form expression having at most 2 literals per clause, or
> (6) each R_i is the "solution set" for some system of linear equations over GF[2].
>
> The NP-completeness of 3SAT, ONE-IN-THREE 3SAT, and NOT-ALL-EQUAL 3SAT all follow from this classification. If the tuples in each C_i are allowed to be in (U ∪ {T,F})^{k_i} ("formulas with constants"), the problem is NP-complete even if (1) or (2) holds, but is still polynomially solvable if (3), (4), (5), or (6) holds. The quantified version of the problem "with constants," where we are also given a sequence Q_1,Q_2,...,Q_n of quantifiers (each Q_i being either ∀ or ∃) and ask if
>
> (Q_1 u_1)(Q_2 u_2)···(Q_n u_n) [c E R_i for all c E C_i, 1 ≤ i ≤ m]
>
> is PSPACE-complete, even for fixed S, so long as S does not meet any of (3), (4), (5), or (6), and is solvable in polynomial time for any fixed S that does meet one of (3), (4), (5), or (6).

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

- **[Schaefer, 1978b]**: [`Schaefer1978b`] T. J. Schaefer (1978). "The complexity of satisfiability problems". In: *Proceedings of the 10th Annual ACM Symposium on Theory of Computing*, pp. 216–226. Association for Computing Machinery.