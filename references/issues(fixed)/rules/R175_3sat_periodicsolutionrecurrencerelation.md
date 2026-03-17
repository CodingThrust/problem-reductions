---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PERIODIC SOLUTION RECURRENCE RELATION"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** PERIODIC SOLUTION RECURRENCE RELATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.251

## GJ Source Entry

> [AN12] PERIODIC SOLUTION RECURRENCE RELATION (*)
> INSTANCE: Ordered pairs (c_i, b_i), 1 ≤ i ≤ m, of integers, with all b_i positive.
> QUESTION: Is there a sequence a_0,a_1,...,a_{n-1} of integers, with n ≥ max{b_i}, such that the infinite sequence a_0,a_1,... defined by the recurrence relation
>
> a_i = Σ_{j=1}^m c_j·a_{(i-b_j)}
>
> satisfies a_i ≡ a_{i(mod n)}, for all i ≥ n?
> Reference: [Plaisted, 1977b]. Tranformation from 3SAT
> Comment: Not known to be in NP or co-NP. See reference for related results.

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

- **[Plaisted, 1977b]**: [`Plaisted1977b`] D. Plaisted (1977). "New {NP}-hard and {NP}-complete polynomial and integer divisibility problems". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 241–253. IEEE Computer Society.